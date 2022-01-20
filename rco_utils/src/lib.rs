// Taken from https://stackoverflow.com/questions/36669427/does-rust-have-a-way-to-convert-several-bytes-to-a-number
pub fn array_to_u32_big_end(array: &[u8; 4]) -> u32 {
    (array[0] as u32) << 24 |
    (array[1] as u32) << 16 |
    (array[2] as u32) <<  8 |
    (array[3] as u32)
}

// Taken from https://stackoverflow.com/questions/36669427/does-rust-have-a-way-to-convert-several-bytes-to-a-number
pub fn array_to_u32_lit_end(array: &[u8; 4]) -> u32 {
    (array[0] as u32)       |
    (array[1] as u32) <<  8 |
    (array[2] as u32) << 16 |
    (array[3] as u32) << 24
}
