enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 在这里定义方法体
        println!("Hello world!");
    }
}
fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
