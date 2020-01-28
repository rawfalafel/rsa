use num_bigint_dig::{ToBigUint,BigUint};
use num_traits::Zero;

pub fn diff(i: &BigUint, j: &BigUint) -> BigUint {
    if i > j { i - j } else { j - i }
}

pub fn lcm(i: &BigUint, j: &BigUint) -> BigUint {
    i * j / gcd(i, j)
}

pub fn gcd(i: &BigUint, j: &BigUint) -> BigUint {
    let mut i = i.to_biguint().unwrap();
    let mut j = j.to_biguint().unwrap();
    while j != BigUint::zero() {
        let t = j;
        j = i % &t;
        i = t;
    }

    return i;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff() {
        assert_eq!(diff(&10.to_biguint().unwrap(), &5.to_biguint().unwrap()), 5.to_biguint().unwrap());
    }

    #[test]
    fn test_neg_diff() {
        assert_eq!(diff(&5.to_biguint().unwrap(), &10.to_biguint().unwrap()), 5.to_biguint().unwrap());
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(&10.to_biguint().unwrap(), &15.to_biguint().unwrap()), 5.to_biguint().unwrap());
    }

    #[test]
    fn test_gcd_inverse() {
        assert_eq!(gcd(&5.to_biguint().unwrap(), &10.to_biguint().unwrap()), 5.to_biguint().unwrap());
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(&5.to_biguint().unwrap(), &10.to_biguint().unwrap()), 10.to_biguint().unwrap());
    }

    #[test]
    fn test_lcm_inverse() {
        assert_eq!(lcm(&5.to_biguint().unwrap(), &10.to_biguint().unwrap()), 10.to_biguint().unwrap());
    }
}