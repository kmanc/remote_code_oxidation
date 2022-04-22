# RCO: Process Migration

[![Custom badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fkmanc%2Fremote_code_oxidation%2Fmaster%2F.custom_shields%2Fprocess_migration.json)](https://github.com/kmanc/remote_code_oxidation/tree/master/process_migration)

Linux target               |  Windows target
:-------------------------:|:-------------------------:
![gif](https://user-images.githubusercontent.com/14863147/151044951-5ee5b376-9f62-4e2e-a773-8c3b7a7d580e.gif)  |  ![gif](https://user-images.githubusercontent.com/14863147/151059013-b053e9de-d75c-4470-97a7-a109c7f2ef55.gif)


## How it works

Windows process migration works by obtaining a handle to the target process and writing [shellcode](https://en.wikipedia.org/wiki/Shellcode) to it. A remote thread is then created; the starting point of this thread is the newly written shellcode.

Linux process migration works slightly differently. After temporarily pausing the target process, RCO writes shellcode over the process's [instruction pointer](https://datacadamia.com/computer/instruction/instruction_pointer). This can cause issues (the most likely of which is crashing) for the victim machine.


## Using it

1. [Not shown in GIF] Generate shellcode for the desired end result (for example, use [msfvenom](https://book.hacktricks.xyz/shells/shells/msfvenom) to generate a reverse TCP
shell shellcode for the target operating system)
2. [Not shown in GIF] Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs) 
and change the shellcode to the shellcode generated in step 1
3. [Optional] Encrypt the shellcode and target process using [xor_params](https://github.com/kmanc/remote_code_oxidation/blob/master/xor_shellcode) and update the encrypted shellcode value in [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs) 
4. [Not shown in GIF] Compile the executable, only including `--features xor` if you did step 3

    #### Build for Linux target
    ```commandline
    cargo build -p process_migration [xor][,][antisand][,][antistring]] --release
    ```

    #### Build for Windows target
    ```commandline
    cargo build --target x86_64-pc-windows-gnu -p process_migration [xor][,][antisand][,][antistring]] --release
    ```
5. Start a netcat listener on the attacking machine on the same port you configured the shellcode to connect to in step 1
    ```commandline
    nc -nlvp 4444
    ```   
6. Execute the payload on the victim machine
7. Return to the listener and enter desired commands for the victim machine to run


## Detection rates

<p align="center"> Linux </p>

[No features: 7 / 40](https://kleenscan.com/scan_result/3eb81783f8424844677211df497e521613ee5b0b82a5996f96a63b941ba3c87e) | [Using `--features xor`: 0 / 40](https://kleenscan.com/scan_result/b9c0d8a42da33d14be8422bd087624e548a6ecdd22c1d25aed06873e28d1dd96)
:-------------------------:|:-------------------------:
![image](https://user-images.githubusercontent.com/14863147/152621843-b78927d0-cfeb-40d0-b150-15d332b0fd61.png) | ![image](https://user-images.githubusercontent.com/14863147/152621872-4be9e051-f9c4-4ecb-bd3d-dcc58980e061.png)

<p align="center"> Windows </p>

[No features: 13 / 40](https://kleenscan.com/scan_result/f7c2b651b9629421550a380c09c7b9739d806a62db6fd9d571164b3f5df6a5b9) | [Using `--features xor`: 1 / 40](https://kleenscan.com/scan_result/ac4a1184e97abf31ebd968d2455535b92be646af54d077cbdb3b0a24254aae78) | [Using `--features xor,antisand`: 0 / 40](https://kleenscan.com/scan_result/f7e459f40276d8d1d12bb738b3aaf972997b8275da00967d8dbc264993ed5dc8)
:-------------------------:|:-------------------------:|:-------------------------:
![image](https://user-images.githubusercontent.com/14863147/152622089-0e41fc02-9c17-4017-882a-26b58d6166c5.png) | ![image](https://user-images.githubusercontent.com/14863147/152622105-268df42f-b240-463b-baf0-3df350e77110.png) | ![image](https://user-images.githubusercontent.com/14863147/152622122-3705c608-62b5-4650-8d79-a7ad128c7a9d.png)
