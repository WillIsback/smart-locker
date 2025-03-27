#!/bin/bash

set -e

# Couleurs pour les messages
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # Pas de couleur

echo -e "${GREEN}🔐 SmartLocker - Installation automatique pour Linux${NC}"

# Vérifier si Rust est installé
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Rust n'est pas installé. Veuillez installer Rust avant de continuer.${NC}"
    echo -e "${YELLOW}Vous pouvez l'installer avec : curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh${NC}"
    exit 1
fi

# Cloner le dépôt GitHub
echo -e "${GREEN}📥 Téléchargement du projet SmartLocker...${NC}"
if [ -d "smart-locker" ]; then
    echo -e "${YELLOW}Le dossier 'smart-locker' existe déjà. Suppression...${NC}"
    rm -rf smart-locker
fi
git clone https://github.com/WillIsback/smart-locker.git
cd smart-locker

# Compiler le projet en mode release
echo -e "${GREEN}⚙️ Compilation du projet en mode release...${NC}"
cargo build --release

# Copier le binaire dans /usr/local/bin
echo -e "${GREEN}🚀 Installation du binaire dans /usr/local/bin...${NC}"
sudo cp target/release/smart-locker /usr/local/bin/

# Vérifier l'installation
if command -v smart-locker &> /dev/null; then
    echo -e "${GREEN}✅ Installation réussie ! Vous pouvez maintenant utiliser SmartLocker.${NC}"
    echo -e "${YELLOW}Exemple : smart-locker --help${NC}"
else
    echo -e "${RED}❌ Une erreur s'est produite lors de l'installation.${NC}"
    exit 1
fi