use rustrict::{Censor, Trie, Type};
use std::sync::LazyLock;

use crate::{controller::SuperUserMessageTransform, types::{CreateEmbed, CreateEmbedFooter}};

/// Create a trie with safe word list.
fn make_safe_words_trie() -> &'static Trie {
    let words = include_str!("../../../data/clean_words_alpha.txt");

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
            new_word.push(c.to_ascii_lowercase());
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

/// Injects a disclaimer text into the given message to be transformed before sending, if the controller has a disclaimer configured. 
/// 
/// This can be used to ensure that all messages sent by the bot include a disclaimer
pub fn inject_disclaimer(transform: SuperUserMessageTransform, disclaimer: &str) -> Result<SuperUserMessageTransform, crate::Error> {
    let mut embeds = transform.embeds;

    if embeds.len() >= 10 {
        return Err("Messages must have less than 10 embeds".into());
    }

    // Insert disclaimer into all embeds about being user-generated content
    embeds.push(CreateEmbed {
        footer: Some(CreateEmbedFooter {
            text: disclaimer.to_string(),
            icon_url: None,
        }),
        ..Default::default()
    });


    Ok(SuperUserMessageTransform {
        embeds,
        content: transform.content,
    })
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

        assert!(is_safe_word("provides"));
    }
}
