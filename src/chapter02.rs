#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: Option<isize>,
    pub y: Option<isize>,
    pub a: isize,
    pub b: isize,
}

impl Point {
    pub fn new(x: Option<isize>, y: Option<isize>, a: isize, b: isize) -> Point {
        let x_real;
        let y_real;
        match (x, y) {
            (Some(x_), Some(y_)) => {
                x_real = x_;
                y_real = y_;
                if y_real.pow(2) != x_real.pow(3) + a * x_real + b {
                    panic!("({},{}) is not on the curve", x_real, y_real);
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
        if self.a != other.a || self.b != other.b {
            panic!("Points {:?}, {:?} are not on the same curve", self, other);
        }

        if self.x == None {
            return *other;
        }

        if other.x == None {
            return *self;
        }

        let (y2, x2, y1, x1) = (other.y.unwrap(), other.x.unwrap(), self.y.unwrap(), self.x.unwrap());
        let mut x3:Option<isize> = None;
        let mut y3:Option<isize> = None;

        if x1 == x2 && y1 != y2 {
            x3 = None;
            y3 = None;
        }

        if x1 != x2 {
            let s = (y2 - y1)/(x2 - x1);
            let x3_ = s.pow(2) - x1 - x2;
            let y3_ = s * (x1 - x3_) - y1;
            x3=Some(x3_);
            y3=Some(y3_);
        }

        if x1 == x2 && y1 == y2 {
            if y1 == 0 {
                x3 = None;
                y3 = None;
            } else {
                let s = (3 * x1.pow(2) + self.a) / (2 * y1);
                let x3_ = s.pow(2) - 2 * x1;
                let y3_ = s * (x1 - x3_) - y1;
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

    pub fn eq(&self, other: &Point) -> bool {
        self.a == other.a &&  self.b == other.b &&  self.x == other.x &&  self.y == other.y
    }

    pub fn ne(&self, other: &Point) -> bool {
        self.a != other.a || self.b != other.b || self.x != other.x ||  self.y != other.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test02() {
        let p1 = Point::new(Some(-1), Some(-1), 5, 7);
        let p2 = Point::new(Some(18), Some(77), 5, 7);
        assert_eq!(p1.eq(&p2), false);
        assert_eq!(p1.ne(&p2), true);

        let p1 = Point::new(Some(-1), Some(-1), 5, 7);
        let p2 = Point::new(Some(-1), Some(-1), 5, 7);
        let p3 = Point::new(Some(18), Some(77), 5, 7);
        assert_eq!(p1.add(&p2), p3);

        let p1 = Point::new(Some(2), Some(5), 5, 7);
        let p2 = Point::new(Some(-1), Some(-1), 5, 7);
        let p3 = Point::new(Some(3), Some(-7), 5, 7);
        assert_eq!(p1.add(&p2), p3);

    }
}


