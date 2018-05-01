extern crate libc;
extern crate sha2;

mod curve;
mod element;
mod element_pp;
mod group;
mod integer;
mod pairing;

pub use curve::Curve;
pub use element::Element;
pub use element_pp::ElementPP;
pub use group::Group;
pub use integer::Integer;
pub use pairing::Pairing;

pub fn xor(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    assert_eq!(a.len(), b.len());

    let mut c = vec![0u8; a.len()];
    for i in 0..a.len() {
        c[i] = a[i] ^ b[i];
    }
    c
}

pub fn polynomial(in_x: &Element, pairing: &Pairing, coefficients: &Vec<Element>) -> Element {
    // y = 0
    let mut y = Element::new(Group::Zr, pairing);
    y.set0();
    // x = 1
    let mut x = Element::new(Group::Zr, pairing);
    x.set1();

    for coefficient in coefficients {
        // tmp = coefficient * x
        let mut tmp = Element::new(Group::Zr, pairing);
        tmp.set(&coefficient);
        tmp.mul(&x);

        // y = y + tmp
        y.add(&tmp);

        // x = x * in_x;
        x.mul(in_x);
    }

    // ret y
    y
}
