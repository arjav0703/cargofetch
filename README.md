# Cargofetch 🦀

Cargofetch is a lightweight CLI tool written in Rust that fetches metadata about your Rust project.

![Image](https://hc-cdn.hel1.your-objectstorage.com/s/v3/08442b9bd462c34a0aa9944b1599ea74080ee375_image.png)

## Installation

#### Install via Cargo (Rust's package manager)

If you have Cargo installed, you can easily install cargofetch by running the following command:
```bash
cargo install cargofetch
```
#### 🍺 Homebrew (macOS)
Add homebrew tap and install cargofetch:
```bash
brew tap arjav0703/cargofetch https://github.com/arjav0703/cargofetch.git
brew install arjav0703/cargofetch/Cargofetch
```

#### AUR
If you are using an Arch-based Linux distribution, you can install cargofetch from the AUR:
```bash
yay -S cargofetch
```

#### Download Pre-built Binary
You can download the pre-built binary for your platform from the [releases page](https://github.com/arjav0703/cargofetch/releases).

---
### Building from Source

To build cargofetch from source, clone the repository and use Cargo to build it:
```bash
git clone https://github.com/arjav0703/cargofetch.git
cd cargofetch
cargo build --release
```
The binary will be located in the target/release/ directory.


## Acknowledgements
This projcet was inspired by [neofetch](https://en.wikipedia.org/wiki/Neofetch) and [onefetch](https://onefetch.dev). Also, the ASCII art logo was also taken from onefetch.

## License

This project is licensed under the MIT License - see the LICENSE.md file for details.
