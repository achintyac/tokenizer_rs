use crate::tokenizer::TokenizerRegex;
use crate::utils::*;


pub struct GPT4Tokenizer {
    pub path: Path,
    pub vocab: Vocab,
    pub regex_pattern: String,
}

impl TokenizerRegex for GPT4Tokenizer {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::fs;

    #[test]
    fn regex_compile() {
        let paths = Path {
            data: PathBuf::from("./tests/foo.txt"),
            merges: PathBuf::from("./tests/merges.txt"),
        };
    
        let vocab = Vocab { size: 280 };
    
        let tokenizer = GPT4Tokenizer {
            path: paths,
            vocab: vocab,
            regex_pattern: String::from(r"'(?i:[sdmt]|ll|ve|re)|[^\r\n\p{L}\p{N}]?+\p{L}+|\p{N}{1,3}| ?[^\s\p{L}\p{N}]++[\r\n]*|\s*[\r\n]|\s+(?!\S)|\s+"),
        };

        let result = tokenizer.process_data_chunk(
            &tokenizer.regex_pattern,
            String::from("Sherlock Holmes ran 22 miles"),
        );
        assert_eq!(result, ["Sherlock", " Holmes", " ran",  " ", "22", " miles"]);
    }

    #[test]
    fn run_through_regex_tokenizer() {
        let paths = Path {
            data: PathBuf::from("./tests/foo.txt"),
            merges: PathBuf::from("./tests/merges.txt"),
        };
    
        let vocab = Vocab { size: 280 };
    
        let tokenizer = GPT4Tokenizer {
            path: paths,
            vocab: vocab,
            regex_pattern: String::from(r"'(?i:[sdmt]|ll|ve|re)|[^\r\n\p{L}\p{N}]?+\p{L}+|\p{N}{1,3}| ?[^\s\p{L}\p{N}]++[\r\n]*|\s*[\r\n]|\s+(?!\S)|\s+"),
        };

        let initial_tokens = tokenizer.read_chunks_to_bytes(&tokenizer.regex_pattern, &tokenizer.path.data);
        let contents: Vec<Vec<u32>>  = convert_to_u32_nested(initial_tokens);
        let (contents, merges) = tokenizer.encode(contents, &tokenizer.vocab);
        let contents_new = tokenizer.decode(&contents, &merges);
        let ans = convert_to_u8(contents_new);
    
        let original_contents =
            fs::read_to_string(tokenizer.path.data).expect("Should have been a file here to read");
        let final_converted_ans = String::from_utf8(ans).unwrap();

        assert_eq!(original_contents, final_converted_ans);    
    }
}