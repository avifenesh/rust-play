use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();
    let word_sorted = get_sorted(&lower_word);
    // asign once
    let word_length = word.len();
    possible_anagrams
        .iter()
        .filter(|anagram_candidate| {
            // first, we check the more cheap one
            if anagram_candidate.len() != word_length {
                return false;
            }
            // This statment will be avoided if we get true on the prev check
            let lower_anagram_candidate = anagram_candidate.to_lowercase();
            lower_anagram_candidate != lower_word
                && word_sorted == get_sorted(&lower_anagram_candidate)
        })
        .copied()
        .collect()
}

fn get_sorted(word: &str) -> Vec<char> {
    let mut word_sorted: Vec<char> = word.chars().collect();
    word_sorted.sort_unstable();
    word_sorted
}
