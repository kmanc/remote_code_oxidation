# Remote code oxidation (RCO)

[![RCO for Linux targets](https://github.com/kmanc/remote_code_oxidation/actions/workflows/linux.yml/badge.svg)](https://github.com/kmanc/remote_code_oxidation/actions/workflows/linux.yml)
[![RCO for Windows targets](https://github.com/kmanc/remote_code_oxidation/actions/workflows/windows.yml/badge.svg)](https://github.com/kmanc/remote_code_oxidation/actions/workflows/windows.yml)
![language](https://img.shields.io/github/languages/top/kmanc/remote_code_oxidation?style=plastic)
![license](https://img.shields.io/github/license/kmanc/remote_code_oxidation?style=plastic)

A collection of offensive security tools written in Rust. More details to come

## Tools list
- [TCP reverse shell](https://github.com/kmanc/remote_code_oxidation/tree/master/tcp_reverse_shell)
- [Process migration](https://github.com/kmanc/remote_code_oxidation/tree/master/process_migration)

## Setup

Clone the repo
```commandline
git clone https://github.com/kmanc/remote_code_oxidation.git
```

### From Linux host for Linux target

Install Rust
```commandline
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Add dependencies for compiling
```commandline
sudo apt install build-essential
```

Build!
```commandline
cargo build [-p package_name] [--release]
```

### From Linux host for Windows target

Install Rust
```commandline
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Add dependencies for cross-compiling
```commandline
sudo apt install mingw-w64
rustup target add x86_64-pc-windows-gnu
```

Build!
```commandline
cargo build --target x86_64-pc-windows-gnu [-p package_name] [--release]
```
