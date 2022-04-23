---
title: "TCP Reverse Shell"
datatable: true
---

# RCO: TCP Reverse Shell

[![Custom badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fkmanc%2Fremote_code_oxidation%2Fmaster%2F.custom_shields%2Ftcp_reverse_shell.json)](https://github.com/kmanc/remote_code_oxidation/tree/master/tcp_reverse_shell)

<div class="datatable-begin"></div>

Target OS | Demo
--------- | ----
Linux     | ![gif](/gifs/tcp_reverse_shell.gif)
Windows   | ![gif](/gifs/tcp_reverse_shell_windows.gif)

<div class="datatable-end"></div>

## How it works

The reverse shells for both Linux and Windows work by establishing a [Transmission Control Protocol (TCP) session](https://www.scottklement.com/rpg/socktut/overview.html) from the victim machine to the attacking machine. Then a Terminal (Linux) or Command Prompt (Windows) process starts with its [standard input](https://en.wikipedia.org/wiki/Standard_streams#Standard_input_(stdin)), [standard output](https://en.wikipedia.org/wiki/Standard_streams#Standard_output_(stdout)), and [standard error](https://en.wikipedia.org/wiki/Standard_streams#Standard_error_(stderr)) all assigned to the TCP session. This means that input commands and output results are read from and written to (respectively) the TCP stream.


## Using it

1.  [Not shown in GIF] Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs) 
and change the IP address and port to match the IP address of your attacking machine and the port you will use for a listener respectively
2.  [Not shown in GIF] Compile the executable
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
Linux     | None                      | [0 / 40](https://kleenscan.com/scan_result/c01984f5bc45f0ff82723fe6ceab770fe48e955081f8b02e17a8232e6ba2bbeb)  | [![image](https://user-images.githubusercontent.com/14863147/164796610-611fa25b-a631-4f44-8e03-d80d0333dced.png)](https://user-images.githubusercontent.com/14863147/164796610-611fa25b-a631-4f44-8e03-d80d0333dced.png)
Windows   | None                      | [0 / 40](https://kleenscan.com/scan_result/ce74ac206b59e9acc4e7f528bcec06f2a1dcc8ac0a1fb622c0b646cdfd2602d5) | [![image](https://user-images.githubusercontent.com/14863147/164800253-42ae28de-dded-4d81-98b5-8169372247de.png)](https://user-images.githubusercontent.com/14863147/164800253-42ae28de-dded-4d81-98b5-8169372247de.png)
Windows   | `antisand`                | [0 / 40](https://kleenscan.com/scan_result/28fce6da1a75b3d0073649613d5e69b73019091e1a7c2a2033b1551755c5fad4) | [![image](https://user-images.githubusercontent.com/14863147/164803213-b5320794-8e57-4552-ab67-2358ffdd5a81.png)](https://user-images.githubusercontent.com/14863147/164803213-b5320794-8e57-4552-ab67-2358ffdd5a81.png)
Windows   | `antistring`              | [0 / 40](https://kleenscan.com/scan_result/fafcad9c3689cf811184cacc3c1e9f939017b4e5d362712468839a6126f82278) | [![image](https://user-images.githubusercontent.com/14863147/164803228-2ef5ddb5-cfee-4a98-8cad-c2a2c0aec4f9.png)](https://user-images.githubusercontent.com/14863147/164803228-2ef5ddb5-cfee-4a98-8cad-c2a2c0aec4f9.png)
Windows   | `antisand,antistring`     | [0 / 40](https://kleenscan.com/scan_result/ff8c1a3fda94bd5f73314e15c9861284250b88720f045351aedc937435b9d8bd) | [![image](https://user-images.githubusercontent.com/14863147/164803261-073277a8-af2d-4a77-bf6d-7c3ea25bc663.png)](https://user-images.githubusercontent.com/14863147/164803261-073277a8-af2d-4a77-bf6d-7c3ea25bc663.png)

<div class="datatable-end"></div>