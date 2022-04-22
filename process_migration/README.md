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

[No features: 7 / 40](https://kleenscan.com/scan_result/5d88b167a6fdf674a0a81514e37f171a4d0eb63c0b063dec1dd02a5d9b63d4fb) | [Using `--features xor`: 0 / 40](https://kleenscan.com/scan_result/5568475e28d65306af33f75df28e215e7024daa922241fbd9c1e9205cd27a96d)
:-------------------------:|:-------------------------:
![image](https://user-images.githubusercontent.com/14863147/164811534-e6234f39-8aa4-4e66-b5fa-4af14721ae17.png) | ![image](https://user-images.githubusercontent.com/14863147/164811826-de37a797-2e41-4856-a3dd-db02e3b677ad.png)


<p align="center"> Windows </p>

[No features: 12 / 40](https://kleenscan.com/scan_result/5588bdf5da5bd1c25e08ca6cb7a07d7729160a5fdcbc218b8f7cd112620f6f67) | [Using `--features antisand`: 12 / 40](https://kleenscan.com/scan_result/ebebddfa24b6d95c65900003629914cbcadf09fddcd9a70db614b9f8e9f5fc42) | [Using `--features antistring`: 12 / 40](https://kleenscan.com/scan_result/7200bae53ce50bd8b0f3a528026ee72d71b47615235cf96384fe0752a1ff6145) | [Using `--features antisand,antistring`: 12 / 40](https://kleenscan.com/scan_result/e702816970ee629f718e6dbec58a129b03742b0ac7644bc3de942d8368e7252b) | [Using `--features xor`: 1 / 40](https://kleenscan.com/scan_result/8b3feb5f4db1b06a9fd33a9597b62d22847f518f607d7f049579b87b44ce8fea) | [Using `--features antistring,xor`: 1 / 40](https://kleenscan.com/scan_result/f580330422109325f3c83fd1fa51a966798cb173d0edca5c1b4c310a2c95c082) | [Using `--features antisand,xor`: 0 / 40](https://kleenscan.com/scan_result/19a7640ebedb91c375aeebf9d576ea005260610ca0eb23621413dc058a8ff067) | [Using `--features antisand,antistring,xor`: 0 / 40](https://kleenscan.com/scan_result/1de23cfca021214907bb51174df2b8d69d2fe45cb6ebc903c1e3328bb958678f)
:-------------------------:|:-------------------------:|:-------------------------:|:-------------------------:|:-------------------------:|:-------------------------:|:-------------------------:|:-------------------------:
![image](https://user-images.githubusercontent.com/14863147/164814085-63e17b83-8136-45f3-8e86-9a0e56b60d25.png) | ![image](https://user-images.githubusercontent.com/14863147/164814102-137956c9-aff4-449c-95ff-e3f82be0d057.png) | ![image](https://user-images.githubusercontent.com/14863147/164814134-1fb427ae-f112-458c-9e76-b9c637fa1afb.png) | ![image](https://user-images.githubusercontent.com/14863147/164814162-16f2c0f6-2444-46e4-90a0-42549ef5adac.png) | ![image](https://user-images.githubusercontent.com/14863147/164814200-83ead2dc-6ab6-4401-873f-f7c6d43ddfcd.png) | ![image](https://user-images.githubusercontent.com/14863147/164814265-66705820-2fe6-49d8-af8b-9be17871a925.png) | ![image](https://user-images.githubusercontent.com/14863147/164814286-70ad83a9-56e1-40c0-b4c4-211887361507.png) | ![image](https://user-images.githubusercontent.com/14863147/164814306-c8ebceac-c685-4e24-9ea9-548fbc84c43c.png)
