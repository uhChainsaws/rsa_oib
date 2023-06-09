#![allow(non_snake_case)]

use std::ops::BitOr;

use rand::Rng;

fn main() {
    println!("4------------4\n---67--4-647--\n");
    let n = 2;
    let p = 11;
    println!("is {} prime? \n{}\n", n, is_prime(n, 42));
    println!("is {} prime? \n{}\n", p, is_prime(p, 42));
    println!("is {} prime? \n{}\n", 319, is_prime(319, 42));
    let p = gen_a_prime();
    let q = gen_a_prime();
    println!("p: {}\nq: {}\n", p, q);
    println!("p is prime? {}\nq is prime? {}\n", is_prime(p, 128), is_prime(q, 128));
}

fn gen_a_prime() -> u128 {
    loop {
        let mut candidate:u128 = rand::random();
        candidate = candidate.bitor(0x10000000000000000000000000000001);
        if is_prime(candidate, 42){
            return candidate;
        }
    }
}

fn is_prime(n: u128, k: u16) -> bool {
    // println!("checking {:42} for primality", n);

    if n <= 1 || n == 4 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    let mut d = n - 1;
    while d % 2 == 0 {
        d /= 2;
    }
    for i in 0..k {
        if i!= 0 {println!("{}'th iteration", i);};
        if !miller_test(n, d) {
            return false;
        }
    }
    return true;
}

fn miller_test(n: u128, mut d: u128) -> bool {
    let mut rng = rand::thread_rng();
    let mut a = rng.gen_range(2..(n - 2));
    let mut x = mod_exp(a, d, n);
    if x == 1 || x == n - 1 {
        return true;
    }
    while d != n - 1 {
        x = (x * x) % n;
        d *= 2;
        if x == 1 {
            return false;
        }
        if x == n - 1 {
            return true;
        }
    }
    return false;

}

// fn mod_exp(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
//     if modulus == 1 {
//         return 0;
//     }
//     let mut c = 1;
//     for _ in 0..exp {
//         c = (c * base) % modulus;
//     }
//     return c;
// }
// too slow
fn mod_exp(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }
    let mut c = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            c = (c * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    return c;
}


fn is_prime_definitely(n: u128) -> bool {
    let mut i = 3;
    while i < n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    return true;
}