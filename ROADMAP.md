# 🛣️ Roadmap de Développement - SmartLocker

Ce document décrit les étapes logiques pour développer, finaliser et publier **SmartLocker**, un outil CLI pour chiffrer et gérer des secrets sensibles.

---

## 📅 Étape 1 : Initialisation du projet

1. **Créer la structure de base du projet :**
   - Initialiser un projet Rust avec `cargo init`.
   - Configurer les dépendances dans `Cargo.toml` :
     - `aes-gcm` : Pour le chiffrement AES-GCM.
     - `rand` : Pour générer des nombres aléatoires.
     - `clap` : Pour gérer les arguments CLI.
     - `serde` : Pour la sérialisation/désérialisation.
     - `directories` : Pour gérer les chemins de fichiers spécifiques à l'utilisateur.

2. **Créer les dossiers et fichiers nécessaires :**
   - `src/main.rs` : Point d'entrée du programme.
   - `src/commands/` : Dossier pour les commandes CLI (`encrypt`, `decrypt`, etc.).
   - `src/utils/` : Dossier pour les fonctions utilitaires.
   - `src/config.rs` : Fichier pour gérer la configuration globale.

---

## 📅 Étape 2 : Implémentation des fonctionnalités principales

### **2.1 Commande `init`**
   - Générer une clé symétrique (`locker.key`) et la stocker dans `~/.locker/`.
   - Créer le dossier `~/.locker/` s'il n'existe pas.
   - Vérifier si une clé existe déjà avant d'en générer une nouvelle.

### **2.2 Commande `encrypt`**
   - Lire un secret depuis l'entrée CLI ou STDIN.
   - Chiffrer le secret avec la clé symétrique (`locker.key`) en utilisant AES-GCM.
   - Sauvegarder le secret chiffré dans un fichier `.slock` dans `~/.locker/`.

### **2.3 Commande `decrypt`**
   - Lire un fichier `.slock` depuis `~/.locker/`.
   - Déchiffrer le contenu avec la clé symétrique (`locker.key`).
   - Afficher le secret déchiffré dans la sortie standard (STDOUT).

### **2.4 Commande `list`**
   - Lister tous les fichiers `.slock` présents dans `~/.locker/`.

### **2.5 Commande `remove`**
   - Supprimer un fichier `.slock` spécifique de `~/.locker/`.

---

## 📅 Étape 3 : Tests et validation

1. **Écrire des tests unitaires :**
   - Tester chaque commande individuellement (`init`, `encrypt`, `decrypt`, etc.).
   - Vérifier les cas d'erreur (ex. : clé manquante, fichier introuvable).

2. **Tester l'intégration :**
   - Simuler des scénarios réels (ex. : chiffrer un secret, puis le déchiffrer).

3. **Vérifier la sécurité :**
   - S'assurer que les fichiers `.slock` ne peuvent pas être lus sans la clé.
   - Vérifier que la clé symétrique est générée de manière sécurisée.

---

## 📅 Étape 4 : Optimisation et fonctionnalités avancées

1. **Ajouter des options avancées :**
   - Génération de la clé à partir d'une passphrase hashée (PBKDF2).
   - Support du piping (ex. : `cat secret.txt | smartlocker encrypt -n my_secret`).
   - Option `--clipboard` pour copier un secret déchiffré dans le presse-papier.

2. **Améliorer l'expérience utilisateur :**
   - Ajouter des messages d'erreur clairs.
   - Ajouter des couleurs ou des icônes pour rendre la CLI plus conviviale.

3. **Optimiser les performances :**
   - Réduire la taille des fichiers `.slock`.
   - Minimiser l'utilisation de la mémoire.

---

## 📅 Étape 5 : Documentation

1. **Compléter le fichier `README.md` :**
   - Ajouter des exemples d'utilisation pour chaque commande.
   - Expliquer comment installer et utiliser l'outil.

2. **Créer une documentation technique :**
   - Décrire l'architecture du projet.
   - Expliquer les choix techniques (ex. : pourquoi AES-GCM ?).

3. **Créer une page de manuel (`man`) :**
   - Fournir une documentation CLI accessible via `man smartlocker`.

---

## 📅 Étape 6 : Préparation pour la publication

1. **Créer une version stable :**
   - Vérifier que toutes les fonctionnalités principales sont implémentées et testées.
   - Fixer les bugs connus.

2. **Créer un binaire exécutable :**
   - Compiler le projet pour les principales plateformes (Linux, Windows, macOS).

3. **Publier sur GitHub :**
   - Héberger le code source sur un dépôt public.
   - Ajouter des instructions pour compiler ou télécharger le binaire.

4. **Publier sur crates.io :**
   - Préparer le projet pour être publié en tant que crate Rust.
   - Ajouter les métadonnées nécessaires dans `Cargo.toml`.

---

## 📅 Étape 7 : Maintenance et évolutions

1. **Corriger les bugs signalés par les utilisateurs.**
2. **Ajouter de nouvelles fonctionnalités basées sur les retours.**
3. **Maintenir les dépendances à jour.**
4. **Créer un plugin Git pre-commit pour empêcher les push de secrets.**

---

## 📅 Étape 8 : Marketing et adoption

1. **Écrire un article de blog :**
   - Présenter SmartLocker et ses avantages.
   - Expliquer comment il peut être utilisé dans des projets DevOps.

2. **Partager sur les réseaux sociaux :**
   - Publier sur Twitter, LinkedIn, Reddit, etc.

3. **Proposer à des communautés open-source :**
   - Partager le projet avec des développeurs Rust ou DevOps.

---

## 🏁 Objectif final

- **Un outil CLI robuste, sécurisé et facile à utiliser.**
- **Une communauté d'utilisateurs et de contributeurs.**
- **Un projet open-source qui t'a permis d'apprendre Rust en profondeur.**

---

> 🔐 **SmartLocker** : Plus qu'un projet, une aventure pour apprendre et créer un outil utile au quotidien.