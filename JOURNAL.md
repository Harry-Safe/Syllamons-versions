# Journal des publications

Le plus récent en tête.

## 2026-11-08 — Essai 1.9.0 (premier dépôt)

- Site créé : page de téléchargement, `outils/assembler.py`, `outils/signer-appli.py`, action « Publier les mises à jour ».
- Canal **essai** : application Windows 1.9.0 (`appli/essai/`), Syllamons pour Ma classe, Syllamons.html ; fichiers
  fabriqués par l'action « Application Windows » du dépôt Syllamons (commit source `694d9a2`, branche `claude/essai-1.9`).
- Installateur signé avant l'envoi (le secret `TAURI_SIGNING_PRIVATE_KEY` n'était pas encore en place) ; signature
  vérifiée avec la clé publique de l'application (`82C7B1B477DEA579`). Aucun canal stable : rien n'est publié sans
  l'accord de Franck.
- À faire par Franck, une fois : *Settings › Pages › Source = GitHub Actions* ; secret `TAURI_SIGNING_PRIVATE_KEY`.
