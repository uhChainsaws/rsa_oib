use num::{BigInt, bigint::ToBigInt};


/// Returns (e, n, d)
pub fn generate_keypair_pq(p: &num::BigInt, q: &num::BigInt) -> (num::BigInt, num::BigInt, num::BigInt) {
    let e = 65537.to_bigint().unwrap();
    let n = p.clone()*q.clone();
    let φ = (p.clone()-1.to_bigint().unwrap())*(q.clone()-1.to_bigint().unwrap());
    let d = mod_inverse(&e, &φ);
    return (e,n,d);
}

pub fn gen_a_prime() -> BigInt {
    loop {
        use num::bigint::RandBigInt;
        let mut rng = rand::thread_rng();
        let candidate:BigInt = rng.gen_bigint(256);
        
        if  is_rabin_miller_prime(&candidate, Some(42)){
            return candidate;
        }
    }
}

pub fn encrypt(mess: &num::BigInt, e: &num::BigInt, n: &num::BigInt) -> num::BigInt{
    modular_exponentiation(mess, e, n)
}

pub fn decrypt(c: &BigInt, d: &BigInt, n: &BigInt) -> BigInt {
    modular_exponentiation(c, d, n)
}

fn modular_exponentiation<T: ToBigInt>(n: &T, e: &T, m: &T) -> BigInt {
    let n = n.to_bigint().unwrap();
    let e = e.to_bigint().unwrap();
    let m = m.to_bigint().unwrap();
    use num::traits::{Zero, One};
    if e == Zero::zero() {
        return One::one()
    }

    let mut result: BigInt = One::one();
    let mut base = n % &m;
    let mut exp = e;
    
    // yay square & multiply 
    loop {
        if &exp % 2 == One::one() {
            result *= &base;
            result %= &m;
        }
 
        if exp == One::one() {
            return result
        }
 
        exp /= 2;
        base *= base.clone();
        base %= &m;
    }
}

fn get_random_bigint(low: &BigInt, high: &BigInt) -> BigInt {
    if low == high {  
        return low.clone()
    }
 
    let middle = (low.clone() + high) / 2.to_bigint().unwrap();
 
    let go_low: bool = rand::random();
 
    if go_low {
        return get_random_bigint(low, &middle)
    } else {
        return get_random_bigint(&middle, high)
    }
}
 
fn is_rabin_miller_prime<T: ToBigInt>(n: &T, k: Option<usize>) -> bool {
    let n = n.to_bigint().unwrap();
    let k = k.unwrap_or(40); 
 
    use num::traits::{Zero, One};
    let zero: BigInt = Zero::zero();
    let one: BigInt = One::one();
    let two: BigInt = 2.to_bigint().unwrap();
 
    if n <= one {
        return false
    } else if n == two {
        return true 
    } else if n.clone() % &two == Zero::zero() {
        return false
    }

    let mut t: BigInt = zero.clone();
    let n_minus_one: BigInt = n.clone() - &one;
    let mut s = n_minus_one.clone();
    while &s % &two == one {
        s /= &two;
        t += &one;
    }

    // dont know why this works. black magic. 
    'outer: for _ in 0..k {
        let a = get_random_bigint(&two, &n_minus_one);
        let mut v = modular_exponentiation(&a, &s, &n);
        if v == one {
            continue 'outer;
        }
        let mut i: BigInt = zero.clone();
        while &i < &t {
            v = (v.clone() * &v) % &n;
            if &v == &n_minus_one {
                continue 'outer;
            }
            i += &one;
        }
        return false;
    }
    true
}

fn mod_inverse<T: ToBigInt>(a: &T, n: &T) -> BigInt {
    use num::traits::{Zero, One};
    let a = a.to_bigint().unwrap();
    let n = n.to_bigint().unwrap();
 
    let mut t: BigInt = Zero::zero();
    let mut newt: BigInt = One::one();
    let mut r = n.clone();
    let mut newr = a.clone();
 
    while newr != Zero::zero() {
        let quotient = &r / &newr;
 
        let oldt = t.clone();
        t = newt.clone();
        newt = oldt - &quotient * &newt;
 
        let oldr = r.clone();
        r = newr.clone();
        newr = oldr - &quotient * &newr;
    }
    if r > One::one() {
        panic!("{} is not invertible mod {}", a, n)
    }
    if t < Zero::zero() {
        t += n;
    }
    t
}

