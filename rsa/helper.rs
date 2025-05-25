#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use crate::primes;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct MyRng {
    state: u64,
}

impl MyRng {
    pub fn new() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u128;

        // Mix the nanos a bit to avoid just truncating
        let seed = ((nanos >> 64) ^ nanos) as u64;

        MyRng { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        // Use a 64-bit LCG: https://en.wikipedia.org/wiki/Linear_congruential_generator
        // Constants from Numerical Recipes
        self.state = self.state.wrapping_mul(16128525).wrapping_add(1013904223);
        self.state
    }

    pub fn next_range(&mut self, min: u64, max: u64) -> u64 {
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
        let p128 = p as u64;
        if p128 * p128 > n {
            return true;
        }
        if n % p128 == 0 {
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
pub fn gen_prime(min: u64, max: u64) -> u64 {
    let mut rng = MyRng::new();
    let mut candidate = rng.next_range(min, max) | 1; // Ensure candidate is odd

    while !is_prime(candidate) {
        candidate = rng.next_range(min, max) | 1;
    }

    candidate
}

// Mod Power
pub fn modpow(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut result: u64 = 1;
    let mut base = base;
    let mut exponent = exponent;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
        exponent = (exponent / 2) as u64;
    }
    return result;
}

// Extended uclidian algorithm mod inverse
pub fn modinv(a: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }
    if !is_coprime(a, m) {
        return 0;
    }
    let m0 = m as i128;
    let mut x0: i128 = 0;
    let mut x1: i128 = 1;
    let mut a = a as i128;
    let mut m = m as i128;
    let mut q: i128 = 0;
    let mut t: i128 = 0;

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

    return x1 as u64;
}

pub fn is_coprime(a: u64, b: u64) -> bool {
    gcd(a, b) == 1
}

pub fn bytes_to_u16(input: &[u8; 2]) -> u16 {
    ((input[0] as u16) << 8) | (input[1] as u16)
}

pub fn bytes_to_u32(input: &[u8; 4]) -> u32 {
    ((input[0] as u32) << 24)
        | ((input[1] as u32) << 16)
        | ((input[2] as u32) << 8)
        | (input[3] as u32)
}

pub fn u16_to_bytes(input: &u16) -> [u8; 2] {
    let val = *input;
    [((val >> 8) & 0xFF) as u8, (val & 0xFF) as u8]
}

pub fn u32_to_bytes(input: &u32) -> [u8; 4] {
    let val = *input;
    [
        ((val >> 24) & 0xFF) as u8,
        ((val >> 16) & 0xFF) as u8,
        ((val >> 8) & 0xFF) as u8,
        (val & 0xFF) as u8,
    ]
}
