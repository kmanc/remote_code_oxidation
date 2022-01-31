# RCO: Process Migration

[![Custom badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fkmanc%2Fremote_code_oxidation%2Fmaster%2F.custom_shields%2Fprocess_migration.json)](https://github.com/kmanc/remote_code_oxidation/tree/master/process_migration)

Linux target               |  Windows target
:-------------------------:|:-------------------------:
![](https://user-images.githubusercontent.com/14863147/151044951-5ee5b376-9f62-4e2e-a773-8c3b7a7d580e.gif)  |  ![](https://user-images.githubusercontent.com/14863147/151059013-b053e9de-d75c-4470-97a7-a109c7f2ef55.gif)


## How it works

Windows process migration works by obtaining a handle to the target process and writing [shellcode](https://en.wikipedia.org/wiki/Shellcode) to it. A remote thread is then created; the starting point of this thread is the newly written shellcode.

Linux process migration works slightly differently. After temporarily pausing the target process, RCO writes shellcode over the process's [instruction pointer](https://datacadamia.com/computer/instruction/instruction_pointer). This can cause issues (the most likely of which is crashing) for the victim machine.


## Using it

1. [Not shown in GIF] Generate shellcode for the desired end result (for example, use [msfvenom](https://book.hacktricks.xyz/shells/shells/msfvenom) to generate a reverse TCP
shell shellcode for the target operating system)
2. [Not shown in GIF] Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs) 
and change the shellcode to the shellcode generated in step 1
3. [Optional] Encrypt the shellcode using [xor_shellcode](https://github.com/kmanc/remote_code_oxidation/blob/master/xor_shellcode) and update the encrypted shellcode value in [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs) 
4. [Not shown in GIF] Compile the executable, only including `--features encrypted` if you did step 3

    #### Build for Linux target
    ```commandline
    cargo build -p process_migration [--features encrypted] --release
    ```

    #### Build for Windows target
    ```commandline
    cargo build --target x86_64-pc-windows-gnu -p process_migration [--features encrypted] --release
    ```
5. Start a netcat listener on the attacking machine on the same port you configured the shellcode to connect to in step 1
    ```commandline
    nc -nlvp 4444
    ```   
6. Execute the payload on the victim machine
7. Return to the listener and enter desired commands for the victim machine to run


## Detection rates

### No XOR encryption

[Linux - 7 / 40](https://kleenscan.com/scan_result/d3a8ca03f0337b8c9dd5917c769d2267f0aa3d9f8da8413b28425cafc5b8426b)

![image](https://user-images.githubusercontent.com/14863147/151648519-26612702-de25-429a-88ce-21f7fd5e8f7a.png)

[Windows - 13 / 40](https://kleenscan.com/scan_result/fb860f5b0e4f835412787d3e20c6d9129bdc4c4bd87b67966aae2f5efa0973f1)

![image](https://user-images.githubusercontent.com/14863147/151648536-11d1cd0d-fa38-4824-a9c4-6c952e94007c.png)

### XOR encrypted with default key

[Linux - 0 / 40](https://kleenscan.com/scan_result/e93d72187555dabfb89de5dc7c69ebd224e9aae58ab8c114ab6720b87cca48c0)

![image](https://user-images.githubusercontent.com/14863147/151732309-c5451437-a6ad-41cf-81db-89b2c50cf48b.png)

[Windows - 1 / 40](https://kleenscan.com/scan_result/a6cd65ff1dd24cf60a9b39a77c8ccb96fe1d27528857fa4754e70bf4578bcf11)

![image](https://user-images.githubusercontent.com/14863147/151732274-53a95559-c4c2-4137-9d0e-89b098fbad88.png)
