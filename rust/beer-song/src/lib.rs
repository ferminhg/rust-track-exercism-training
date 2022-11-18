pub fn verse(n: u32) -> String {
    let bottles_part = match n {
        0 => "No more bottles".to_string(),
        1 => "1 bottle".to_string(),
        _ => format!("{n} bottles"),
    };
    let action_part = match n {
        0 => "Go to the store and buy some more, 99 bottles of beer on the wall.".to_string(),
        1 => "Take it down and pass it around, no more bottles of beer on the wall.".to_string(),
        2 => format!("Take one down and pass it around, {} bottle of beer on the wall.", n-1),
        _ => format!("Take one down and pass it around, {} bottles of beer on the wall.", n-1),
    };

    format!("{bottles_part} of beer on the wall, {} of beer.\n{action_part}\n", bottles_part.to_lowercase())
}

pub fn sing(start: u32, end: u32) -> String {
    let mut number = start;
    let mut verses = vec![];

    while number >= end  {
        verses.push(verse(number));
        if number <= 0 {break;}
        number -= 1;
    }
    verses.join("\n ")
}
