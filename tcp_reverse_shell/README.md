# RCO: TCP Reverse Shell

Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/tcp_reverse_shell/src/config.rs) and change the IP address and port to match your listener before compiling

#### Build for Unix target
```commandline
cargo build -p tcp_reverse_shell --release
```

#### Build for Windows target
```commandline
cargo build --target x86_64-pc-windows-gnu -p tcp_reverse_shell --release
```