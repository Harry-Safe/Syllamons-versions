# Journal des publications

Le plus récent en tête.

## 2026-11-09 — Installateur fabriqué par ce dépôt (branche `claude/fabrication-publique`, non publiée)

- **Demande :** quota de minutes GitHub du dépôt privé épuisé ; Franck : « Mets l'option 2 en route » (fabrication
  dans les dépôts publics, sur des branches d'essai). **`main` inchangé**, rien n'est publié sans sa phrase explicite.
- **Réalisation :** `outils/fabriquer-appli.py` et action « Publier les mises à jour » : un travail Windows par canal
  fabrique l'installateur à partir de `coquille/<canal>/` et de la page du canal (empreinte contrôlée, repère d'essai
  ajouté comme dans `build/construire-appli.py`), le garde en cache ; la publication le range dans `appli/`, complète
  sa taille et son empreinte dans `entrees/<canal>.json`, puis assemble, signe et publie comme avant. `.gitattributes`
  (aucune conversion de fins de ligne sur Windows), `.gitignore`, `README.md`. Essai 1.9.1 repassé par le nouveau
  chemin depuis la branche source `claude/fabrication-publique` (= `claude/essai-1.9` + ce changement de procédure) : page
  identique à l'octet, installateur retiré du dépôt, coquille et fiches écrites.
- **Livraison :** branche `claude/fabrication-publique` seulement ; résultat de la fabrication : voir l'action de la
  branche.

## 2026-11-09 — Essai 1.9.1

- Canal **essai** : cursive à la bonne taille et majuscules bâton dans L'Œil (commit source `34e584b`, branche
  `claude/essai-1.9`). Installateur signé par l'action (secret `TAURI_SIGNING_PRIVATE_KEY`).

## 2026-11-08 — Essai 1.9.0 (premier dépôt)

- Site créé : page de téléchargement, `outils/assembler.py`, `outils/signer-appli.py`, action « Publier les mises à jour ».
- Canal **essai** : application Windows 1.9.0 (`appli/essai/`), Syllamons pour Ma classe, Syllamons.html ; fichiers
  fabriqués par l'action « Application Windows » du dépôt Syllamons (commit source `694d9a2`, branche `claude/essai-1.9`).
- Installateur signé avant l'envoi (le secret `TAURI_SIGNING_PRIVATE_KEY` n'était pas encore en place) ; signature
  vérifiée avec la clé publique de l'application (`82C7B1B477DEA579`). Aucun canal stable : rien n'est publié sans
  l'accord de Franck.
- Mis en place par Franck le même soir : GitHub Pages (source GitHub Actions) et secret `TAURI_SIGNING_PRIVATE_KEY`.
  Site en ligne à 19 h 28 (essai 1.9.0, installateur re-signé par l'action avec le secret).
