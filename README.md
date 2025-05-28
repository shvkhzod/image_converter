# 🖼️ image_converter

A blazing-fast, easy-to-use Rust CLI tool to batch convert and compress images to the modern [WebP](https://developers.google.com/speed/webp) format.

![Rust](https://img.shields.io/badge/Rust-2021-orange?logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![CI](https://img.shields.io/github/workflow/status/shvkhzod/image_converter/CI)

---

## ✨ Features

- 🚀 **Batch conversion**: Convert multiple images at once.
- 🗜️ **Compression**: Adjustable quality for optimal file size.
- 🖼️ **Format support**: Accepts PNG, JPEG, and more.
- ⚡ **Fast**: Built with Rust for speed and reliability.
- 🛠️ **Simple CLI**: Easy to use with helpful flags.

---

## 📦 Installation

### With Cargo

```sh
cargo install --git [https://github.com/shvkhzod/image_converter](https://github.com/shvkhzod/image_converter)

git clone https://github.com/shvkhzod/image_converter.git
cd image_converter
cargo build --release

cargo run --release -- input.jpg --output-dir webp_output --quality 80
