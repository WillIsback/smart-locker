# Couleurs pour les messages
$Green = "`e[32m"
$Red = "`e[31m"
$Yellow = "`e[33m"
$Reset = "`e[0m"

Write-Host "${Green}🔐 SmartLocker - Installation automatique pour Windows${Reset}"

# Vérifier si Rust est installé
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "${Red}Rust n'est pas installé. Veuillez installer Rust avant de continuer.${Reset}"
    exit 1
}

# Définir le dépôt Git et la branche par défaut
$RepoUrl = "https://github.com/WillIsback/SmartLocker.git"
$Branch = "main"

# Vérifier si l'argument "nightly" est passé
if ($args -contains "nightly") {
    Write-Host "${Yellow}Mode nightly activé : utilisation du dépôt GitLab et de la branche preprod.${Reset}"
    $RepoUrl = "git@gitlab.com:WillIsback/SmartLocker.git"
    $Branch = "preprod"
}

# Cloner le dépôt
Write-Host "${Green}Clonage du dépôt : $RepoUrl (branche : $Branch)${Reset}"
git clone --branch $Branch $RepoUrl smart-locker-temp
Set-Location smart-locker-temp

# Construire le projet
Write-Host "${Green}Construction du projet...${Reset}"
cargo build --release

# Installer le binaire
Write-Host "${Green}Installation du binaire...${Reset}"
Copy-Item -Path .\target\release\smart-locker.exe -Destination "C:\Program Files\smart-locker\smart-locker.exe" -Force

# Nettoyer les fichiers temporaires
Set-Location ..
Remove-Item -Recurse -Force smart-locker-temp

Write-Host "${Green}✅ Installation terminée avec succès !${Reset}"