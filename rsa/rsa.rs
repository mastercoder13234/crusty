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
        e = rng.next_range(3, phi-1) | 1;
    }

    let d: u32 = helper::modinv(e,phi);

    RsaKeys { e, d, n }
}

pub fn encrypt_chunk(message: u16, keys: &RsaKeys) -> u32 {
    helper::modpow(message as u32, keys.e, keys.n)
}

pub fn decrypt_chunk(encrypted: u32, keys: &RsaKeys) -> u16 {
    helper::modpow(encrypted, keys.d, keys.n) as u16
}

pub fn bytes_to_u16(input: &[u8; 2]) -> u16 {
    ((input[0] as u16) << 8) | (input[1] as u16)
}

pub fn bytes_to_u32(input: &[u8; 4]) -> u32 {
    ((input[0] as u32) << 24) | ((input[1] as u32) << 16) | ((input[2] as u32) << 8) | (input[3] as u32)
}

pub fn u16_to_bytes(input: &u16) -> [u8; 2] {
    let val = *input;
    [
        ((val >> 8) & 0xFF) as u8,
        (val & 0xFF) as u8,
    ]
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

