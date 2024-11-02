fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}         

fn mod_inverse(a: i128, m: i128) -> Option<i128> {
    let (g, x, _) = extended_gcd(a, m);
    if g != 1 {
        None // No modular inverse if gcd(a, m) != 1
    } else {
        Some((x % m + m) % m) // Ensure the result is positive
    }
}

fn main() {
    let a = 11;
    let m = 17;
    match mod_inverse(a, m) {
        Some(inv) => println!("The modular inverse of {} modulo {} is {}", a, m, inv),
        None => println!("The modular inverse does not exist"),
    }
}

