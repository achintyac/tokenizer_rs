use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

pub mod standard_tokenizer;

use standard_tokenizer::*;

fn main() {

    let paths = standard_tokenizer::Path {
        data: PathBuf::from("./foo.txt"),
        merges: PathBuf::from("./merges.txt"),
    };

    let vocab = standard_tokenizer::Vocab {
        size: 280
    };

    let tokenizer = standard_tokenizer::BaseTokenizer {
        path: paths,
        vocab: vocab
    };

    let initial_tokens = BaseTokenizer::read_to_bytes(tokenizer.path.data);
    let contents = BaseTokenizer::convert_to_u32(initial_tokens);
    let (contents, merges) = BaseTokenizer::encode(contents, tokenizer.vocab);
    let contents_new = BaseTokenizer::decode(contents, &merges);
    let ans = BaseTokenizer::convert_to_u8(contents_new);
    println!("tokens: \n {}", String::from_utf8(ans).unwrap());
}