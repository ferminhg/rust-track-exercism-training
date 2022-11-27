

pub fn reply(message: &str) -> &str {
    let msg = message.trim();
    if msg.len() == 0 {
        return "Fine. Be that way!";
    }

    if  msg.chars().last().unwrap() == '?' {
        if msg.to_uppercase() == msg && 
            msg.chars().any(|x| x.is_alphabetic()){
            return "Calm down, I know what I'm doing!";
        }
        return "Sure.";
    }
    
    if msg.to_uppercase() == msg && 
        msg.chars().any(|x| x.is_alphabetic()){
        return "Whoa, chill out!";
    }

    "Whatever."
}
