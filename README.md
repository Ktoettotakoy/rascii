# 🎨 RASCII - ASCII Art Generator CLI

![Build](https://img.shields.io/badge/build-passing-brightgreen)
![License](https://img.shields.io/badge/license-MIT-blue)
![Rust](https://img.shields.io/badge/Rust-2021-orange)
![Platform](https://img.shields.io/badge/platform-cross--platform-lightgrey)

**RASCII** is a blazing-fast CLI tool written in Rust that converts **images** and **videos** into expressive **ASCII art**.

---

## Features

- 📷 Terminal-rendered ASCII from images
- 🖼️ Save ASCII art as high-resolution PNGs
- 🎞️ Convert video files into ASCII animations
- 🎨 Multiple styles and font options
- 🧠 Supports named or custom output resolutions

---
## Help and Version
rascii --help
rascii text --help
rascii image --help
rascii video --help
rascii --version

---
## Installation

For now you need [Rust and Cargo](https://www.rust-lang.org/tools/install) installed:

```bash
git clone https://github.com/your-user/rascii
cd rascii
cargo build --release
```
The binary will be located at: `target/release/rascii`
