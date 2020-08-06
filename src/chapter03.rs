use crate::chapter01::FieldElement;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: Option<FieldElement>,
    pub y: Option<FieldElement>,
    pub a: FieldElement,
    pub b: FieldElement,
}

impl Point {
    pub fn new(x: Option<FieldElement>, y: Option<FieldElement>, a: FieldElement, b: FieldElement) -> Point {
        let x_real;
        let y_real;
        match (x, y) {
            (Some(x_), Some(y_)) => {
                x_real = x_;
                y_real = y_;
                if y_real.pow(2).ne(&x_real.pow(3).add(&a.mul(&x_real)).add(&b)) {
                    panic!("({:?},{:?}) is not on the curve", x_real, y_real);
                }
                Point {
                    x: Some(x_real),
                    y: Some(y_real),
                    a: a,
                    b: b, 
                }
            },
            (None, None) => {
                Point {
                    x: None,
                    y: None,
                    a: a,
                    b: b,
                }
            },
            _ => {
                panic!("The point does not exist");
            }
        }
    }

    pub fn add(&self, other: &Point) -> Point {
        if self.a.ne(&other.a) || self.b.ne(&other.b) {
            panic!("Points {:?}, {:?} are not on the same curve", self, other);
        }

        if self.x == None {
            return *other;
        }

        if other.x == None {
            return *self;
        }

        let (y2, x2, y1, x1) = (other.y.unwrap(), other.x.unwrap(), self.y.unwrap(), self.x.unwrap());
        let mut x3:Option<FieldElement> = None; 
        let mut y3:Option<FieldElement> = None;

        if x1.eq(&x2) && y1.ne(&y2) {
            x3 = None;
            y3 = None;
        }

        if x1.ne(&x2) {
            let s = y2.sub(&y1).div(&x2.sub(&x1));
            let x3_ = s.pow(2).sub(&x1).sub(&x2);
            let y3_ = s.mul(&x1.sub(&x3_)).sub(&y1);
            x3=Some(x3_);
            y3=Some(y3_);
        }

        if x1.eq(&x2) && y1.eq(&y2) {
            let zero = FieldElement::new(0, x1.prime);
            let two = FieldElement::new(2, x1.prime);
            let three = FieldElement::new(3, x1.prime);

            if y1.eq(&zero) {
                x3 = None;
                y3 = None;
            } else {
                let s = three.mul(&x1.pow(2)).add(&self.a).div(&two.mul(&y1));
                let x3_ = s.pow(2).sub(&two.mul(&x1));
                let y3_ = s.mul(&x1.sub(&x3_)).sub(&y1);
                x3=Some(x3_);
                y3=Some(y3_);
            }
        }

        Point {
            x: x3,
            y: y3,
            a: self.a,
            b: self.b,
        }
    }

    // learn
    pub fn rmul(&self, coefficient: usize) -> Point {
        let mut coef = coefficient;
        let mut current = *self;
        let mut result = Point::new(None, None, self.a, self.b);

        while coef > 0 {
            if coef & 1 == 1 {
                result = result.add(&current);
            }
            current = current.add(&current);
            coef = coef >> 1;
        }

        result
    }

    pub fn eq(&self, other: &Point) -> bool {
        self.a.eq(&other.a) &&  self.b.eq(&other.b) &&  self.x.eq(&other.x) &&  self.y.eq(&other.y)
    }

    pub fn ne(&self, other: &Point) -> bool {
        self.a.ne(&other.a )|| self.b.ne(&other.b) || self.x.ne(&other.x) ||  self.y.ne(&other.y)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test03() {
        let a = FieldElement::new(0, 223);
        let b = FieldElement::new(7, 223);

        let x1 = FieldElement::new(170, 223);
        let y1 = FieldElement::new(142, 223);
        let x2 = FieldElement::new(60, 223);
        let y2 = FieldElement::new(139, 223);
        let x3 = FieldElement::new(220, 223);
        let y3 = FieldElement::new(181, 223);
        let p1 = Point::new(Some(x1), Some(y1), a, b);
        let p2 = Point::new(Some(x2), Some(y2), a, b);
        let p3 = Point::new(Some(x3), Some(y3), a, b);
        assert_eq!(p1.add(&p2), p3);

        let x1 = FieldElement::new(47, 223);
        let y1 = FieldElement::new(71, 223);
        let x2 = FieldElement::new(17, 223);
        let y2 = FieldElement::new(56, 223);
        let x3 = FieldElement::new(215, 223);
        let y3 = FieldElement::new(68, 223);
        let p1 = Point::new(Some(x1), Some(y1), a, b);
        let p2 = Point::new(Some(x2), Some(y2), a, b);
        let p3 = Point::new(Some(x3), Some(y3), a, b);
        assert_eq!(p1.add(&p2), p3);

        let x1 = FieldElement::new(143, 223);
        let y1 = FieldElement::new(98, 223);
        let x2 = FieldElement::new(76, 223);
        let y2 = FieldElement::new(66, 223);
        let x3 = FieldElement::new(47, 223);
        let y3 = FieldElement::new(71, 223);
        let p1 = Point::new(Some(x1), Some(y1), a, b);
        let p2 = Point::new(Some(x2), Some(y2), a, b);
        let p3 = Point::new(Some(x3), Some(y3), a, b);
        assert_eq!(p1.add(&p2), p3);

        let x1 = FieldElement::new(15, 223);
        let y1 = FieldElement::new(86, 223);
        let p1 = Point::new(Some(x1), Some(y1), a, b);
        let pp = Point::new(None, None, a, b);
        assert_eq!(p1.rmul(7), pp);
    }
}



