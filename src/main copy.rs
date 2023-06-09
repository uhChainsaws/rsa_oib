#![allow(non_snake_case)]

fn main() {
    println!("4------------4\n---67--4-647--\n");
    let p: u128 = 3471661859;
    let q: u128 = 2800521109;
    // let p: u128 = 3;
    // let q: u128 = 7;
    let n  = p * q;
    println!("n: {}\n", n);
    let e: u128 = 65537;
    let φ = (p - 1) * (q - 1);
    println!("φ: {}\n", φ);
    let c = mod_exp(12, e, n);
    println!("c: {}\n", c);
    let k = 2;
    let d = (1 + (k*φ))/e;
    println!("d: {}\n", d);
    let mess = 42;
    let c = mod_exp(mess, e, n);
    println!("encrypted {}\n", c);
    let m = mod_exp(c, d, n);
    println!("decrypted {}\n", m);

}

fn mod_exp(base: u128, exp: u128, modulus: u128) -> u128 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}