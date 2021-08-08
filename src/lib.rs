//! # poriborton (পরিবর্তন)
//! A Rust crate for interconversion between Unicode and various Bengali ANSI encodings (precisely [Windows-1252](https://en.m.wikipedia.org/wiki/Windows-1252)).
//! 
//! ## Supports
//! * Unicode to Bijoy 2000 encoding
//! 
//! ## Example
//! ```
//! use poriborton::bijoy2000::Bijoy2000;
//! 
//! fn main() {
//!     // Converts Unicode to Bijoy2000 encoding.
//!     let converter = Bijoy2000::new();
//! 
//!     assert_eq!(converter.convert("আমি বাংলায় গান গাই"), "Avwg evsjvq Mvb MvB");
//! }
//! ```

pub mod bijoy2000;
pub(crate) mod chars;
mod utility;
