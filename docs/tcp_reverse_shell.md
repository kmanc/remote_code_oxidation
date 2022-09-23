---
title: "TCP Reverse Shell"
datatable: true
---

# RCO: TCP Reverse Shell

[![TCP reverse shell version unavailable](https://img.shields.io/crates/v/tcp_reverse_shell?label=tcp_reverse_shell)](https://github.com/kmanc/remote_code_oxidation/tree/master/tcp_reverse_shell)

<div class="datatable-begin"></div>

Target OS | Demo
--------- | ----
Linux     | [![tcp_reverse_shell_linux](/assets/gifs/tcp_reverse_shell.gif)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/gifs/tcp_reverse_shell.gif)
Windows   | [![tcp_reverse_shell_windows](/assets/gifs/tcp_reverse_shell_windows.gif)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/gifs/tcp_reverse_shell_windows.gif)

<div class="datatable-end"></div>

## How it works

The reverse shells for both Linux and Windows work by establishing a [Transmission Control Protocol (TCP) session](https://www.scottklement.com/rpg/socktut/overview.html) from the victim machine to the attacking machine. Then a Terminal (Linux) or Command Prompt (Windows) process starts with its [standard input](https://en.wikipedia.org/wiki/Standard_streams#Standard_input_(stdin)), [standard output](https://en.wikipedia.org/wiki/Standard_streams#Standard_output_(stdout)), and [standard error](https://en.wikipedia.org/wiki/Standard_streams#Standard_error_(stderr)) all assigned to the TCP session. This means that input commands and output results are read from and written to (respectively) the TCP stream.


## Using it

1.  *[Not shown in demo]* Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs) 
and change the IP address and port to match the IP address of your attacking machine and the port you will use for a listener respectively
2.  *[Not shown in demo]* Compile the executable
    1.  For Linux targets
    ```commandline
    cargo build -p tcp_reverse_shell [antisand][,][antistring]] --release
    ```
    2.  For Windows targets
    ```commandline
    cargo build --target x86_64-pc-windows-gnu -p tcp_reverse_shell [antisand][,][antistring]] --release
    ```
3.  Start a netcat listener on the attacking machine on the same port you configured in step 1
```commandline
nc -nlvp 4444
```
4.  Execute the payload on the victim machine
5.  Return to the listener and enter desired commands for the victim machine to run


## Detection rates

<div class="datatable-begin"></div>

Target OS | Features                      | Detections                             | Screenshot
--------- | ----------------------------- | -------------------------------------- | ----------
Linux     | None                      | [0 / 40](https://kleenscan.com/scan_result/a940621ddd4de3ac694ab97584536a1e0a06a222f174d023051c5c7786cb02ce) | [![tcp_reverse_shell_linux](/assets/images/linux/shell.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/linux/shell.png)
Windows   | None                      | [0 / 40](https://kleenscan.com/scan_result/3125f79d0b309eaecab29d4dbd8fb1521a53ca9bbd0f0b08f469a9c21cb0cb7b) | [![tcp_reverse_shell_windows](/assets/images/windows/shell.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/shell.png)
Windows   | `antisand`                | [0 / 40](https://kleenscan.com/scan_result/5554067d19d276e3b9c85967d1c0044bdf3d1ab3ca36be816cc4f80d0296df1b) | [![tcp_reverse_shell_windows_antisand](/assets/images/windows/shell_antisand.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/shell_antisand.png)
Windows   | `antisand,antistring`     | [0 / 40](https://kleenscan.com/scan_result/a0dc07b781618acdbc07c32cb12e5a41a860904672c288ba860a830e645eaaf8) | [![tcp_reverse_shell_windows_antisand_antistring](/assets/images/windows/shell_antisand_antistring.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/shell_antisand_antistring.png)
Windows   | `antistring`              | [0 / 40](https://kleenscan.com/scan_result/2430b38a062a6aa57ac52f508308bcc171b258fc04c06d9d05ae39c0bd1e7417) | [![tcp_reverse_shell_windows_antistring](/assets/images/windows/shell_antistring.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/shell_antistring.png)

<div class="datatable-end"></div>
