# poriborton (পরিবর্তন)
![crates.io](https://img.shields.io/crates/v/poriborton.svg)
![docs.rs](https://docs.rs/poriborton/badge.svg)

A Rust crate for interconversion between Unicode and various Bengali ANSI encodings (precisely [Windows-1252](https://en.m.wikipedia.org/wiki/Windows-1252)).

## Supports
* Unicode to Bijoy 2000 encoding

## Example
```rust
use poriborton::bijoy2000::unicode_to_bijoy;

fn main() {
    // Converts Unicode to Bijoy2000 encoding.
    
    assert_eq!(unicode_to_bijoy("আমি বাংলায় গান গাই"), "Avwg evsjvq Mvb MvB");
}
```

## Acknowledgement
* [Avro Keyboard](https://github.com/mugli/Avro-Keyboard/) - Unicode to Bijoy2000 encoding conversion implementation.
* [bondhon](https://github.com/banglakit/bondhon) - An encoding conversion library for the Bengali (বাংলা) script.
* [বাংলা যুক্তবর্ণের তালিকা](https://bn.wikibooks.org/wiki/%E0%A6%AC%E0%A6%BE%E0%A6%82%E0%A6%B2%E0%A6%BE_%E0%A6%AF%E0%A7%81%E0%A6%95%E0%A7%8D%E0%A6%A4%E0%A6%BE%E0%A6%95%E0%A7%8D%E0%A6%B7%E0%A6%B0) - উইকিবই বাংলা
