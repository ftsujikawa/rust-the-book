fn main() {
    let hello = "こんにちは";

    let s = &hello[0..3];

    println!("s is {}", s);

    for c in hello.chars() {
        println!("{}", c);
    }
}
