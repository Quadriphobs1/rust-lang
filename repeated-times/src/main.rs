/// Count the number of times we have repeasted item in the vec
fn main() {
    let vec = vec![
        1, 2, 3, 3, 4, 2, 3, 2, 3, 4, 5, 3, 4, 5, 2, 4, 5, 6, 3, 5, 6, 4, 2, 4, 6, 7, 4, 4, 5, 6,
        7, 5, 4, 5, 8, 89, 9, 8, 7, 6, 5, 5, 6, 7, 8, 8, 6, 5, 4, 5, 6, 7, 8, 8, 9, 8, 4, 5, 6, 4,
        3, 4, 5, 6, 4, 3, 4, 5, 6, 4, 4, 5, 5, 4, 5, 5, 4, 3, 3, 45,
    ];
    for &i in &vec {
        let r = count(&vec, i);
        println!("{} is repeased {} times", i, r);
    }
}

fn count(v: &Vec<i32>, val: i32) -> usize {
    v.into_iter().filter(|&&x| x == val).count()
}
