#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    // Variant for resizing with width and height
    Resize { width: i32, height: i32 },

    // Variant for moving to a point, assuming Point is a struct or tuple
    Move(Point),

    // Variant for echoing a string message
    Echo(String),

    // Variant for changing color with RGB values
    ChangeColor(i32, i32, i32),

    // Variant for quitting, no associated data
    Quit,
}
impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
