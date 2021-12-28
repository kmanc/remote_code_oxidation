# RCO: Process Migration

## What it is

RCO's process migration is a technique used to hide malicious code running on a victim machine. It has the added benefit
of making it less likely the the malicious code is accidentally closed by a user on a victim machine by moving the payload
to a process that is unlikely to be terminated.

## How it works

RCO's Windows process migration works by obtaining a handle to the target process and writing the shellcode to it. Then it
spawns a remote thread within the process whose starting point is the newly written shellcode. At the time of this writing,
RCO's Unix process migration is not yet written.

## Using it

1. Generate shellcode for the desired end result (for example, use [msfvenom](http://127.0.0.1) to generate a reverse TCP
shell shellcode for the target operating system)

2. Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/process_migration/src/config.rs) 
and change the shellcode to the shellcode generated in step 1

3. Compile the executable

    #### Build for Unix target
    ```commandline
    cargo build -p process_migration --release
    ```

    #### Build for Windows target
    ```commandline
    cargo build --target x86_64-pc-windows-gnu -p process_migration --release
    ```
   
4. Execute the payload on the victim machine
