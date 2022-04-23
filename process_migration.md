---
title: "Process Migration"
datatable: true
---

# RCO: Process Migration

[![Custom badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fkmanc%2Fremote_code_oxidation%2Fmaster%2F.custom_shields%2Fprocess_migration.json)](https://github.com/kmanc/remote_code_oxidation/tree/master/process_migration)

<div class="datatable-begin"></div>

Target OS | Demo
--------- | ----
Linux     | [![process_migration_linux](/assets/gifs/process_migration.gif)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/gifs/process_migration.gif)
Windows   | [![process_migration_windows](/assets/gifs/process_migration_windows.gif)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/gifs/process_migration_windows.gif)

<div class="datatable-end"></div>


## How it works

Windows process migration works by obtaining a handle to the target process and writing [shellcode](https://en.wikipedia.org/wiki/Shellcode) to it. A remote thread is then created; the starting point of this thread is the newly written shellcode.

Linux process migration works slightly differently. After temporarily pausing the target process, RCO writes shellcode over the process's [instruction pointer](https://datacadamia.com/computer/instruction/instruction_pointer). This can cause issues (the most likely of which is crashing) for the victim machine.


## Using it

1.  *[Not shown in demo]* Generate shellcode for the desired end result (for example, use [msfvenom](https://book.hacktricks.xyz/shells/shells/msfvenom) to generate a reverse TCP
shell shellcode for the target operating system)
2.  *[Not shown in demo]* Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs) 
and change the shellcode to the shellcode generated in step 1
3.  *[Optional - shown in the [xor_params demo](https://kmanc.github.io/remote_code_oxidation/xor_params.html)]* Encrypt the shellcode and target process using [xor_params](https://github.com/kmanc/remote_code_oxidation/blob/master/xor_shellcode) and update the encrypted shellcode value in [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs) 
4.  *[Not shown in demo]* Compile the executable, only including `--features xor` if you did step 3
    1.  Build for Linux target
    ```commandline
    cargo build -p process_migration [antisand][,][antistring][,][xor]] --release
    ```
    2.  Build for Windows target
    ```commandline
    cargo build --target x86_64-pc-windows-gnu -p process_migration [antisand][,][antistring][,][xor]] --release
    ```
5.  Start a netcat listener on the attacking machine on the same port you configured the shellcode to connect to in step 1
```commandline
nc -nlvp 4444
```   
6.  Execute the payload on the victim machine
7.  Return to the listener and enter desired commands for the victim machine to run


## Detection rates

<div class="datatable-begin"></div>

Target OS | Features                      | Detections                             | Screenshot
--------- | ----------------------------- | -------------------------------------- | ----------
Linux     | None                      | [7 / 40](https://kleenscan.com/scan_result/5d88b167a6fdf674a0a81514e37f171a4d0eb63c0b063dec1dd02a5d9b63d4fb)  | [![process_migration_linux](/assets/images/process_migration.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/process_migration.png)
Linux     | `xor`                     | [0 / 40](https://kleenscan.com/scan_result/5568475e28d65306af33f75df28e215e7024daa922241fbd9c1e9205cd27a96d)  | [![process_migration_linux_xor](/assets/images/process_migration_xor.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/process_migration_xor.png)
Windows   | None                      | [12 / 40](https://kleenscan.com/scan_result/ebebddfa24b6d95c65900003629914cbcadf09fddcd9a70db614b9f8e9f5fc42) | [![process_migration_windows](/assets/images/process_migration_exe.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/process_migration_exe.png)
Windows   | `antisand`                | [12 / 40](https://kleenscan.com/scan_result/dc73a322924b772b90957aaffe8d2735acd6d6049e0607a1befada2bc5aa86f3) | [![process_migration_windows_antisand](/assets/images/process_migration_antisand_exe.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/process_migration_antisand_exe.png)
Windows   | `antistring`              | [12 / 40](https://kleenscan.com/scan_result/7200bae53ce50bd8b0f3a528026ee72d71b47615235cf96384fe0752a1ff6145) | [![process_migration_windows_antistring](/assets/images/process_migration_antistring_exe.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/process_migration_antistring_exe.png)
Windows   | `antisand,antistring`     | [12 / 40](https://kleenscan.com/scan_result/e702816970ee629f718e6dbec58a129b03742b0ac7644bc3de942d8368e7252b) | [![process_migration_windows_antisand_antistring](/assets/images/process_migration_antisand_antistring_exe.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/process_migration_antisand_antistring_exe.png)
Windows   | `xor`                     | [1 / 40](https://kleenscan.com/scan_result/8b3feb5f4db1b06a9fd33a9597b62d22847f518f607d7f049579b87b44ce8fea)  | [![process_migration_windows_xor](/assets/images/process_migration_xor_exe.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/process_migration_xor_exe.png)
Windows   | `antistring,xor`          | [1 / 40](https://kleenscan.com/scan_result/f580330422109325f3c83fd1fa51a966798cb173d0edca5c1b4c310a2c95c082)  | [![process_migration_windows_antistring_xor](/assets/images/process_migration_antistring_xor_exe.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/process_migration_antistring_xor_exe.png)
Windows   | `antisand,xor`            | [0 / 40](https://kleenscan.com/scan_result/19a7640ebedb91c375aeebf9d576ea005260610ca0eb23621413dc058a8ff067)  | [![process_migration_windows_antisand_xor](/assets/images/process_migration_antisand_xor_exe.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/process_migration_antisand_xor_exe.png)
Windows   | `antisand,antistring,xor` | [0 / 40](https://kleenscan.com/scan_result/1de23cfca021214907bb51174df2b8d69d2fe45cb6ebc903c1e3328bb958678f)  | [![process_migration_windows_antisand_antistring_xor](/assets/images/process_migration_antisand_antistring_xor_exe.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/process_migration_antisand_antistring_xor_exe.png)

<div class="datatable-end"></div>
