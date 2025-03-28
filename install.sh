#!/bin/bash

set -e

# Couleurs pour les messages
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # Pas de couleur

echo -e "${GREEN}🦀🔐 SmartLocker - Installation automatique pour Linux${NC}"

# Vérifier si Rust est installé
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Rust n'est pas installé. Veuillez installer Rust avant de continuer.${NC}"
    exit 1
fi

# Définir le dépôt Git et la branche par défaut
REPO_URL="https://github.com/WillIsback/smart-locker.git"
BRANCH="main"

# Vérifier si l'argument "nightly" est passé
if [[ "$1" == "nightly" ]]; then
    echo -e "${YELLOW}Mode nightly activé : utilisation du dépôt GitLab et de la branche preprod.${NC}"
    REPO_URL="git@gitlab.com:WillIsback/SmartLocker.git"
    BRANCH="preprod"
fi

# Cloner le dépôt
echo -e "${GREEN}Clonage du dépôt : $REPO_URL (branche : $BRANCH)${NC}"
git clone --branch "$BRANCH" "$REPO_URL" smart-locker-temp
cd smart-locker-temp

# Construire le projet
echo -e "${GREEN}Construction du projet...${NC}"
cargo build --release

# Installer le binaire
echo -e "${GREEN}Installation du binaire...${NC}"
sudo cp target/release/smart-locker /usr/local/bin/

# Nettoyer les fichiers temporaires
cd ..
rm -rf smart-locker-temp

echo -e "${GREEN}✅ Installation terminée avec succès !${NC}"