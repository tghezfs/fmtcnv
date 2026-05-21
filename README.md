# fmtcnv

![License](https://img.shields.io/github/license/tghezfs/fmtcnv)
![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange)

## 📖 About

`fmtcnv` is a fast CLI tool to convert between **JSON**, **TOML**, and **YAML** formats. Built in **Rust** for performance and reliability.

## ✨ Features

- **All-in-One Conversion:** Supports conversion between **JSON**, **TOML**, and **YAML** in any direction.
- **Smart Format Detection:** Automatically identifies input format if the file extension is missing or unknown.
- **Flexible Output:** Allows custom output paths and filenames; defaults to the current execution directory.

## 📦 Installation

### Prerequisites

- **Rust** 1.85.1+
- **Git**

### Option 1: Via Cargo (Recommended)

Installs the binary directly to your Cargo bin directory.

```bash
cargo install --git https://github.com/tghezfs/fmtcnv.git
```

### Option 2: Manual Build

```bash
git clone https://github.com/tghezfs/fmtcnv.git
cd fmtcnv
cargo build --release
# Binary located at ./target/release/fmtcnv
```

## 🚀 Usage

### Basic Syntax

```bash
fmtcnv --file <INPUT> --to-format <FORMAT> [--out-file <OUTPUT>]
```

### Examples

Convert JSON to YAML

```bash
fmtcnv -f config.json -t yaml
```

Convert TOML to JSON with custom output path

```bash
fmtcnv --file data.toml --to-format json --out-file ./output/data.json
```

View Help

```bash
fmtcnv --help
```

Options

| Flag              | Description                                     | Required |
| :---------------- | :---------------------------------------------- | :------- |
| `-f, --file`      | Input file path (auto-detects format)           | ✅ Yes   |
| `-t, --to-format` | Target format (`json`, `toml`, `yaml` or `yml`) | ✅ Yes   |
| `-o, --out-file`  | Custom output path (defaults to current dir)    | ❌ No    |

## 🤝 Contributing

Feel free to open an issue if you find a bug or have a suggestion.

## License

MIT
