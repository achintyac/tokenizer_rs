use std::path::PathBuf;

/// Paths for tokenizer
pub struct Path {
    pub data: PathBuf,
    pub merges: PathBuf,
}

/// Vocabulary size that determines how many merges to do
pub struct Vocab {
    pub size: u32,
}

/// Casting token bytes to u32
pub fn convert_to_u32(tokens: Vec<u8>) -> Vec<u32> {
    let mut tokens_converted: Vec<u32> = Vec::new();
    for token in tokens {
        tokens_converted.push(u32::from(token));
    }
    tokens_converted
}

/// Casting nested token bytes to u32
pub fn convert_to_u32_nested(tokens: Vec<Vec<u8>>) -> Vec<Vec<u32>> {
    tokens
        .iter()
        .map(|inner_vec| inner_vec.iter().map(|u8_val| *u8_val as u32).collect())
        .collect()
}

/// Casting token bytes to u8
pub fn convert_to_u8(tokens: Vec<u32>) -> Vec<u8> {
    let mut tokens_converted: Vec<u8> = Vec::new();
    for token in tokens {
        tokens_converted.push(token as u8);
    }
    tokens_converted
}