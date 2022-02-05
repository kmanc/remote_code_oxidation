# RCO: Process Hollowing

[![Custom badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fkmanc%2Fremote_code_oxidation%2Fmaster%2F.custom_shields%2Fprocess_hollowing.json)](https://github.com/kmanc/remote_code_oxidation/tree/master/process_hollowing)

Linux target               |  Windows target
:-------------------------:|:-------------------------:
![gif](https://user-images.githubusercontent.com/14863147/151645583-68e054fc-ba49-4691-bca6-4e924f8ad498.gif)  |  ![gif](https://user-images.githubusercontent.com/14863147/151642061-6df0f601-3f07-4e0c-aaf5-fbbc229de2e0.gif)


## How it works

Windows process hollowing works by creating a suspended process and writing the [shellcode](https://en.wikipedia.org/wiki/Shellcode) over the entry point of the process's main thread. It then resumes the suspended process, which in turn executes the shellcode.

Linux process hollowing functions a little differently. First, the executable creates a child process by cloning itself. Then it overwrites the child process's [instruction pointer](https://datacadamia.com/computer/instruction/instruction_pointer) with shellcode. Because this process was not running before the attack, it is unlikely this will cause any issues on the victim machine.

## Using it

1. [Not shown in GIF] Generate shellcode for the desired end result (for example, use [msfvenom](https://book.hacktricks.xyz/shells/shells/msfvenom) to generate a reverse TCP shell shellcode for the target operating system)
2. [Not shown in GIF] Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs) 
and change the shellcode to the shellcode generated in step 1
3. [Optional] Encrypt the shellcode and target process using [xor_params](https://github.com/kmanc/remote_code_oxidation/blob/master/xor_shellcode) and update the encrypted shellcode value in [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs)  
4. [Not shown in GIF] Compile the executable, only including `--features xor` if you did step 3

    #### Build for Linux target
    ```commandline
    cargo build -p process_hollowing [--features [xor][antisand]] --release
    ```

    #### Build for Windows target
    ```commandline
    cargo build --target x86_64-pc-windows-gnu -p process_hollowing [--features [xor][antisand]] --release
    ```
5. Start a netcat listener on the attacking machine on the same port you configured the shellcode to connect to in step 1
    ```commandline
    nc -nlvp 4444
    ```   
6. Execute the payload on the victim machine
7. Return to the listener and enter desired commands for the victim machine to run


## Detection rates


<p align="center"> Linux </p>

[No features: 7 / 40](https://kleenscan.com/scan_result/9f584f6ba01c5d4cd09db05ccfa0d0be592a9522eeaaae6b8fa2c4d4f9d86433) | [Using `--features xor`: 0 / 40](https://kleenscan.com/scan_result/d9087bca23d0a3d74f335f404e66233a0fe6bf8954cddbac86c1028d17e36410)
:-------------------------:|:-------------------------:
![image](https://user-images.githubusercontent.com/14863147/151746886-343dac24-da1f-447e-b4df-2c35036c09dc.png) | ![image](https://user-images.githubusercontent.com/14863147/151746865-0be49000-efff-4d14-a2e1-afb3bd601bb1.png)

<p align="center"> Windows </p>

[No features: 13 / 40](https://kleenscan.com/scan_result/bac19828b35032fd7fa41f9293823b18aca6372fbf606c5428df0ca931aea502) | [Using `--features xor`: 1 / 40](https://kleenscan.com/scan_result/23d6063cc9bf35222c9aa604cc258de8aa8fb40a1fb443bfc97c8cdcb6ec2ad5) | [Using `--features xor,antisand`: 0 / 40](https://kleenscan.com/scan_result/52369fcedfa0029ec43806ea3b6b74d3163cff5adc94e561faf4707bbe502c85)
:-------------------------:|:-------------------------:|:-------------------------:
![image](https://user-images.githubusercontent.com/14863147/151746908-58824664-8072-4ce3-8895-e01057b868a6.png) | ![image](https://user-images.githubusercontent.com/14863147/151746900-49ec8f35-6718-4ac7-83fe-5e67610ff4c0.png)  | ![image](https://user-images.githubusercontent.com/14863147/152626795-6ca41258-2032-4753-b404-02322e9f0196.png)
