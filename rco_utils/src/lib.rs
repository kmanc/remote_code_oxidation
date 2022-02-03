use std::error::Error;

fn equalize_slice_len<T: std::clone::Clone>(slice_one: &[T], slice_two: &[T]) -> (Vec<T>, Vec<T>) {
    if slice_one.len() > slice_two.len() {
        (slice_one.to_vec(), slice_two.iter().cloned().cycle().take(slice_one.len()).collect())
    } else {
        (slice_one.iter().cloned().cycle().take(slice_two.len()).collect(), slice_two.to_vec())
    }
}

fn xor_u8_slices(slice_one: &[u8], slice_two: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    if slice_one.len() != slice_two.len() {
        return Err("The given slices are not the same length".into());
    }
    Ok(slice_one.iter()
             .zip(slice_two.iter())
             .map(|(&x1, &x2)| x1 ^ x2)
             .collect())
}

pub fn xor_encrypt_decrypt(key: &[u8], text: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let equalilzed = equalize_slice_len(key, text);
    let key: &[u8] = &equalilzed.0[..];
    let text: &[u8] = &equalilzed.1[..];
    xor_u8_slices(key, text)
}