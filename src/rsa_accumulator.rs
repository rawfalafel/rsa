extern crate rand;
extern crate num_bigint_dig;
extern crate sha2;

use crate::group_element::GroupElement;
use num_bigint_dig::{BigUint};
use num_bigint_dig::prime::probably_prime;
use sha2::{Sha256, Digest};

pub fn hash_transcript_to_prime(base: &GroupElement, exp: &BigUint, result: &GroupElement) -> BigUint {
    let mut transcript = vec!{};
    transcript.extend(base.to_bytes_be());
    transcript.extend(exp.to_bytes_be());
    transcript.extend(result.to_bytes_be());
    hash_to_prime(transcript.as_slice())
}

fn hash_to_prime(input: &[u8]) -> BigUint {
    let mut hasher = Sha256::new();
    hasher.input(input);

    loop {
        let result = hasher.result_reset();

        let hash = BigUint::from_bytes_be(&result.as_slice());
        if probably_prime(&hash, 64) {
            return hash;
        }

        hasher.input(&result.as_slice());
    }
}

pub fn prove_ni_poe(base: &GroupElement, exp: &BigUint, result: &GroupElement) -> GroupElement {
    let l = hash_transcript_to_prime(base, exp, result);

    let q = exp / &l;
    base.pow(&q)
}

pub fn verify_ni_poe(base: &GroupElement, exp: &BigUint, result: &GroupElement, proof: &GroupElement) -> bool {
    let l = hash_transcript_to_prime(base, exp, result);
    
    let r = exp % &l;
    let w = proof.pow(&l) * base.pow(&r).get_value();
    w == result
}

#[cfg(test)]
mod tests {
    use super::{GroupElement,prove_ni_poe,verify_ni_poe,hash_to_prime};
    use crate::consts::RSA_2048;
    use num_bigint_dig::{ToBigUint,BigUint};
    use std::str::FromStr;

    fn str_to_byte_vector(input: &str) -> Vec<u8> {
        input.chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>()
    }

    #[test]
    fn test_prove_ni_poe() {
        let big_20 = 20.to_biguint().unwrap();
        
        let big_2 = 2.to_biguint().unwrap();
        let base = GroupElement::new(big_2, BigUint::from_str(RSA_2048).unwrap());
        let result = GroupElement::new(1048576.to_biguint().unwrap(), BigUint::from_str(RSA_2048).unwrap());

        let proof = prove_ni_poe(&base, &big_20, &result);
        assert_eq!(true, verify_ni_poe(&base, &big_20, &result, &proof));
    }

    #[test] 
    fn test_hash2prime() {
        let result = hash_to_prime(b"0123456");

        let expected_result = BigUint::from_str("12198435539266165472078934019524317306641741105794929457926836852115476293041").unwrap();
        assert_eq!(result, expected_result); 
    }
}