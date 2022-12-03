---
title: "Process Hollowing"
datatable: true
---

# RCO: Process Hollowing

[![Process hollowing version unavailable](https://img.shields.io/crates/v/process_hollowing?label=process_hollowing)](https://github.com/kmanc/remote_code_oxidation/tree/master/process_hollowing)

<div class="datatable-begin"></div>

Target OS | Demo
--------- | ----
Linux     | [![process_hollowing_linux](/assets/gifs/process_hollowing.gif)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/gifs/process_hollowing.gif)
Windows   | [![process_hollowing_windows](/assets/gifs/process_hollowing_windows.gif)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/gifs/process_hollowing_windows.gif)

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
Linux     | None                      | [7 / 40](https://kleenscan.com/scan_result/a6de6fa00c7b8c0d3bb1fbd1f207509987610fd7037bd3ab7818c12030d6c266)  | [![process_hollowing_linux](/assets/images/linux/hollowing.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/linux/hollowing.png)
Linux     | `xor`                     | [1 / 40](https://kleenscan.com/scan_result/fd3affec1eaed16e9d6077e05f3807897a0994c33d3067ba53cdb907690b70e6)  | [![process_hollowing_linux_xor](/assets/images/linux/hollowing_xor.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/linux/hollowing_xor.png)
Windows   | None                      | [12 / 40](https://kleenscan.com/scan_result/74e2475b5b5e881d3b31c5d3bac5e36f4c8bfa33235eb810706c33f338dbeb7c) | [![process_hollowing_windows](/assets/images/windows/hollowing.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/hollowing.png)
Windows   | `antisand`                | [11 / 40](https://kleenscan.com/scan_result/1ce970448fa81ba854643bf4663afef87b2bdb7aa05ceda720ef82a70f8d932a) | [![process_hollowing_windows_antisand](/assets/images/windows/hollowing_antisand.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/hollowing_antisand.png)
Windows   | `antisand,antistring`     | [11 / 40](https://kleenscan.com/scan_result/643bf03eb63541f6854d2898d2c809c368a14f82b57b9f85f7ec75f216aceef9) | [![process_hollowing_windows_antisand_antistring](/assets/images/windows/hollowing_antisand_antistring.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/hollowing_antisand_antistring.png)
Windows   | `antistring`              | [11 / 40](https://kleenscan.com/scan_result/6a5599f63c58f1d3e09a7c11add05ab3abb2226682c8290d1b0dc445ac8279bc) | [![process_hollowing_windows_antistring](/assets/images/windows/hollowing_antistring.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/hollowing_antistring.png)
Windows   | `antistring,xor`          | [1 / 40](https://kleenscan.com/scan_result/08572b59b640b6fea8a5f164d17056c48d4252a43a6a336e5091f024e3d25a4b)  | [![process_hollowing_windows_antistring_xor](/assets/images/windows/hollowing_antistring_xor.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/hollowing_antistring_xor.png)
Windows   | `xor`                     | [1 / 40](https://kleenscan.com/scan_result/fd3affec1eaed16e9d6077e05f3807897a0994c33d3067ba53cdb907690b70e6)  | [![process_hollowing_windows_xor](/assets/images/windows/hollowing_xor.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/hollowing_xor.png)
Windows   | `antisand,xor`            | [0 / 40](https://kleenscan.com/scan_result/a7baee8c968a997f48257e8e67d197f92dfd52f1281a65e2a5557f654adb33f8)  | [![process_hollowing_windows_antisand_xor](/assets/images/windows/hollowing_antisand_xor.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/hollowing_antisand_xor.png)
Windows   | `antisand,antistring,xor` | [0 / 40](https://kleenscan.com/scan_result/b2fb0120a966d36e2158fad5867a4caed0cea52db00d87244f1122e1f380122e)  | [![process_hollowing_windows_antisand_antistring_xor](/assets/images/windows/hollowing_antisand_antistring_xor.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/hollowing_antisand_antistring_xor.png)

<div class="datatable-end"></div>
