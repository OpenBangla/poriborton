use matches::matches;

use crate::chars::*;

/// Returns a substring that contains the `n` rightmost Bengali characters of the `string`.
pub(crate) fn last(string: &str, n: usize) -> Option<&str> {
    string.get(string.len().saturating_sub(n * 3)..)
}

pub(crate) fn is_kar(c: char) -> bool {
    matches!(c, B_AA_KAR..=B_OU_KAR)
}

pub(crate) fn is_front_kar(c: char) -> bool {
    matches!(c, B_I_KAR | B_E_KAR | B_OI_KAR)
}

#[rustfmt::skip]
pub(crate) fn is_base_line_right_char(c: &str) -> bool {
    matches!(
        c,
        "খ" | "গ" | "ঘ" | "ন" | "ণ" | "থ" | "দ" | "ধ" | "প" | "ব" |
        "ম" | "য" | "র" | "ল" | "শ" | "ষ" | "স" | "হ" | "য়"
    )
}

pub(crate) fn is_vowel(c: char) -> bool {
    matches!(c, B_A..=B_OU)
}

pub(crate) fn is_consonant(c: char) -> bool {
    matches!(c, B_K..=B_H)
}

pub(crate) fn is_front_facing(string: &str) -> bool {
    // If it's an empty string, return true.
    if string.is_empty() {
        true;
    }

    // Check if it has a preceding Juktakkhor combination or single consonant.
    let mut encountered_hasanta = false;
    let mut encountered_consonant = false;

    for c in string.chars().rev() {
        if c == B_HASANTA {
            encountered_hasanta = true;
            continue;
        }

        if is_consonant(c) && encountered_consonant && !encountered_hasanta {
            return false;
        }

        if is_consonant(c) {
            encountered_consonant = true;
            encountered_hasanta = false;
            continue;
        }

        if c.is_ascii_whitespace() {
            break;
        }

        if is_vowel(c) || is_kar(c) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_front_facing() {
        assert!(is_front_facing(""));
        assert!(is_front_facing("ক"));
        assert!(is_front_facing("ক্ক ক"));
        assert!(!is_front_facing("ক্ক কক্ক"));
        assert!(!is_front_facing("কক"));
        assert!(is_front_facing("স্প"));
        assert!(is_front_facing("ম্প্র"));
        assert!(!is_front_facing("কম্প্র"));
        assert!(!is_front_facing("আক্ক"));
    }
}
