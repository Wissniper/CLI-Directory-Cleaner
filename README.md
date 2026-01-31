# CLI Directory Cleaner (Rust CLI Tool)

[![Rust CI Pipeline](https://github.com/Wissniper/CLI-Directory-Cleaner/actions/workflows/ci.yml/badge.svg)](https://github.com/Wissniper/CLI-Directory-Cleaner/actions)
[![Release Builder](https://github.com/Wissniper/CLI-Directory-Cleaner/actions/workflows/release.yml/badge.svg)](https://github.com/Wissniper/CLI-Directory-Cleaner/releases)
![Language](https://img.shields.io/badge/Language-Rust-orange)
![License](https://img.shields.io/badge/License-MIT-blue)

**A high-performance, concurrent CLI tool to organize chaotic directories.**

As a Computer Science student at Ghent University, my Downloads folder is often a mess of PDFs, scripts, and images. I built this tool to learn **Rust** and **Systems Programming** concepts by creating something that is not just educational, but actually useful.

It recursively scans a target directory and organizes files into subfolders based on their extensions, using multi-threading to handle thousands of files in milliseconds.

---

## Key Features

* **Fast:** Uses `rayon` for data parallelism to process files concurrently across all CPU cores.
* **Memory Safe:** Built with Rust's strict ownership and borrowing rules (no segfaults, no garbage collection pauses).
* **Automated CI/CD:** Fully automated testing and release pipeline using GitHub Actions.
* **Error Handling:** Handles permission errors (e.g., open files) using Rust's `Result<T, E>` pattern.

---

## Installation

You do not need to install Rust to use this tool. You can download the pre-compiled binary for your OS from the [Releases Page](https://github.com/Wissniper/CLI-Directory-Cleaner/releases).

### Option 1: Download Binary (Recommended)
1. Go to **[Releases](https://github.com/Wissniper/CLI-Directory-Cleaner/releases)**.
2. Download the version for your OS:
   * `directory-cleaner-windows.exe` (Windows)
   * `directory-cleaner-linux` (Linux)
   * `directory-cleaner-macos` (macOS)
3. Run it from your terminal.

### Option 2: Build from Source
If you have `cargo` installed:
```bash
git clone [https://github.com/Wissniper/CLI-Directory-Cleaner.git](https://github.com/Wissniper/CLI-Directory-Cleaner.git)
cd directory-cleaner
cargo build --release
```

---

## Usage

```bash
# Basic Usage: Organize the current folder
./directory-cleaner --path .

# Organize a specific target folder
./directory-cleaner --path ./Downloads

# Dry Run (See what WOULD happen without moving files)
./directory-cleaner --path ./Downloads --dry-run
```

**Output Example:**

```text
Scanning directory: ./Downloads
Found 1,402 files. Processing with Rayon...
-------------------------------------------
[SUCCESS] Moved report.pdf -> ./Downloads/pdf/report.pdf
[SUCCESS] Moved script.py -> ./Downloads/python/script.py
[SKIP]    Locked file detected: system.log
-------------------------------------------
Done! Processed 1,402 files in 0.12 seconds.
```

---

## Project Structure

```plaintext
directory-cleaner/
├── .github/                 # GitHub configuration folder
│   └── workflows/           # Where your CI/CD pipelines live
│       ├── ci.yml           # The "Inspector" (Test & Check)
│       └── release.yml      # The "Factory" (Build & Publish)
├── src/                     # The actual Rust code goes here
│   ├── main.rs              # The entry point (initializes CLI, calls logic)
│   ├── args.rs              # Defines the Clap struct (CLI arguments)
│   └── logic.rs             # Contains the moving/scanning functions
├── target/                  # (Auto-generated) Compiled binaries live here
├── .gitignore               # Tells git to ignore the 'target' folder
├── Cargo.lock               # (Auto-generated) Exact versions of dependencies
├── Cargo.toml               # Project metadata and dependencies (like package.json)
├── LICENSE                  # The MIT License text file
└── README.md                # The documentation
```

## Technical Implementation

This was my first deep dive into **Rust**. Moving from C/C++ to Rust required a shift in mindset, specifically regarding memory ownership and concurrency.

### 1. Concurrency with `Rayon`

Instead of a standard `for` loop, I utilized Rayon's parallel iterator. This turns a synchronous task into a parallel one, automatically distributing the workload across available CPU threads.

```rust
// Standard Iteration (Single Threaded)
files.iter().for_each(|file| process(file));

// Rayon Iteration (Multi-Threaded)
files.par_iter().for_each(|file| process(file));
```

### 2. Ownership & Thread Safety (`Arc<Mutex>`)

To track statistics (e.g., "50 PDFs moved") across multiple threads, I had to use an `Arc` (Atomic Reference Counter) to share ownership of the data, wrapped in a `Mutex` to ensure safe concurrent writes.

### 3. CI/CD Pipeline (GitHub Actions)

I implemented a full CI/CD pipeline to automate quality control:

* **CI (`ci.yml`):** Runs `cargo fmt`, `clippy`, and `cargo test` on every push to ensure code quality and style consistency.
* **CD (`cd.yml`):** Automatically compiles binaries for **Windows, Linux, and macOS** when a new tag (e.g., `v1.0.0`) is pushed.
---

## Dependencies

* [clap](https://crates.io/crates/clap) - Command Line Argument Parsing.
* [walkdir](https://crates.io/crates/walkdir) - Efficient recursive directory traversal.
* [rayon](https://crates.io/crates/rayon) - Data parallelism library.
* [anyhow](https://crates.io/crates/anyhow) - Idiomatic error handling.

---

## License

This project is licensed under the MIT License - see the [LICENSE](https://www.google.com/search?q=LICENSE) file for details.

---

*Built with ❤️ (and a lot of fighting with the Borrow Checker) by a UGent CS Student.*

