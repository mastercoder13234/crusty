#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    println!("Adding {} and {} in Rust!", a, b);
    a + b
}
