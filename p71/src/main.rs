fn main() {
    let s = String::from("hello");

    println!("s is {}", s);

    takes_ownership(s);

    //println!("s is {}", s);   // コンパイルエラー

    let x = 5;

    println!("x is {}", x);

    makes_copy(x);

    println!("x is {}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}