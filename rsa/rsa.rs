#![allow(dead_code)]
#![allow(unused_variables)]

use crate::helper;

#[repr(C)]
pub struct RsaKeys {
    pub e: u32,
    pub d: u32,
    pub n: u32,
}

pub fn keygen() -> RsaKeys {
    let mut rng = helper::MyRng::new();
    let p: u32 = helper::gen_prime();
    let mut q: u32 = helper::gen_prime();
    while p == q {
        q = helper::gen_prime();
    }

    let n = (p as u64 * q as u64) as u32;
    let phi = ((p as u64 - 1) * (q as u64 - 1)) as u32;

    let mut e: u32 = rng.next_u32() | 1;
    while !helper::is_coprime(phi, e) {
        e = rng.next_range(3, phi - 1) | 1;
    }

    let d: u32 = helper::modinv(e, phi);

    RsaKeys { e, d, n }
}

fn encrypt_chunk(message: u16, keys: &RsaKeys) -> u32 {
    helper::modpow(message as u32, keys.e, keys.n)
}

fn decrypt_chunk(encrypted: u32, keys: &RsaKeys) -> u16 {
    helper::modpow(encrypted, keys.d, keys.n) as u16
}

fn encrypt(text: &str, keys: &RsaKeys) -> Box<[u8]> {
    // Step 1: Convert to bytes
    let mut bytes = text.as_bytes().to_vec();

    // Step 2: Pad with space if length is odd
    if bytes.len() % 2 != 0 {
        bytes.push(b' ');
    }

    let mut result = Vec::with_capacity(bytes.len() * 2);

    // Encrypt all
    for chunk in bytes.chunks(2) {
        let pair: [u8; 2] = [chunk[0], chunk[1]];
        let val: u16 = helper::bytes_to_u16(&pair);
        let encrypted: u32 = encrypt_chunk(val, keys);
        let encrypted_bytes: [u8; 4] = helper::u32_to_bytes(&encrypted);
        result.extend_from_slice(&encrypted_bytes);
    }

    // Return Result
    result.into_boxed_slice()
}
