#[warn(unused_variables)]
pub fn reverse(input: &str) -> String {
    let mut reverse = String::new();
    for char in input.chars() {
        reverse.insert(0, char);
    }
    reverse.to_string()
}
