# Cover Library (`cover-files`)

The `cover-files` crate provides the core logic for the Cover application, including file synchronization, archiving, restoring, and backup management.  
This crate is meant to be used by the [`cover`](https://crates.io/crates/cover) binary.

## Table of Content

- [Features](#features)
- [Docs](#docs)
- [License](#license)
- [Contributing](#contributing)

## Features

### Current Features

- **Sync** files and directories between source and destination
- **Change Detection**: sync only changed files
- **Delete Mode**: remove files in destination that no longer exist in source

### Upcoming Features

- **Archive** projects into `.zip` or `.tar.gz`
- **Restore** backups
- **Schedule** automatic backups (daily, weekly, interval-based)
- **List** archives and scheduled jobs
- **Clean** old backups with rules (`--keep-last`, `--older-than`)

## Docs

Here is the docs that you can read it yourself.

- [Cover Docs](https://docs.rs/crate/cover-files/)

## License

This project is licensed under the [Apache-2.0 License](LICENSE).

## Contributing

Contributions, issues, and feature requests are welcome! Open a PR or file an issue on [GitHub](https://github.com/ibilalkayy/cover/issues).
