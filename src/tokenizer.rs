use std::collections::HashMap;

use crate::utils::Vocab;

// Base trait for all tokenizers
pub trait Tokenizer {
    // take tokens (decimal representation of bytes) and find all existence of token tuples (pairs) replace them with idx
    fn merge(ids: Vec<u32>, pair: &(u32, u32),idx: u32) -> Vec<u32>;

    // count the number of occurrences of each token pair
    fn get_counts(ids: &[u32]) -> HashMap<(u32, u32), u32>;

    // given a dictionary of token pairs and their corresponding new token id, recursively
    // loop through tokens to get the decoded token list
    fn decode(tokens: &Vec<u32>, merges: &HashMap<u32,(u32, u32)>) -> Vec<u32>;

    // given a vector of tokens generate a mutates list of tokens with the desired encoding and the record of all merges to generate new token ids
    fn encode(tokens: Vec<u32>, vocab: Vocab) -> (Vec<u32>, HashMap<u32,(u32,u32)>);

}