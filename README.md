# Auto Declutter

A lightweight command-line utility written in Rust that organizes files into subdirectories based on their file extensions.

## Features

- Scans the files directly inside a target directory.
- Creates one subdirectory per file extension when needed.
- Moves each file into the matching extension folder.
- Leaves directories and files without an extension untouched.
- Uses only the Rust standard library; no external dependencies are required.

> **Current scope:** Auto Declutter processes only the immediate contents of the selected directory. It does not recursively scan nested directories.

## Requirements

- Rust stable with support for the **2024 edition**.
- Cargo, installed through [rustup](https://rustup.rs/) or your preferred Rust toolchain manager.

## Installation

Clone the repository and build the project with Cargo:

```bash
git clone https://github.com/Aledev746/auto-declutter.git
cd auto-declutter
cargo build --release
```

The compiled executable is generated at:

`target/release/file_organizer`

The executable name currently follows the package name declared in `Cargo.toml`.

## Usage

Run the application by passing the path of the directory you want to organize:

```bash
cargo run -- /path/to/your/folder
```

For example:

```bash
cargo run --release -- ~/Downloads
```

If the directory contains files such as:

```text
Downloads/
├── photo.jpg
├── report.pdf
└── notes.txt
```

Auto Declutter organizes them as follows:

```text
Downloads/
├── jpg/
│   └── photo.jpg
├── pdf/
│   └── report.pdf
└── txt/
    └── notes.txt
```

## How It Works

1. The target directory is read from the first command-line argument.
2. The application examines each direct child of that directory.
3. Files with an extension are assigned to a folder named after that extension.
4. Missing extension folders are created automatically.
5. Files are moved using the filesystem rename operation.

## Important Notes

- **Files are moved immediately.** There is currently no dry-run mode or undo functionality. Test with a copy of important data first.
- **Nested directories are not scanned.** Files inside subdirectories are not processed.
- **Files without extensions are skipped.**
- **Extensions are case-sensitive.** For example, `.jpg` and `.JPG` are treated as different categories.
- **Name collisions are not handled automatically.** Moving a file to an occupied destination may fail.
- The application reports errors through the command line and stops when a required filesystem operation fails.

## Project Structure

```text
src/
└── main.rs       # Application entry point and file-organization logic
Cargo.toml        # Package metadata and Rust configuration
Cargo.lock        # Locked dependency metadata
.gitignore        # Ignores Cargo build artifacts
LICENSE           # MIT License
```

## Development

Check and format the code locally with:

```bash
cargo check
cargo fmt -- --check
cargo test
```

Automated tests and GitHub Actions workflows are not included yet.

## Roadmap

Potential improvements include:

- Add a dry-run mode and a confirmation prompt.
- Support recursive directory traversal.
- Normalize extension casing.
- Handle duplicate filenames safely.
- Improve user-facing error messages.
- Add automated tests and continuous integration.
- Provide configurable sorting rules and destination folders.

## License

This project is licensed under the [MIT License](LICENSE).
