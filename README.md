# Cover

A fast and reliable Rust-powered command-line tool for syncing, archiving, restoring, and scheduling backups with ease.

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-stable-orange)](https://www.rust-lang.org)

---

## ✨ Features

- 🔄 **Sync** files and folders with incremental updates
- 📦 **Archive** projects into `.zip` or `.tar.gz` (optional encryption & timestamps)
- ♻️ **Restore** from backups easily
- ⏰ **Schedule** automatic backups (daily, weekly, interval-based)
- 📋 **List** archives and scheduled jobs
- 🧹 **Clean** old backups with rules (`--keep-last`, `--older-than`)

---

## 🚀 Installation

Clone and build from source:

```bash
git clone https://github.com/ibilalkayy/cover.git
cd cover
cargo build --release
````

The binary will be available at:

```
target/release/cover
```

Optionally, move it to your `$PATH`:

```bash
cp target/release/cover /usr/local/bin/cover
```

## 🛡 License

This project is licensed under the [Apache-2.0 License](LICENSE).

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome!
Feel free to open a PR or file an issue on [GitHub](https://github.com/ibilalkayy/cover/issues).
