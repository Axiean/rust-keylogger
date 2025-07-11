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

### 4. Compile the Binder Binary

```bash
cargo build --release --bin binder
```

You’ll get target/release/binder.exe , this is your final payload that runs the decoy and background logger.

---

## 🧪 Testing Instructions

1. Use a Virtual Machine or Isolated Environment (NEVER run this on your main OS).

2. Run binder.exe. The PDF will open as expected.

3. Meanwhile, the keylogger will silently start, capturing keystrokes and sending logs to your Discord channel.

4. Every 10 minutes, logs are flushed to the webhook and the local file is cleared.

---

## ❗ Ethical Usage Reminder

This project is a learning tool for red teamers, malware analysts, and ethical hackers to:

- Understand keylogging mechanisms.

- Simulate social engineering via file binding.

- Build stronger blue team detection strategies.

By using or cloning this project, you agree to use it solely in ethical, permitted environments, such as:

- Your own systems or VMs

- Offensive security labs

- Red team exercises with explicit permission

---

## 🛡️ Blue Team Mitigation Insights

This tool demonstrates real-world attacker behavior, which can be detected through:

- Monitoring abnormal .scr execution

- Detecting outbound traffic to Discord domains

- Watching for GetAsyncKeyState or CreateFileA usage

- Application whitelisting or endpoint behavior analytics (EDR)

---

## 🧠 Ideas for Future Improvements

- 🔐 Encrypt logs before sending.

- 📦 Use a custom packer to obfuscate binary.

- 🧬 Add persistence via registry or scheduled tasks.

- 🎭 Use process injection or memory-only execution for stealth.

- 🗝️ Include anti-debugging or sandbox evasion routines.

---

## 📢 Disclaimer

This project is intended exclusively for:

- Red Team education

- Blue Team defense simulation

- Malware analysis training

Any misuse of this tool, including unauthorized deployment, violates the ethical use policy and may be illegal in your country. The author assumes no liability for misuse.
