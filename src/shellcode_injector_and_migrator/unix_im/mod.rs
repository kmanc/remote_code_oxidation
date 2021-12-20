use std::str;

pub fn inject_and_migrate(shellcode: &[u8]) {
    let shellcode = str::from_utf8(shellcode).unwrap();
    println!("{}", shellcode);
}