// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_words: HashMap<&str, u32> = HashMap::new();
    for word in magazine {
        magazine_words.entry(word)
            .and_modify(|e| { *e += 1 })
            .or_insert(1);
    }

    for word in note {
        if !magazine_words.contains_key(word) || magazine_words.get(word) == Some(&0) {
            return false;
        }
        magazine_words.entry(word).and_modify(|e| { *e -= 1 });
    }
    true
}
