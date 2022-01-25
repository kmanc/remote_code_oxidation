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

2. Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/process_migration/src/config.rs) 
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

## Detection rates

[Linux - 6 / 40](https://kleenscan.com/scan_result/81ac12b76a4be897145f1772b8c57d6c5330ee9bd574f480a825232bd846d113)

[Windows - 12 / 40](https://kleenscan.com/scan_result/402c207f8a1e53a0f9b9ed533c53b96077956bc91367520ac28289b2fc6cc511)