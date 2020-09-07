use num_bigint::{BigUint, BigInt, ToBigInt, ToBigUint};
// use num_bigint::Sign::{Plus, Minus};

#[derive(Debug, PartialEq, Clone)]
pub struct S256Field {
    pub num: BigUint,
    pub prime: BigUint,
}

impl S256Field {

    pub fn new(num: BigUint) -> S256Field {
        let big2:BigUint = BigUint::new(vec![2]);
        let big977:BigUint = BigUint::new(vec![977]);
        let p:BigUint = big2.pow(256) - big2.pow(32) - big977;

        if num > p {
            panic!("Num {} not in field range 0 to {}.", num, p);
        }

        S256Field {
            num: num,
            prime: p,
        }
    }

    pub fn repr(&self) {
        println!("S256Field_{}({})",self.num, self.prime);
    }

    pub fn eq(&self, other: &S256Field) -> bool {
        self.num == other.num && self.prime == self.prime
    }

    pub fn ne(&self, other: &S256Field) -> bool {
        self.num != other.num || self.prime != self.prime
    }

    pub fn add(&self, other: &S256Field) -> S256Field {
        let num = (self.num.clone() + other.num.clone()) % self.prime.clone();
        S256Field::new(num)
    }

    pub fn sub(&self, other: &S256Field) -> S256Field {
        let num: BigUint = if self.num >= other.num {
            (self.num.clone() - other.num.clone()) % self.prime.clone()
        } else {
            ((self.num.clone().to_bigint().unwrap() - other.num.clone().to_bigint().unwrap()) % self.prime.clone().to_bigint().unwrap() + self.prime.clone().to_bigint().unwrap()).to_biguint().unwrap()
        };
        
        S256Field::new(num)
    }

    pub fn mul(&self, other: &S256Field) -> S256Field {
        let num = (self.num.clone() * other.num.clone()) % self.prime.clone();
        S256Field::new(num)
    }

    pub fn pow(&self, other: BigInt) -> S256Field {
        let num = if other >= 0.to_bigint().unwrap() {
            self.num.modpow(&other.to_biguint().unwrap(), &self.prime)
        } else {
            self.num.modpow(&(self.prime.to_bigint().unwrap() - 1.to_bigint().unwrap() + other).to_biguint().unwrap(), &self.prime)
        };

        S256Field::new(num)
    }

    pub fn div(&self, other: &S256Field) -> S256Field {
        let num = (self.num.clone() * other.num.modpow(&(other.prime.clone() - 2.to_biguint().unwrap()), &other.prime)) % other.prime.clone();

        S256Field::new(num)
    }

}

// 由于P太大了，测试不太好实现，所以功能并不完全，待后续完善

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test03_1() {
        let f1 = S256Field::new(3.to_biguint().unwrap());
        let f2 = S256Field::new(3.to_biguint().unwrap());
        let f3 = S256Field::new(6.to_biguint().unwrap());
        
        assert_eq!(f1.eq(&f2), true);
        assert_eq!(f1.eq(&f3), false);
        assert_eq!(f1.ne(&f2), false);

        let a = S256Field::new(7.to_biguint().unwrap());
        let b = S256Field::new(12.to_biguint().unwrap());
        let c = S256Field::new(19.to_biguint().unwrap());
        let d = S256Field::new(5.to_biguint().unwrap());


        assert_eq!(c, a.add(&b));
        assert_eq!(d, b.sub(&a));

        let a = S256Field::new(3.to_biguint().unwrap());
        let b = S256Field::new(12.to_biguint().unwrap());
        let c = S256Field::new(36.to_biguint().unwrap());
        let d = S256Field::new(27.to_biguint().unwrap());

        assert_eq!(a.mul(&b), c);
        assert_eq!(a.pow(3.to_bigint().unwrap()), d);

        // let a = S256Field::new(2.to_biguint().unwrap());
        // let b = S256Field::new(7.to_biguint().unwrap());
        // let c = S256Field::new(3.to_biguint().unwrap());
        // let d = S256Field::new(5.to_biguint().unwrap());
        // let e = S256Field::new(9.to_biguint().unwrap());

        // assert_eq!(a.div(&b), c);
        // assert_eq!(b.div(&d), e);

    }
}