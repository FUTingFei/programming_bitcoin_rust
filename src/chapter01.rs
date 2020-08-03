
#[derive(Debug, PartialEq)]
struct FieldElement {
    num: usize,
    prime: usize,
}

impl FieldElement {
    fn new(num: usize, prime: usize) -> FieldElement {
        if num > prime {
            panic!("Num {} not in field range 0 to {}.", num, prime);
        }

        FieldElement {
            num: num,
            prime: prime
        }
    }

    fn repr(&self) {
        println!("FieldElement_{}({})",self.num, self.prime);
    }

    fn eq(&self, other: &FieldElement) -> bool {
        self.num == other.num && self.prime == self.prime
    }

    fn ne(&self, other: &FieldElement) -> bool {
        self.num != other.num || self.prime != self.prime
    }

    fn add(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields.");
        }
        let num = (self.num + other.num) % self.prime;
        FieldElement::new(num, self.prime)
    }

    fn sub(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields.");
        }

        let i_num = self.num as isize - other.num as isize;
        
        let num = if i_num >= 0 {
            (i_num % self.prime as isize) as usize
        } else {
            ((i_num % self.prime as isize) + self.prime as isize) as usize
        };
        
        FieldElement::new(num, self.prime)
    }

    fn mul(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot multi two numbers in different Fields.");
        }
        let num = (self.num * other.num) % self.prime;
        FieldElement::new(num, self.prime)
    }

    fn pow(&self, other: i32) -> FieldElement {
        let num = if other >= 0 {
            self.num.pow(other as u32)  % self.prime
        } else {
            self.num.pow((self.prime as isize - 1 + other as isize) as u32) % self.prime
        };

        FieldElement::new(num, self.prime)
    }

    fn div(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot multi two numbers in different Fields.");
        }

        let num = (self.num * other.num.pow(other.prime as u32 - 2)) % other.prime;

        FieldElement::new(num, self.prime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chapter() {
        let f1 = FieldElement::new(3, 13);
        let f2 = FieldElement::new(3, 13);
        let f3 = FieldElement::new(6, 13);
        
        assert_eq!(f1.eq(&f2), true);
        assert_eq!(f1.eq(&f3), false);
        assert_eq!(f1.ne(&f2), false);

        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(12, 13);
        let c = FieldElement::new(6, 13);
        let d = FieldElement::new(8, 13);

        assert_eq!(c, a.add(&b));
        assert_eq!(d, a.sub(&b));

        let a = FieldElement::new(3, 13);
        let b = FieldElement::new(12, 13);
        let c = FieldElement::new(10, 13);
        let d = FieldElement::new(1, 13);
        let e = FieldElement::new(8, 13);
        let f = FieldElement::new(7, 13);


        assert_eq!(a.mul(&b), c);
        assert_eq!(a.pow(3), d);
        assert_eq!(f.pow(-3), e);

        let a = FieldElement::new(2, 19);
        let b = FieldElement::new(7, 19);
        let c = FieldElement::new(3, 19);
        let d = FieldElement::new(5, 19);
        let e = FieldElement::new(9, 19);

        assert_eq!(a.div(&b), c);
        assert_eq!(b.div(&d), e);

    }
}
