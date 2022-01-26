# RCO: Process Migration

Linux target               |  Windows target
:-------------------------:|:-------------------------:
![](https://user-images.githubusercontent.com/14863147/151044951-5ee5b376-9f62-4e2e-a773-8c3b7a7d580e.gif)  |  ![](https://user-images.githubusercontent.com/14863147/151059013-b053e9de-d75c-4470-97a7-a109c7f2ef55.gif)


## Using it

1. [Not shown in GIF] Generate shellcode for the desired end result (for example, use [msfvenom](https://book.hacktricks.xyz/shells/shells/msfvenom) to generate a reverse TCP
shell shellcode for the target operating system)
2. [Not shown in GIF] Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs) 
and change the shellcode to the shellcode generated in step 1
3. [Not shown in GIF] Compile the executable

    #### Build for Linux target
    ```commandline
    cargo build -p process_migration --release
    ```

    #### Build for Windows target
    ```commandline
    cargo build --target x86_64-pc-windows-gnu -p process_migration --release
    ```
4. Start a netcat listener on the attacking machine on the same port you configured the shellcode to connect to in step 1
    ```commandline
    nc -nlvp 4444
    ```   
5. Execute the payload on the victim machine
6. Return to the listener and enter desired commands for the victim machine to run


## Detection rates

[Linux - 6 / 40](https://kleenscan.com/scan_result/81ac12b76a4be897145f1772b8c57d6c5330ee9bd574f480a825232bd846d113)

![image](https://user-images.githubusercontent.com/14863147/151022870-a65ecbcc-7579-42c3-be6c-6c117c64bbd3.png)

[Windows - 12 / 40](https://kleenscan.com/scan_result/402c207f8a1e53a0f9b9ed533c53b96077956bc91367520ac28289b2fc6cc511)

![image](https://user-images.githubusercontent.com/14863147/151023019-735113d1-df25-41d7-8edc-b031320c7cea.png)


## How it works

RCO's Windows process migration works by obtaining a handle to the target process and writing [shellcode](https://en.wikipedia.org/wiki/Shellcode) to it. Then it
spawns a remote thread within the process whose starting point is the newly written shellcode. RCO's Linux process migration works slightly differently; it temporarily pauses the target process, then writes the shellcode over the [instruction pointer](https://datacadamia.com/computer/instruction/instruction_pointer) for that process. This will likely cause issues with the process.
