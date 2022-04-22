# RCO: Hash Params

[![Custom badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fkmanc%2Fremote_code_oxidation%2Fmaster%2F.custom_shields%2Fhash_params.json)](https://github.com/kmanc/remote_code_oxidation/tree/master/hash_params)

![gif](https://user-images.githubusercontent.com/PUTREALLINK.gif)


## How it works

Hash params performs Rust's default hash algorithm ([SipHash-1-3 at the time of this writing](https://en.wikipedia.org/wiki/SipHash)) on commandline arguments and prints their output in hex. This should not be considered cryptographically secure, but works for obfuscating a value in a repeatable way.


## Using it

1. [Not shown in GIF] Compile the executable

    #### For Linux
    ```commandline
    cargo build -p hash_params --release
    ```

    #### For Windows
    ```commandline
    cargo build --target x86_64-pc-windows-gnu -p hash_params --release
    ```
2. Run the executable with the desired hash targets