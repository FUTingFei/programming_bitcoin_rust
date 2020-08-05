use programming_bitcoin_rust::chapter03::Point;
use programming_bitcoin_rust::chapter01::FieldElement;

fn main() {
    let a = FieldElement::new(0, 223);
    let b = FieldElement::new(7, 223);
    
    let x = FieldElement::new(192, 223);
    let y = FieldElement::new(105, 223);
    let p1 = Point::new(Some(x), Some(y), a, b);
    println!("p1 is {:?}", p1);

    let x = FieldElement::new(17, 223);
    let y = FieldElement::new(56, 223);
    let p1 = Point::new(Some(x), Some(y), a, b);
    println!("p1 is {:?}", p1);

    // let x = FieldElement::new(200, 223);
    // let y = FieldElement::new(119, 223);
    // let p1 = Point::new(Some(x), Some(y), a, b);
    // println!("p1 is {:?}", p1);

    let x = FieldElement::new(1, 223);
    let y = FieldElement::new(193, 223);
    let p1 = Point::new(Some(x), Some(y), a, b);
    println!("p1 is {:?}", p1);

    // let x = FieldElement::new(42, 223);
    // let y = FieldElement::new(99, 223);
    // let p1 = Point::new(Some(x), Some(y), a, b);
    // println!("p1 is {:?}", p1);

}