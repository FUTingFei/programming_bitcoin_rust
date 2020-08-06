use programming_bitcoin_rust::chapter03::Point;
use programming_bitcoin_rust::chapter01::FieldElement;

fn main() {
    let a = FieldElement::new(0, 223);
    let b = FieldElement::new(7, 223);
    
    let x1 = FieldElement::new(15, 223);
    let y1 = FieldElement::new(86, 223);
    let p1 = Point::new(Some(x1), Some(y1), a, b);
    let mut pp = p1.add(&p1);
    let mut n = 2;

    while pp.x != None {
        n += 1;
        pp = pp.add(&p1);
    }

    println!("n = {}", n);
}