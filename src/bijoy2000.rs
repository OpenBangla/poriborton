//! Unicode to Bijoy 2000 encoding conversion.
use phf::phf_map;

use crate::chars::*;
use crate::utility::{is_base_line_right_char, is_front_facing, is_front_kar, is_kar, last};

static MAP: phf::Map<&'static str, &'static str> = phf_map![
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
    // Quotation marks (in the range U+2018..=U+201D)
    "“" => "Ò",
    "”" => "Ó",
    "‘" => "Ô",
    "’" => "Õ",
    // Vowels and Consonants
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
    "ঢ়" => "p",
    "ড়" => "o",
    "য়" => "q",
    "য়" => "q",
    // Juktaokkhors
    "স্প্র" => "¯cÖ",
    "স্ত্র" => "¯¿",
    "স্ত্ব" => "¯—¡",
    "স্ট্র" => "÷ª",
    "স্ক্র" => "¯Œ",
    "ষ্প্র" => "®cÖ",
    "ষ্ট্র" => "óª",
    "ষ্ক্র" => "®Œ",
    "ষ্ক্ব" => "®‹¡",
    "শ্ল" => "k−",
    "ম্ভ্র" => "¤£",
    "ম্ব্র" => "¤^ª",
    "ম্প্র" => "¤cÖ",
    "ন্ধ্র" => "Üª",
    "ন্দ্র" => "›`ª",
    "ন্দ্ব" => "›Ø",
    "ন্থ্র" => "š’ª",
    "ন্ত্র" => "š¿",
    "ন্ত্ব" => "š—¡",
    "ন্ড্র" => "Ûª",
    "ন্ট্র" => "›Uª",
    "ন্স" => "Ý",
    "দ্ভ্র" => "™£",
    "দ্দ্ব" => "Ï¡",
    "ত্ত্র" => "Ë«",
    "ত্ত্ব" => "Ë¡",
    "ণ্ড্র" => "Êª",
    "জ্জ্ব" => "¾¡",
    "চ্ছ্র" => "”Qª",
    "চ্ছ্ব" => "”Q¡",
    "ঙ্ঘ্র" => "•Nª",
    "ঙ্ক্ষ" => "•¶",
    "গ্ধ্র" => "»ª",
    "ক্ষ্ম" => "²",
    "ক্ষ্ব" => "¶¡",
    "ক্ষ্ণ" => "¶è",
    "ক্ত্র" => "³«",
    "ক্ট্র" => "±ª",
    "হ্র" => "nª",
    "হ্ম" => "þ",
    "হ্ব" => "nŸ",
    "হ্ন" => "ý",
    "হ্ণ" => "nœ",
    "হ্ল" => "n¬",
    "স্ল" => "¯¬",
    "স্র" => "mª",
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
    "শ্ম" => "k¥",
    "শ্ব" => "k¦",
    "শ্ন" => "kœ",
    "শ্ছ" => "ñ",
    "শ্চ" => "ð",
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
    "ম্ল" => "¤¬",
    "ম্র" => "gª",
    "ম্ম" => "¤§",
    "ম্ভ" => "¤¢",
    "ম্ব" => "¤^",
    "ম্ফ" => "ç",
    "ম্প" => "¤c",
    "ম্ন" => "æ",
    "ভ্র" => "å",
    "ভ্ব" => "f¡",
    "ভ্ল" => "f¬",
    "ব্র" => "eª",
    "ব্ব" => "eŸ",
    "ব্ধ" => "ä",
    "ব্দ" => "ã",
    "ব্জ" => "â",
    "ব্ল" => "e­",
    "ফ্ল" => "d¬",
    "ফ্র" => "d«",
    "প্স" => "á",
    "প্র" => "cÖ",
    "প্প" => "à",
    "প্ল" => "c­",
    "প্ন" => "cœ",
    "প্ত" => "ß",
    "প্ট" => "Þ",
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
    "ধ্ম" => "a¥",
    "ধ্ব" => "aŸ",
    "ধ্ন" => "aœ",
    "দ্র" => "`ª",
    "দ্ম" => "Ù",
    "দ্ভ" => "™¢",
    "দ্ব" => "Ø",
    "দ্ধ" => "×",
    "দ্দ" => "Ï",
    "দ্ঘ" => "™N",
    "দ্গ" => "˜M",
    "থ্র" => "_ª",
    "থ্ব" => "_¡",
    "ত্র" => "Î",
    "ত্ম" => "Í",
    "ত্ব" => "Z¡",
    "ত্ন" => "Zœ",
    "ত্থ" => "Ì",
    "ত্ত" => "Ë",
    "ণ্ম" => "Y¥",
    "ণ্ব" => "Y¡",
    "ণ্ণ" => "Yè",
    "ণ্ঢ" => "YX",
    "ণ্ড" => "Ê",
    "ণ্ঠ" => "É",
    "ণ্ট" => "È",
    "ঢ্র" => "Xª",
    "ড্র" => "Wª",
    "ড্ব" => "W¡",
    "ড্ড" => "Ç",
    "ড্ম" => "W¥",
    "ন্ড্ব" => "Û¡",
    "ট্র" => "Uª",
    "ট্ম" => "U¥",
    "ট্ব" => "U¡",
    "ট্ট" => "Æ",
    "ঞ্ঝ" => "Å",
    "ঞ্জ" => "Ä",
    "ঞ্ছ" => "Ã",
    "ঞ্চ" => "Â",
    "জ্র" => "Rª",
    "জ্ব" => "R¡",
    "জ্ঞ" => "Á",
    "জ্ঝ" => "À",
    "জ্জ" => "¾",
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
    "ঘ্ন" => "Nœ",
    "গ্র" => "MÖ",
    "গ্ম" => "M¥",
    "গ্ব" => "M¦",
    "গ্ন" => "Mœ",
    "গ্ল" => "M−",
    "গ্ধ" => "»",
    "খ্র" => "Lª",
    "ক্স" => "·",
    "ক্ষ" => "¶",
    "ক্ল" => "K¬",
    "ক্ন" => "Kè",
    "ক্র" => "µ",
    "ক্ম" => "´",
    "ক্ব" => "K¡",
    "ক্ত" => "³",
    "ক্ট" => "±",
    "ক্ক" => "°",
    "ড়্গ" => "ÿ",
];

/// Convert Unicode `input` text to Bijoy 2000 encoding.
///
/// # Example
/// ```
/// use poriborton::bijoy2000::unicode_to_bijoy;
/// # fn main() {
///
/// assert_eq!(unicode_to_bijoy("বিজয় ২০০০"), "weRq 2000");
/// # }
pub fn unicode_to_bijoy(input: &str) -> String {
    let mut output = String::with_capacity(input.len() / 3);
    let mut buffer = String::with_capacity(5*3);
    let mut encountered_hasanta = false;

    for (pos, c) in input.char_indices() {
        match c {
            B_O_KAR => {
                output.push(replace_kar('ে', is_front_facing(&input[..pos]), ""));
                convert_buffer(&mut buffer, &mut output);
                output.push('v');
            }
            B_OU_KAR => {
                output.push(replace_kar('ে', is_front_facing(&input[..pos]), ""));
                convert_buffer(&mut buffer, &mut output);
                output.push('Š');
            }
            c if is_front_kar(c) => {
                output.push(replace_kar(c, is_front_facing(&input[..pos]), ""));
                convert_buffer(&mut buffer, &mut output);
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
                convert_buffer(&mut buffer, &mut output);
                output.pop(); // Pop '—'
                output.push('‘');
            }
            B_RRI_KAR if buffer == "হ" => {
                output.push('ü');
                buffer.clear();
            }
            c if is_kar(c) => {
                let kar = replace_kar(c, false, &buffer);
                convert_buffer(&mut buffer, &mut output);
                output.push(kar);
            }
            B_HASANTA => {
                encountered_hasanta = true;
                buffer.push(B_HASANTA);
            }
            B_DARI => {
                convert_buffer(&mut buffer, &mut output);
                output.push('|');
            }
            B_DDARI => {
                convert_buffer(&mut buffer, &mut output);
                output.push('\\');
            }
            ZWJ => buffer.push(ZWJ),
            ZWNJ => {
                if buffer.ends_with(B_HASANTA) {
                    buffer.pop();
                    convert_buffer(&mut buffer, &mut output);
                    output.push('&'); // Hasanta
                    encountered_hasanta = false; // Reset
                }
            }
            c if encountered_hasanta => {
                buffer.push(c);
                encountered_hasanta = false;
            }
            // Bengali characters and curly quotation marks
            '\u{0980}'..='\u{09FF}' | '\u{2018}' | '\u{2019}' | '\u{201C}' | '\u{201D}' => {
                convert_buffer(&mut buffer, &mut output);
                buffer.push(c);
            }
            c => {
                convert_buffer(&mut buffer, &mut output);
                output.push(c);
            }
        }
    }

    if !buffer.is_empty() {
        convert_buffer(&mut buffer, &mut output);
    }

    output
}

/// Converts the `buffer` into Bijoy encoding and updates the `output`.
/// Clears the `buffer` after the conversion.
fn convert_buffer(buffer: &mut String, output: &mut String) {
    let mut reph = false;
    let mut z_fola = false;
    let mut hasanta = false;
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
    // Non Juktokkhor making trailing Hasanta
    if buffer.ends_with('্') {
        buffer.pop();
        hasanta = true;
    }

    if let Some(replace) = MAP.get(buffer.as_str()) {
        output.push_str(replace);
    }
    buffer.clear();

    if z_fola {
        output.push('¨');
    }

    if reph {
        output.push('©');
    }

    if hasanta {
        output.push('&');
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
    fn test_various() {
        assert_eq!(unicode_to_bijoy("কতল"), "KZj");
        assert_eq!(unicode_to_bijoy("সুতরাং"), "myZivs");
        assert_eq!(unicode_to_bijoy("চাঁদ"), "Pvu`");
        assert_eq!(unicode_to_bijoy("দুঃখ"), "`ytL");
        assert_eq!(unicode_to_bijoy("অর্ক"), "AK©");
        assert_eq!(unicode_to_bijoy("কিংকর্তব্যবিমূঢ়"), "wKsKZ©e¨weg~p");
        assert_eq!(
            unicode_to_bijoy("ংঃঅআইঈউঊঋএঐওঔকখগঘঙচছজঝঞটঠডঢণতথদধনপফবভমযরলশষসহঢ়ড়য়য"),
            "stAAvBCDEFGHIJKLMNOPQRSTUVWXYZ_`abcdefghijklmnpoqh"
        );
        assert_eq!(unicode_to_bijoy("০১২৩৪৫৬৭৮৯৳।\u{0965}"), "0123456789$|\\");
        assert_eq!(
            unicode_to_bijoy("বাংলা আমার ভাষা। আমি বাংলায় দেখি স্বপ্ন!"),
            "evsjv Avgvi fvlv| Avwg evsjvq †`wL ¯^cœ!"
        );
        assert_eq!(unicode_to_bijoy("\"'/{}();:![]\\"), "\"'/{}();:![]\\");
        assert_eq!(unicode_to_bijoy("“আজো তার ফুলকলিদের ঘুম টুটেনি, তন্দ্রাতে বিলোল”"), "ÒAv‡Rv Zvi dzjKwj‡`i Nyg Uz‡Uwb, Z›`ªv‡Z we‡jvjÓ");
        assert_eq!(unicode_to_bijoy("‘আসেনি দখিন হাওয়া গজল গাওয়া মৌমাছি বিভোল’"), "ÔAv‡mwb `wLb nvIqv MRj MvIqv †gŠgvwQ we‡fvjÕ");
    }

    #[test]
    fn test_with_kars() {
        assert_eq!(unicode_to_bijoy("কুহেলিকা"), "Kz‡nwjKv");
        assert_eq!(unicode_to_bijoy("কূকেকৈকী"), "K‚‡K‰KKx");
        assert_eq!(unicode_to_bijoy("কি"), "wK");
        assert_eq!(unicode_to_bijoy("কীট"), "KxU");
        assert_eq!(unicode_to_bijoy("কে"), "†K");
        assert_eq!(unicode_to_bijoy("স্পেক"), "†¯cK");
        assert_eq!(unicode_to_bijoy("কৈ"), "ˆK");
        assert_eq!(unicode_to_bijoy("কো"), "†Kv");
        assert_eq!(unicode_to_bijoy("কৌ"), "†KŠ");
        assert_eq!(unicode_to_bijoy("ককোন"), "K‡Kvb");
        assert_eq!(unicode_to_bijoy("ককৌন"), "K‡KŠb");
        // U Kar
        assert_eq!(unicode_to_bijoy("রু"), "i“");
        assert_eq!(unicode_to_bijoy("নু"), "by");
        assert_eq!(unicode_to_bijoy("ণু"), "Yy");
        assert_eq!(unicode_to_bijoy("ড়ু"), "o–");
        assert_eq!(unicode_to_bijoy("কু"), "Kz");
        assert_eq!(unicode_to_bijoy("গু"), "¸");
        assert_eq!(unicode_to_bijoy("শু"), "ï");
        assert_eq!(unicode_to_bijoy("হু"), "û");
        assert_eq!(unicode_to_bijoy("ষু"), "ly");
        assert_eq!(unicode_to_bijoy("ক্ষু"), "¶z");
        // UU Kar
        assert_eq!(unicode_to_bijoy("রূ"), "iƒ");
        assert_eq!(unicode_to_bijoy("ণূ"), "Y~");
        assert_eq!(unicode_to_bijoy("কূ"), "K‚");
        assert_eq!(unicode_to_bijoy("ষূ"), "l~");
        assert_eq!(unicode_to_bijoy("ক্ষূ"), "¶‚");
        // RRI Kar
        assert_eq!(unicode_to_bijoy("হৃ"), "ü");
        assert_eq!(unicode_to_bijoy("রৃ"), "i„");
        assert_eq!(unicode_to_bijoy("কৃ"), "K…");
        assert_eq!(unicode_to_bijoy("নৃ"), "b„");
    }

    #[test]
    fn test_with_juktakkhors() {
        assert_eq!(unicode_to_bijoy("ন্ত্র্য"), "š¿¨");
        assert_eq!(unicode_to_bijoy("অল্প"), "Aí");
        assert_eq!(unicode_to_bijoy("বক্স"), "e·");
        assert_eq!(unicode_to_bijoy("পক্ব"), "cK¡");
        assert_eq!(unicode_to_bijoy("সংখ্যা"), "msL¨v");
        assert_eq!(unicode_to_bijoy("উৎস"), "Drm");
        assert_eq!(unicode_to_bijoy("বিদ্যুৎ"), "we`¨yr");
        assert_eq!(unicode_to_bijoy("কিন্তু"), "wKš‘");
        assert_eq!(unicode_to_bijoy("আগন্তুক"), "AvMš‘K");
        assert_eq!(unicode_to_bijoy("র‍্যাব"), "i¨ve");
        assert_eq!(unicode_to_bijoy("ফিনান্সিয়াল"), "wdbvwÝqvj");
    }

    #[test]
    fn test_zwnj() {
        assert_eq!(unicode_to_bijoy("স্প্‌ল"), "¯c&j");
        assert_eq!(unicode_to_bijoy("ঙ্‌ক্ত"), "O&³");
        assert_eq!(unicode_to_bijoy("ল্‌ভ"), "j&f");
        assert_eq!(unicode_to_bijoy("ল্‌ফ"), "j&d");
        assert_eq!(unicode_to_bijoy("গ্‌ণ"), "M&Y");
    }

    #[test]
    fn test_hasanta() {
        assert_eq!(unicode_to_bijoy("্"), "&");
        assert_eq!(unicode_to_bijoy("ক্"), "K&");
        assert_eq!(unicode_to_bijoy("কক্ষ্"), "K¶&");
    }
}
