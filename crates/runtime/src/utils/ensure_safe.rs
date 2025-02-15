use rustrict::{Censor, Trie, Type};
use std::sync::LazyLock;

/// Create a trie with safe word list.
fn make_safe_words_trie() -> &'static Trie {
    let words = include_str!("../../../../data/clean_words_alpha.txt");

    let mut trie = Trie::new();

    for word in words.lines() {
        trie.set(word, Type::SAFE);
    }

    Box::leak(Box::new(trie))
}

static SAFE_WORDS_TRIE: LazyLock<&'static Trie> = LazyLock::new(make_safe_words_trie);

/// Check if a word is safe to use.
///
/// This also handles non-ASCII characters and special characters.
pub fn is_safe_word(word: &str) -> bool {
    // Remove numbers and special characters from the word
    let mut new_word = String::with_capacity(word.len());

    for c in word.chars() {
        if !c.is_ascii() {
            return false; // Prevent zalgo
        }

        // Ignore numbers and special characters for analysis
        if c.is_alphabetic() {
            new_word.push(c);
        }
    }

    if new_word.is_empty() {
        return true;
    }

    Censor::from_str(&new_word)
        .with_trie(&SAFE_WORDS_TRIE)
        .analyze()
        .is(Type::SAFE)
}

#[cfg(test)]
mod trie_speed_test {
    use super::*;

    #[test]
    fn test_trie_speed() {
        make_safe_words_trie();
    }

    #[test]
    fn test_censor() {
        assert!(is_safe_word("hello"));
        assert!(is_safe_word("hello."));
        assert!(!is_safe_word("crap"));

        // Numbers are safe words
        assert!(is_safe_word("1"));
        assert!(is_safe_word("hello1"));

        // Special characters are safe words
        assert!(is_safe_word("!"));
    }
}
