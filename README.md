# Remote code oxidation (RCO)
[![version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/kmanc/remote_code_oxidation/releases/tag/0.1.0)

A collection of offensive security tools written in Rust. More details to come

## Compilation

### From Linux host for Linux target

Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Set up Environment
```
sudo apt install cmake
```

Build!
```
cargo build [--release]
```

### From Linux host for Windows target

Add dependencies for cross-compiling
```
sudo apt install mingw-w64
rustup target add x86_64-pc-windows-gnu
```

Build!
```
cargo build --target x86_64-pc-windows-gnu [--release]
```

## basic_tcp_reverse_shell

A simple TCP reverse shell, no bells or whistles.

Navigate to [its config file](https://github.com/kmanc/remote_code_oxidation/src/basic_tcp_reverse_shell/config.rs) and change the IP address and port before compiling