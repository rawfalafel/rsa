extern crate rand;
extern crate num_bigint_dig as num_bigint;

mod bignum_ops;

use num_bigint::{ToBigUint,BigUint,RandPrime};
use num_bigint::traits::ModInverse;
use num_traits::{FromPrimitive,Zero,One,Pow};
use bignum_ops::{diff, lcm, gcd};

fn gen_prime(bits: usize) -> BigUint {
    let mut rng = rand::thread_rng();
    let big_2 = 2.to_biguint().unwrap();
    // 2^(1/2) * 2^(bits - 1)
    let min_threshold =  big_2.pow(bits - 33) * BigUint::from_u64(6074001000).unwrap();

    loop {
        let prime = rng.gen_prime(bits);
        if prime > min_threshold {
            return prime;
        }
    }
}

pub fn gen_distinct_primes(bits: usize, e: &BigUint) -> (BigUint, BigUint, BigUint) {
    let big_2 = 2.to_biguint().unwrap();
    let diff_threshold = big_2.pow(bits - 100);

    loop {
        let p = gen_prime(bits);
        let q = gen_prime(bits); 

        let lambda = lcm(&(&p - BigUint::one()), &(&q - BigUint::one()));
        let gcd_e_lambda = gcd(e, &lambda);
        // Ensure that lambda is coprime with e and that the diff between p and q is sufficiently small.
        if gcd_e_lambda == BigUint::one() && diff(&p, &q) > diff_threshold {
            return (p, q, lambda)
        }
    }
}

pub fn generate(bits: usize) -> (BigUint, BigUint, BigUint) {
    // Commonly used value as a compromise btwn avoiding small exponent attack and allowing efficient encryption.
    let e = 65537.to_biguint().unwrap();
    let (p, q, lambda) = gen_distinct_primes(bits, &e);
    let d = (&e).mod_inverse(lambda).unwrap().to_biguint().unwrap();

    (p * q, e, d)
}

pub fn encrypt(m: &BigUint, n: &BigUint, e: &BigUint) -> BigUint {
    m.modpow(e, n)
}

pub fn decrypt(c: &BigUint, d: &BigUint, n: &BigUint) -> BigUint {
    c.modpow(d, n)
}

#[cfg(test)]
mod tests {
    use super::*;
    // use super::{generate, encrypt, decrypt};
    use num_bigint::{RandBigInt};

    #[test]
    fn test_generate() {
        let (n, e, d) = generate(200);
        
        let mut rng = rand::thread_rng();

        let m = rng.gen_biguint(200);
        let c = encrypt(&m, &n, &e);
        assert_eq!(m, decrypt(&c, &d, &n), "Encryption and decryption works");
    }
}