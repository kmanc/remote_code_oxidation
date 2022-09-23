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
Linux     | None                      | [6 / 40](https://kleenscan.com/scan_result/6dac826ee10612cecc1dec4043f590638a2287416904518d38c8347d55bda054)  | [![process_migration_linux](/assets/images/linux/migration.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/linux/migration.png)
Linux     | `xor`                     | [0 / 40](https://kleenscan.com/scan_result/22551c73a19a51c251c8a3d95cd226bafad298db08bd0ec726591e86ef383ded)  | [![process_migration_linux_xor](/assets/images/linux/migration_xor.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/linux/migration_xor.png)
Windows   | None                      | [11 / 40](https://kleenscan.com/scan_result/53ed56235d4d13d7e64fb567e4033a6e72743a0fc6bf7be3fc6af2c538170cf7) | [![process_migration_windows](/assets/images/windows/migration.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/migration.png)
Windows   | `antisand`                | [11 / 40](https://kleenscan.com/scan_result/65a24b211b0f9c7012c6deebb0e46dab75314a5c7422a262f77c64a196599c3f) | [![process_migration_windows_antisand](/assets/images/windows/migration_antisand.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/migration_antisand.png)
Windows   | `antisand,antistring`     | [11 / 40](https://kleenscan.com/scan_result/28c7306b456435e6794752ad0965ad62c2c330bfcf99c8026c425ca6188f3b0c) | [![process_migration_windows_antistring](/assets/images/windows/migration_antistring.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/migration_antistring.png)
Windows   | `antistring`              | [11 / 40](https://kleenscan.com/scan_result/838c5b612c0419346395df55dfcd6c9278228e1003be78734bcf2210244d627d) | [![process_migration_windows_antistring](/assets/images/windows/migration_antistring.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/migration_antistring.png)
Windows   | `antistring,xor`          | [1 / 40](https://kleenscan.com/scan_result/9466c0bbfacb8f7ebc8e92b50947bdf836cc2b4adeed10ed1d92040c9366f555)  | [![process_migration_windows_antistring_xor](/assets/images/windows/migration_antistring_xor.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/migration_antistring_xor.png)
Windows   | `xor`                     | [1 / 40](https://kleenscan.com/scan_result/b65915b1a318d18c48195aa11fafd26937deb40c7fd60f123e447c279d9e9010)  | [![process_migration_windows_xor](/assets/images/windows/migration_xor.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/migration_xor.png)
Windows   | `antisand,xor`            | [0 / 40](https://kleenscan.com/scan_result/f343f23b7a8f3704784beb3f0902f994d97d4aae21f60383617a3c778365d9eb)  | [![process_migration_windows_antisand_xor](/assets/images/windows/migration_antisand_xor.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/migration_antisand_xor.png)
Windows   | `antisand,antistring,xor` | [0 / 40](https://kleenscan.com/scan_result/3291825d7fe6b514dfb40f28d8beae090bbf6d040b5f7206b6eb3c6ad10f43d7)  | [![process_migration_windows_antisand_antistring_xor](/assets/images/windows/migration_antisand_antistring_xor.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/migration_antisand_antistring_xor.png)

<div class="datatable-end"></div>
