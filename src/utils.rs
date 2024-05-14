use std::path::PathBuf;

pub struct Path {
    pub data: PathBuf,
    pub merges: PathBuf
}

pub struct Vocab {
    pub size: u32
}

pub fn convert_to_u32(
    tokens: Vec<u8>
) -> Vec<u32> {
    let mut tokens_converted: Vec<u32> = Vec::new();
    for token in tokens {
        tokens_converted.push(u32::from(token));
    }
    tokens_converted
}

pub fn convert_to_u8(
    tokens: Vec<u32>
) -> Vec<u8> {
    let mut tokens_converted: Vec<u8> = Vec::new();
    for token in tokens {
        tokens_converted.push(token as u8);
    }
    tokens_converted
}