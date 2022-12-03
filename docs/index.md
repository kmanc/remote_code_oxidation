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

1. [hash_params](https://kmanc.github.io/remote_code_oxidation/hash_params.html)

2. [xor_params](https://kmanc.github.io/remote_code_oxidation/xor_params.html) 

## Building the executables

Clone the repo
```commandline
git clone https://github.com/kmanc/remote_code_oxidation.git
```


### From Linux
---

##### For Linux
---

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
cargo build [-p package_name] [--features [antisand][,][antistring][,][xor]] [--release]
```

##### For Windows
---

Install Rust
```commandline
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Add dependencies for cross-compiling (1)
```commandline
rustup target add x86_64-pc-windows-gnu
```

Add dependencies for cross-compiling (2)
```commandline
sudo apt install mingw-w64
```

Build!
```commandline
cargo build --target x86_64-pc-windows-gnu [-p package_name] [--features [antisand][,][antistring][,][xor]] [--release]
```


### From Mac
---

##### For Linux
---

Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Add dependencies for cross-compiling (1)
```commandline
rustup target add x86_64-unknown-linux-musl
```

Add dependencies for cross-compiling (2)
```commandline
brew install filosottile/musl-cross/musl-cross
```

Configure linker for cross-compiling
```commandline
Create a file in your home directory's .cargo directory called config.toml with the following contents
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
```

Build!
```commandline
cargo build --target x86_64-unknown-linux-musl [-p package_name] [--features [antisand][,][antistring][,][xor]] [--release]
```

##### For Windows
---

Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Add dependencies for cross-compiling (1)
```commandline
rustup target add x86_64-pc-windows-gnu
```

Add dependencies for cross-compiling (2)
```commandline
brew install mingw-w64
```

Build!
```commandline
cargo build --target x86_64-pc-windows-gnu [-p package_name] [--features [antisand][,][antistring][,][xor]] [--release]
```


### From Windows
---

##### For Linux
---

Install Rust
```
Download and run the installer from the Rust website
```

Add dependencies for cross-compiling
```commandline
rustup target add x86_64-pc-windows-gnu
```

Configure linker for cross-compiling
```commandline
Create a file in your home directory's .cargo directory called config.toml with the following contents
[target.x86_64-unknown-linux-musl]
linker = "rust-lld"
```

Build!
```commandline
cargo build --target x86_64-unknown-linux-musl [-p package_name] [--features [antisand][,][antistring][,][xor]] [--release]
```

##### For Windows
---

Install Rust
```
Download and run the installer from the Rust website
```

Build!
```commandline
cargo build [-p package_name] [--features [antisand][,][antistring][,][xor]] [--release]
```
