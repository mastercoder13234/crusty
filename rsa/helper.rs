use crate::primes;
use std::time::{SystemTime, UNIX_EPOCH};

struct MyRng {
    state: u64,
}

impl MyRng {
    fn new() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        MyRng { state: nanos }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }

    fn next_range(&mut self, min: u64, max: u64) -> u64 {
        let span = max - min;
        min + self.next_u64() % span
    }
}

pub fn gcd(left: u64, right: u64) -> u64 {
    let mut a = left;
    let mut b = right;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Fast primality check using known primes followed by trial division
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    // Use the small primes first (converted to u64 for safe comparison)
    for &p in primes::PRIMES.iter() {
        let p64 = p as u64;
        if p64 * p64 > n {
            return true;
        }
        if n % p64 == 0 {
            return false;
        }
    }

    // Start trial division from next odd number after last prime
    let mut i = (primes::PRIMES.last().copied().unwrap_or(2) as u64) | 1;
    let limit = (n as f64).sqrt() as u64;

    while i <= limit {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

/// Generate a random prime using the improved is_prime
pub fn gen_prime() -> u64 {
    let mut rng = MyRng::new();
    let mut candidate = rng.next_u64() | 1; // Ensure candidate is odd

    while !is_prime(candidate) {
        candidate = rng.next_u64() | 1;
    }

    candidate
}
