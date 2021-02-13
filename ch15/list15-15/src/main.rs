struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointe with data `{}`!", self.data);
    }
}
fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    println!("CustomSmartPointers created.");
    c.drop();
    println!("CustomSmartPointer dropped before the end of main");
}
