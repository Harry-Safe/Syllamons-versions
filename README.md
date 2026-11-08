# Les Syllamons — site des mises à jour

Site public : **https://harry-safe.github.io/Syllamons-versions/**

« Les Syllamons » est l'aventure de lecture de la classe de CP de **Franck VARAULT** (code source dans un dépôt
privé). Ce dépôt ne contient que les **versions à télécharger** et ce qu'il faut pour les publier :

| Dossier | Contenu | Lu par |
|---|---|---|
| `appli/latest.json` + `Syllamons_<v>_x64-setup.exe` | application Windows, version publiée | l'application (canal « publiées ») |
| `appli/essai/latest.json` + `…-setup-essai.exe` | application Windows, version d'essai | l'application (canal « d'essai ») |
| `integree/[essai/]Syllamons-pour-Ma-classe_<v>.zip` | dossier `syllamons` à ajouter dans Ma classe › Applis | l'enseignant |
| `html/[essai/]Syllamons.html` | page pour le navigateur | l'enseignant |
| `entrees/stable.json`, `entrees/essai.json` | numéro, date, nouveautés, tailles et empreintes | `outils/assembler.py` → `versions.json` |

## Publier

1. Dans le dépôt Syllamons : `python build/publier-appli.py <ce dossier> --canal essai` (ou sans `--canal` pour une
   version publiée, **seulement avec l'accord explicite de Franck**). Le script prend les fichiers fabriqués par
   l'action « Application Windows » (branche `appli-paquets` ou `appli-paquets-essai`).
2. Commit et envoi sur `main` de ce dépôt : l'action **« Publier les mises à jour »** vérifie les empreintes, signe
   l'installateur et met le site en ligne (GitHub Pages).

## Signature des mises à jour

- L'application n'installe une mise à jour que si sa **signature Minisign** correspond à la clé publique écrite dans
  l'application (`tauri/src-tauri/src/lib.rs` du dépôt Syllamons).
- La clé privée est **seulement** dans le secret `TAURI_SIGNING_PRIVATE_KEY` de ce dépôt (*Settings › Secrets and
  variables › Actions*), sans mot de passe. Jamais dans un fichier.
- Sans ce secret, une fiche `latest.json` déjà signée est gardée telle quelle ; une fiche non signée fait échouer
  la publication et le site en ligne ne change pas.

## Règles

- Aucune donnée d'élève, aucun lien ni code de classe en ligne dans les fichiers publiés (le script de publication
  refuse une page qui en contient).
- Commits sous l'identité « Franck VARAULT » ; aucune signature de co-écriture d'assistant.
- Ne jamais réécrire l'historique de `main`.

© 2026 Franck VARAULT, tous droits réservés. Le code a été écrit avec l'aide d'outils d'intelligence artificielle,
sous la direction de l'auteur.
