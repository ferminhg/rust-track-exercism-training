use std::{vec};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 { return Err(Error::InvalidInputBase); }
    if to_base <= 1 { return Err(Error::InvalidOutputBase); }
    match calculate_to_base(number, from_base) {
        Err(why) => return Err(why),
        Ok(value) => match value {
            0 => Ok(vec![0]),
            _ => Ok(convert_to_vec(value, to_base)),
        }
    }
}

fn calculate_to_base(number: &[u32], from_base: u32) -> Result<u32, Error>{
    let mut sum:u32 = 0;
    for (i, n) in number.into_iter().enumerate() {
        if n >= &from_base { return Err(Error::InvalidDigit(n.clone())); }

        sum += n * u32::pow(from_base, (number.len()-1 - i) as u32);
    }
    Ok(sum)
}

fn convert_to_vec(mut sum: u32, to_base: u32) -> Vec<u32>{
    let mut result = vec![];
    while sum > 0 {
        result.insert(0, sum % to_base);
        sum = sum / to_base;
    }
    result
}
