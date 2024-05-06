use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Path {
    pub data: PathBuf,
    pub merges: PathBuf
}

pub struct Vocab {
    pub size: u32
}

pub struct BaseTokenizer {
    pub path: Path,
    pub vocab: Vocab
}

impl BaseTokenizer {

    pub fn read_to_bytes(
        path: PathBuf
    ) -> Vec<u8> {
        let file = fs::read_to_string(path)
            .expect("Should have been a file here to read");
        Vec::from(file.as_bytes())
    }
    
    // given a dictionary of token pairs and their corresponding new token id, recursively
    // loop through tokens to get the decoded token list
    pub fn decode(
        tokens: Vec<u32>, 
        merges: &HashMap<u32,(u32, u32)>
    ) -> Vec<u32> {
        let mut new_tokens: Vec<u32> = Vec::new();
        let mut temp_tokens: Vec<u32> = tokens.clone();
        let mut max_value = *tokens.iter().max().unwrap();
        while max_value > u8::MAX as u32 {
            new_tokens = Vec::new();
            for token in &temp_tokens {
                if *token == max_value {
                    let pair_encoding = merges.get(&token).unwrap(); // get matching token pair
                    new_tokens.push(pair_encoding.0);
                    new_tokens.push(pair_encoding.1);
                } else {
                    new_tokens.push(*token);
                }
            }
            temp_tokens = new_tokens.clone();
            max_value -= 1;
        }
        new_tokens
    }

    // take tokens (decimal representation of bytes) and find all 
    // existence of token tuples (pairs) replace them with idx
    pub fn merge(
        ids: Vec<u32>, 
        pair: &(u32, u32), 
        idx: u32
    ) -> Vec<u32> {
        let mut new_tokens: Vec<u32> = Vec::new();
        let mut i = 0;
        let length  = ids.len();
        while i < length {
            if i < length - 1 && ids[i] == pair.0 && ids[i+1] == pair.1 {
                new_tokens.push(idx);
                i += 2;
            }
            else {
                new_tokens.push(ids[i].into());
                i += 1;
            }
        }
        new_tokens
    }

    // count the number of occurrences of each token pair
    pub fn get_counts(
        ids: &[u32]
    ) -> HashMap<(u32, u32), u32> {
        let mut tokens: HashMap<(u32, u32), u32> = HashMap::new(); 
        for idx in 1..ids.len() {
            tokens.entry((ids[idx-1], ids[idx])).and_modify(|counter| *counter += 1).or_insert(1);
        }
        tokens
    }

    pub fn encode(
        mut tokens: Vec<u32>,
        vocab: Vocab
    ) -> (Vec<u32>, HashMap<u32,(u32,u32)>) {
        let vocab_size = vocab.size;
        let num_merges = vocab_size - ((u8::MAX as u32) + 1);
        let mut merges: HashMap<u32, (u32, u32)> = HashMap::new(); 
    
        for i in 0..num_merges {
            let bytes = BaseTokenizer::get_counts(&tokens);
            let mut bytes_vec: Vec<(&(u32, u32), &u32)> = bytes.iter().collect(); // sort bytes into pairs by the descending order of occurrence
            bytes_vec.sort_by(|a, b| b.1.cmp(a.1));
            let idx = 256 + i;
            let max_pair = bytes_vec[0].0;
            tokens = BaseTokenizer::merge(tokens, &max_pair, idx);
            merges.entry(idx).or_insert(*max_pair);
        }
        (tokens, merges)
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
}