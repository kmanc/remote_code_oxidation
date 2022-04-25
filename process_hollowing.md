---
title: "Process Hollowing"
datatable: true
---

# RCO: Process Hollowing

[![Custom badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fkmanc%2Fremote_code_oxidation%2Fmaster%2F.custom_shields%2Fprocess_hollowing.json)](https://github.com/kmanc/remote_code_oxidation/tree/master/process_hollowing)

<div class="datatable-begin"></div>

Target OS | Demo
--------- | ----
Linux     | [![process_hollowing_linux](/assets/gifs/process_hollowing.gif)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/gifs/process_hollowing.gif)
Windows   | [![process_hollowing_windows](/assets/gifs/process_hollowing_windows.gif)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/gifs/process_hollowing_windows.gif)

<div class="datatable-end"></div>

## How it works

Windows process hollowing works by creating a suspended process and writing the [shellcode](https://en.wikipedia.org/wiki/Shellcode) over the entry point of the process's main thread. It then resumes the suspended process, which in turn executes the shellcode.

Linux process hollowing functions a little differently. First, the executable creates a child process by cloning itself. Then it overwrites the child process's [instruction pointer](https://datacadamia.com/computer/instruction/instruction_pointer) with shellcode. Because this process was not running before the attack, it is unlikely this will cause any issues on the victim machine.


## Using it

1.  *[Not shown in demo]* Generate shellcode for the desired end result (for example, use [msfvenom](https://book.hacktricks.xyz/shells/shells/msfvenom) to generate a reverse TCP shell shellcode for the target operating system)
2.  *[Not shown in demo]* Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs) 
and change the shellcode to the shellcode generated in step 1
3.  *[Optional - shown in [xor_params demo](https://kmanc.github.io/remote_code_oxidation/xor_params.html)]* Encrypt the shellcode and target process using [xor_params](https://github.com/kmanc/remote_code_oxidation/blob/master/xor_shellcode) and update the encrypted shellcode value in [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs)  
4.  *[Not shown in demo]* Compile the executable, only including `--features xor` if you did step 3
    1.  Build for Linux target
    ```commandline
    cargo build -p process_hollowing [antisand][,][antistring][,][xor]] --release
    ```
    2.  Build for Windows target
    ```commandline
    cargo build --target x86_64-pc-windows-gnu -p process_hollowing [antisand][,][antistring][,][xor]] --release
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
Linux     | None                      | [7 / 40](https://kleenscan.com/scan_result/1177abafe77dc580337ec6294c68bdc4873ceb36a4eeac057fd0673c3ae50e7f)  | [![process_hollowing_linux](/assets/images/process_hollowing.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/process_hollowing.png)
Linux     | `xor`                     | [0 / 40](https://kleenscan.com/scan_result/8a0268ca750a14fc93f40f6b1864f13ce94318c4c4a7ecc49dfeb332b9c9d860)  | [![process_hollowing_linux_xor](/assets/images/process_hollowing_xor.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/process_hollowing_xor.png)
Windows   | None                      | [12 / 40](https://kleenscan.com/scan_result/dd7858b48235bc782383fa5a929125369c7918d3c119a9196b0fdab791624763) | [![process_hollowing_windows](/assets/images/process_hollowing_exe.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/process_hollowing_exe.png)
Windows   | `antisand`                | [12 / 40](https://kleenscan.com/scan_result/dc73a322924b772b90957aaffe8d2735acd6d6049e0607a1befada2bc5aa86f3) | [![process_hollowing_windows_antisand](/assets/images/process_hollowing_antisand_exe.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/process_hollowing_antisand_exe.png)
Windows   | `antistring`              | [12 / 40](https://kleenscan.com/scan_result/1505ac5f33afe16a79796045d80c6c55617944c86396411487f1cbd934e875fb) | [![process_hollowing_windows_antistring](/assets/images/process_hollowing_antistring_exe.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/process_hollowing_antistring_exe.png)
Windows   | `antisand,antistring`     | [12 / 40](https://kleenscan.com/scan_result/177242f39b392107e4953a8cb717afbc6f912daa5bd9ec8d71a959834942db8d) | [![process_hollowing_windows_antisand_antistring](/assets/images/process_hollowing_antisand_antistring_exe.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/process_hollowing_antisand_antistring_exe.png)
Windows   | `xor`                     | [6 / 40](https://kleenscan.com/scan_result/455d775c517cf26a6e83a42b3eae7982364d8a8174127eca377094c05e0dd948)  | [![process_hollowing_windows_xor](/assets/images/process_hollowing_xor_exe.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/process_hollowing_xor_exe.png)
Windows   | `antistring,xor`          | [1 / 40](https://kleenscan.com/scan_result/e6214cb0175737d1e3bba8bafbaa17d5aa575f613dab718a6d35dd46c7af8767)  | [![process_hollowing_windows_antistring_xor](/assets/images/process_hollowing_antistring_xor_exe.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/process_hollowing_antistring_xor_exe.png)
Windows   | `antisand,xor`            | [0 / 40](https://kleenscan.com/scan_result/de899245ec6a258d741b6243d18cf10fae5e6a1fe344ab3d02f17899a67d2bb7)  | [![process_hollowing_windows_antisand_xor](/assets/images/process_hollowing_antisand_xor_exe.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/process_hollowing_antisand_xor_exe.png)
Windows   | `antisand,antistring,xor` | [0 / 40](https://kleenscan.com/scan_result/49f53e2e15b86d9e5425d684e9ab964289d2d96fef8ca61ba927e3826ebd0392)  | [![process_hollowing_windows_antisand_antistring_xor](/assets/images/process_hollowing_antisand_antistring_xor_exe.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/process_hollowing_antisand_antistring_xor_exe.png)

<div class="datatable-end"></div>
