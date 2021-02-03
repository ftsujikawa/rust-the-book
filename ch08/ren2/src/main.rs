fn main() {
    let word = "first";
    let word = "apple";
    let first = word.chars().next();
    //println!("{:?}", first);
    let result = match first {
        Some('a') => format!("{}-hay", word),
        Some('i') => format!("{}-hay", word),
        Some('u') => format!("{}-hay", word),
        Some('e') => format!("{}-hay", word),
        Some('o') => format!("{}-hay", word),
        Some(x) => format!("{}-{}ay", &word[1..], x),
        None => format!("{}", word),
    };
    println!("{}", result);
}
