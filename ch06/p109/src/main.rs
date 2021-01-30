enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {

    }
}
fn main() {
    let msg = Message::Move { x: 10, y: 20};
    msg.call();
    let m = Message::Write(String::from("hello"));
    m.call();
}
