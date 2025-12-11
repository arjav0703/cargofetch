# Cargofetch 🦀
![GitHub License](https://img.shields.io/github/license/arjav0703/cargofetch)
![GitHub last commit](https://img.shields.io/github/last-commit/arjav0703/cargofetch)


Cargofetch is a lightweight CLI tool written in Rust that fetches metadata about your Rust project.

![Image](https://hc-cdn.hel1.your-objectstorage.com/s/v3/08442b9bd462c34a0aa9944b1599ea74080ee375_image.png)

## Installation
[![Crates.io](https://img.shields.io/crates/v/cargofetch.svg)](https://crates.io/crates/cargofetch)
[![Homebrew Tap](https://img.shields.io/badge/homebrew-cargofetch-blue)](https://github.com/arjav0703/cargofetch)
[![AUR](https://img.shields.io/aur/version/cargofetch)](https://aur.archlinux.org/packages/cargofetch)


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
![AUR Last Modified](https://img.shields.io/aur/last-modified/cargofetch)
If you are using an Arch-based Linux distribution, you can install cargofetch from the AUR:
```bash
yay -S cargofetch
```

#### NIX flakes
If you are using Nix, you can install cargofetch using flakes:
```bash
nix profile install github:arjav0703/cargofetch
```

Manual:
You can download the pre-built binary for your platform from the [releases page](https://github.com/arjav0703/cargofetch/releases).

---
### 🔧 Building from Source
Automatic:
```bash
curl -fsSL https://raw.githubusercontent.com/arjav0703/cargofetch/refs/heads/v1/install-source.sh | bash
```

Manual:
```bash
git clone https://github.com/arjav0703/cargofetch.git
cd cargofetch
cargo build --release
```
The binary will be located in the target/release/ directory.


## Acknowledgements
This projcet was inspired by [neofetch](https://en.wikipedia.org/wiki/Neofetch) and [onefetch](https://onefetch.dev). Also, the ASCII art logo was also taken from onefetch.

---
<div align="center">
  <a href="https://shipwrecked.hackclub.com/?t=ghrm" target="_blank">
    <img src="https://hc-cdn.hel1.your-objectstorage.com/s/v3/739361f1d440b17fc9e2f74e49fc185d86cbec14_badge.png" 
         alt="This project is part of Shipwrecked, the world's first hackathon on an island!" 
         style="width: 35%;">
  </a>
</div>
