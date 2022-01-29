# RCO: Process Hollowing

[![Custom badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fkmanc%2Fremote_code_oxidation%2Fmaster%2F.custom_shields%2Fprocess_hollowing.json)](https://github.com/kmanc/remote_code_oxidation/tree/master/process_hollowing)

Linux target               |  Windows target
:-------------------------:|:-------------------------:
![](https://user-images.githubusercontent.com/14863147/151645583-68e054fc-ba49-4691-bca6-4e924f8ad498.gif)  |  ![](https://user-images.githubusercontent.com/14863147/151642061-6df0f601-3f07-4e0c-aaf5-fbbc229de2e0.gif)


## How it works

Windows process hollowing works by createing a suspended process and writing the [shellcode](https://en.wikipedia.org/wiki/Shellcode) over the entry point of the process's main thread. It then resumes the suspended process, which in turn executes the shellcode.

Linux process hollowing functions a little differently. The executable first creates a child process that is a clone of itself. Then it overwrites the child process's [instruction pointer](https://datacadamia.com/computer/instruction/instruction_pointer) with shellcode. Because this process was not running before, it is unlikely this will cause any issues on the victim machine.

## Using it

1. [Not shown in GIF] Generate shellcode for the desired end result (for example, use [msfvenom](https://book.hacktricks.xyz/shells/shells/msfvenom) to generate a reverse TCP shell shellcode for the target operating system)
2. Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs) 
and change the shellcode to the shellcode generated in step 1
3. Compile the executable

    #### Build for Linux target
    ```commandline
    cargo build -p process_hollowing --release
    ```

    #### Build for Windows target
    ```commandline
    cargo build --target x86_64-pc-windows-gnu -p process_hollowing --release
    ```
4. Start a netcat listener on the attacking machine on the same port you configured the shellcode to connect to in step 1
    ```commandline
    nc -nlvp 4444
    ```   
5. Execute the payload on the victim machine
6. Return to the listener and enter desired commands for the victim machine to run


## Detection rates

[Linux - 7 / 40](https://kleenscan.com/scan_result/9f584f6ba01c5d4cd09db05ccfa0d0be592a9522eeaaae6b8fa2c4d4f9d86433)

![image](https://user-images.githubusercontent.com/14863147/151648580-225124c1-eb34-42f2-81c8-645f68b68a29.png)

[Windows - 13 / 40](https://kleenscan.com/scan_result/bac19828b35032fd7fa41f9293823b18aca6372fbf606c5428df0ca931aea502)

![image](https://user-images.githubusercontent.com/14863147/151648602-95557f13-5fc4-46ea-96fb-6a9f6022097b.png)
