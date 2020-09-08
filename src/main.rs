use num_bigint::*;
use programming_bitcoin_rust::chapter03::s256field::*;
use programming_bitcoin_rust::chapter03::s256point::*;

fn main() {
    let gx:BigUint = BigUint::parse_bytes(b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).unwrap();
    let gy:BigUint = BigUint::parse_bytes(b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).unwrap();
    let n:BigUint = BigUint::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141", 16).unwrap();
    let gx = S256Field::new(gx);
    let gy = S256Field::new(gy);
    let g = S256Point::new(Some(gx), Some(gy));
    println!{"{:?}", g.rmul(n)};
}