# 🎨 RASCII - ASCII Art Generator CLI

![Build](https://img.shields.io/badge/build-passing-brightgreen)
![Rust](https://img.shields.io/badge/Rust-2021-orange)
![Platform](https://img.shields.io/badge/platform-cross--platform-lightgrey)

**RASCII** is a blazing-fast CLI tool written in Rust that converts **images** and **videos** into expressive **ASCII art**.

---

## Features

- 📷 Terminal-rendered ASCII from images
- 🖼️ Save ASCII art as high-resolution PNGs
- 🎞️ Convert video files into ASCII animations
- 🧠 Supports named or custom output resolutions


---
## Installation

For now you need [Rust and Cargo](https://www.rust-lang.org/tools/install) installed:

```bash
git clone https://github.com/your-user/rascii
cd rascii
cargo build --release
```
The binary will be located at: `target/release/rascii`. if you want the tool to be accessible from anywhere, add it to `/usr/local/bin/`

Alternatively run `cargo install --path .`

---
## Commands & Options

Below are the main CLI options and example usages:

### 1. console
Converts an image from a file to ascii, and returns it to the terminal

**Command:**
```
rascii console --input <input_image> [options]
```

**Options:**
- `-i, --input <input_video>`: Path to the input video (required)
- `-w, --width <width>`: Set output width
- `-s, --style <style>`: Set a style (see styles section)

**Example:**
```
rascii console --input h.png --width 80 --style 3
```

---
### 2. image
Converts an image to ASCII and saves it as an image file.

**Command:**
```
rascii image --input <input_image> [options]
```

**Options:**
- `-i, `--input <input_image>`: Path to the input image (required)
- `-r, --res <resolution>`: Output resolution (default: fhd). Supports named resolutions: 2k, fhd, wxga Or custom: WIDTHxHEIGHT
- `-w, --char-width <char_width>`: Width in characters (default: 100)
- `-f, --f-size <font_size>`: Font size (default: 9.0)
- `-s, --style <style>`: ASCII art style (default: 0)
- `-o, --output <output_file>`: Output file name (default: res.png)

**Example:**
```
rascii image --input photo.jpg --res 2k --char-width 120 --f-size 10.0 --style 2 --output ascii_art.png

```

---

### 3. video
Converts a video to ASCII and saves it as a video file.

**Command:**
```
rascii video --input <input_video> [options]
```

**Options:**
- `-i, --input <input_video>`: Path to the input video (required)
- `-r, --res <resolution>`: Output resolution (default: fhd). Supports named resolutions: 2k, fhd, wxga Or custom: WIDTHxHEIGHT
- `-w, --char-width <char_width>`: ASCII character width per frame (default: 120)
- `-f, --f-size <font_size>`: Font size (default: 9.0)
- `-s, --style <style>`: ASCII art style (default: 0)
- `--fps <fps>`: Frames per second of the output video (default: matches source video)
- `-o, --output <output_file>`: Output video file (default: ascii_res.mp4)

**Example:**
```
rascii video --input test/v_test.mp4 --res fhd --char-width 100 --f-size 8.5 --style 1 --fps 15 --output ascii_video.mp4

```

#### Style section
1. Minimal char set
2. Inverted minimal char set
3. Extended char set
4. Inverted extended char set

---
## Help and Version
`rascii help`
`rascii help console`
`rascii help image`
`rascii help video`
`rascii --version`

---
## Requirements
- Rust (latest stable)
