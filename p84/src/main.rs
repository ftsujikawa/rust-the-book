fn main() {
    let s = String::from("hello world");

    let x = first_word(&s);
    println!("x is {}", x);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}