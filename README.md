# wget

<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Tokio](https://img.shields.io/badge/Tokio-000000?style=for-the-badge&logo=rust&logoColor=white)
![Async](https://img.shields.io/badge/Async-ED8B00?style=for-the-badge)
![CLI](https://img.shields.io/badge/CLI-4EAA25?style=for-the-badge&logo=gnu-bash&logoColor=white)

</div>

## 📖 About

This project is a modern reimplementation of the classic GNU Wget utility, built from the ground up in Rust. It's designed to be a fast, reliable, and feature-rich command-line tool for downloading files from the web.

**What is wget?** Wget is a non-interactive network downloader that retrieves files using HTTP, HTTPS, and other protocols. This Rust version maintains the familiar interface while leveraging modern async/await patterns for superior performance.

**Why Rust?** By rebuilding wget in Rust, this project benefits from:
- **Memory safety** without garbage collection
- **Concurrent downloads** using Tokio's async runtime
- **Zero-cost abstractions** for high performance
- **Modern error handling** with Result types
- **Strong type system** preventing common bugs

**Use Cases:**
- Download files from web servers efficiently
- Mirror entire websites for offline browsing
- Automate batch downloads from URL lists
- Integrate into scripts and automation workflows
- Learn Rust by studying a practical CLI application

Whether you're downloading a single file or mirroring an entire website, this tool provides a robust and efficient solution with real-time progress tracking and flexible configuration options.

## ✨ Features

- **Asynchronous downloads** with tokio for high performance
- **Multiple URL downloads** from command line or input file
- **Website mirroring** with link conversion for offline viewing
- **Real-time progress bars** with download speed and ETA
- **Rate limiting** to control bandwidth usage
- **Selective downloads** with file type rejection and directory exclusion
- **Background mode** for long-running downloads
- **Flexible output** with custom paths and filenames

## 🚀 Installation

Make sure you have Rust installed. Then clone and build:

```bash
git clone https://github.com/yourusername/wget.git
cd wget
cargo build --release
```

The binary will be available at `target/release/wget`.

## 📖 Usage

### Basic Download

Download a single file:
```bash
wget https://example.com/file.zip
```

Download multiple files:
```bash
wget https://example.com/file1.zip https://example.com/file2.pdf
```

### Download from File

Download URLs listed in a file:
```bash
wget -i downloads.txt
```

### Custom Output

Specify output directory:
```bash
wget -P /path/to/directory https://example.com/file.zip
```

Specify output filename:
```bash
wget -O myfile.zip https://example.com/file.zip
```

### Rate Limiting

Limit download speed:
```bash
wget --rate-limit 200k https://example.com/largefile.iso
wget --rate-limit 2M https://example.com/largefile.iso
```

### Background Mode

Run download in background (output saved to `wget-log`):
```bash
wget -B https://example.com/largefile.iso
```

### Website Mirroring

Mirror an entire website for offline viewing:
```bash
wget --mirror --convert-links https://example.com
```

### Selective Downloads

Reject specific file types:
```bash
wget --mirror -R jpg,png,gif https://example.com
```

Exclude specific directories:
```bash
wget --mirror -X /ads,/tracking https://example.com
```

## ⚙️ Command-Line Options

| Option | Description |
|--------|-------------|
| `-i <file>` | Read URLs from input file |
| `-O <name>` | Save file with specified name |
| `-P <path>` | Save files to specified directory |
| `-B` | Run in background mode |
| `--rate-limit <rate>` | Limit download speed (e.g., "200k", "2M") |
| `--mirror` | Mirror website recursively |
| `-R, --reject <types>` | Comma-separated list of file extensions to reject |
| `-X, --exclude <paths>` | Comma-separated list of directories to exclude |
| `--convert-links` | Convert links for offline viewing |

## 💡 Examples

Mirror a website excluding images:
```bash
wget --mirror --convert-links -R jpg,jpeg,png,gif,svg https://blog.example.com
```

Download files from list with rate limiting:
```bash
wget -i urls.txt --rate-limit 500k -P ./downloads
```

Background download with progress logging:
```bash
wget -B https://releases.ubuntu.com/22.04/ubuntu-22.04.3-desktop-amd64.iso
```

## 📦 Dependencies

- **tokio** - Asynchronous runtime
- **reqwest** - HTTP client with async support
- **clap** - Command-line argument parsing
- **indicatif** - Progress bars and spinners
- **scraper** - HTML parsing for website mirroring
- **futures-util** - Async utilities for streaming
- **chrono** - Date and time formatting

## 📁 Project Structure

```
src/
├── main.rs         # Entry point
├── lib.rs          # Library exports
├── args.rs         # Command-line argument definitions
├── downloader.rs   # Core download logic
└── mirror.rs       # Website mirroring functionality
```

## 🔨 Building from Source

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with arguments
cargo run -- https://example.com/file.zip
```

## 🙏 Acknowledgments

Inspired by the original GNU Wget utility, reimplemented in Rust with modern async capabilities.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

<div align="center">

**⭐ Star this repository if you found it helpful! ⭐**

Made with ❤️ from 🇸🇳

</div>