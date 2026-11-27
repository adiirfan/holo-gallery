//! Searching by name: what was typed, split into words, each of which must
//! appear in the file's name or its folder's name, ignoring capitals.

/// The words of a search, lower-cased; empty when nothing useful was typed.
pub fn words(typed: &str) -> Vec<String> {
    typed.split_whitespace().map(str::to_lowercase).collect()
}

/// Whether a file named `name` in folder `folder` matches every word.
pub fn matches(name: &str, folder: &str, words: &[String]) -> bool {
    if words.is_empty() {
        return false;
    }
    let (name, folder) = (name.to_lowercase(), folder.to_lowercase());
    words.iter().all(|w| name.contains(w.as_str()) || folder.contains(w.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_are_split_and_lower_cased() {
        assert_eq!(words("  Beach  IMG_2024 "), vec!["beach", "img_2024"]);
        assert!(words("   ").is_empty());
    }

    #[test]
    fn part_of_a_name_matches_whatever_the_capitals() {
        assert!(matches("IMG_20261126_Beach.jpg", "Camera", &words("beach")));
        assert!(matches("Screenshot_20261126.png", "Screenshots", &words("SCREENSHOT 1126")));
        assert!(!matches("IMG_1.jpg", "Camera", &words("beach")));
    }

    #[test]
    fn every_word_must_match_but_may_be_in_the_folder() {
        assert!(matches("IMG-20261126-WA0003.jpg", "WhatsApp Images", &words("whatsapp 1126")));
        assert!(!matches("IMG-20261126-WA0003.jpg", "WhatsApp Images", &words("whatsapp 1127")));
    }

    #[test]
    fn nothing_typed_matches_nothing() {
        assert!(!matches("a.jpg", "Camera", &[]));
    }

    #[test]
    fn letters_outside_english_match_too() {
        assert!(matches("ÉTÉ_Paris.jpg", "Voyage", &words("été")));
        assert!(matches("東京タワー.jpg", "旅行", &words("東京")));
    }
}
