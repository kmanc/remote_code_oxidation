---
title: "Remote Code Oxidation"
---

# Remote Code Oxidation (RCO)

[![RCO for Linux targets](https://github.com/kmanc/remote_code_oxidation/actions/workflows/linux.yml/badge.svg?style=flat)](https://github.com/kmanc/remote_code_oxidation/actions/workflows/linux.yml)
[![RCO for Windows targets](https://github.com/kmanc/remote_code_oxidation/actions/workflows/windows.yml/badge.svg?style=flat)](https://github.com/kmanc/remote_code_oxidation/actions/workflows/windows.yml)
![language](https://img.shields.io/github/languages/top/kmanc/remote_code_oxidation?style=flat&color=orange)
[![license](https://img.shields.io/github/license/kmanc/remote_code_oxidation?style=flat&color=blueviolet)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/master/LICENSE)

Remote Code Oxidation is a collection of offensive security tools written in Rust. My main goal for the project is to enable offensive security professionals and practitioners to prepare the tools needed for an engagement with as little overhead as possible.

RCO tools can be compiled on either Linux or Windows systems to provide its users flexibility in their attack infrastructure. Similarly the tools work against either Linux or Windows targets to suit the needs of the task at hand. 


## Tools list

[![Process hollowing version unavailable](https://img.shields.io/crates/v/process_hollowing?label=process_hollowing)](https://kmanc.github.io/remote_code_oxidation/process_hollowing.html)

[![Process migration version unavailable](https://img.shields.io/crates/v/process_migration?label=process_migration)](https://kmanc.github.io/remote_code_oxidation/process_migration.html) 

[![TCP reverse shell version unavailable](https://img.shields.io/crates/v/tcp_reverse_shell?label=tcp_reverse_shell)](https://kmanc.github.io/remote_code_oxidation/tcp_reverse_shell.html)


## Helper tools

- [hash_params](https://kmanc.github.io/remote_code_oxidation/hash_params.html)

- [xor_params](https://kmanc.github.io/remote_code_oxidation/xor_params.html) 

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
cargo build [-p package_name] [--features [xor][antisand]] [--release]
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
cargo build --target x86_64-pc-windows-gnu [-p package_name] [--features [xor][antisand]] [--release]
```


### From Windows host for Linux target
#### Todo

### From Windows host for Windows target
#### Todo
