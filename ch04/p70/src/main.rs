fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    println!("s1.ptr = {:?}, s2.ptr = {:?}", s1.as_ptr(), s2.as_ptr());
}
