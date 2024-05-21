use std::fs;
use std::path::PathBuf;
use tokenizer_rs::tokenizer::{Tokenizer, TokenizerRegex};
use tokenizer_rs::{tokenizer_naive, tokenizer_gpt4, utils};

#[test]
fn full_test_base_tokenizer() {
    let paths = utils::Path {
        data: PathBuf::from("./tests/foo.txt"),
        merges: PathBuf::from("./tests/merges.txt"),
    };

    let vocab = utils::Vocab { size: 280 };

    let tokenizer = tokenizer_naive::BaseTokenizer {
        path: paths,
        vocab: vocab,
    };

    let initial_tokens: Vec<u8> = tokenizer.read_to_bytes(&tokenizer.path.data);
    let contents: Vec<u32> = utils::convert_to_u32(initial_tokens);
    let (contents, merges) = tokenizer.encode(contents, &tokenizer.vocab);
    let contents_new = tokenizer.decode(&contents, &merges);
    let ans = utils::convert_to_u8(contents_new);

    let original_contents =
        fs::read_to_string(tokenizer.path.data).expect("Should have been a file here to read");
    let final_converted_ans = String::from_utf8(ans).unwrap();
    assert_eq!(original_contents, final_converted_ans);
}

#[test]
fn run_through_regex_tokenizer() {
    let paths = utils::Path {
        data: PathBuf::from("./tests/foo.txt"),
        merges: PathBuf::from("./tests/merges.txt"),
    };

    let vocab = utils::Vocab { size: 280 };

    let tokenizer = tokenizer_gpt4::GPT4Tokenizer {
        path: paths,
        vocab: vocab,
        regex_pattern: String::from(r"'(?i:[sdmt]|ll|ve|re)|[^\r\n\p{L}\p{N}]?+\p{L}+|\p{N}{1,3}| ?[^\s\p{L}\p{N}]++[\r\n]*|\s*[\r\n]|\s+(?!\S)|\s+"),
    };

    let initial_tokens = tokenizer.read_chunks_to_bytes(&tokenizer.regex_pattern, &tokenizer.path.data);
    let contents: Vec<Vec<u32>>  = utils::convert_to_u32_nested(initial_tokens);
    let (contents, merges) = tokenizer.encode(contents, &tokenizer.vocab);
    let contents_new = tokenizer.decode(&contents, &merges);
    let ans = utils::convert_to_u8(contents_new);

    let original_contents =
        fs::read_to_string(tokenizer.path.data).expect("Should have been a file here to read");
    let final_converted_ans = String::from_utf8(ans).unwrap();

    assert_eq!(original_contents, final_converted_ans);    
}

#[test]
#[should_panic]
fn file_does_not_exist_naive() {
    let paths = utils::Path {
        data: PathBuf::from("./tests/file_does_not_exist.txt"),
        merges: PathBuf::from("./tests/merges.txt"),
    };

    let vocab = utils::Vocab { size: 280 };

    let tokenizer = tokenizer_naive::BaseTokenizer {
        path: paths,
        vocab: vocab,
    };

    let _initial_tokens = tokenizer.read_to_bytes(&tokenizer.path.data);
}

#[test]
#[should_panic]
fn file_does_not_exist_gpt4() {
    let paths = utils::Path {
        data: PathBuf::from("./tests/file_does_not_exist.txt"),
        merges: PathBuf::from("./tests/merges.txt"),
    };

    let vocab = utils::Vocab { size: 280 };

    let tokenizer = tokenizer_gpt4::GPT4Tokenizer {
        path: paths,
        vocab: vocab,
        regex_pattern: String::from(r"'(?i:[sdmt]|ll|ve|re)|[^\r\n\p{L}\p{N}]?+\p{L}+|\p{N}{1,3}| ?[^\s\p{L}\p{N}]++[\r\n]*|\s*[\r\n]|\s+(?!\S)|\s+"),
    };

    let _nitial_tokens = tokenizer.read_chunks_to_bytes(&tokenizer.regex_pattern, &tokenizer.path.data);

}