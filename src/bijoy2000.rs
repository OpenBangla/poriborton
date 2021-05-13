use std::collections::HashMap;

use maplit::hashmap;

use crate::chars::B_HASANTA;

struct Bijoy2000 {
    map: HashMap<char, &'static str>,
}

impl Bijoy2000 {
    pub fn new() -> Self {
        let map = hashmap! [
        'ং' => "s",
        'ঃ' => "t",
        'অ' => "A",
        'আ' => "Av",
        'ই' => "B",
        'ঈ' => "C",
        'উ' => "D",
        'ঊ' => "E",
        'ঋ' => "F",
        'এ' => "G",
        'ঐ' => "H",
        'ও' => "I",
        'ঔ' => "J",
        'ক' => "K",
        'খ' => "L",
        'গ' => "M",
        'ঘ' => "N",
        'ঙ' => "O",
        'চ' => "P",
        'ছ' => "Q",
        'জ' => "R",
        'ঝ' => "S",
        'ঞ' => "T",
        'ট' => "U",
        'ঠ' => "V",
        'ড' => "W",
        'ঢ' => "X",
        'ণ' => "Y",
        'ত' => "Z",
        'থ' => "_",
        'দ' => "`",
        'ধ' => "a",
        'ন' => "b",
        'প' => "c",
        'ফ' => "d",
        'ব' => "e",
        'ভ' => "f",
        'ম' => "g",
        'য' => "h",
        'র' => "i",
        'ল' => "j",
        'শ' => "k",
        'ষ' => "l",
        'স' => "m",
        'হ' => "n",
        'ঢ়' => "~p",
        'ড়' => "o",
        'য়' => "q",
        //'য়' => "q",
        ];

        Self { map }
    }

    pub fn convert(&self, input: &str) -> String {
        let mut output = String::new();
        let mut temp = String::new();
        let mut encountered_hasanta = false;

        let mut iter = input.chars().peekable();

        loop {
            if let Some(c) = iter.next() {
                if let Some(next) = iter.peek() {
                    if !is_kar(c) && *next != B_HASANTA && !encountered_hasanta {
                        output.push_str(self.map.get(&c).unwrap());
                    }
                } else {
                    if !is_kar(c) && !encountered_hasanta {
                        output.push_str(self.map.get(&c).unwrap());
                    }
                }
            } else {
                if !temp.is_empty() {
                    output.push_str(&temp);
                }
                break;
            }
        }

        output
    }
}

const fn is_kar(c: char) -> bool {
    matches!(c, '\u{09BE}'..='\u{09CC}')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_without_juktakkhor() {
        let converter = Bijoy2000::new();
        
        assert_eq!(converter.convert("কতল"), "KZj");
    }
}
