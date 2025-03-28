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

> 📝 **Note:** If you encounter any issues during installation, please check the [Issues section](https://github.com/WillIsback/smart-locker/issues) or open a new ticket.

> 🦀🔐 *SmartLocker is a personal project to explore Rust deeply while building a useful security tool for everyday DevOps workflows.*
```

