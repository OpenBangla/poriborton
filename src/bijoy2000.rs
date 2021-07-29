use std::collections::HashMap;

use maplit::hashmap;
use matches::matches;

use crate::chars::*;
use crate::utility::{is_base_line_right_char, is_front_facing, is_front_kar, is_kar, last};

pub struct Bijoy2000 {
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
            "ৎ" => "r",
            "ঁ" => "u",
            "ং" => "s",
            "ঃ" => "t",
            "৳" => "$",
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
            // Juktaokkhors
            "ন্ত্র্য" => "š—¨©",
            "ক্ষ্ম্য" => "²¨",
            "স্প্‌ল" => "¯c&j",
            "ঙ্‌ক্ত" => "O&³",
            "স্প্র" => "¯cÖ",
            "স্থ্য" => "¯’¨",
            "স্ত্র" => "¯¿",
            "স্ত্য" => "¯—¨",
            "স্ত্ব" => "¯—¡",
            "স্ট্র" => "÷ª",
            "স্ক্র" => "¯Œ",
            "ষ্প্র" => "®cÖ",
            "ষ্ঠ্য" => "ô¨",
            "ষ্ট্র" => "óª",
            "ষ্ট্য" => "ó¨",
            "ষ্ক্র" => "®Œ",
            "ষ্ক্ব" => "®‹¡",
            "ল্ক্য" => "é¨",
            "র্হ্য" => "n¨©",
            "র্ষ্য" => "l¨©",
            "র্শ্য" => "k¨©",
            "র্শ্ব" => "k¦©",
            "শ্ল" => "k−",
            "র্ম্য" => "g¨©",
            "র্ব্য" => "e¨©",
            "র্ধ্ব" => "aŸ©",
            "র্দ্র" => "`ª©",
            "র্দ্ব" => "Ø©",
            "র্থ্য" => "_¨©",
            "র্ত্র" => "Î©",
            "র্ত্য" => "Z¨©",
            "র্ণ্য" => "Y¨©",
            "র্ঢ্য" => "X¨©",
            "র্জ্য" => "R¨©",
            "র্চ্য" => "P¨©",
            "র্ঘ্য" => "N¨©",
            "র্গ্র" => "MÖ©",
            "র্গ্য" => "M¨©",
            "র্ক্য" => "K¨©",
            "ম্ভ্র" => "¤£",
            "ম্ব্র" => "¤^ª",
            "ম্প্র" => "¤cÖ",
            "প্র্য" => "c¨©",
            "ন্ধ্র" => "Üª",
            "ন্ধ্য" => "Ü¨",
            "ন্দ্র" => "›`ª",
            "ন্দ্য" => "›`¨",
            "ন্দ্ব" => "›Ø",
            "ন্থ্র" => "š’ª",
            "ন্ত্র" => "š¿",
            "ন্ত্য" => "š—¨",
            "ন্ত্ব" => "š—¡",
            "ন্ড্র" => "Ûª",
            "ন্ট্র" => "›Uª",
            "দ্র্য" => "`¨©",
            "দ্ভ্র" => "™£",
            "দ্দ্ব" => "Ï¡",
            "ত্র্য" => "Z¨©",
            "ত্ম্য" => "Í¨",
            "ত্ত্র" => "Ë«",
            "ত্ত্য" => "Ë¨",
            "ত্ত্ব" => "Ë¡",
            "ণ্ড্র" => "Êª",
            "ণ্ড্য" => "Ê¨",
            "ণ্ঠ্য" => "É¨",
            "জ্জ্ব" => "¾¡",
            "চ্ছ্র" => "”Qª",
            "চ্ছ্ব" => "”Q¡",
            "ঙ্ঘ্র" => "•Nª",
            "ঙ্ঘ্য" => "•N¨",
            "ঙ্গ্য" => "½¨",
            "ঙ্ক্ষ" => "•¶",
            "ঙ্ক্য" => "¼¨",
            "গ্র্য" => "M¨©",
            "গ্ন্য" => "Mœ¨",
            "গ্ধ্র" => "»ª",
            "গ্ধ্য" => "»¨",
            "ক্ষ্য" => "¶¨",
            "ক্ষ্ম" => "²",
            "ক্ষ্ব" => "¶¡",
            "ক্ষ্ণ" => "¶è",
            "ক্ত্র" => "³«",
            "ক্ট্র" => "±ª",
            "ল্‌ভ" => "j&f",
            "ল্‌ফ" => "j&d",
            "গ্‌ণ" => "M&Y",
            "হ্র" => "nª",
            "হ্য" => "n¨",
            "হ্ম" => "þ",
            "হ্ব" => "nŸ",
            "হ্ন" => "ý",
            "হ্ণ" => "nœ",
            "হ্ল" => "n¬",
            "স্ল" => "¯¬",
            "স্র" => "mª",
            "স্য" => "m¨",
            "স্ম" => "¯§",
            "স্ব" => "¯^",
            "স্ফ" => "ù",
            "স্প" => "¯c",
            "স্প্ল" => "¯c−",
            "স্ন" => "ø",
            "স্থ" => "¯’",
            "স্ত" => "¯—",
            "স্ট" => "÷",
            "স্খ" => "ö",
            "স্ক" => "¯‹",
            "ষ্য" => "l¨",
            "ষ্ম" => "®§",
            "ষ্ব" => "®^",
            "ষ্ফ" => "õ",
            "ষ্প" => "®c",
            "ষ্ণ" => "ò",
            "ষ্ণ্ব" => "ò¡",
            "ষ্ঠ" => "ô",
            "ষ্ট" => "ó",
            "ষ্ক" => "®‹",
            "শ্র" => "kª",
            "শ্য" => "k¨",
            "শ্ম" => "k¥",
            "শ্ব" => "k¦",
            "শ্ন" => "kœ",
            "শ্ছ" => "ñ",
            "শ্চ" => "ð",
            "ল্য" => "j¨",
            "ল্ল" => "j−",
            "ল্ম" => "j¥",
            "ল্ব" => "j¡",
            "ল্প" => "í",
            "ল্ড" => "ì",
            "ল্ট" => "ë",
            "ল্গ" => "ê",
            "ল্ক" => "é",
            "ল্ভ" => "j¢",
            "ল্ফ" => "î",
            "র্হ" => "n©",
            "র্স" => "m©",
            "র্ষ" => "l©",
            "র্শ" => "k©",
            "র্ল" => "j©",
            "র্য" => "h©",
            "র্ম" => "g©",
            "র্ভ" => "f©",
            "র্ফ" => "d©",
            "র্প" => "c©",
            "র্ন" => "b©",
            "র্ধ" => "a©",
            "র্দ" => "`©",
            "র্থ" => "_©",
            "র্ত" => "Z©",
            "র্ণ" => "Y©",
            "র্ড" => "W©",
            "র্ট" => "U©",
            "র্ঝ" => "S©",
            "র্জ" => "R©",
            "র্ছ" => "Q©",
            "র্চ" => "P©",
            "র্ঘ" => "N©",
            "র্গ" => "M©",
            "র্খ" => "L©",
            "র্ক" => "K©",
            "য্য" => "h¨",
            "ম্ল" => "¤¬",
            "ম্র" => "gª",
            "ম্য" => "g¨",
            "ম্ম" => "¤§",
            "ম্ভ" => "¤¢",
            "ম্ব" => "¤^",
            "ম্ফ" => "ç",
            "ম্প" => "¤c",
            "ম্ন" => "æ",
            "ভ্র" => "å",
            "ভ্য" => "f¨",
            "ভ্ব" => "f¡",
            "ভ্ল" => "f¬",
            "ব্র" => "eª",
            "ব্য" => "e¨",
            "ব্ব" => "eŸ",
            "ব্ধ" => "ä",
            "ব্দ" => "ã",
            "ব্জ" => "â",
            "ব্ল" => "e­",
            "ফ্ল" => "d¬",
            "ফ্র" => "d«",
            "প্স" => "á",
            "প্র" => "cÖ",
            "প্য" => "c¨",
            "প্প" => "à",
            "প্ল" => "c­",
            "প্ন" => "cœ",
            "প্ত" => "ß",
            "প্ট" => "Þ",
            "ন্য" => "b¨",
            "ন্ম" => "b¥",
            "ন্ব" => "š^",
            "ন্ন" => "bœ",
            "ন্হ" => "›n",
            "ন্ধ" => "Ü",
            "ন্দ" => "›`",
            "ন্থ" => "š’",
            "ন্ত" => "š—",
            "ন্ড" => "Û",
            "ন্ঠ" => "Ú",
            "ন্ট" => "›U",
            "ধ্র" => "aª",
            "ধ্য" => "a¨",
            "ধ্ম" => "a¥",
            "ধ্ব" => "aŸ",
            "ধ্ন" => "aœ",
            "দ্র" => "`ª",
            "দ্য" => "`¨",
            "দ্ম" => "Ù",
            "দ্ভ" => "™¢",
            "দ্ব" => "Ø",
            "দ্ধ" => "×",
            "দ্দ" => "Ï",
            "দ্ঘ" => "™N",
            "দ্গ" => "˜M",
            "থ্র" => "_ª",
            "থ্য" => "_¨",
            "থ্ব" => "_¡",
            "ত্র" => "Î",
            "ত্য" => "Z¨",
            "ত্ম" => "Í",
            "ত্ব" => "Z¡",
            "ত্ন" => "Zœ",
            "ত্থ" => "Ì",
            "ত্ত" => "Ë",
            "ণ্য" => "Y¨",
            "ণ্ম" => "Y¥",
            "ণ্ব" => "Y¡",
            "ণ্ণ" => "Yè",
            "ণ্ঢ" => "YX",
            "ণ্ড" => "Ê",
            "ণ্ঠ" => "É",
            "ণ্ট" => "È",
            "ঢ্র" => "Xª",
            "ঢ্য" => "X¨",
            "ড্র" => "Wª",
            "ড্য" => "W¨",
            "ড্ব" => "W¡",
            "ড্ড" => "Ç",
            "ড্ম" => "W¥",
            "ড্ব" => "W¡",
            "ন্ড্ব" => "Û¡",
            "ট্র" => "Uª",
            "ট্য" => "U¨",
            "ট্ম" => "U¥",
            "ট্ব" => "U¡",
            "ট্ট" => "Æ",
            "ঞ্ঝ" => "Å",
            "ঞ্জ" => "Ä",
            "ঞ্ছ" => "Ã",
            "ঞ্চ" => "Â",
            "জ্র" => "Rª",
            "জ্য" => "R¨",
            "জ্ব" => "R¡",
            "জ্ঞ" => "Á",
            "জ্ঝ" => "À",
            "জ্জ" => "¾",
            "চ্য" => "P¨",
            "চ্ব" => "”¡",
            "চ্ঞ" => "”T",
            "চ্ছ" => "”Q",
            "চ্চ" => "”P",
            "ঙ্ম" => "•g",
            "ঙ্ঘ" => "•N",
            "ঙ্গ" => "½",
            "ঙ্খ" => "•L",
            "ঙ্ক" => "¼",
            "ঘ্র" => "Nª",
            "ঘ্য" => "N¨",
            "ঘ্ন" => "Nœ",
            "গ্র" => "MÖ",
            "গ্য" => "M¨",
            "গ্ম" => "M¥",
            "গ্ব" => "M¦",
            "গ্ন" => "Mœ",
            "গ্ল" => "M−",
            "গ্ধ" => "»",
            "খ্র" => "Lª",
            "খ্য" => "L¨",
            "ক্স" => "·",
            "ক্ষ" => "¶",
            "ক্ল" => "K¬",
            "ক্ন" => "Kè",
            "ক্র" => "µ",
            "ক্য" => "K¨",
            "ক্ম" => "´",
            "ক্ব" => "K¡",
            "ক্ত" => "³",
            "ক্ট" => "±",
            "ক্ক" => "°",
            "ড়্গ" => "ÿ",
            "র‍্য" => "i¨",
        ];

        Self { map }
    }

    pub fn convert(&self, input: &str) -> String {
        let mut output = String::new();
        let mut buffer = String::new();
        let mut encountered_hasanta = false;

        for (pos, c) in input.char_indices() {
            match c {
                B_O_KAR => {
                    output.push(replace_kar('ে', is_front_facing(&input[..pos]), ""));
                    self.convert_buffer(&mut buffer, &mut output);
                    output.push('v');
                }
                B_OU_KAR => {
                    output.push(replace_kar('ে', is_front_facing(&input[..pos]), ""));
                    self.convert_buffer(&mut buffer, &mut output);
                    output.push('Š');
                }
                c if is_front_kar(c) => {
                    output.push(replace_kar(c, is_front_facing(&input[..pos]), ""));
                    self.convert_buffer(&mut buffer, &mut output);
                }
                B_U_KAR if buffer == "গ" => {
                    output.push('¸');
                    buffer.clear();
                }
                B_U_KAR if buffer == "শ" => {
                    output.push('ï');
                    buffer.clear();
                }
                B_U_KAR if buffer == "হ" => {
                    output.push('û');
                    buffer.clear();
                }
                B_U_KAR if buffer.ends_with("্ত") => {
                    self.convert_buffer(&mut buffer, &mut output);
                    output.pop(); // Pop '—'
                    output.push('‘');
                }
                B_RRI_KAR if buffer == "হ" => {
                    output.push('ü');
                    buffer.clear();
                }
                c if is_kar(c) => {
                    let kar = replace_kar(c, false, &buffer);
                    self.convert_buffer(&mut buffer, &mut output);
                    output.push(kar);
                }
                B_HASANTA => {
                    encountered_hasanta = true;
                    buffer.push(B_HASANTA);
                }
                B_DARI => {
                    self.convert_buffer(&mut buffer, &mut output);
                    output.push('|');
                }
                B_DDARI => {
                    self.convert_buffer(&mut buffer, &mut output);
                    output.push('\\');
                }
                ZWJ => buffer.push(ZWJ),
                ZWNJ => {
                    if buffer.ends_with(B_HASANTA) {
                        buffer.pop();
                        self.convert_buffer(&mut buffer, &mut output);
                        output.push('&'); // Hasanta
                        encountered_hasanta = false; // Reset
                    }
                }
                c if encountered_hasanta => {
                    buffer.push(c);
                    encountered_hasanta = false;
                }
                c @ '\u{0980}'..='\u{09FF}' => {
                    self.convert_buffer(&mut buffer, &mut output);
                    buffer.push(c);
                }
                c => {
                    self.convert_buffer(&mut buffer, &mut output);
                    output.push(c);
                }
            }
        }

        if !buffer.is_empty() {
            self.convert_buffer(&mut buffer, &mut output);
        }

        output
    }

    /// Converts the `buffer` into Bijoy encoding and updates the `output`.
    /// Clears the `buffer` after the conversion.
    fn convert_buffer(&self, buffer: &mut String, output: &mut String) {
        let mut reph = false;
        let mut z_fola = false;
        // Reph
        if buffer.starts_with("র্") {
            buffer.drain(..6);
            reph = true;
        }
        // R + ZWJ + Hasanta
        if buffer.starts_with("র\u{200D}") {
            // Remove the ZWJ so the buffer will have just R only as we'll be removing Z fola next.
            buffer.drain(3..6);
        }
        // Z fola
        if buffer.ends_with("্য") {
            buffer.truncate(buffer.len() - 6);
            z_fola = true;
        }

        if let Some(replace) = self.map.get(buffer.as_str()) {
            output.push_str(replace);
        }
        buffer.clear();

        if z_fola {
            output.push('¨');
        }

        if reph {
            output.push('©');
        }
    }
}

fn replace_kar(kar: char, front: bool, preceding: &str) -> char {
    match (kar, front) {
        ('া', _) => 'v',
        ('ী', _) => 'x',
        // U Kar
        ('ু', _) if preceding == "র" => '“', // রু
        ('ু', _) => match last(preceding, 1) {
            Some("র") => {
                if is_special_combination_with_r_fola(last(preceding, 3))
                    || is_special_combination_with_r_fola(last(preceding, 5))
                {
                    '“'
                } else {
                    'y'
                }
            }
            Some("ল") => {
                if is_special_combination_with_l(last(preceding, 3))
                    || matches!(last(preceding, 5), Some("স্প্ল"))
                {
                    '“'
                } else {
                    'y'
                }
            }
            Some("ণ") => {
                if matches!(last(preceding, 3), Some("ষ্ণ")) {
                    'z'
                } else {
                    'y'
                }
            }
            Some("ষ") => {
                if matches!(last(preceding, 3), Some("ক্ষ")) {
                    'z'
                } else {
                    'y'
                }
            }
            Some(c) if is_base_line_right_char(c) => 'y',
            Some("ড়") | Some("ঢ়") => '–',
            _ => 'z',
        },
        // UU Kar
        ('ূ', _) if preceding == "র" => 'ƒ', // রূ
        ('ূ', _) => match last(preceding, 1) {
            Some("র") => {
                if is_special_combination_with_r_fola(last(preceding, 3))
                    || is_special_combination_with_r_fola(last(preceding, 5))
                {
                    'ƒ'
                } else {
                    '~'
                }
            }
            Some("ল") => {
                if is_special_combination_with_l(last(preceding, 3))
                    || matches!(last(preceding, 5), Some("স্প্ল"))
                {
                    'ƒ'
                } else {
                    '~'
                }
            }
            Some("ষ") => {
                if matches!(last(preceding, 3), Some("ক্ষ")) {
                    '‚'
                } else {
                    '~'
                }
            }
            Some(c) if is_base_line_right_char(c) => '~',
            _ => '‚',
        },
        // RRI Kar
        ('ৃ', _) => match last(preceding, 1) {
            Some(c) if is_base_line_right_char(c) => '„',
            _ => '…',
        },
        ('ি', _) => 'w',
        ('ে', true) => '†', // Front facing
        ('ে', _) => '‡',
        ('ৈ', true) => 'ˆ', // Front facing
        ('ৈ', _) => '‰',
        _ => panic!("Unknown Kar replacement combination!"),
    }
}

#[rustfmt::skip]
fn is_special_combination_with_r_fola(string: Option<&str>) -> bool {
    matches!(
        string,
        // Two character combination (excluding Hasanta).
        Some("শ্র") | Some("দ্র") | Some("গ্র") | Some("ত্র") | Some("জ্র") | Some("থ্র") |
        Some("ধ্র") | Some("প্র") | Some("ব্র") | Some("ভ্র") | Some("ম্র") | Some("স্র") |
        // Three character combination (excluding Hasanta).
        Some("ন্দ্র") | Some("ম্প্র") | Some("ষ্প্র") | Some("স্প্র")
    )
}

fn is_special_combination_with_l(string: Option<&str>) -> bool {
    matches!(
        string,
        Some("গ্ল") | Some("প্ল") | Some("ব্ল") | Some("শ্ল") | Some("স্ল")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_without_juktakkhor() {
        let converter = Bijoy2000::new();

        assert_eq!(converter.convert("কতল"), "KZj");
        assert_eq!(converter.convert("সুতরাং"), "myZivs");
        assert_eq!(converter.convert("চাঁদ"), "Pvu`");
        assert_eq!(converter.convert("দুঃখ"), "`ytL");
        assert_eq!(
            converter.convert("ংঃঅআইঈউঊঋএঐওঔকখগঘঙচছজঝঞটঠডঢণতথদধনপফবভমযরলশষসহঢ়ড়য়য"),
            "stAAvBCDEFGHIJKLMNOPQRSTUVWXYZ_`abcdefghijklmnpoqh"
        );
        assert_eq!(converter.convert("০১২৩৪৫৬৭৮৯৳।\u{0965}"), "0123456789$|\\");
        assert_eq!(
            converter.convert("বাংলা আমার ভাষা। আমি বাংলায় দেখি স্বপ্ন!"),
            "evsjv Avgvi fvlv| Avwg evsjvq †`wL ¯^cœ!"
        );
    }

    #[test]
    fn test_with_kars() {
        let converter = Bijoy2000::new();

        assert_eq!(converter.convert("কুহেলিকা"), "Kz‡nwjKv");
        assert_eq!(converter.convert("কূকেকৈকী"), "K‚‡K‰KKx");
        assert_eq!(converter.convert("কি"), "wK");
        assert_eq!(converter.convert("কীট"), "KxU");
        assert_eq!(converter.convert("কে"), "†K");
        assert_eq!(converter.convert("স্পেক"), "†¯cK");
        assert_eq!(converter.convert("কৈ"), "ˆK");
        assert_eq!(converter.convert("কো"), "†Kv");
        assert_eq!(converter.convert("কৌ"), "†KŠ");
        assert_eq!(converter.convert("ককোন"), "K‡Kvb");
        assert_eq!(converter.convert("ককৌন"), "K‡KŠb");
        // U Kar
        assert_eq!(converter.convert("রু"), "i“");
        assert_eq!(converter.convert("নু"), "by");
        assert_eq!(converter.convert("ণু"), "Yy");
        assert_eq!(converter.convert("ড়ু"), "o–");
        assert_eq!(converter.convert("কু"), "Kz");
        assert_eq!(converter.convert("গু"), "¸");
        assert_eq!(converter.convert("শু"), "ï");
        assert_eq!(converter.convert("হু"), "û");
        assert_eq!(converter.convert("ষু"), "ly");
        assert_eq!(converter.convert("ক্ষু"), "¶z");
        // UU Kar
        assert_eq!(converter.convert("রূ"), "iƒ");
        assert_eq!(converter.convert("ণূ"), "Y~");
        assert_eq!(converter.convert("কূ"), "K‚");
        assert_eq!(converter.convert("ষূ"), "l~");
        assert_eq!(converter.convert("ক্ষূ"), "¶‚");
        // RRI Kar
        assert_eq!(converter.convert("হৃ"), "ü");
        assert_eq!(converter.convert("রৃ"), "i„");
        assert_eq!(converter.convert("কৃ"), "K…");
        assert_eq!(converter.convert("নৃ"), "b„");
    }

    #[test]
    fn test_with_juktakkhors() {
        let converter = Bijoy2000::new();

        assert_eq!(converter.convert("অল্প"), "Aí");
        assert_eq!(converter.convert("বক্স"), "e·");
        assert_eq!(converter.convert("পক্ব"), "cK¡");
        assert_eq!(converter.convert("সংখ্যা"), "msL¨v");
        assert_eq!(converter.convert("উৎস"), "Drm");
        assert_eq!(converter.convert("বিদ্যুৎ"), "we`¨yr");
        assert_eq!(converter.convert("কিন্তু"), "wKš‘");
        assert_eq!(converter.convert("আগন্তুক"), "AvMš‘K");
        assert_eq!(converter.convert("র‍্যাব"), "i¨ve");
    }

    #[test]
    fn test_reph() {
        let converter = Bijoy2000::new();

        assert_eq!(converter.convert("অর্ক"), "AK©");
    }
}
