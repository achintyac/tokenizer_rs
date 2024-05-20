use std::fs;
use std::path::PathBuf;

use tokenizer_rs::tokenizer::Tokenizer;
use tokenizer_rs::{tokenizer_naive, utils};

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
#[should_panic]
fn file_does_not_exist() {
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
