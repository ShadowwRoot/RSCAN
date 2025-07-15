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
Utilisation
bash
Copier
Modifier
```
cargo run <IP/plage> <port_debut> <port_fin> [--verbose]
Exemples :

Scan local :

bash
Copier
Modifier
cargo run 127.0.0.1 20 100 --verbose
Scan réseau local (range) :

bash
Copier
Modifier
cargo run 192.168.1.1-5 22 22 --verbose
Scan externe :

bash
Copier
Modifier
cargo run scanme.nmap.org 20 100 --verbose
Services à tester
🔐 SSH
bash
Copier
Modifier
sudo systemctl start ssh
cargo run 127.0.0.1 22 22 --verbose
🌐 HTTP
bash
Copier
Modifier
sudo systemctl start apache2
cargo run 127.0.0.1 80 80 --verbose
🔒 HTTPS
bash
Copier
Modifier
sudo systemctl start apache2
cargo run 127.0.0.1 443 443 --verbose
📦 MySQL / MariaDB
bash
Copier
Modifier
sudo systemctl start mariadb
cargo run 127.0.0.1 3306 3306 --verbose
📡 FTP
bash
Copier
Modifier
sudo systemctl start vsftpd
cargo run 127.0.0.1 21 21 --verbose
🖧 Telnet
bash
Copier
Modifier
sudo systemctl start telnet
cargo run 127.0.0.1 23 23 --verbose
