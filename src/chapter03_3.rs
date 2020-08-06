use crate::chapter03_2::Field;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FieldElement {
    pub num: usize,
    pub prime: usize,
}

impl Field for FieldElement {
    fn new(num: usize, prime: usize) -> FieldElement {
        if num > prime {
            panic!("Num {} not in field range 0 to {}.", num, prime);
        }

        FieldElement {
            num: num,
            prime: prime
        }
    }

    fn num(&self) -> usize {
        self.num
    }

    fn prime(&self) -> usize {
        self.prime
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test03_3() {
        let f1 = FieldElement::new(3, 13);
        let f2 = FieldElement::new(3, 13);
        let f3 = FieldElement::new(6, 13);
        
        assert_eq!(f1.eq_(&f2), true);
        assert_eq!(f1.eq_(&f3), false);
        assert_eq!(f1.ne_(&f2), false);

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
