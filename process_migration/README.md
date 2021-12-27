# RCO: Process Migration

Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/process_migration/src/config.rs) and change the shellcode to your desired payload before compiling

#### Build for Unix target
```commandline
cargo build -p process_migration --release
```

#### Build for Windows target
```commandline
cargo build --target x86_64-pc-windows-gnu -p process_migration --release
```