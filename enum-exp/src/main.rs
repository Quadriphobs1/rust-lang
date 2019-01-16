#[derive(Debug)]
enum Direction {
    Up(Point),
    Down(Point),
    Right(Point),
    Left(Point),
}

#[derive(Debug)]
enum Keys {
    UpKey(String),
    DownKey(String),
    LeftKey(String),
    RightKey(String),
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Direction {
    fn match_durection(&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::UpKey(String::from("Pressed w")),
            Direction::Down(_) => Keys::DownKey(String::from("Pressed s")),
            Direction::Left(_) => Keys::LeftKey(String::from("Pressed a")),
            Direction::Right(_) => Keys::RightKey(String::from("Pressed d")),
        }
    }
}

impl Keys {
    fn destruct(&self) -> &String {
        match *self {
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}

fn main() {
    let u = Direction::Up(Point { x: 0, y: 1 });
    let k = u.match_durection();
    let x = k.destruct();

    println!("{:?}", x);
}
