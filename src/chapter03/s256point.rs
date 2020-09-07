use num_bigint::{BigUint, BigInt, ToBigInt, ToBigUint};
use super::s256field::S256Field;

#[derive(Debug, PartialEq, Clone)]
pub struct S256Point {
    pub x: Option<S256Field>,
    pub y: Option<S256Field>,
    pub a: S256Field,
    pub b: S256Field,
}

impl S256Point {
    pub fn new(x: Option<S256Field>, y: Option<S256Field>) -> S256Point {
        let x_real;
        let y_real;
        let a = S256Field::new(0.to_biguint().unwrap());
        let b = S256Field::new(7.to_biguint().unwrap());
        match (x, y) {
            (Some(x_), Some(y_)) => {
                x_real = x_;
                y_real = y_;
                if y_real.pow(2.to_bigint().unwrap()).ne(&x_real.pow(3.to_bigint().unwrap()).add(&b)) {
                    panic!("({:?},{:?}) is not on the curve", x_real, y_real);
                }
                S256Point {
                    x: Some(x_real),
                    y: Some(y_real),
                    a: a,
                    b: b, 
                }
            },
            (None, None) => {
                S256Point {
                    x: None,
                    y: None,
                    a: a,
                    b: b,
                }
            },
            _ => {
                panic!("The S256Point does not exist");
            }
        }
    }

    pub fn add(&self, other: &S256Point) -> S256Point {
        if self.a.ne(&other.a) || self.b.ne(&other.b) {
            panic!("Points {:?}, {:?} are not on the same curve", self, other);
        }

        if self.x == None {
            return other.clone();
        }

        if other.x == None {
            return self.clone();
        }

        let (y2, x2, y1, x1) = (other.clone().y.unwrap(), other.x.clone().unwrap(), self.y.clone().unwrap(), self.x.clone().unwrap());
        let mut x3:Option<S256Field> = None; 
        let mut y3:Option<S256Field> = None;

        if x1.eq(&x2) && y1.ne(&y2) {
            x3 = None;
            y3 = None;
        }

        if x1.ne(&x2) {
            let s = y2.sub(&y1).div(&x2.sub(&x1));
            let x3_ = s.pow(2.to_bigint().unwrap()).sub(&x1).sub(&x2);
            let y3_ = s.mul(&x1.sub(&x3_)).sub(&y1);
            x3=Some(x3_);
            y3=Some(y3_);
        }

        if x1.eq(&x2) && y1.eq(&y2) {
            let zero = S256Field::new(0.to_biguint().unwrap());
            let two = S256Field::new(2.to_biguint().unwrap());
            let three = S256Field::new(3.to_biguint().unwrap());

            if y1.eq(&zero) {
                x3 = None;
                y3 = None;
            } else {
                let s = three.mul(&x1.pow(2.to_bigint().unwrap())).add(&self.a).div(&two.mul(&y1));
                let x3_ = s.pow(2.to_bigint().unwrap()).sub(&two.mul(&x1));
                let y3_ = s.mul(&x1.sub(&x3_)).sub(&y1);
                x3=Some(x3_);
                y3=Some(y3_);
            }
        }

        S256Point::new(x3, y3)
    }

    // 二进制展开算法，需要进一步学习
    pub fn rmul(&self, coefficient: BigUint) -> S256Point {
        let mut coef = coefficient;
        let mut current = self.clone();
        let mut result = S256Point::new(None, None);

        while coef > 0.to_biguint().unwrap() {
            if coef.clone() & 1.to_biguint().unwrap() == 1.to_biguint().unwrap() {
                result = result.add(&current);
            }
            current = current.add(&current);
            coef = coef >> 1;
        }

        result
    }

    pub fn eq(&self, other: &S256Point) -> bool {
        self.a.eq(&other.a) &&  self.b.eq(&other.b) &&  self.x.eq(&other.x) &&  self.y.eq(&other.y)
    }

    pub fn ne(&self, other: &S256Point) -> bool {
        self.a.ne(&other.a )|| self.b.ne(&other.b) || self.x.ne(&other.x) ||  self.y.ne(&other.y)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test03_2() {
        let gx:BigUint = BigUint::parse_bytes(b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).unwrap();
        let gy:BigUint = BigUint::parse_bytes(b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).unwrap();
        let gx = S256Field::new(gx);
        let gy = S256Field::new(gy);
        let p = S256Point::new(Some(gx), Some(gy));
        println!("{:?}", p);
    }
}





