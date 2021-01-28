fn main() {
    let reference_to_nothing = dangle();
}

/* fn dangle() -> &String { // コンパイルエラー
    let s = String::from("hello");

    &s
} */
fn dangle() -> String {
    let s = String::from("hello");

    s
}