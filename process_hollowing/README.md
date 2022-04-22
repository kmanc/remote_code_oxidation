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
    cargo build -p process_hollowing [xor][,][antisand][,][antistring]] --release
    ```

    #### Build for Windows target
    ```commandline
    cargo build --target x86_64-pc-windows-gnu -p process_hollowing [xor][,][antisand][,][antistring]] --release
    ```
5. Start a netcat listener on the attacking machine on the same port you configured the shellcode to connect to in step 1
    ```commandline
    nc -nlvp 4444
    ```   
6. Execute the payload on the victim machine
7. Return to the listener and enter desired commands for the victim machine to run


## Detection rates


<p align="center"> Linux </p>

[No features - 7 / 40](https://kleenscan.com/scan_result/455ff90547b103920f1c15ecf9c7189031828105db42541d7b4af1ec35c3351b) | [Using `--features xor` - 0 / 40](https://kleenscan.com/scan_result/bc54ab43e9eb41f529eb79416a54b29db3c8730b5b04964ac3cc7f7d4e5c8517)
:-------------------------:|:-------------------------:
![image](https://user-images.githubusercontent.com/14863147/152622612-63ff6018-7c51-46bf-ab39-b602d7deb2c9.png) | ![image](https://user-images.githubusercontent.com/14863147/152622626-5f66edf1-9e66-4ead-8014-42df0259c1ce.png)

<p align="center"> Windows </p>

[No features - 13 / 40](https://kleenscan.com/scan_result/71843ced31794501973ee125c957a9c98b7194e6e47114aa6b3a22adf1dbf8b1) | [Using `--features xor` - 1 / 40](https://kleenscan.com/scan_result/78f3797dc160a38d75afec8c8ef98fe9f0221aef11089ca020d481df60028c21) | [Using `--features xor,antisand` - 0 / 40](https://kleenscan.com/scan_result/52369fcedfa0029ec43806ea3b6b74d3163cff5adc94e561faf4707bbe502c85)
:-------------------------:|:-------------------------:|:-------------------------:
![image](https://user-images.githubusercontent.com/14863147/152622727-917b3374-a951-4e13-805c-4face1aa00e9.png) | ![image](https://user-images.githubusercontent.com/14863147/152622738-85205628-10dd-4a19-849a-51139b0035f4.png) | ![image](https://user-images.githubusercontent.com/14863147/152622747-5a2492f8-aeb7-421a-b802-f694419e5720.png)
