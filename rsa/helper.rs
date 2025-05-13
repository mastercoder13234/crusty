pub fn gcd(left: &i32, right: &i32) -> i32 {
    let mut tempr = *right;
    let mut templ = *left;
    while tempr > 0 {
        let temp = tempr;
        tempr = templ % tempr;
        templ = temp;
    }
    templ
}
