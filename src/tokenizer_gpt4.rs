use crate::tokenizer::TokenizerRegex;
use crate::utils::*;
use std::collections::HashMap;

pub struct GPT4SpecialTokens {
    pub tokens: HashMap<String, u32>,
}

impl GPT4SpecialTokens {
    pub fn new() -> Self {
        let mut tokens = HashMap::new();
        tokens.insert(String::from("<|endoftext|>"), 100257);
        tokens.insert(String::from("<|fim_prefix|>"), 100258);
        tokens.insert(String::from("<|fim_middle|>"), 100259);
        tokens.insert(String::from("<|fim_suffix|>"), 100260);
        tokens.insert(String::from("<|endofprompt|>"), 100276);

        GPT4SpecialTokens { tokens }
    }
}

pub struct GPT4Tokenizer {
    pub path: Path,
    pub vocab: Vocab,
    pub regex_pattern: String,
    pub special_tokens: GPT4SpecialTokens,
}

impl TokenizerRegex for GPT4Tokenizer {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn regex_compile() {
        let paths = Path {
            data: PathBuf::from("./tests/foo.txt"),
            merges: PathBuf::from("./tests/merges.txt"),
        };

        let vocab = Vocab { size: 280 };
        let gpt4_special_tokens = GPT4SpecialTokens::new();

        let tokenizer = GPT4Tokenizer {
            path: paths,
            vocab: vocab,
            regex_pattern: String::from(
                r"'(?i:[sdmt]|ll|ve|re)|[^\r\n\p{L}\p{N}]?+\p{L}+|\p{N}{1,3}| ?[^\s\p{L}\p{N}]++[\r\n]*|\s*[\r\n]|\s+(?!\S)|\s+",
            ),
            special_tokens: gpt4_special_tokens,
        };

        let result = tokenizer.process_data_chunk(
            &tokenizer.regex_pattern,
            String::from("Sherlock Holmes ran 22 miles"),
        );
        assert_eq!(result, ["Sherlock", " Holmes", " ran", " ", "22", " miles"]);
    }
}
