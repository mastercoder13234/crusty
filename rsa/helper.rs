use crate::primes;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct MyRng {
    state: u32,
}

impl MyRng {
    pub fn new() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        // Mix the nanos a bit to avoid just truncating
        let seed = ((nanos >> 32) ^ nanos) as u32;

        MyRng { state: seed }
    }

    pub fn next_u32(&mut self) -> u32 {
        // Use a 32-bit LCG: https://en.wikipedia.org/wiki/Linear_congruential_generator
        // Constants from Numerical Recipes
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        self.state
    }

    pub fn next_range(&mut self, min: u32, max: u32) -> u32 {
        let span = max - min;
        min + self.next_u32() % span
    }
}


pub fn gcd(left: u32, right: u32) -> u32 {
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
pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    // Use the small primes first (converted to u32 for safe comparison)
    for &p in primes::PRIMES.iter() {
        let p64 = p as u32;
        if p64 * p64 > n {
            return true;
        }
        if n % p64 == 0 {
            return false;
        }
    }

    // Start trial division from next odd number after last prime
    let mut i = (primes::PRIMES.last().copied().unwrap_or(2) as u32) | 1;
    let limit = (n as f64).sqrt() as u32;

    while i <= limit {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

/// Generate a random prime using the improved is_prime
pub fn gen_prime() -> u32 {
    let mut rng = MyRng::new();
    let mut candidate = rng.next_range(3, 65000) | 1; // Ensure candidate is odd

    while !is_prime(candidate) {
        candidate = rng.next_range(3, 65000) | 1;
    }

    candidate
}

// Mod Power
pub fn modpow(base: u32, exponent: u32, modulus: u32) -> u32 {
    let mut result: u32 = 1; 
    let mut base = base;
    let mut exponent = exponent;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = ((result as u64 * base as u64) % modulus as u64) as u32;
        }
        base = ((base as u64 * base as u64) % modulus as u64) as u32;
        exponent = (exponent / 2) as u32;
    }
    return result;

}

// Extended uclidian algorithm mod inverse
pub fn modinv(a: u32, m: u32) -> u32 {
    if m == 1 {
        return 0;
    }
    if !is_coprime(a,m){
        return 0;
    }
    let m0 = m as i64;
    let mut x0: i64 = 0;
    let mut x1: i64 = 1;
    let mut a = a as i64;
    let mut m = m as i64;
    let mut q: i64 = 0;
    let mut t: i64 = 0;

    while a > 1 {
        // Q is quitient
        q = a / m;
        t = m;

        // m is remainder now same as uclidian algorithm
        m = a % m;
        a = t;
        t = x0;

        x0 = x1 - q * x0;
        x1 = t;
    }

    if x1 < 0 {
        x1 += m0;
    }

    return x1 as u32;
}

pub fn is_coprime(a: u32, b: u32) -> bool {
    gcd(a, b) == 1
}