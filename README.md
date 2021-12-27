# Remote code oxidation (RCO)

![license](https://img.shields.io/github/license/kmanc/remote_code_oxidation)

![language](https://img.shields.io/github/languages/top/kmanc/remote_code_oxidation)

[![RCO Unix](https://github.com/kmanc/remote_code_oxidation/actions/workflows/unix.yml/badge.svg)](https://github.com/kmanc/remote_code_oxidation/actions/workflows/unix.yml)

[![RCO Windows](https://github.com/kmanc/remote_code_oxidation/actions/workflows/windows.yml/badge.svg)](https://github.com/kmanc/remote_code_oxidation/actions/workflows/windows.yml)

A collection of offensive security tools written in Rust. More details to come

## Tools list
- [TCP reverse shell](https://github.com/kmanc/remote_code_oxidation/tree/master/tcp_reverse_shell)
  - Navigate to [its config file](https://github.com/kmanc/remote_code_oxidation/blob/master/tcp_reverse_shell/src/config.rs) and change the IP address and port before compiling
- [Shellcode injection and process migration](https://github.com/kmanc/remote_code_oxidation/tree/master/process_migration)
  - Navigate to [its config file](https://github.com/kmanc/remote_code_oxidation/blob/master/process_migration/src/config.rs) and change the shellcode before compiling

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
cargo build [-p package_name] [--release]
```

Example
```
cargo build -p tcp_reverse_shell --release
```

### From Linux host for Windows target

Add dependencies for cross-compiling
```
sudo apt install mingw-w64
rustup target add x86_64-pc-windows-gnu
```

Build!
```
cargo build --target x86_64-pc-windows-gnu [-p package_name] [--release]
```

Example
```
cargo build --target x86_64-pc-windows-gnu -p process_migration --release
```
