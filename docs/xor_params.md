---
title: "XOR Params"
---

# RCO: XOR Params

[![xor_params](https://user-images.githubusercontent.com/14863147/152621001-8de291e1-58dd-4f7e-9916-1846a65f1c83.gif)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/gifs/xor_params.gif)


## How it works

XOR params performs an [exclusive OR (XOR)](https://en.wikipedia.org/wiki/Exclusive_or) operation on each byte of the shellcode with each byte of the key (repeating the key if need be).


## Using it

1.  *[Not shown in demo]* Generate shellcode for the desired end result (for example, use [msfvenom](https://book.hacktricks.xyz/shells/shells/msfvenom) to generate a reverse TCP shell shellcode for the target operating system)
2.  *[Not shown in demo]* Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs) and change the shellcode to the shellcode generated in step 1
3.  *[Not shown in demo]* Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs) and change the key to a desired key
4.  *[Not shown in demo]* Compile the executable
    1.  For Linux
    ```commandline
    cargo build -p xor_params --release
    ```
    2.  For Windows
    ```commandline
    cargo build --target x86_64-pc-windows-gnu -p xor_params --release
    ```
5.  Run the executable
6.  Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs) and change encrypted payload to the output of step 5
