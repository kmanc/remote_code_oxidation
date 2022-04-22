# RCO: TCP Reverse Shell

[![Custom badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fkmanc%2Fremote_code_oxidation%2Fmaster%2F.custom_shields%2Ftcp_reverse_shell.json)](https://github.com/kmanc/remote_code_oxidation/tree/master/tcp_reverse_shell)

Linux target               |  Windows target
:-------------------------:|:-------------------------:
![gif](https://user-images.githubusercontent.com/14863147/151044738-72dbec76-73b1-47c5-831a-2e995673b559.gif)  |  ![gif](https://user-images.githubusercontent.com/14863147/151058869-8e5e9f6e-3f7b-4a98-ab65-2ac5017a4e76.gif)


## How it works

The reverse shells for both Linux and Windows work by establishing a [Transmission Control Protocol (TCP) session](https://www.scottklement.com/rpg/socktut/overview.html) from the victim machine to the attacking machine. Then a Terminal (Linux) or Command Prompt (Windows) process starts with its [standard input](https://en.wikipedia.org/wiki/Standard_streams#Standard_input_(stdin)), [standard output](https://en.wikipedia.org/wiki/Standard_streams#Standard_output_(stdout)), and [standard error](https://en.wikipedia.org/wiki/Standard_streams#Standard_error_(stderr)) all assigned to the TCP session. This means that input commands and output results are read from and written to (respectively) the TCP stream.


## Using it

1. [Not shown in GIF] Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs) 
and change the IP address and port to match the IP address of your attacking machine and the port you will use for a listener respectively
2. [Not shown in GIF] Compile the executable

    #### For Linux targets
    ```commandline
    cargo build -p tcp_reverse_shell [antisand][,][antistring]] --release
    ```

    #### For Windows targets
    ```commandline
    cargo build --target x86_64-pc-windows-gnu -p tcp_reverse_shell [--features [antisand][,][antistring]] --release
    ```
3. Start a netcat listener on the attacking machine on the same port you configured in step 1
    ```commandline
    nc -nlvp 4444
    ```
4. Execute the payload on the victim machine
5. Return to the listener and enter desired commands for the victim machine to run


## Detection rates

<p align="center"> Linux </p>

[No features: 0 / 40](https://kleenscan.com/scan_result/c01984f5bc45f0ff82723fe6ceab770fe48e955081f8b02e17a8232e6ba2bbeb)
:-------------------------:
![image](https://user-images.githubusercontent.com/14863147/164814492-64a4c3e7-e736-4bee-9baa-87d9d49b417b.png)


<p align="center"> Windows </p>

[No features: 0 / 40](https://kleenscan.com/scan_result/ce74ac206b59e9acc4e7f528bcec06f2a1dcc8ac0a1fb622c0b646cdfd2602d5) | [Using `--features antisand`: 0 / 40](https://kleenscan.com/scan_result/28fce6da1a75b3d0073649613d5e69b73019091e1a7c2a2033b1551755c5fad4) | [Using `--features antistring`: 0 / 40](https://kleenscan.com/scan_result/fafcad9c3689cf811184cacc3c1e9f939017b4e5d362712468839a6126f82278) | [Using `--features antisand,antistring`: 0 / 40](https://kleenscan.com/scan_result/ff8c1a3fda94bd5f73314e15c9861284250b88720f045351aedc937435b9d8bd) 
:-------------------------:|:-------------------------:|:-------------------------:|:-------------------------:
![image](https://user-images.githubusercontent.com/14863147/164814640-1355d414-087b-48c0-9067-2ab6f2ad12bf.png) | ![image](https://user-images.githubusercontent.com/14863147/164814649-9f3ccebd-427e-432f-adcf-06cab5b37c45.png) | ![image](https://user-images.githubusercontent.com/14863147/164814643-a32b5580-c6f0-4ff4-ba44-9c71b408f386.png) | ![image](https://user-images.githubusercontent.com/14863147/164814653-4d6dfe0c-a0b2-44ef-b513-920b8a5530e1.png)
