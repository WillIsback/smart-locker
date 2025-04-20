<!-- markdownlint-disable -->

# Changelog

All notable changes to this project will be documented in this file.

## [0.3.1] - 2025-04-20

### 🚜 Refactor

- Simplify cleanup_test_directory function by removing unnecessary PathBuf variable

## [0.3.0] - 2025-04-20

### 🚀 Features

- *(list, encrypt)* Add support for secret metadata with tags in encryption and listing commands
- *(renew)* Implement secret renewal functionality with expiration management
- *(migrate)* Implement metadata migration functionality and add metadata handling utilities
- Add renew command to extend expiration dates for secrets; enhance metadata and toolbox documentation

### 🐛 Bug Fixes

- *(list)* Simplify timestamp fallback logic in list_secrets function
- *(clipboard)* Remove debug log from decrypt function; enhance clipboard copy functionality with OS-specific handling

### 💼 Other

- *(encryption)* Process to utilize EncryptionConfig for improved structure and maintainability

### 🚜 Refactor

- Simplify code formatting and improve readability in encrypt and list commands
- Reorganize module imports and improve code formatting across multiple files
- Clean up imports and improve code formatting in multiple files
- Clean up module imports and improve code formatting across multiple files
- Simplify error handling in encryption and decryption processes; update path type in is_this_secret function
- Reorder import statements for improved clarity and consistency
- Remove unnecessary blank line in is_this_secret function; ensure consistent formatting
- Add Debug trait to SecretMetadata and MetadataFile structs for improved logging and debugging

### 📚 Documentation

- Mise à jour du changelog pour la version 0.3.0

### ⚙️ Miscellaneous Tasks

- Release smart-locker version 0.3.0

## [0.2.12] - 2025-04-20

### 🐛 Bug Fixes

- Correct version extraction and update asset naming in release workflow

### 📚 Documentation

- Mise à jour du changelog pour la version 0.2.12

### ⚙️ Miscellaneous Tasks

- Release smart-locker version 0.2.12

## [0.2.11] - 2025-04-20

### 🐛 Bug Fixes

- Update version extraction in release workflow and improve changelog formatting

### 📚 Documentation

- Mise à jour du changelog pour la version 0.2.11

### ⚙️ Miscellaneous Tasks

- Release smart-locker version 0.2.11

## [0.2.10] - 2025-04-20

### 🐛 Bug Fixes

- *(release)* Extract version from tag and update .deb/.rpm build steps

### 📚 Documentation

- Mise à jour du changelog pour la version 0.2.10

### ⚙️ Miscellaneous Tasks

- Release smart-locker version 0.2.10

## [0.2.9] - 2025-04-20

### 🐛 Bug Fixes

- *(release)* Remove pre-release hook in cargo-release.toml

### 📚 Documentation

- Mise à jour du changelog pour la version 0.2.9

### ⚙️ Miscellaneous Tasks

- Release smart-locker version 0.2.9

## [0.2.8] - 2025-04-20

### 🐛 Bug Fixes

- *(release)* Extract version from tag for .deb and .rpm builds

### 📚 Documentation

- Mise à jour du changelog pour la version 2.0.0
- Mise à jour du changelog pour la version 2.0.0

### ⚙️ Miscellaneous Tasks

- Release smart-locker version 0.2.8

## [0.2.7] - 2025-04-20

### 🐛 Bug Fixes

- *(ci)* Update .deb build step to include versioning from tag

### ⚙️ Miscellaneous Tasks

- Update pre-release hook path and enhance changelog for version 0.2.6
- Release smart-locker version 0.2.7

## [0.2.6] - 2025-04-20

### ⚙️ Miscellaneous Tasks

- Remove changelog generation step from release workflow
- Release smart-locker version 0.2.6

## [0.2.5] - 2025-04-20

### ⚙️ Miscellaneous Tasks

- Update CI/CD workflow and configuration for improved release process
- Add initialization step to build and test process
- Add release-check.sh to .gitignore
- Release smart-locker version 0.2.4
- Release smart-locker version 0.2.5

## [0.2.3] - 2025-04-20

### ⚙️ Miscellaneous Tasks

- Update changelog for version v0.2.2
- Release smart-locker version 0.2.3

## [0.2.2] - 2025-04-19

### ⚙️ Miscellaneous Tasks

- *(release)* Release of the version 0.2.1
- Release smart-locker version 0.2.2

## [0.2.1] - 2025-04-19

### ⚙️ Miscellaneous Tasks

- Release smart-locker version 0.2.1

## [0.2.0] - 2025-04-19

### 🚀 Features

- Add backup and restore key functionality, export secrets to .env file, and improve init command
- Add post-release hook to update and push changelog

### 🐛 Bug Fixes

- *(tests)* Update secret name in test_list_secrets to match expected format

### 🚜 Refactor

- Reorder module exports and improve formatting in multiple files
- Standardize error handling and return types across commands and utilities

### ⚙️ Miscellaneous Tasks

- Bump version to 0.2.0 in Cargo.toml and Cargo.lock

## [0.1.5] - 2025-04-07

### 🚀 Features

- Update version to 0.1.5 and enhance error handling across commands

## [0.1.4] - 2025-03-31

### ⚙️ Miscellaneous Tasks

- Release smart-locker version 0.1.4

## [0.1.3] - 2025-03-31

### ⚙️ Miscellaneous Tasks

- Bump version to 0.1.3

## [0.1.2] - 2025-03-31

### 🐛 Bug Fixes

- *(ci)* Add artifact download step and update asset paths in release workflow

### ⚙️ Miscellaneous Tasks

- Release smart-locker version 0.1.2

## [0.1.2-rc.4] - 2025-03-31

### 🐛 Bug Fixes

- *(ci)* Update asset paths in release workflow for Linux and Windows binaries

### ⚙️ Miscellaneous Tasks

- Release smart-locker version 0.1.2-rc.4

## [0.1.2-rc.3] - 2025-03-31

### 🐛 Bug Fixes

- *(ci)* Update release asset naming and add debug steps for .deb and .rpm files
- Bump version to 0.1.2-rc.1 for smart-locker
- Update version to 0.1.2-rc.2 and improve release workflow for .deb and .rpm packages

### ⚙️ Miscellaneous Tasks

- Release smart-locker version 0.1.2-rc.3

## [0.1.2-rc.0] - 2025-03-31

### 🚀 Features

- *(ci)* Add artifact download and upload steps for Linux and Windows binaries

### 🐛 Bug Fixes

- *(ci)* Update release workflow to prevent pushing and tagging during dry run
- *(ci)* Change version type from beta to rc for preprod branch
- *(ci)* Improve version comparison logic for preprod and main branches

### 🧪 Testing

- *(release)* Test cargo release

### ⚙️ Miscellaneous Tasks

- Remove local CI/CD pipeline configuration

## [0.1.1-rc.9] - 2025-03-30

### 🐛 Bug Fixes

- *(ci)* Update release workflow to export version as environment variable and adjust tag naming
- *(ci)* Add pre-release hook to generate changelog with version tag

### ⚙️ Miscellaneous Tasks

- Sync version with latest rc tag (0.1.1-rc.8)
- Release smart-locker version 0.1.1-rc.9

## [0.1.1] - 2025-03-30

### 🐛 Bug Fixes

- *(ci)* Update release workflow to prevent publishing during dry-run
- *(ci)* Update release workflow to conditionally create and upload assets for main and preprod branches

### 💼 Other

- *(fmt)* Format code with rustfmt

### ⚙️ Miscellaneous Tasks

- *(ci)* Enhance release workflow to create and upload GitHub release assets
- *(ci)* Add check for main branch to skip version management
- Release smart-locker version 0.1.1

## [0.1.1-rc.6] - 2025-03-30

### 📚 Documentation

- Update README

### ⚙️ Miscellaneous Tasks

- *(ci)* Add pre-release version preparation step to release workflow
- *(ci)* Enhance pre-release workflow to sync Cargo.toml version with latest rc tag
- *(ci)* Enhance versioning logic in release workflow to support multiple version types
- *(ci)* Enhance versioning logic in release workflow to support dynamic version types
- *(ci)* Simplify version type determination and update cargo release command
- Sync version with latest rc tag (0.1.1-rc.5)
- Release smart-locker version 0.1.1-rc.6

## [0.1.1-rc.4] - 2025-03-30

### ⚙️ Miscellaneous Tasks

- *(ci)* Reorder system dependencies installation in release workflow
- *(ci)* Update CI workflow to use custom Docker image and streamline steps
- *(ci)* Update CI workflow to use specific Docker image for consistency
- *(ci)* Add Docker credentials to CI workflow for authentication
- *(ci)* Format Docker credentials section in release workflow for clarity
- *(ci)* Remove Docker credentials from release workflow for security
- *(ci)* Remove Docker credentials from CI workflow for security
- *(ci)* Update cargo release command in release workflow for accuracy
- *(ci)* Add support for .deb and .rpm package generation in release workflow
- *(ci)* Update cargo release command in release workflow and modify RPM metadata
- *(ci)* Add cargo build step to release workflow for package generation
- *(ci)* No code changes made
- Bump to 0.1.1-rc.3
- *(ci)* No code changes made
- Release smart-locker version 0.1.1-rc.4

## [0.1.1-rc.1] - 2025-03-29

### ⚙️ Miscellaneous Tasks

- *(ci)* Update release workflow to support multiple branches and enhance release process
- *(ci)* Add dependency on test and docs jobs in release workflow
- *(ci)* Refactor release workflow to include setup job and streamline dependency installation
- *(ci)* Move Rust target installation to a separate step in the release workflow
- Release smart-locker version 0.1.1-rc.1

## [0.1.0] - 2025-03-29

### 🚀 Features

- *(devops)* Automate binary delivery and release publication
- *(translation)* Translate all messages and comments to English

### 🐛 Bug Fixes

- *(ci)* Fix CI pipeline connection to repo and info
- *(ci)* Fix CI pipeline connection to repo and info
- Fix Windows installation and Docker simulation
- *(init)* Fix path for ~/.locker in init_locker()
- *(test)* Remove clipboard test from CI Docker
- *(test)* Remove clipboard test from CI Docker
- *(test)* Remove clipboard test from CI Docker
- *(test)* Remove clipboard test from CI Docker
- *(ci)* Fix CI pipeline and release tag + add changelog to GitLab
- *(devops)* Add release repo to .gitignore
- *(ci)* Fix commit format for changelog git-cliff
- *(ci)* Merge branch 'reformat-commits' into dev
- Ignore release-* directories in .gitignore
- Ignore release-* directories in .gitignore
- Ignore target/ directory in .gitignore and resolve conflicts in Cargo.lock

### ⚙️ Miscellaneous Tasks

- Initial commit
- *(release)* Pre-Release alpha
- *(release)* Pre release beta
- *(release)* Release of first stable version v0.1.0
- *(release)* Prepare release v0.1.0
- *(release)* Prepare release v0.1.0
- *(release)* Prepare release v0.1.0
- *(release)* Prepare release v0.1.0
- *(release)* Prepare release v0.1.0
- *(release)* Prepare release v0.1.0
- *(release)* Prepare release v0.1.0
- Merge dev into preprod for release v0.1.0
- *(release)* Prepare release v0.1.0
- *(release)* Prepare release v0.1.0
- *(release)* Prepare release v0.1.0
- *(release)* Prepare release v0.1.0
- Merge preprod into main for release v0.1.0
- *(ci)* Update documentation for clarity and consistency
- *(ci)* Update release workflow to initialize before running tests
- *(ci)* Add safe.directory configuration for GitHub Actions
- *(ci)* Enhance release workflow with version extraction and changelog generation
- *(ci)* Update release workflow to force push tags
- *(ci)* Modify release workflow to disable tagging and enable changelog overwrite
- *(ci)* Install git-cliff for enhanced changelog generation

<!-- generated by git-cliff -->
