# RCO: Process Hollowing

[![Custom badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fkmanc%2Fremote_code_oxidation%2Fmaster%2F.custom_shields%2Fprocess_hollowing.json)](https://github.com/kmanc/remote_code_oxidation/tree/master/process_hollowing)

Linux target               |  Windows target
:-------------------------:|:-------------------------:
![](http://127.0.0.1)  |  ![](http://127.0.0.1)


## How it works

RCO's Windows process hollowing creates a suspended process and writes the [shellcode](https://en.wikipedia.org/wiki/Shellcode) over the entry point of the process's main thread. It then resumes the suspended process, which in turn executes the shellcode.

## Using it

1. [Not shown in GIF] Generate shellcode for the desired end result (for example, use [msfvenom](https://book.hacktricks.xyz/shells/shells/msfvenom) to generate a reverse TCP shell shellcode for the target operating system)
2. Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs) 
and change the shellcode to the shellcode generated in step 1
3. Compile the executable

    #### Build for Linux target
    ```commandline
    cargo build -p process_hollowing --release
    ```

    #### Build for Windows target
    ```commandline
    cargo build --target x86_64-pc-windows-gnu -p process_hollowing --release
    ```
4. Start a netcat listener on the attacking machine on the same port you configured the shellcode to connect to in step 1
    ```commandline
    nc -nlvp 4444
    ```   
5. Execute the payload on the victim machine
6. Return to the listener and enter desired commands for the victim machine to run
