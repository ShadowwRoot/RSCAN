# RSCAN# Rscan

**Auteurs :** Ton prénom + celui de ton binôme

## Description

Rscan est un scanner de ports écrit en Rust permettant de détecter les ports ouverts sur une IP ou une plage d'IP.  
Il récupère aussi les bannières (banners) quand c'est possible, pour identifier les services derrière les ports.

⚠️ **Avertissement :**  
Cet outil est à utiliser uniquement dans un cadre légal et éthique (tests locaux, lab perso, CTF, autorisation explicite).  
Il est interdit de scanner des systèmes qui ne vous appartiennent pas sans autorisation.

## Compilation

```bash
git clone <ton-repo-github>
cd Rscan
cargo build --release
