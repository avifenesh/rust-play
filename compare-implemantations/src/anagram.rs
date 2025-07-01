use std::collections::HashSet;
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_sorted = get_sorted(&word.to_lowercase());
    possible_anagrams
        .iter()
        .filter(|&anagram| {
            anagram.len() == word.len()
                && anagram.to_lowercase() != word.to_lowercase()
                && word_sorted == get_sorted(&anagram.to_lowercase())
        })
        .copied()
        .collect()
}
fn get_sorted(word: &str) -> Vec<char> {
    let mut word_sorted: Vec<char> = word.chars().collect();
    word_sorted.sort_unstable();
    word_sorted
}
