const DIVISIBLE_BY: u32 = 10;

pub fn is_valid(code: &str) -> bool {
    let vec_code = code.chars().filter(|c| !c.is_whitespace()).collect::<Vec<char>>();
    
    match code_validator(vec_code) {
        Ok(value) => value % DIVISIBLE_BY == 0,
        Err(_) => false,
    }
}

fn code_validator(vec_code: Vec<char>) -> Result<u32, bool> {
    if vec_code.len() <= 1 { return Err(false);}
    let mut sum = 0;
    
    for (i, c) in vec_code.iter().rev().enumerate() {
        if !c.is_digit(10) { return Err(false); }
        let mut digit = c.to_digit(10).unwrap();
        if i % 2 == 1 {
            digit *= 2;
            if digit > 9 { digit -= 9; }
        }
        sum += digit;
    }
    Ok(sum)
}
