pub fn abbreviate(phrase: &str) -> String {
    let mut acronyms = "".to_string();
    for word in phrase.split(&[' ','-', '_']) {
        for (i, c) in word.chars().enumerate() {
            if i == 0 || c.is_uppercase() {
                acronyms.push_str(&c.to_string().to_uppercase());
            } 
            if word.to_uppercase() == word { break; }
        }
    }
    acronyms
}
