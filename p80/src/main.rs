fn main() {
    let s = String::from("hello world");

    let x = first_word(&s);
    println!("x is {}", x);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}