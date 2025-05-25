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
    return helper::gcd(left as u64, right as u64) as u32;
}

#[no_mangle]
pub extern "C" fn is_prime(n: u32) -> bool {
    return helper::is_prime(n as u64);
}

#[no_mangle]
pub extern "C" fn gen_prime(min: u32, max: u32) -> u32 {
    return helper::gen_prime(min as u64, max as u64) as u32;
}

#[no_mangle]
pub extern "C" fn modpow(base: u32, exponent: u32, modulus: u32) -> u32 {
    return helper::modpow(base as u64, exponent as u64, modulus as u64) as u32;
}

#[no_mangle]
pub extern "C" fn modinv(a: u32, m: u32) -> u32 {
    return helper::modinv(a as u64, m as u64) as u32;
}

#[no_mangle]
pub extern "C" fn is_coprime(a: u32, b: u32) -> bool {
    return helper::is_coprime(a as u64, b as u64);
}

#[no_mangle]
pub extern "C" fn keygen() -> rsa::RsaKeys {
    rsa::keygen()
}
