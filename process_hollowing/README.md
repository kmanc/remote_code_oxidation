# RCO: Process Hollowing

## What it is

RCO's process hollowing is a technique used to hide malicious code running on a victim machine. It has the added benefit
of making it less likely the the malicious code is accidentally closed by a user on a victim machine by moving the payload
to a process that is unlikely to be terminated.

## How it works

Not yet written

## Using it

1. Generate shellcode for the desired end result (for example, use [msfvenom](http://127.0.0.1) to generate a reverse TCP
shell shellcode for the target operating system)

2. Open [the config file](https://github.com/kmanc/remote_code_oxidation/blob/master/process_hollowing/src/config.rs) 
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
   
4. Execute the payload on the victim machine