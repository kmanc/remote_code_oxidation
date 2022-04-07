# Remote Code Oxidation (RCO)

[![RCO for Linux targets](https://github.com/kmanc/remote_code_oxidation/actions/workflows/linux.yml/badge.svg?style=flat)](https://github.com/kmanc/remote_code_oxidation/actions/workflows/linux.yml)
[![RCO for Windows targets](https://github.com/kmanc/remote_code_oxidation/actions/workflows/windows.yml/badge.svg?style=flat)](https://github.com/kmanc/remote_code_oxidation/actions/workflows/windows.yml)
![language](https://img.shields.io/github/languages/top/kmanc/remote_code_oxidation?style=flat&color=orange)
![license](https://img.shields.io/github/license/kmanc/remote_code_oxidation?style=flat&color=blueviolet)

Remote Code Oxidation is a collection of offensive security tools written in Rust. My main goal for the project is to enable offensive security professionals and practitioners to prepare the tools needed for an engagement with as little overhead as possible.

RCO tools can be compiled on either Linux or Windows systems to provide its users flexibility in their attack infrastructure. Similarly the tools work against either Linux or Windows targets to suit the needs of the task at hand. 


## Tools list
[![Custom badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fkmanc%2Fremote_code_oxidation%2Fmaster%2F.custom_shields%2Ftcp_reverse_shell.json)](https://github.com/kmanc/remote_code_oxidation/tree/master/tcp_reverse_shell)

[![Custom badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fkmanc%2Fremote_code_oxidation%2Fmaster%2F.custom_shields%2Fprocess_migration.json)](https://github.com/kmanc/remote_code_oxidation/tree/master/process_migration)   

[![Custom badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fkmanc%2Fremote_code_oxidation%2Fmaster%2F.custom_shields%2Fprocess_hollowing.json)](https://github.com/kmanc/remote_code_oxidation/tree/master/process_hollowing)

[![Custom badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fkmanc%2Fremote_code_oxidation%2Fmaster%2F.custom_shields%2Fxor_params.json)](https://github.com/kmanc/remote_code_oxidation/tree/master/xor_params) 

[![Custom badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fkmanc%2Fremote_code_oxidation%2Fmaster%2F.custom_shields%2Fremote_access_trojan.json)](https://github.com/kmanc/remote_code_oxidation/tree/master/remote_access_trojan) 

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
####Todo

### From Windows host for Windows target
####Todo
