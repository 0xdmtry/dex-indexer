use crate::models::classifier::{ClassificationResult, Confidence, StringType};
use std::collections::HashMap;

const SOLANA_ADDRESS_LENGTH_MIN: usize = 32;
const SOLANA_ADDRESS_LENGTH_MAX: usize = 44;
const SYMBOL_LENGTH_MAX: usize = 10;

pub fn classify_string(input: &str) -> ClassificationResult {
    let trimmed = input.trim();
    let len = trimmed.len();
    let mut matches = HashMap::new();

    // Check address
    if (SOLANA_ADDRESS_LENGTH_MIN..=SOLANA_ADDRESS_LENGTH_MAX).contains(&len) && is_base58(trimmed)
    {
        matches.insert(StringType::Address, Confidence::High);
    }

    // Check symbol
    if is_alphanumeric(trimmed) && len <= SYMBOL_LENGTH_MAX {
        let confidence = if len <= 6 {
            Confidence::High
        } else {
            Confidence::Medium
        };
        matches.insert(StringType::Symbol, confidence);
    }

    // Check name
    if len > 0 {
        let confidence = if has_non_alphanumeric(trimmed) {
            Confidence::High
        } else if len > SYMBOL_LENGTH_MAX {
            Confidence::Medium
        } else {
            Confidence::Low
        };
        matches.insert(StringType::Name, confidence);
    }

    ClassificationResult {
        matches,
        raw: trimmed.to_string(),
    }
}

fn is_base58(s: &str) -> bool {
    s.chars()
        .all(|c| matches!(c, '1'..='9' | 'A'..='H' | 'J'..='N' | 'P'..='Z' | 'a'..='k' | 'm'..='z'))
}

fn is_alphanumeric(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_alphanumeric())
}

fn has_non_alphanumeric(s: &str) -> bool {
    s.chars().any(|c| !c.is_ascii_alphanumeric())
}
