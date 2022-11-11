#[warn(unused_variables)]
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect::<String>()
}
