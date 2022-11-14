use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let lower_word: Vec<_> = word.chars()
        .map(|c| c.to_lowercase().to_string())
        .collect();
    let mut word_sorted: Vec<_>= word.chars()
        .map(|c| c.to_lowercase().to_string()).collect();
    word_sorted.sort_unstable();

    possible_anagrams
        .iter()
        .filter(|w| {
            let mut sci: Vec<_> = w.chars()
                .map(|c| c.to_lowercase().to_string())
                .collect();
            let same_word = lower_word == sci;
            sci.sort_unstable();
            sci == word_sorted && !same_word
        })
        .cloned()
        .collect()
}
