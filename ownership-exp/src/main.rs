fn re(v: Vec<i32>) -> Vec<i32> {
    println!("{}", v[120] + v[111]);
    v
}

fn borrow(v: &Vec<i32>) {
    println!("{}", v[10] + v[999]);
}

fn main() {
    let mut vec = Vec::new();
    for i in 1..1001 {
        vec.push(i);
    }

    vec = re(vec);

    println!("Exist {}", vec[0]);

    borrow(&vec);
}
