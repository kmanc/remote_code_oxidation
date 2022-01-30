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

pub fn array_to_u64_big_end(array: &[u8; 8]) -> u64 {
    (array[0] as u64) << 56 |
    (array[1] as u64) << 48 |
    (array[2] as u64) << 40 |
    (array[3] as u64) << 32 |
    (array[4] as u64) << 24 |
    (array[5] as u64) << 16 |
    (array[6] as u64) << 8  |
    (array[7] as u64)
}

pub fn array_to_u64_lit_end(array: &[u8; 8]) -> u64 {
    (array[0] as u64)       |
    (array[1] as u64) <<  8 |
    (array[2] as u64) << 16 |
    (array[3] as u64) << 24 |
    (array[4] as u64) << 32 |
    (array[5] as u64) << 40 |
    (array[6] as u64) << 48 |
    (array[7] as u64) << 56
}

pub fn xor_u8_slices(slice_one: &[u8], slice_two: &[u8]) -> Vec<u8> {
    slice_one.iter()
             .zip(slice_two.iter())
             .map(|(&x1, &x2)| x1 ^ x2)
             .collect()
}