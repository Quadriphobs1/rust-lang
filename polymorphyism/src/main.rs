enum Shape {
    Rectangle { width: u32, height: u32 },
    Square(u32),
    Circle(f64),
}

/// Implementation block on the shape enum
impl Shape {
    // Method on each shape which uses a reference to the shape
    fn area(&self) -> f64 {
        // Dereference operator `*` on the self
        match *self {
            Shape::Rectangle { width, height } => (width * height) as f64,
            Shape::Square(ref s) => (s * s) as f64,
            Shape::Circle(ref r) => (r * r) * 3.142,
        }
    }
}

fn main() {
    let r = Shape::Rectangle {
        width: 10,
        height: 20,
    };
    let s = Shape::Square(25);
    let c = Shape::Circle(4.5);

    let ar = r.area();
    println!("Reactangle area {}", ar);

    let asr = s.area();
    println!("Square area {}", asr);

    let ac = c.area();
    println!("Circle area {}", ac)
}
