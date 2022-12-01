/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let clean_isbn = isbn.to_string().replace(['-'], "");
    if clean_isbn.len() != 10 { return false; }
    let mut result:i32 = 0;
    for (mult, digit) in clean_isbn.chars().enumerate() {
        result += match digit {
            'X' => {
                if mult != 9 { return false;}
                multiply_digit(10, mult as i32)
            },
            '0'..='9' => {
                multiply_digit(
                    digit.to_string().parse::<i32>().unwrap(), 
                    mult as i32)
            },
            _ => return false,
        }
    }
    result % 11 == 0
}

pub fn multiply_digit(digit: i32, mult: i32) -> i32 {
    digit * (10 - mult)
}