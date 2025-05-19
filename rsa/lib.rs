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
pub extern "C" fn gcd(left: u64, right: u64) -> u64 {
    return helper::gcd(left, right);
}

#[no_mangle]
pub extern "C" fn is_prime(n: u64) -> bool {
    return helper::is_prime(n);
}

#[no_mangle]
pub extern "C" fn gen_prime() -> u64 {
    return helper::gen_prime();
}
