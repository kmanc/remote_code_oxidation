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

[No features - 7 / 40](https://kleenscan.com/scan_result/1177abafe77dc580337ec6294c68bdc4873ceb36a4eeac057fd0673c3ae50e7f) | [Using `--features xor` - 0 / 40](https://kleenscan.com/scan_result/8a0268ca750a14fc93f40f6b1864f13ce94318c4c4a7ecc49dfeb332b9c9d860)
:-------------------------:|:-------------------------:
![image](https://user-images.githubusercontent.com/14863147/164796610-611fa25b-a631-4f44-8e03-d80d0333dced.png) | ![image](https://user-images.githubusercontent.com/14863147/164796718-c37e8298-0a99-4b11-bcce-17638e31988a.png)


<p align="center"> Windows </p>

[No features - 12 / 40](https://kleenscan.com/scan_result/dd7858b48235bc782383fa5a929125369c7918d3c119a9196b0fdab791624763) | [Using `--features antisand` - 12 / 40](https://kleenscan.com/scan_result/dc73a322924b772b90957aaffe8d2735acd6d6049e0607a1befada2bc5aa86f3) | [Using `--features antistring` - 12 / 40](https://kleenscan.com/scan_result/1505ac5f33afe16a79796045d80c6c55617944c86396411487f1cbd934e875fb) | [Using `--features antisand,antistring` - 12 / 40](https://kleenscan.com/scan_result/177242f39b392107e4953a8cb717afbc6f912daa5bd9ec8d71a959834942db8d) | [Using `--features xor` - 6 / 40](https://kleenscan.com/scan_result/455d775c517cf26a6e83a42b3eae7982364d8a8174127eca377094c05e0dd948) | [Using `--features antistring,xor` - 1 / 40](https://kleenscan.com/scan_result/e6214cb0175737d1e3bba8bafbaa17d5aa575f613dab718a6d35dd46c7af8767) | [Using `--features antisand,xor` - 0 / 40](https://kleenscan.com/scan_result/de899245ec6a258d741b6243d18cf10fae5e6a1fe344ab3d02f17899a67d2bb7) | [Using `--features antisand,antistring,xor` - 0 / 40](https://kleenscan.com/scan_result/49f53e2e15b86d9e5425d684e9ab964289d2d96fef8ca61ba927e3826ebd0392)
:-------------------------:|:-------------------------:|:-------------------------:|:-------------------------:|:-------------------------:|:-------------------------:|:-------------------------:|:-------------------------:
![image](https://user-images.githubusercontent.com/14863147/164800253-42ae28de-dded-4d81-98b5-8169372247de.png) | ![image](https://user-images.githubusercontent.com/14863147/164803213-b5320794-8e57-4552-ab67-2358ffdd5a81.png) | ![image](https://user-images.githubusercontent.com/14863147/164803228-2ef5ddb5-cfee-4a98-8cad-c2a2c0aec4f9.png) | ![image](https://user-images.githubusercontent.com/14863147/164803261-073277a8-af2d-4a77-bf6d-7c3ea25bc663.png) | ![image](https://user-images.githubusercontent.com/14863147/164803307-97d7b75c-4c79-4699-a36c-d99923789751.png) | ![image](https://user-images.githubusercontent.com/14863147/164803332-2d97dd6b-7c78-4b9b-9a14-62430ea83f06.png) | ![image](https://user-images.githubusercontent.com/14863147/164803356-4e45da59-b6bb-4432-89c9-cc91cf1b895e.png) | ![image](https://user-images.githubusercontent.com/14863147/164803373-2cd49344-b5bb-48c1-b853-a7c893435f73.png)
