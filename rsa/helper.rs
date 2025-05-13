pub fn gcd(left: i32, right: i32) -> i32 {
    let mut a = left;
    let mut b = right;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as i32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
