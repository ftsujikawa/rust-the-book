struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointe with data `{}`!", self.data);
    }
}
fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main");
}