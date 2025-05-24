mod helper;
mod primes;
mod rsa;

#[no_mangle]
pub extern "C" fn add(left: i32, right: i32) -> i32 {
    println!("Rust is adding {left} and {right} together.");
    left + right
}

#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("Hello World from Rust!");
}

#[no_mangle]
pub extern "C" fn test_func() {
    println!("It works!!!");
}

#[no_mangle]
pub extern "C" fn gcd(left: u32, right: u32) -> u32 {
    return helper::gcd(left, right);
}

#[no_mangle]
pub extern "C" fn is_prime(n: u32) -> bool {
    return helper::is_prime(n);
}

#[no_mangle]
pub extern "C" fn gen_prime() -> u32 {
    return helper::gen_prime();
}

#[no_mangle]
pub extern "C" fn modpow(base: u32, exponent: u32, modulus: u32) -> u32 {
    return helper::modpow(base, exponent, modulus);
}

#[no_mangle]
pub extern "C" fn modinv(a: u32, m: u32) -> u32 {
    return helper::modinv(a,m);
}

#[no_mangle]
pub extern "C" fn is_coprime(a: u32, b: u32) -> bool {
    return helper::is_coprime(a,b);
}

#[no_mangle]
pub extern "C" fn keygen() -> rsa::RsaKeys {
    rsa::keygen()
}

#[no_mangle]
pub extern "C" fn encrypt_chunk(message: u16, keys: &rsa::RsaKeys) -> u32 {
    rsa::encrypt_chunk(message, keys)
}

#[no_mangle]
pub extern "C" fn decrypt_chunk(encrypted: u32, keys: &rsa::RsaKeys) -> u16 {
    rsa::decrypt_chunk(encrypted, keys)
}