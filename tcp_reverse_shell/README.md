# RCO: TCP Reverse Shell

## What it is

RCO's TCP reverse shell is a no-frills [reverse shell](https://www.hackingtutorials.org/networking/hacking-netcat-part-2-bind-reverse-shells/). This means it allows you to run Terminal (Linux)
or Command Prompt (Windows) commands _on the victim machine_ from the attacking machine. Getting a remote shell on a
victim machine usually allows much more control over the victim than other attacks. 

## How it works

Both RCO's Linux and Windows TCP reverse shells work by establishing a [TCP session](https://www.scottklement.com/rpg/socktut/overview.html) from the victim machine
to the attacking machine. Then a Terminal (Linux) or Command Prompt (Windows) process is spun up with its [standard input](https://en.wikipedia.org/wiki/Standard_streams#Standard_input_(stdin)),
[standard output](https://en.wikipedia.org/wiki/Standard_streams#Standard_output_(stdout)), and [standard error](https://en.wikipedia.org/wiki/Standard_streams#Standard_error_(stderr)) all assigned to the TCP session. This means that
input commands and output results are read from and written to (respectively) a TCP stream.

## Using it

1. Start a listener on the attacking machine
    ```commandline
    nc -nlvp 4444
    ```

2. Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/tcp_reverse_shell/src/config.rs) 
and change the IP address and port to match your attacking machine and listener, respectively

3. Compile the executable

    #### For Linux targets
    ```commandline
    cargo build -p tcp_reverse_shell --release
    ```

    #### For Windows targets
    ```commandline
    cargo build --target x86_64-pc-windows-gnu -p tcp_reverse_shell --release
    ```

4. Execute the payload on the victim machine

5. Return to the listener and enter desired commands for the victim machine to run

![tcp_reverse_shell_linux](https://user-images.githubusercontent.com/14863147/151044738-72dbec76-73b1-47c5-831a-2e995673b559.gif)

![tcp_reverse_shell_windows](https://user-images.githubusercontent.com/14863147/151058869-8e5e9f6e-3f7b-4a98-ab65-2ac5017a4e76.gif)


## Detection rates

[Linux - 0 / 40](https://kleenscan.com/scan_result/d185d1b1ba09f1a9133b72cb1bb12f177b32f3e65962baf7789b03c968ec8e47)

![image](https://user-images.githubusercontent.com/14863147/151023219-84d7ab95-002e-4d20-89c7-264894d72683.png)

[Windows - 0 / 40](https://kleenscan.com/scan_result/e9ffcf8fffd390b68a4cbc9d506571f0618e473732015be748e1960300bc6a85)

![image](https://user-images.githubusercontent.com/14863147/151023325-d917df57-18ce-4e99-b059-968a479b65ce.png)
