// enums1.rs
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    Quit, // TODO: define a few types of messages as used below
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
