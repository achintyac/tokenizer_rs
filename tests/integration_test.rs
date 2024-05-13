use std::path::PathBuf;
use std::fs;

use tokenizer_rs::tokenizer;

#[test]
fn full_run_through() {
    let paths = tokenizer::Path {
        data: PathBuf::from("./tests/foo.txt"),
        merges: PathBuf::from("./tests/merges.txt"),
    };

    let vocab = tokenizer::Vocab {
        size: 280
    };

    let tokenizer = tokenizer::BaseTokenizer {
        path: paths,
        vocab: vocab
    };

    let initial_tokens = tokenizer::BaseTokenizer::read_to_bytes(&tokenizer.path.data);
    let contents = tokenizer::BaseTokenizer::convert_to_u32(initial_tokens);
    let (contents, merges) = tokenizer::BaseTokenizer::encode(contents, tokenizer.vocab);
    let contents_new = tokenizer::BaseTokenizer::decode(&contents, &merges);
    let ans = tokenizer::BaseTokenizer::convert_to_u8(contents_new);

    let original_contents = fs::read_to_string(tokenizer.path.data).expect("Should have been a file here to read");
    let final_converted_ans = String::from_utf8(ans).unwrap();
    assert_eq!(original_contents, final_converted_ans);
}