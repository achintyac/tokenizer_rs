use crate::tokenizer::{Tokenizer, TokenizerRegex};
use crate::utils::{Path, Vocab};
use std::path::PathBuf;
use fancy_regex::Regex;
use std::fs::read_to_string;


pub struct GPT4Tokenizer {
    pub path: Path,
    pub vocab: Vocab,
    pub regex_pattern: String,
}

impl TokenizerRegex for GPT4Tokenizer {}

#[cfg(test)]
mod tests {
    use super::*;

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
            read_to_string(&tokenizer.path.data).expect("Should have been a file here to read")
        );
        assert_eq!(result, ["Sherlock", " Holmes"]);
    }
}