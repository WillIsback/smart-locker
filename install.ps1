# Couleurs pour les messages
$Green = "`e[32m"
$Red = "`e[31m"
$Yellow = "`e[33m"
$Reset = "`e[0m"

Write-Host "${Green}🔐 SmartLocker - Installation automatique pour Windows${Reset}"

# Vérifier si Rust est installé
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "${Red}Rust n'est pas installé. Veuillez installer Rust avant de continuer.${Reset}"
    Write-Host "${Yellow}Vous pouvez l'installer avec : https://rustup.rs/${Reset}"
    exit 1
}

# Cloner le dépôt GitHub
Write-Host "${Green}📥 Téléchargement du projet SmartLocker...${Reset}"
if (Test-Path "smart-locker") {
    Write-Host "${Yellow}Le dossier 'smart-locker' existe déjà. Suppression...${Reset}"
    Remove-Item -Recurse -Force "smart-locker"
}
git clone https://github.com/WillIsback/smart-locker.git
Set-Location "smart-locker"

# Compiler le projet en mode release
Write-Host "${Green}⚙️ Compilation du projet en mode release...${Reset}"
cargo build --release

# Ajouter le binaire au PATH
Write-Host "${Green}🚀 Ajout du binaire au PATH...${Reset}"
$Binaire = Join-Path (Get-Location) "target\release\smart-locker.exe"
$Env:Path += ";$($Binaire | Split-Path -Parent)"

# Vérifier l'installation
if (Get-Command smart-locker -ErrorAction SilentlyContinue) {
    Write-Host "${Green}✅ Installation réussie ! Vous pouvez maintenant utiliser SmartLocker.${Reset}"
    Write-Host "${Yellow}Exemple : smart-locker --help${Reset}"
} else {
    Write-Host "${Red}❌ Une erreur s'est produite lors de l'installation.${Reset}"
    exit 1
}