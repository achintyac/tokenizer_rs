# tokenizer

## Overview

This Rust library provides a simple BPE (byte pair encoding) solution for text tokenization, including multiple tokenizers such as GPT-4 (without special tokens) and naive implementations. The library supports various functionalities such as data processing, encoding, decoding, and merging of tokens.

*This library is a hobby project with the main goal of learning Rust better.*

## Features

- **Tokenizer Implementations**: Includes both GPT-4 and naive tokenizers.
- **Regex-Based Tokenization**: Efficient text chunking using regular expressions.
- **Utility Functions**: Helpers for token conversion and path management.
- **Extensive Testing**: Comprehensive test suite ensuring the reliability of tokenization processes.

## Modules

### `lib.rs`

The main module exposing the submodules:
- `tokenizer`
- `tokenizer_gpt4`
- `tokenizer_naive`
- `utils`

### `tokenizer.rs`

Defines the base `Tokenizer` trait with essential methods for:
- Processing data
- Reading data to bytes
- Merging token pairs
- Counting token pair occurrences
- Encoding and decoding token sequences

### `tokenizer_gpt4.rs`

Implements the `GPT4Tokenizer` struct and the `TokenizerRegex` trait, providing methods for:
- Processing data chunks using regex patterns
- Reading and converting data chunks to bytes
- Encoding and decoding with regex-based tokenization

### `utils.rs`

Provides utility functions and structures:
- `Path`: Manages paths for data and merge files.
- `Vocab`: Defines vocabulary size for merges.
- Token conversion functions: `convert_to_u32`, `convert_to_u32_nested`, and `convert_to_u8`.

## Usage

To use this library, include it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
tokenizer_rs = { path = "/path/to/tokenizer_rs" }
```

## Test/examples
You can see how to use the library at  `integration_test.rs`.


## Contributing

Contributions are welcome! Please follow the standard GitHub workflow to submit issues and pull requests.

## License

This project is licensed under the MIT License. See the LICENSE file for details.