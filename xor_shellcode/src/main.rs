fn main() {
    let win_shellcode = rco_config::WINDOWS_SHELLCODE;
    let lin_shellcode = rco_config::LINUX_SHELLCODE;
    let key = rco_config::XOR_KEY;

    let equalilzed = rco_utils::equalize_slice_len(key, win_shellcode);
    let win_key: &[u8] = &equalilzed.0[..];
    let win_shellcode: &[u8] = &equalilzed.1[..];
    let win_output = rco_utils::xor_u8_slices(win_key, win_shellcode).unwrap();
    let mut win_print = "".to_owned();
    for byte in win_output.iter() {
        let formatted = format!("{byte:#04x}, ");
        win_print.push_str(&formatted);
    }
    win_print.pop();
    win_print.pop();
    println!("pub const ENCRYPTED_WINDOWS_SHELLCODE: &[u8] = &[{win_print}];");

    println!();

    let equalilzed = rco_utils::equalize_slice_len(key, lin_shellcode);
    let lin_key: &[u8] = &equalilzed.0[..];
    let lin_shellcode: &[u8] = &equalilzed.1[..];
    let lin_output = rco_utils::xor_u8_slices(lin_key, lin_shellcode).unwrap();
    let mut lin_print = "".to_owned();
    for byte in lin_output.iter() {
        let formatted = format!("{byte:#04x}, ");
        lin_print.push_str(&formatted);
    }
    lin_print.pop();
    lin_print.pop();
    println!("pub const ENCRYPTED_LINUX_SHELLCODE: &[u8] = &[{lin_print}];");
     
}
