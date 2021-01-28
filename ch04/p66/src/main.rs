fn main() {
    let s1 = String::from("hello");
    println!("{:?}", s1.as_ptr());
    let s2 = s1;
    println!("{:?}", s2.as_ptr());
    //println!("{}", s1);
    println!("{}", s2);
}
