/// Greatest common divisor of positive numbers
fn main() {
    let divisor: i32 = gcd(14, 120);

    println!("{}", divisor)
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    let r: i32 = a % b;
    println!("{}", r);
    return gcd(b, r);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_returns_smallest_divisor() {
        assert_eq!(gcd(120, 14), 2);
    }
    #[test]
    fn test_left_or_right() {
        assert_eq!(gcd(14, 120), 2);
    }
    #[test]
    fn returns_b_if_zeros() {
        assert_eq!(gcd(2, 0), 2);
    }
}
