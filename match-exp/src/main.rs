fn main() {
    let n = 30;

    /// Match with a range
    match n {
        x @ 25...30 => println!("{}", x),
        _ => println!("Out of scope"),
    };

    /// Matchcing a tuple with case
    let case = (0, 12);
    match case {
        (x, y) if x == y => println!("Equals"),
        (x, y) if x < y => println!("Y is greater"),
        (x, y) if x > y => println!("X is greater"),
        _ => println!("None"),
    };

    let p = 20;

    let a = match p {
        n @ 1...12 => n,
        n @ 13...25 => n * 2,
        _ => 0,
    };

    println!("{}", a);
}
