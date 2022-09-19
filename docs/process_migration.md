---
title: "Process Migration"
datatable: true
---

# RCO: Process Migration

[![Process migration version unavailable](https://img.shields.io/crates/v/process_migration?label=process_migration)](https://github.com/kmanc/remote_code_oxidation/tree/master/process_migration)

<div class="datatable-begin"></div>

Target OS | Demo
--------- | ----
Linux     | [![process_migration_linux](/assets/gifs/process_migration.gif)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/gifs/process_migration.gif)
Windows   | [![process_migration_windows](/assets/gifs/process_migration_windows.gif)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/gifs/process_migration_windows.gif)

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
Linux     | None                      | [5 / 63](https://www.virustotal.com/gui/file/8253dcc8c4a14d62dd750b88f668c33388e2cd59d23d77cc9f424b520c52837d)  | [![process_migration_linux](/assets/images/linux/migration.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/linux/migration.png)
Linux     | `xor`                     | [0 / 62](https://www.virustotal.com/gui/file/a0e103bb36ca4ca51554226f95f777dfb658e8a2e56bff2e261f21736cf68aae)  | [![process_migration_linux_xor](/assets/images/linux/migration_xor.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/linux/migration_xor.png)
Windows   | `antisand,antistring`     | [22 / 70](https://www.virustotal.com/gui/file/b6d2382639613eaf2eb650a9a6c54eb78f45d3dd242a9d4ef332d0504f142267) | [![process_migration_windows_antistring](/assets/images/windows/migration_antistring.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/migration_antistring.png)
Windows   | `antisand`                | [21 / 70](https://www.virustotal.com/gui/file/89bb913356b81ebe209744369610c5cabc007ed5c2e6d1d1b94ced3c364cf6b1) | [![process_migration_windows_antisand](/assets/images/windows/migration_antisand.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/migration_antisand.png)
Windows   | None                      | [19 / 70](https://www.virustotal.com/gui/file/175ccccdae734fb2b3ce41e288c5c07e5816f77a798c63b6b3887b87c814ebe5) | [![process_migration_windows](/assets/images/windows/migration.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/migration.png)
Windows   | `antistring`              | [18 / 70](https://www.virustotal.com/gui/file/2b41b2e6501a211895c98588c36817e2f98e3092083ab9814e43b6041a188756) | [![process_migration_windows_antistring](/assets/images/windows/migration_antistring.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/migration_antistring.png)
Windows   | `xor`                     | [3 / 70](https://www.virustotal.com/gui/file/7c1e6496e7fa4bf223adffc8de6112de00f4a6abf792fa2773cbb11572c40402)  | [![process_migration_windows_xor](/assets/images/windows/migration_xor.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/migration_xor.png)
Windows   | `antistring,xor`          | [2 / 70](https://www.virustotal.com/gui/file/f7418c0f8a6d23603507c36930559d2d582352e458aa9361a2ba51aec75ec6ec/detection)  | [![process_migration_windows_antistring_xor](/assets/images/windows/migration_antistring_xor.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/migration_antistring_xor.png)
Windows   | `antisand,xor`            | [2 / 70](https://www.virustotal.com/gui/file/b16c369b164675b38cb90096f052bb9ba1bea205f58f08161531028047392736)  | [![process_migration_windows_antisand_xor](/assets/images/windows/migration_antisand_xor.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/migration_antisand_xor.png)
Windows   | `antisand,antistring,xor` | [1 / 70](https://www.virustotal.com/gui/file/c6d8812c5a771d731392aff19d11b0c7df8a5c966bf8a59ea98668fcd9f199a0)  | [![process_migration_windows_antisand_antistring_xor](/assets/images/windows/migration_antisand_antistring_xor.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/migration_antisand_antistring_xor.png)

<div class="datatable-end"></div>
