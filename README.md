
# RSCAN

## Auteurs

Ton prénom + celui de ton binôme

## Description

Rscan est un scanner de ports écrit en Rust permettant de détecter les ports ouverts sur une IP ou une plage d'IP.  
Il récupère aussi les bannières (banners) quand c'est possible, pour identifier les services derrière les ports.

⚠️ **Avertissement**  
Cet outil est à utiliser uniquement dans un cadre légal et éthique (tests locaux, lab perso, CTF, autorisation explicite).  
Il est interdit de scanner des systèmes qui ne vous appartiennent pas sans autorisation.

## Compilation

```bash
git clone <ton-repo-github>
cd Rscan
cargo build --release
```

## Utilisation

```bash
cargo run <IP/plage> <port_debut> <port_fin> [--verbose]
```

## Exemples de tests

### Scan local

```bash
cargo run 127.0.0.1 20 100 --verbose
```

### Scan réseau local (plage d'IP)

```bash
cargo run 192.168.1.1-5 22 22 --verbose
```

### Scan site externe autorisé (scanme.nmap.org)

```bash
cargo run scanme.nmap.org 20 100 --verbose
```

## Services à démarrer pour les tests locaux

### SSH

```bash
sudo systemctl start ssh
cargo run 127.0.0.1 22 22 --verbose
```

### HTTP (Apache2)

```bash
sudo systemctl start apache2
cargo run 127.0.0.1 80 80 --verbose
```

### HTTPS (Apache2 SSL)

```bash
sudo systemctl start apache2
cargo run 127.0.0.1 443 443 --verbose
```

### MySQL / MariaDB

```bash
sudo systemctl start mysql
cargo run 127.0.0.1 3306 3306 --verbose
```

## Scan de ta box (gateway routeur)

```bash
ip route | grep default
# repère l'IP de ta gateway, par ex : 172.20.10.1

cargo run 172.20.10.1 1 100 --verbose
```

## Important

- Pense à ouvrir les ports/services côté VM/machine si tu veux tester en local.
- Ne scanne jamais de machine sans autorisation.
