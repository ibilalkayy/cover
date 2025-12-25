<p align="center">
  <img src="logo.png" alt="Logo" width="500"/>
</p>

<p align="center">
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/license-Apache--2.0-blue.svg" alt="License">
  </a>
  <a href="https://www.rust-lang.org">
    <img src="https://img.shields.io/badge/Rust-stable-orange" alt="Rust">
  </a>
</p>

✨ If you would like to help spread the word of Cover, please consider starring the repo!

# Table of Content

- [What is Cover?](#what-is-cover)
- [Features](#features)
  - [Present Features](#present-features)
  - [Upcoming Features](#upcoming-features)
- [Get Started](#get-started)
- [Installation](#installation)
- [License](#license)
- [Contributing](#contributing)

# What is Cover?

Cover is a Rust application where the source and destination are synced and any change in the source will be happening in the destination at the same time.

## Features

### Present Features

- **Sync** files and folders of source with destination

### Upcoming Features

- **Archive** projects into `.zip` or `.tar.gz`
- **Restore** from backups easily
- **Schedule** automatic backups (daily, weekly, interval-based)
- **List** archives and scheduled jobs
- **Clean** old backups with rules (`--keep-last`, `--older-than`)

## Get Started

```bash
cargo add cover
```

```bash
cover sync
cover archive
cover restore
cover schedule
cover list
cover clean
cover help
```

```bash
cover sync --source src_directory --destination dest_directory --changed-only
cover sync --source src_directory --destination dest_directory --delete
cover sync --source src_directory --destination dest_directory --dry-run
cover sync --source src_directory --destination dest_directory --verbose
```

**Note:** Other commands are in the process.

## Installation

Fork, clone and build from the source:

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

## License

This project is licensed under the [Apache-2.0 License](LICENSE).

## Contributing

Contributions, issues, and feature requests are welcome! Feel free to open a PR or file an issue on [GitHub](https://github.com/ibilalkayy/cover/issues).
