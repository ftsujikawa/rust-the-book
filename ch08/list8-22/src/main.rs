use std::collections::HashMap;
fn main() {
    let field_name = String::from("Favarite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{}, {}", field_name, field_value);
}
