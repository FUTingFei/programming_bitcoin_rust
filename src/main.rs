use programming_bitcoin_rust::chapter02::Point;

fn main() {
    let p1 = Point::new(Some(-1), Some(-1), 5, 7);
    let p2 = Point::new(Some(-1), Some(-1), 5, 7);
    let p3 = p1.add(&p2);
    println!("{:?}", p3);
}