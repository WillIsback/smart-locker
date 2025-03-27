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
- 🛡️ Option : génération de la clé à partir d'une passphrase hashée (PBKDF2)

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
- [ ]  Option `--clipboard` pour copier en RAM temporaire
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

> 🔐 Projet personnel pour apprendre le Rust en profondeur tout en créant un outil utile au quotidien.