//! Les Syllamons — application Windows (Tauri 2).
//! La page (tauri/www/index.html, construite par build/construire-appli.py) est la même que dans le
//! navigateur. Ce fichier ne fait que ce qu'une page ne peut pas faire seule (voir src/coeur/appli-bureau.js) :
//!  · lire et écrire dans les deux dossiers choisis une fois — le dossier de la classe et celui de la voix de
//!    l'enseignant — dont les chemins sont retenus dans le profil de l'application (ils survivent aux
//!    redémarrages et aux mises à jour) : lister, infos, lire, ecrire, creer_dossier, supprimer ;
//!  · « Enregistrer sous… » (sauvegardes, rapports, fichier de liaison) ;
//!  · plein écran de la fenêtre, ouverture d'une adresse dans le navigateur ;
//!  · se mettre à jour : la page lit appli/latest.json sur le site public et télécharge l'installateur (par le
//!    réseau de Windows : proxy et certificats de l'école compris), puis installer_paquet vérifie la signature
//!    (CLE_MAJ ; la clé privée n'est que dans le secret du dépôt public Syllamons-versions) et lance l'installation.
//! Les données de l'appli (élèves, résultats, voix) ne sont jamais dans le dossier du programme : une mise à
//! jour ne les touche pas.

use base64::{engine::general_purpose::STANDARD, Engine};
use minisign_verify::{PublicKey, Signature};
use percent_encoding::percent_decode_str;
use serde::Serialize;
use std::{fs, io::ErrorKind, path::PathBuf, time::UNIX_EPOCH};
use tauri::{
    ipc::{InvokeBody, Request, Response},
    AppHandle, Manager, Url, WebviewWindow,
};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_opener::OpenerExt;

fn texte<E: std::fmt::Display>(e: E) -> String {
    e.to_string()
}

fn modif(m: &fs::Metadata) -> u64 {
    m.modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[derive(Serialize)]
struct Entree {
    nom: String,
    dossier: bool,
    modif: u64,
}

#[derive(Serialize)]
struct Infos {
    dossier: bool,
    taille: u64,
    modif: u64,
}

/// Contenu d'un dossier : noms, nature et date (la date évite de relire chaque enregistrement de la voix).
#[tauri::command]
fn lister(chemin: String) -> Result<Vec<Entree>, String> {
    let mut v = Vec::new();
    for e in fs::read_dir(&chemin).map_err(texte)? {
        let e = e.map_err(texte)?;
        let m = e.metadata().ok();
        v.push(Entree {
            nom: e.file_name().to_string_lossy().into_owned(),
            dossier: e.path().is_dir(),
            modif: m.as_ref().map(modif).unwrap_or(0),
        });
    }
    Ok(v)
}

/// Nature, taille et date d'un fichier ou d'un dossier ; rien s'il n'existe pas.
#[tauri::command]
fn infos(chemin: String) -> Option<Infos> {
    let m = fs::metadata(&chemin).ok()?;
    Some(Infos { dossier: m.is_dir(), taille: m.len(), modif: modif(&m) })
}

/// Octets d'un fichier (rendus tels quels à la page).
#[tauri::command]
fn lire(chemin: String) -> Result<Response, String> {
    fs::read(&chemin).map(Response::new).map_err(texte)
}

fn entete(request: &Request<'_>, nom: &str) -> Result<String, String> {
    let brut = request.headers().get(nom).and_then(|v| v.to_str().ok()).ok_or(format!("{nom} manquant"))?;
    Ok(percent_decode_str(brut).decode_utf8().map_err(texte)?.into_owned())
}

fn octets<'a>(request: &'a Request<'_>) -> Result<&'a Vec<u8>, String> {
    match request.body() {
        InvokeBody::Raw(o) => Ok(o),
        _ => Err("contenu attendu en octets".into()),
    }
}

/// Écrit un fichier (chemin dans l'en-tête « chemin », octets dans le corps) : fichier temporaire à côté,
/// puis remplacement — un fichier n'est jamais à moitié écrit.
#[tauri::command]
fn ecrire(request: Request<'_>) -> Result<(), String> {
    let chemin = entete(&request, "chemin")?;
    let o = octets(&request)?;
    let p = PathBuf::from(&chemin);
    let nom = p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let tmp = p.with_file_name(format!(".{nom}.ecriture"));
    fs::write(&tmp, o).map_err(texte)?;
    if let Err(e) = fs::rename(&tmp, &p) {
        let _ = fs::remove_file(&tmp);
        fs::write(&p, o).map_err(|e2| format!("{e} / {e2}"))?;
    }
    Ok(())
}

/// Crée UN dossier (son parent doit exister : un dossier de la classe absent n'est jamais recréé vide).
#[tauri::command]
fn creer_dossier(chemin: String) -> Result<(), String> {
    match fs::create_dir(&chemin) {
        Err(e) if e.kind() != ErrorKind::AlreadyExists => Err(texte(e)),
        _ => Ok(()),
    }
}

#[tauri::command]
fn supprimer(chemin: String, recursif: bool) -> Result<(), String> {
    let p = PathBuf::from(&chemin);
    let r = if p.is_dir() {
        if recursif { fs::remove_dir_all(&p) } else { fs::remove_dir(&p) }
    } else {
        fs::remove_file(&p)
    };
    r.map_err(texte)
}

/// Les dossiers retenus : %APPDATA%\fr.franckvarault.syllamons\dossier-classe.txt et dossier-voix.txt.
fn fichier_dossier(app: &AppHandle, quoi: &str) -> Result<PathBuf, String> {
    if !matches!(quoi, "classe" | "voix") {
        return Err("dossier inconnu".into());
    }
    let d = app.path().app_config_dir().map_err(texte)?;
    fs::create_dir_all(&d).map_err(texte)?;
    Ok(d.join(format!("dossier-{quoi}.txt")))
}

#[tauri::command]
fn dossier_retenu(app: AppHandle, quoi: String) -> Option<String> {
    let c = fs::read_to_string(fichier_dossier(&app, &quoi).ok()?).ok()?;
    let c = c.trim();
    (!c.is_empty()).then(|| c.to_string())
}

#[tauri::command]
fn oublier_dossier(app: AppHandle, quoi: String) -> Result<(), String> {
    let f = fichier_dossier(&app, &quoi)?;
    match fs::remove_file(f) {
        Err(e) if e.kind() != ErrorKind::NotFound => Err(texte(e)),
        _ => Ok(()),
    }
}

/// Fenêtre de Windows pour choisir un dossier ; « classe » et « voix » sont retenus, « import » ne l'est pas.
#[tauri::command]
async fn choisir_dossier(app: AppHandle, quoi: String, titre: Option<String>) -> Result<Option<String>, String> {
    let a = app.clone();
    let t = titre.unwrap_or_else(|| "Choisir un dossier".into());
    let choix = tauri::async_runtime::spawn_blocking(move || a.dialog().file().set_title(t).blocking_pick_folder())
        .await
        .map_err(texte)?;
    let chemin = choix
        .and_then(|f| f.into_path().ok())
        .map(|p| p.to_string_lossy().replace('\\', "/"));
    if let (Some(c), true) = (&chemin, matches!(quoi.as_str(), "classe" | "voix")) {
        fs::write(fichier_dossier(&app, &quoi)?, c).map_err(texte)?;
    }
    Ok(chemin)
}

/// « Enregistrer sous… » : nom proposé dans l'en-tête « nom », octets dans le corps ; rend le chemin choisi.
#[tauri::command]
async fn enregistrer_fichier(app: AppHandle, request: Request<'_>) -> Result<Option<String>, String> {
    let nom = entete(&request, "nom")?;
    let o = octets(&request)?.clone();
    let a = app.clone();
    let choix = tauri::async_runtime::spawn_blocking(move || a.dialog().file().set_file_name(nom).blocking_save_file())
        .await
        .map_err(texte)?;
    let Some(p) = choix.and_then(|f| f.into_path().ok()) else {
        return Ok(None);
    };
    fs::write(&p, o).map_err(texte)?;
    Ok(Some(p.to_string_lossy().into_owned()))
}

/// Plein écran de la fenêtre : etat vrai / faux, ou bascule ; rend l'état d'avant.
#[tauri::command]
fn plein_ecran(window: WebviewWindow, etat: Option<bool>) -> Result<bool, String> {
    let avant = window.is_fullscreen().map_err(texte)?;
    window.set_fullscreen(etat.unwrap_or(!avant)).map_err(texte)?;
    Ok(avant)
}

/// Une adresse dans le navigateur de l'ordinateur (http/https seulement).
#[tauri::command]
fn ouvrir_url(app: AppHandle, url: String) -> Result<(), String> {
    let u: Url = url.parse().map_err(texte)?;
    if !matches!(u.scheme(), "http" | "https") {
        return Err("adresse refusée".into());
    }
    app.opener().open_url(u.as_str(), None::<&str>).map_err(texte)
}

/// Clé publique des mises à jour (la clé privée n'est que dans le secret TAURI_SIGNING_PRIVATE_KEY de Syllamons-versions).
const CLE_MAJ: &str = "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IDgyQzdCMUI0NzdERUE1NzkKUldSNXBkNTN0TEhIZ3BUVlpQR0dyeVh4QmttMk5QZG41MVZZbERHampGZHV0UXlEQVNpZVp2cFIK";

fn texte_b64(s: &str) -> Result<String, String> {
    String::from_utf8(STANDARD.decode(s.trim()).map_err(texte)?).map_err(texte)
}

/// Installe une nouvelle version téléchargée par la page : signature vérifiée (sinon refus), installateur écrit dans
/// le dossier temporaire puis lancé en mode « passif » (barre de progression, pas de questions) avec relance de
/// l'application (/P /UPDATE /R) ; l'application se ferme aussitôt.
#[tauri::command]
fn installer_paquet(app: AppHandle, request: Request<'_>) -> Result<(), String> {
    let signature = entete(&request, "signature")?;
    let version = entete(&request, "version")?;
    if version.is_empty() || !version.chars().all(|c| c.is_ascii_digit() || c == '.') {
        return Err("numéro de version invalide".into());
    }
    let o = octets(&request)?;
    let cle = PublicKey::decode(&texte_b64(CLE_MAJ)?).map_err(texte)?;
    let sig = Signature::decode(&texte_b64(&signature)?).map_err(texte)?;
    cle.verify(o, &sig, true).map_err(|e| format!("signature refusée ({e}) : installateur non installé"))?;
    let p = std::env::temp_dir().join(format!("Syllamons_{version}_x64-setup.exe"));
    fs::write(&p, o).map_err(texte)?;
    std::process::Command::new(&p).args(["/P", "/UPDATE", "/R"]).spawn().map_err(texte)?;
    app.exit(0);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // une seule fenêtre Syllamons : un 2e lancement ramène celle déjà ouverte
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.unminimize();
                let _ = w.set_focus();
            }
        }))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            lister, infos, lire, ecrire, creer_dossier, supprimer, choisir_dossier, dossier_retenu, oublier_dossier,
            enregistrer_fichier, plein_ecran, ouvrir_url, installer_paquet
        ])
        .run(tauri::generate_context!())
        .expect("impossible de lancer Les Syllamons");
}
