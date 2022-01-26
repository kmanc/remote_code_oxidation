# RCO: Process Migration

## What it is

RCO's process migration is a technique used to hide malicious code running on a victim machine. It has the added benefit
of making it less likely the the malicious code is accidentally closed by a user on a victim machine by moving the payload
to a process that is unlikely to be terminated.

## How it works

RCO's Windows process migration works by obtaining a handle to the target process and writing [shellcode](https://en.wikipedia.org/wiki/Shellcode) to it. Then it
spawns a remote thread within the process whose starting point is the newly written shellcode. RCO's Linux process migration works slightly differently; it temporarily pauses the target process, then writes the shellcode over the [instruction pointer](https://datacadamia.com/computer/instruction/instruction_pointer) for that process. This will likely cause issues with the process.

## Using it

1. Generate shellcode for the desired end result (for example, use [msfvenom](https://book.hacktricks.xyz/shells/shells/msfvenom) to generate a reverse TCP
shell shellcode for the target operating system)

2. Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/rco_config/src/lib.rs) 
and change the shellcode to the shellcode generated in step 1

3. Compile the executable

    #### Build for Linux target
    ```commandline
    cargo build -p process_migration --release
    ```

    #### Build for Windows target
    ```commandline
    cargo build --target x86_64-pc-windows-gnu -p process_migration --release
    ```
   
4. Execute the payload on the victim machine
5. Return to the listener and enter desired commands for the victim machine to run

![process_migration_linux](https://user-images.githubusercontent.com/14863147/151044951-5ee5b376-9f62-4e2e-a773-8c3b7a7d580e.gif)

![process_migration_windows](https://user-images.githubusercontent.com/14863147/151059013-b053e9de-d75c-4470-97a7-a109c7f2ef55.gif)

## Detection rates

[Linux - 6 / 40](https://kleenscan.com/scan_result/81ac12b76a4be897145f1772b8c57d6c5330ee9bd574f480a825232bd846d113)

![image](https://user-images.githubusercontent.com/14863147/151022870-a65ecbcc-7579-42c3-be6c-6c117c64bbd3.png)

[Windows - 12 / 40](https://kleenscan.com/scan_result/402c207f8a1e53a0f9b9ed533c53b96077956bc91367520ac28289b2fc6cc511)

![image](https://user-images.githubusercontent.com/14863147/151023019-735113d1-df25-41d7-8edc-b031320c7cea.png)
