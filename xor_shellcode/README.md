# RCO: XOR Shellcode

[![Custom badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fkmanc%2Fremote_code_oxidation%2Fmaster%2F.custom_shields%2Fxor_shellcode.json)](https://github.com/kmanc/remote_code_oxidation/tree/master/xor_shellcode)

![gif](https://127.0.0.1/gif)


## How it works

XOR shellcode performs an [exclusive OR (XOR)](https://en.wikipedia.org/wiki/Exclusive_or) operation on each byte of the shellcode with each byte of the key (repeating the key if need be).


## Using it

1. Generate shellcode for the desired end result (for example, use [msfvenom](https://book.hacktricks.xyz/shells/shells/msfvenom) to generate a reverse TCP shell shellcode for the target operating system)
2. Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs) and change the shellcode to the shellcode generated in step 1
3. Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs) and change the key to a desired key
4. Compile the executable

    #### For Linux
    ```commandline
    cargo build -p tcp_reverse_shell --release
    ```

    #### For Windows
    ```commandline
    cargo build --target x86_64-pc-windows-gnu -p tcp_reverse_shell --release
    ```
5. Run the executable
6. Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs) and change encrypted payload to the output of step 5
