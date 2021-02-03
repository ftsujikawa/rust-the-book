fn main() {
    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[100];          // カーネルパニック
    println!("third is {}", third);
    let third: Option<&i32> = v.get(100);// None
    println!("third is {:?}", third);
}
