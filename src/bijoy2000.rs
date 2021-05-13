use std::{collections::HashMap, ops::Deref};

use maplit::hashmap;

use crate::chars::*;

struct Bijoy2000 {
    map: HashMap<&'static str, &'static str>,
}

impl Bijoy2000 {
    pub fn new() -> Self {
        let map = hashmap! [
            "০" => "0",
            "১" => "1",
            "২" => "2",
            "৩" => "3",
            "৪" => "4",
            "৫" => "5",
            "৬" => "6",
            "৭" => "7",
            "৮" => "8",
            "৯" => "9",
            "ং" => "s",
            "ঃ" => "t",
            "অ" => "A",
            "আ" => "Av",
            "ই" => "B",
            "ঈ" => "C",
            "উ" => "D",
            "ঊ" => "E",
            "ঋ" => "F",
            "এ" => "G",
            "ঐ" => "H",
            "ও" => "I",
            "ঔ" => "J",
            "ক" => "K",
            "খ" => "L",
            "গ" => "M",
            "ঘ" => "N",
            "ঙ" => "O",
            "চ" => "P",
            "ছ" => "Q",
            "জ" => "R",
            "ঝ" => "S",
            "ঞ" => "T",
            "ট" => "U",
            "ঠ" => "V",
            "ড" => "W",
            "ঢ" => "X",
            "ণ" => "Y",
            "ত" => "Z",
            "থ" => "_",
            "দ" => "`",
            "ধ" => "a",
            "ন" => "b",
            "প" => "c",
            "ফ" => "d",
            "ব" => "e",
            "ভ" => "f",
            "ম" => "g",
            "য" => "h",
            "র" => "i",
            "ল" => "j",
            "শ" => "k",
            "ষ" => "l",
            "স" => "m",
            "হ" => "n",
            "ঢ়" => "p", // was ~p
            "ড়" => "o",
            "য়" => "q",
            "য়" => "q",
        ];

        Self { map }
    }

    pub fn convert(&self, input: &str) -> String {
        let mut output = String::new();
        let mut temp = String::new();
        let mut encountered_hasanta = false;

        let mut iter = input.chars().enumerate();

        loop {
            if let Some((pos, c)) = iter.next() {
                if is_kar(c) {
                    if c == B_O_KAR {
                        output.push(replace_kar('ে', pos, ""));
                        if let Some(replace) = self.map.get(temp.deref()) {
                            output.push_str(replace);
                        }
                        temp.clear();
                        output.push('v');
                    } else if c == B_OU_KAR {
                        output.push(replace_kar('ে', pos, ""));
                        if let Some(replace) = self.map.get(temp.deref()) {
                            output.push_str(replace);
                        }
                        temp.clear();
                        output.push('Š');
                    } else if is_front_kar(c) {
                        output.push(replace_kar(c, pos, ""));
                        if let Some(replace) = self.map.get(temp.deref()) {
                            output.push_str(replace);
                        }
                        temp.clear();
                    } else {
                        let kar = replace_kar(c, pos, &temp);
                        if let Some(replace) = self.map.get(temp.deref()) {
                            output.push_str(replace);
                        }
                        temp.clear();
                        output.push(kar);
                    }
                }
                if !encountered_hasanta {
                    if let Some(replace) = self.map.get(temp.deref()) {
                        output.push_str(replace);
                    }
                    temp.clear();
                    temp.push(c);
                }

                if c == B_HASANTA {
                    encountered_hasanta = true;
                }
            } else {
                if !temp.is_empty() {
                    if let Some(replace) = self.map.get(temp.deref()) {
                        output.push_str(replace);
                    }
                }
                break;
            }
        }

        output
    }
}

const fn is_kar(c: char) -> bool {
    matches!(c, B_AA_KAR..=B_OU_KAR)
}

const fn is_front_kar(c:char) -> bool {
    matches!(c, B_I_KAR | B_E_KAR | B_OI_KAR)
}

fn replace_kar(kar: char, pos: usize, preceding: &str) -> char {
    match (kar, pos) {
        ('া', _) => 'v',
        ('ী', _) => 'x',
        ('ু', _) if preceding == "র" => '“', // রু
        ('ু', _) => 'z',
        ('ূ', _) if preceding == "র" => 'ƒ', // রূ
        ('ূ', _) => '‚',
        ('ৃ', _) => '…',
        ('ি', _) => 'w',
        ('ে', 1) => '†', // Front facing
        ('ে', _) => '‡',
        ('ৈ', 1) => 'ˆ', // Front facing
        ('ৈ', _) => '‰',
        _ => panic!("Unknown Kar replacement combination!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_without_juktakkhor() {
        let converter = Bijoy2000::new();
        
        assert_eq!(converter.convert("কতল"), "KZj");
        assert_eq!(converter.convert("ংঃঅআইঈউঊঋএঐওঔকখগঘঙচছজঝঞটঠডঢণতথদধনপফবভমযরলশষসহঢ়ড়য়য"), "stAAvBCDEFGHIJKLMNOPQRSTUVWXYZ_`abcdefghijklmnpoqh");
        assert_eq!(converter.convert("০১২৩৪৫৬৭৮৯"), "0123456789");
    }

    #[test]
    fn test_with_kars() {
        let converter = Bijoy2000::new();

        assert_eq!(converter.convert("কুহেলিকা"), "Kz‡nwjKv");
        assert_eq!(converter.convert("কূকেকৈকী"), "K‚‡K‰KKx");
        assert_eq!(converter.convert("কি"), "wK");
        assert_eq!(converter.convert("কীট"), "KxU");
        assert_eq!(converter.convert("কে"), "†K");
        assert_eq!(converter.convert("কৈ"), "ˆK");
        assert_eq!(converter.convert("কো"), "†Kv");
        assert_eq!(converter.convert("কৌ"), "†KŠ");
        assert_eq!(converter.convert("ককোন"), "K‡Kvb");
        assert_eq!(converter.convert("ককৌন"), "K‡KŠb");
        assert_eq!(converter.convert("রু"), "i“");
        assert_eq!(converter.convert("রূ"), "iƒ");
    }
}
