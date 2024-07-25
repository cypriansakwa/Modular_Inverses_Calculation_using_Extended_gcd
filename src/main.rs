fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}         

fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = extended_gcd(a, m);
    if g != 1 {
        None // No modular inverse if gcd(a, m) != 1
    } else {
        Some((x % m + m) % m) // Ensure the result is positive
    }
}

fn main() {
    let a = 5;
    let m = 12;
    match mod_inverse(a, m) {
        Some(inv) => println!("The modular inverse of {} modulo {} is {}", a, m, inv),
        None => println!("The modular inverse does not exist"),
    }
}

