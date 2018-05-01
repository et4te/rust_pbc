
extern crate libc;
extern crate sha2;

mod integer;
mod pairing;
mod element_pp;
mod element;

pub use integer::Integer;
pub use pairing::Pairing;
pub use element_pp::ElementPP;
pub use element::Element;

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
    let mut y = Element::new();
    y.init_zr(&pairing);
    y.set0();
    // x = 1
    let mut x = Element::new();
    x.init_zr(&pairing);
    x.set1();
    
    for coefficient in coefficients {
        // tmp = coefficient * x
        let mut tmp = Element::new();
        tmp.init_zr(&pairing);
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
