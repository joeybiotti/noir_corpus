use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct BookMetadata {
    pub gutenberg_id: u32,
    pub title: String,
    pub author: String,
    pub author_birth_year: Option<i32>,
    pub author_death_year: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisPayload {
    #[serde(flatten)]
    pub metadata: BookMetadata,
    pub metrics: TextMetrics,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TextMetrics {
    pub word_count: usize,
    pub unique_word_count: usize,
    pub sentence_count: usize,
    pub avg_sentence_length: f64,
    pub lexical_density: f64,
    pub dialogue_ratio: f64,
}

pub fn analyze_text(text: &str) -> TextMetrics {
    if text.trim().is_empty() {
        return TextMetrics {
            word_count: 0,
            unique_word_count: 0,
            sentence_count: 0,
            avg_sentence_length: 0.0,
            lexical_density: 0.0,
            dialogue_ratio: 0.0,
        };
    }

    let mut total_words = 0usize;
    let mut unique_words = HashSet::new();
    let mut sentence_count = 0usize;
    let mut dialogue_chars = 0usize;
    let total_chars = text.chars().count();

    let mut in_dialogue = false;

    for ch in text.chars() {
        if ch == '"' || ch == '“' || ch == '”' {
            in_dialogue = !in_dialogue;
        } else if in_dialogue {
            dialogue_chars += 1;
        }
        if ch == '.' || ch == '!' || ch == '?' {
            sentence_count += 1;
        }
    }

    for word in text.split_whitespace() {
        let cleaned: String = word
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();

        if !cleaned.is_empty() {
            total_words += 1;
            unique_words.insert(cleaned);
        }
    }

    let sentence_count = sentence_count.max(1);
    let avg_sentence_length = (total_words as f64) / (sentence_count as f64);
    let lexical_density = if total_words > 0 {
        (unique_words.len() as f64) / (total_words as f64)
    } else {
        0.0
    };
    let dialogue_ratio = if total_chars > 0 {
        (dialogue_chars as f64) / (total_chars as f64)
    } else {
        0.0
    };

    TextMetrics {
        word_count: total_words,
        unique_word_count: unique_words.len(),
        sentence_count,
        avg_sentence_length,
        lexical_density,
        dialogue_ratio,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parse() {
        let sample = r#"
        "Where is the money?" asked Vance.
        Marlowe took a drag from his cigarette and stared at the door. "It's gone."
        "#;

        let metrics = analyze_text(sample);
        assert!(metrics.word_count > 0);
        assert!(metrics.dialogue_ratio > 0.0);
        assert_eq!(metrics.sentence_count, 2);
    }
}
