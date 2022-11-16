pub fn is_armstrong_number(num: u32) -> bool {
    let number_digits: u32= num.to_string().len() as u32;
    num == num
        .to_string()
        .chars()
        .map(|digit| 
                digit.to_digit(10).unwrap()
                    .pow(number_digits)
            )
        .sum()
}
