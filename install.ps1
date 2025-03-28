# Couleurs pour les messages
$Green = "`e[32m"
$Red = "`e[31m"
$Yellow = "`e[33m"
$Reset = "`e[0m"

Write-Host "${Green}🦀🔐 SmartLocker - Installation automatique pour Windows${Reset}"

# Vérifier si Rust est installé
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "${Red}Rust n'est pas installé. Veuillez installer Rust avant de continuer.${Reset}"
    exit 1
}

# Définir le dépôt Git et la branche par défaut
$RepoUrl = "https://github.com/WillIsback/smart-locker.git"
$Branch = "main"

# Vérifier si l'argument "nightly" est passé
if ($args -contains "nightly") {
    Write-Host "${Yellow}Mode nightly activé : utilisation du dépôt GitLab et de la branche preprod.${Reset}"
    $RepoUrl = "git@gitlab.com:WillIsback/SmartLocker.git"
    $Branch = "preprod"
}

# Cloner le dépôt
Write-Host "${Green}Clonage du dépôt : $RepoUrl (branche : $Branch)${Reset}"
try {
    git clone --branch $Branch $RepoUrl smart-locker-temp
} catch {
    Write-Host "${Red}Erreur : Impossible de cloner le dépôt. Vérifiez vos permissions et l'URL.${Reset}"
    exit 1
}

Set-Location smart-locker-temp

# Construire le projet
Write-Host "${Green}Construction du projet...${Reset}"
try {
    cargo build --release
} catch {
    Write-Host "${Red}Erreur : La construction du projet a échoué.${Reset}"
    exit 1
}

# Installer le binaire
Write-Host "${Green}Installation du binaire...${Reset}"
try {
    Copy-Item -Path .\target\release\smart-locker.exe -Destination "C:\Program Files\smart-locker\smart-locker.exe" -Force
} catch {
    Write-Host "${Red}Erreur : Impossible de copier le binaire.${Reset}"
    exit 1
}

# Nettoyer les fichiers temporaires
Set-Location ..
Remove-Item -Recurse -Force smart-locker-temp

Write-Host "${Green}✅ Installation terminée avec succès !${Reset}"