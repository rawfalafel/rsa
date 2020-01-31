extern crate num_bigint_dig;

use num_bigint_dig::BigUint;

use std::ops::Mul;

#[derive(Debug,PartialEq)]
pub struct GroupElement {
    value: BigUint,
    modulus: BigUint
}

impl<'a> GroupElement {
    pub fn new(value: BigUint, modulus: BigUint) -> Self {
        Self { value, modulus }
    }

    pub fn get_value(&self) -> &BigUint { &self.value }

    pub fn to_bytes_be(&self) -> Vec<u8> {
        self.value.to_bytes_be()
    }

    pub fn pow(&self, exp: &BigUint) -> Self {
        Self {
            value: self.value.modpow(exp, &self.modulus),
            modulus: self.modulus.clone()
        }
    }
}

impl Mul<BigUint> for GroupElement {
    type Output = Self;

    fn mul(self, rhs: BigUint) -> Self {
        Self {
            value: &self.value * &rhs % &self.modulus,
            modulus: self.modulus.clone()
        }
    }
}

impl<'a> Mul<&'a BigUint> for GroupElement {
    type Output = Self;

    fn mul(self, rhs: &BigUint) -> Self {
        Self {
            value: &self.value * rhs % &self.modulus,
            modulus: self.modulus.clone()
        }
    }
}

impl<'a> Mul<BigUint> for &'a GroupElement {
    type Output = GroupElement;

    fn mul(self, rhs: BigUint) -> GroupElement {
        GroupElement {
            value: &self.value * &rhs % &self.modulus,
            modulus: self.modulus.clone()
        }
    }
}

impl<'a, 'b> Mul<&'b BigUint> for &'a GroupElement {
    type Output = GroupElement;

    fn mul(self, rhs: &'b BigUint) -> GroupElement {
        GroupElement {
            value: &self.value * rhs % &self.modulus,
            modulus: self.modulus.clone()
        }
    }
}

impl<'a> PartialEq<&'a GroupElement> for GroupElement {
    fn eq(&self, other: &&'a Self) -> bool {
        self.value == other.value && self.modulus == other.modulus
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::consts::RSA_2048;
    use num_bigint_dig::ToBigUint;
    use num_traits::One;

    use std::str::FromStr;

    #[test]
    fn test_mul_val_val() {
        let big_1 = One::one();
        let big_2 = 2.to_biguint().unwrap();
        let big_3 = 3.to_biguint().unwrap();
        let modulus = 5.to_biguint().unwrap();

        let group_2_mod_5 = GroupElement { value: big_2, modulus };

        assert_eq!((group_2_mod_5 * big_3).get_value(), &big_1);
    }
    
    #[test]
    fn test_mul_ref_ref() {
        let big_1 = One::one();
        let big_2 = 2.to_biguint().unwrap();
        let big_3 = 3.to_biguint().unwrap();
        let modulus = 5.to_biguint().unwrap();

        let group_2_mod_5 = GroupElement { value: big_2, modulus };

        assert_eq!((&group_2_mod_5 * &big_3).get_value(), &big_1);
    }
    
    
    #[test]
    fn test_partial_eq_val_val() {
        let lhs = GroupElement {
            value: One::one(),
            modulus: 5.to_biguint().unwrap()
        };
        let rhs = GroupElement {
            value: One::one(),
            modulus: 5.to_biguint().unwrap()
        };

        assert_eq!(lhs, rhs);
    }
    
    #[test]
    fn test_partial_eq_val_ref() {
        let lhs = GroupElement {
            value: One::one(),
            modulus: 5.to_biguint().unwrap()
        };
        let rhs = GroupElement {
            value: One::one(),
            modulus: 5.to_biguint().unwrap()
        };

        assert_eq!(lhs, &rhs);
    }

    #[test]
    fn test_partial_eq_ref_ref() {
        let lhs = GroupElement {
            value: One::one(),
            modulus: 5.to_biguint().unwrap()
        };
        let rhs = GroupElement {
            value: One::one(),
            modulus: 5.to_biguint().unwrap()
        };

        assert_eq!(&lhs, &rhs);
    }

    #[test]
    fn test_group_element_ownership() {
        let modulus = BigUint::from_str(RSA_2048).unwrap();
        let n = GroupElement::new(2.to_biguint().unwrap(), modulus);
        let value = n.get_value();
        assert_eq!(&2.to_biguint().unwrap(), value);
    }
}