use crate::tokenizer::Tokenizer;
use crate::utils::{Path, Vocab};

pub struct BaseTokenizer {
    pub path: Path,
    pub vocab: Vocab
}

impl Tokenizer for BaseTokenizer {}