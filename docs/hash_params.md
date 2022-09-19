---
title: "Hash params"
---

# RCO: Hash Params

[![hash_params](https://user-images.githubusercontent.com/14863147/164768272-9b010714-6b81-42d1-9caf-d08324827959.gif)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/gifs/hash_params.gif)


## How it works

Hash params performs Rust's default hash algorithm ([SipHash-1-3 at the time of this writing](https://en.wikipedia.org/wiki/SipHash)) on command line arguments and prints their output in hex. This should not be considered cryptographically secure, but works for obfuscating a value in a repeatable way.


## Using it

1.  *[Not shown in demo]* Compile the executable
    1.  For Linux
    ```commandline
    cargo build -p hash_params --release
    ```
    2.  For Windows
    ```commandline
    cargo build --target x86_64-pc-windows-gnu -p hash_params --release
    ```
2.  Run the executable with the desired hash targets
