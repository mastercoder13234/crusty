mod helper;

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
    println!("It works!!!")
}

#[no_mangle]
pub extern "C" fn gcd(left: i32, right: i32) -> i32 {
    return helper::gcd(&left, &right);
} 