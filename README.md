# Cargofetch 🦀

Cargofetch is a lightweight CLI tool written in Rust that fetches metadata about your Rust project. It displays the project's name, version, and repository URL in a visually appealing format, inspired by the look and feel of neofetch.
Features

## Installation

You can install cargofetch by downloading the pre-built binary or compiling from source.
#### Option 1: Install via Cargo (Rust's package manager)

If you have Cargo installed, you can easily install cargofetch by running the following command:
```bash
cargo install cargofetch
```
#### Option 2: Building from Source

To build cargofetch from source, clone the repository and use Cargo to build it:
```bash
git clone https://github.com/arjav0703/cargofetch.git
cd cargofetch
cargo build --release
```
The binary will be located in the target/release/ directory.

## Usage

After installing cargofetch, you can run it directly from your project directory (containing Cargo.toml):
```
cargofetch
```
Example output:
```
                 R RR RR                  Cargo Version: 1.87.0
              R RRRRRRRR R          R     Package Name: cargofetch
 R RR       R RRRRRRRRRRRRR R      RR     Version: 0.1.0
rR RRR    R RRRRRRRRRRRRRRRRR R   RRR R   Dependencies: 4
RRR RR   RRRRRRRRRRRRRRRRRRRRRRR  RRRRR   Repository: https://github.com/arjav0703/rust-fetch
 RRRRR  RRRRRRRRRRRRRRRRRRRRRRRR  RRRR    
  RRR RRRRRRRRRRRRRRRRRRRRRRRRRRRR RR     
    R  RRRRRRRRRR=  RR = RRRRRRRRRRR      
     RRRRRRRRRRRR=  RR = RRRRRRRRRR       
      RRRRRRRRRRR   RR   RRRRRRRRRR       
     RR==RRRRRRRRRRRRRRRRRRRRRR===RR      
     RR =  ==RRRRRRR  RRRRRR==  = RR      
      RR =     ===========     = RR       
       RR                        R        
        R                       R         
         R                                
```


License

This project is licensed under the MIT License - see the LICENSE.md file for details.
