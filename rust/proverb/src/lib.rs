const LAST_SENTENCE: &str = "And all for the want of a";

pub fn build_longer_proverb(list: &[&str]) -> String {
    let mut proverb:Vec<String> = vec![];
    let mut i = 0;
    while i <= list.len()-2 {
        proverb.push(format!("For want of a {} the {} was lost.", list[i], list[i+1]));
        i += 1;
    }
    proverb.push(format!("{LAST_SENTENCE} {}.", list[0]));
    proverb.join("\n")
}

pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => String::new(),
        1 => format!("{LAST_SENTENCE} {}.", list[0]),
        _ => build_longer_proverb(list),
    }
}
