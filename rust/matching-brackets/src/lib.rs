
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut open_bracket = vec![];
    for bracket in  string.chars() {
        match bracket {
            '[' | '(' | '{' => open_bracket.push(bracket),
            ']' | ')' | '}'  => if close_bracket(bracket) != open_bracket.pop() 
                                    { return false;}
            _ => continue
        }
    }

    if open_bracket.len() != 0 { return false;}
    true
}

pub fn close_bracket(bracket: char) -> Option<char> {
    match bracket {
        ']' => Some('['),
        ')' =>  Some('('),
        '}'=>  Some('{'),
        _ => None,
    }
}