# 🔐 SmartLocker

Un outil CLI ultra-léger écrit en **Rust** pour **chiffrer, stocker et gérer des secrets sensibles localement**, de manière sécurisée et durable.

## 🚀 Objectif

SmartLocker répond à un besoin réel :

> Dans un projet fullstack avec CI/CD, les **tokens, clés privées, passphrases et API keys** deviennent critiques.

Les stocker en clair est risqué. Les encoder en base64 ne suffit pas. SmartLocker propose une solution fiable, simple et efficace.

## 🎯 Fonctionnalités prévues

- ✅ Chiffrement symétrique des secrets (via AES-GCM ou autre)
- ✅ Déchiffrement rapide avec passphrase ou clé principale
- ✅ Dossier sécurisé `~/.locker`
- ✅ Fichiers suffixés `.slock` ou `.aes`
- ✅ CLI ergonomique avec `smartlocker encrypt`, `decrypt`, `list`, etc.
- ✅ Support du piping (ex: `cat secret.txt | smartlocker encrypt -n my_secret`)
- ✅ Option : génération de la clé à partir d'une passphrase hashée (PBKDF2)
- ✅ Option : copier le secret déchiffré dans le presse-papier
- 🔜 Option : plugin Git pre-commit pour empêcher les push de secrets
- 🔜 Option : coffre avec expiration automatique


## 🗂️ Arborescence cible

```
~/.locker/
├── locker.key         # clé symétrique locale (ou générée via passphrase)
├── openai_token.slock
├── ssh_key_prod.slock
└── mydb_pass.slock
```

## 🛠️ Architecture CLI

```
smartlocker <commande> [options]

COMMANDES PRINCIPALES :
  encrypt      Chiffrer un secret et le stocker
  decrypt      Déchiffrer un fichier .slock
  list         Lister les secrets chiffrés
  remove       Supprimer un secret
  init         Générer la clé principale (locker.key)

EXEMPLE :
  smartlocker encrypt -n openai_token -v sk-abc123...
  smartlocker decrypt -n openai_token
```

## 📦 Tech Stack

- 🦀 **Rust** (>= 1.74)
- 📦 `aes-gcm`, `rand`, `clap`, `serde`, `directories`
- 🔐 Chiffrement sécurisé basé sur AES 256 GCM

## 🧱 Étapes futures

- [ ]  Ajout d’un coffre avec expiration automatique
- [V]  Option `--clipboard` pour copier en RAM temporaire
- [ ]  Plugin Git pre-commit pour empêcher les push de secrets

## 📈 Pourquoi ce projet ?

Parce que gérer les secrets dans un projet fullstack, c’est :

- comprendre les failles
- construire des outils fiables et portables
- apprendre à sécuriser ses workflows DevOps

---

## 🧠 Schéma de fonctionnement

```
                +---------------------------+
                |     smartlocker init      |
                +-------------+-------------+
                              |
                         Génère clé 🔑
                              |
               +--------------v-------------+
               |     ~/.locker/locker.key   |
               +--------------+-------------+
                              |
          +-------------------+--------------------+
          |                                        |
+---------v--------+                    +----------v---------+
| smartlocker encrypt |                  | smartlocker decrypt |
+---------+--------+                    +----------+---------+
          |                                        |
     Entrée CLI ou STDIN                      Lecture fichier
          |                                        |
   Fichier chiffré `.slock`           →    Secret déchiffré
```

---
## 🛠️ Installation

SmartLocker peut être installé sur **Linux** et **Windows**. Voici les différentes méthodes d'installation, adaptées à vos besoins.

---

### 📦 Installation automatisée

#### **Linux (via script Bash)**

Exécutez le script suivant pour télécharger, compiler et installer SmartLocker :

```bash
curl -fsSL https://raw.githubusercontent.com/WillIsback/smart-locker/main/install.sh | bash
```

Ce script :
1. Vérifie que **Rust** est installé.
2. Clone le dépôt GitHub.
3. Compile le projet en mode `release`.
4. Installe le binaire dans `/usr/local/bin`.

#### **Windows (via PowerShell)**

Exécutez cette commande dans PowerShell pour télécharger et installer SmartLocker :

```powershell
Invoke-WebRequest -Uri https://raw.githubusercontent.com/WillIsback/smart-locker/main/install.ps1 -OutFile install.ps1; ./install.ps1
```

Ce script :
1. Vérifie que **Rust** est installé.
2. Clone le dépôt GitHub.
3. Compile le projet en mode `release`.
4. Copie le binaire dans un dossier accessible via le `PATH`.

---

### 🛠️ Installation manuelle

#### **Linux**

1. Assurez-vous que **Rust** est installé :
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clonez le dépôt :
   ```bash
   git clone https://github.com/WillIsback/smart-locker.git
   cd smart-locker
   ```

3. Compilez le projet en mode `release` :
   ```bash
   cargo build --release
   ```

4. Installez le binaire dans `/usr/local/bin` :
   ```bash
   sudo cp target/release/smart-locker /usr/local/bin/
   ```

5. Vérifiez l'installation :
   ```bash
   smart-locker --version
   ```

#### **Windows**

1. Installez **Rust** via [rustup](https://rustup.rs/).

2. Clonez le dépôt :
   ```powershell
   git clone https://github.com/WillIsback/smart-locker.git
   cd smart-locker
   ```

3. Compilez le projet en mode `release` :
   ```powershell
   cargo build --release
   ```

4. Ajoutez le binaire au `PATH` :
   ```powershell
   $Env:Path += ";$PWD\target\release"
   ```

5. Vérifiez l'installation :
   ```powershell
   smart-locker --version
   ```

---

### 📦 Packages précompilés (à venir)

Nous prévoyons de fournir des **binaires précompilés** pour les principales plateformes (Linux, Windows, macOS). Vous pourrez les télécharger directement depuis la page [Releases](https://github.com/WillIsback/smart-locker.git/1.0.0).

---

### 🧪 Tester l'installation

Une fois installé, testez SmartLocker avec les commandes suivantes :

```bash
smart-locker init
smart-locker encrypt -n my_secret -v "Ceci est un test"
smart-locker decrypt -n my_secret
```

---

> **Note :** Si vous rencontrez des problèmes lors de l'installation, consultez la section [Issues](https://github.com/WillIsback/smart-locker/issues) ou ouvrez un ticket.

> 🔐 Projet personnel pour apprendre le Rust en profondeur tout en créant un outil utile au quotidien.