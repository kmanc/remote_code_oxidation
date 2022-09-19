---
title: "Process Hollowing"
datatable: true
---

# RCO: Process Hollowing

[![Process hollowing version unavailable](https://img.shields.io/crates/v/process_hollowing?label=process_hollowing)](https://github.com/kmanc/remote_code_oxidation/tree/master/process_hollowing)

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
Linux     | None                      | [5 / 63](https://www.virustotal.com/gui/file/7df9b774203440d4a7d83549c6553f0c553cee8ef1260b3c5efa48b2a9bf9c50)  | [![process_hollowing_linux](/assets/images/linux/hollowing.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/linux/hollowing.png)
Linux     | `xor`                     | [0 / 62](https://www.virustotal.com/gui/file/9565ff2330df0cb2cc76a1f823df7d1d87ef7f607d65e237a86a8c29e7293343)  | [![process_hollowing_linux_xor](/assets/images/linux/hollowing_xor.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/linux/hollowing_xor.png)
Windows   | None                      | [22 / 69](https://www.virustotal.com/gui/file/856ebd9dad645881c48b74a8e9dac6bb991306a5059434166d58aa19f324c9cf) | [![process_hollowing_windows](/assets/images/windows/hollowing.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/windows/hollowing.png)
Windows   | `antistring`              | [21 / 70](https://www.virustotal.com/gui/file/6b0bb17f07d3b6c4ff1c4933673e3c084279ecf57055cd2d755c2adb69e11c50) | [![process_hollowing_windows_antistring](/assets/images/windows/hollowing_antistring.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/windows/hollowing_antistring.png)
Windows   | `antisand,antistring`     | [21 / 69](https://www.virustotal.com/gui/file/008f22866e790cee1fea0c17f247cd01634a7e078e64de3801fa64503604eb49) | [![process_hollowing_windows_antisand_antistring](/assets/images/windows/hollowing_antisand_antistring.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/windows/hollowing_antisand_antistring.png)
Windows   | `antisand`                | [20 / 70](https://www.virustotal.com/gui/file/91d1e17d64615d1651cc03a6d13b4d9826fb4abb72d21ba3cc99e63c07fe63b0) | [![process_hollowing_windows_antisand](/assets/images/windows/hollowing_antisand.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/windows/hollowing_antisand.png)
Windows   | `xor`                     | [14 / 69](https://www.virustotal.com/gui/file/c84ceff2c9a3ed9484123d6274d66fe06e017dfa5c102c197c492cfbda423414)  | [![process_hollowing_windows_xor](/assets/images/windows/hollowing_xor.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/windows/hollowing_xor.png)
Windows   | `antistring,xor`          | [3 / 70](https://www.virustotal.com/gui/file/4f71c2fea7e716e9792660229eecd065170fde1072460f4e8fdbfb4e2acd896c)  | [![process_hollowing_windows_antistring_xor](/assets/images/windows/hollowing_antistring_xor.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/windows/hollowing_antistring_xor.png)
Windows   | `antisand,xor`            | [0 / 70](https://www.virustotal.com/gui/file/b3648518f2f68a724026596a12f894c873298dcb3a53b7f4442eb7ffcade20b7)  | [![process_hollowing_windows_antisand_xor](/assets/images/windows/hollowing_antisand_xor.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/windows/hollowing_antisand_xor.png)
Windows   | `antisand,antistring,xor` | [0 / 70](https://www.virustotal.com/gui/file/415ff816dee411f7a3b2db0fbd2ea2433b86a44cd42644434ead9357d6b46443)  | [![process_hollowing_windows_antisand_antistring_xor](/assets/images/windows/hollowing_antisand_antistring_xor.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/gh-pages/assets/images/windows/hollowing_antisand_antistring_xor.png)

<div class="datatable-end"></div>
