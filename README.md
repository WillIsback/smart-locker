# 🦀🔐 SmartLocker

A ultra-lightweight CLI tool written in **Rust** to **encrypt, store, and manage sensitive secrets locally** in a secure and durable way.

## 🚀 Purpose

SmartLocker solves a real-world problem:

> In a fullstack project with CI/CD pipelines, **tokens, private keys, passphrases, and API keys** become critical.

Storing them in plain text is dangerous. Base64 encoding is not enough. SmartLocker offers a reliable, simple, and effective solution.

## 🎯 Key Features

- ✅ Symmetric encryption of secrets (via AES-GCM or similar)
- ✅ Quick decryption using a passphrase or master key
- ✅ Secure folder `~/.locker`
- ✅ Encrypted files with `.slock` or `.aes` extension
- ✅ User-friendly CLI: `smartlocker encrypt`, `decrypt`, `list`, etc.
- ✅ Pipe support (e.g. `cat secret.txt | smartlocker encrypt -n my_secret`)
- ✅ Option: generate key from hashed passphrase (PBKDF2)
- ✅ Option: copy decrypted secret to clipboard
- 🔜 Option: Git pre-commit hook to prevent secret leaks
- 🔜 Option: vault with automatic expiration

## 🗂️ Target Directory Structure

```
~/.locker/
├── locker.key         # local symmetric key (or derived from a passphrase)
├── openai_token.slock
├── ssh_key_prod.slock
└── mydb_pass.slock
```

## 🛠️ CLI Architecture

```
smartlocker <command> [options]

MAIN COMMANDS:
  encrypt      Encrypt a secret and store it
  decrypt      Decrypt a `.slock` file
  list         List encrypted secrets
  remove       Delete a secret
  init         Generate the master key (locker.key)

EXAMPLE:
  smartlocker encrypt -n openai_token -v sk-abc123...
  smartlocker decrypt -n openai_token
```

## 📦 Tech Stack

- 🦀 **Rust** (>= 1.74)
- 📦 `aes-gcm`, `rand`, `clap`, `serde`, `directories`
- 🔐 Secure encryption based on AES-256 GCM

## 🧱 Future Steps

- [ ] Add vault with auto-expiration
- [V] `--clipboard` option to temporarily copy secrets into RAM
- [ ] Git pre-commit plugin to block secrets from being committed

## 📈 Why This Project?

Because managing secrets in a fullstack project means:

- Understanding security pitfalls
- Building reliable and portable tools
- Learning how to secure DevOps workflows

---

## 🧠 System Diagram

```
                +---------------------------+
                |     smartlocker init      |
                +-------------+-------------+
                              |
                         Generates key 🔑
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
     CLI input or STDIN                     Read encrypted file
          |                                        |
   `.slock` encrypted file         →        Decrypted secret
```

---

## 🛠️ Installation

SmartLocker can be installed on **Linux** and **Windows**. Below are several installation methods tailored to your setup.

---

### 📦 Automated Installation

#### **Linux (via Bash script)**

Run the following script to download, build, and install SmartLocker:

```bash
curl -fsSL https://raw.githubusercontent.com/WillIsback/smart-locker/main/install.sh | bash
```

This script:
1. Checks if **Rust** is installed.
2. Clones the GitHub repository.
3. Builds the project in `release` mode.
4. Installs the binary into `/usr/local/bin`.

#### **Windows (via PowerShell)**

Run this command in PowerShell to download and install SmartLocker:

```powershell
Invoke-WebRequest -Uri https://raw.githubusercontent.com/WillIsback/smart-locker/main/install.ps1 -OutFile install.ps1; ./install.ps1
```

This script:
1. Verifies that **Rust** is installed.
2. Clones the GitHub repository.
3. Builds the project in `release` mode.
4. Copies the binary to a folder included in your system `PATH`.

---

### 🛠️ Manual Installation

#### **Linux**

1. Make sure **Rust** is installed:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone the repository:
   ```bash
   git clone https://github.com/WillIsback/smart-locker.git
   cd smart-locker
   ```

3. Build the project in release mode:
   ```bash
   cargo build --release
   ```

4. Install the binary into `/usr/local/bin`:
   ```bash
   sudo cp target/release/smart-locker /usr/local/bin/
   ```

5. Verify the installation:
   ```bash
   smart-locker --version
   ```

#### **Windows**

1. Install **Rust** via [rustup](https://rustup.rs/).

2. Clone the repository:
   ```powershell
   git clone https://github.com/WillIsback/smart-locker.git
   cd smart-locker
   ```

3. Build the project in release mode:
   ```powershell
   cargo build --release
   ```

4. Add the binary to your `PATH`:
   ```powershell
   $Env:Path += ";$PWD\target\release"
   ```

5. Verify the installation:
   ```powershell
   smart-locker --version
   ```

---

### 📦 Precompiled Packages (coming soon)

We plan to provide **precompiled binaries** for major platforms (Linux, Windows, macOS). You’ll be able to download them directly from the [Releases page](https://github.com/WillIsback/smart-locker.git/1.0.0).

---

### 🧪 Test Your Installation

Once installed, test SmartLocker with the following commands:

```bash
smart-locker init
smart-locker encrypt -n my_secret -v "This is a test"
smart-locker decrypt -n my_secret
```

---

> **Note:** If you encounter any issues during installation, please check the [Issues section](https://github.com/WillIsback/smart-locker/issues) or open a new ticket.

> 🔐 A personal project to dive deep into Rust while building a useful everyday tool.
```

