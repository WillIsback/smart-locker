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

SmartLocker is available for **Linux** and **Windows**. You can either build it from source (for Rust users) or download a ready-to-use binary archive.

---

### ✅ Recommended: Precompiled Binary

#### **Windows**

1. Download the latest release from the [Releases page](https://github.com/WillIsback/smart-locker/releases).
2. Extract the `smartlocker_windows.zip` archive.
3. (Optional) Add the folder to your `PATH` environment variable for easier use.
4. Run `smart-locker.exe` from any terminal (PowerShell, cmd, or Windows Terminal).

#### **Linux**

1. Download the latest release from the [Releases page](https://github.com/WillIsback/smart-locker/releases).
2. Extract the `smartlocker_linux.tar.gz` archive:
   ```bash
   tar -xzf smartlocker_linux.tar.gz
   ```
3. Move the binary into your path:
   ```bash
   sudo mv dist/smart-locker /usr/local/bin/
   ```
4. Run:
   ```bash
   smart-locker --version
   ```

---

### ⚙️ Build from Source (For Developers)

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

4. Install the binary:
   ```bash
   sudo cp target/release/smart-locker /usr/local/bin/
   ```

5. Verify:
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

3. Build in release mode:
   ```powershell
   cargo build --release
   ```

4. (Optional) Add to PATH:
   ```powershell
   $Env:Path += ";$PWD\target\release"
   ```

5. Test:
   ```powershell
   .\target\release\smart-locker.exe --version
   ```

---

### 🧪 Quick Test After Install

```bash
smart-locker init
smart-locker encrypt -n my_secret -v "This is a test"
smart-locker decrypt -n my_secret
```

---
### **🔧 Features and Capabilities of SmartLocker CLI**

The `SmartLocker` CLI provides a robust and user-friendly interface for securely managing sensitive secrets. Below is a detailed breakdown of its features and capabilities:

---

### **🔑 Initialization (`init`)**
- **Purpose:** Initializes the vault by creating a secure folder (`~/.locker`) and generating a symmetric key (`locker.key`).
- **Options:**
  - `--passphrase`: Generate the symmetric key from a passphrase using PBKDF2.
- **Examples:**
  ```bash
  # Generate a random symmetric key
  smart-locker init

  # Generate a symmetric key from a passphrase
  smart-locker init --passphrase "my_secure_passphrase"
  ```

---

### **🔒 Encryption (`encrypt`)**
- **Purpose:** Encrypts a secret and stores it securely in the vault.
- **Options:**
  - `--name (-n)`: Name of the secret (required).
  - `--value (-v)`: Value of the secret to encrypt (optional). If not provided, the value is read from `stdin`.
- **Examples:**
  ```bash
  # Encrypt a secret with a value
  smart-locker encrypt -n my_secret -v "This is a test"

  # Encrypt a secret by reading the value from stdin
  echo "This is a test" | smart-locker encrypt -n my_secret
  ```

---

### **🔓 Decryption (`decrypt`)**
- **Purpose:** Decrypts a secret and either displays it in the terminal or copies it to the clipboard.
- **Options:**
  - `--name (-n)`: Name of the secret to decrypt (required).
  - `--clipboard (-c)`: Copies the decrypted secret to the clipboard instead of displaying it.
- **Examples:**
  ```bash
  # Decrypt a secret and display it in the terminal
  smart-locker decrypt -n my_secret

  # Decrypt a secret and copy it to the clipboard
  smart-locker decrypt -n my_secret --clipboard
  ```

---

### **📜 Listing Secrets (`list`)**
- **Purpose:** Lists all the secrets stored in the vault.
- **Examples:**
  ```bash
  # List all secrets
  smart-locker list
  ```

---

### **🗑️ Removing Secrets (`remove`)**
- **Purpose:** Deletes a secret from the vault.
- **Options:**
  - `--name (-n)`: Name of the secret to delete (required).
- **Examples:**
  ```bash
  # Remove a secret
  smart-locker remove -n my_secret
  ```

---

### **📋 Clipboard Integration**
- **Purpose:** Allows seamless copying of decrypted secrets to the clipboard for quick use.
- **Supported Platforms:**
  - **Linux:** Uses `copypasta` for clipboard management.
  - **Windows (WSL):** Uses `clip.exe` for clipboard integration.
- **Examples:**
  ```bash
  # Decrypt a secret and copy it to the clipboard
  smart-locker decrypt -n my_secret --clipboard
  ```

---

### **📂 Vault Structure**
The vault is stored in the `~/.locker` directory and contains:
- `locker.key`: The symmetric key used for encryption and decryption.
- `.slock` files: Encrypted secrets, named after the secret's name.

Example structure:
```
~/.locker/
├── locker.key         # Symmetric key
├── my_secret.slock    # Encrypted secret
├── api_key.slock      # Encrypted secret
└── db_password.slock  # Encrypted secret
```

---

### **🛠️ Additional Features**
- **Pipe Support:** Allows secrets to be passed via `stdin` for encryption.
  ```bash
  echo "my_secret_value" | smart-locker encrypt -n my_secret
  ```
- **Cross-Platform Compatibility:** Works seamlessly on Linux and Windows.
- **Customizable Key Generation:** Use a passphrase to derive the symmetric key for added security.
---

### **🚀 Example Workflow**
```bash
# Step 1: Initialize the vault
smart-locker init

# Step 2: Encrypt a secret
smart-locker encrypt -n my_secret -v "This is a test"

# Step 3: List all secrets
smart-locker list

# Step 4: Decrypt a secret and display it
smart-locker decrypt -n my_secret

# Step 5: Decrypt a secret and copy it to the clipboard
smart-locker decrypt -n my_secret --clipboard

# Step 6: Remove a secret
smart-locker remove -n my_secret
```

---

> 📝 **Note:** If you encounter any issues during installation, please check the [Issues section](https://github.com/WillIsback/smart-locker/issues) or open a new ticket.

> 🦀🔐 *SmartLocker is a personal project to explore Rust deeply while building a useful security tool for everyday DevOps workflows.


## 📝 License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 🔃 Changelog
See the [CHANGELOG](CHANGELOG.md) for a detailed list of changes and updates.

## 📜 Contributing
Please use the commit message format `feat: <description>` for new features and `fix: <description>` for bug fixes. For more details, see the [Contributing Guide](CONTRIBUTING.md).
