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

use std::collections::HashSet;

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

pub fn lagrange(pairing: &Pairing, k: u32, l: u32, s: &HashSet<i64>, j: i64) -> Element {
    assert_eq!(k as usize, s.len());

    let expected_s: HashSet<i64> = (0..l).map(|i| i as i64).collect();

    assert!(s.is_subset(&expected_s));

    let mut s: Vec<i64> = s.iter().cloned().filter(|x| (*x) != j.clone()).collect();

    s.sort();

    // num = 1
    let mut num = Element::new(Group::Zr, pairing);
    num.set1();
    // den = 1
    let mut den = Element::new(Group::Zr, pairing);
    den.set1();

    for jj in s.iter() {
        // num *= 0 - jj.clone() - 1;;
        let mut num_t = Element::new(Group::Zr, pairing);
        num_t.set_si(0 - jj.clone() - 1);
        num.mul(&num_t);
        let mut den_t = Element::new(Group::Zr, pairing);
        // den *= j.clone() - jj.clone();;
        den_t.set_si(j.clone() - jj.clone());
        den.mul(&den_t);
    }

    num.div(&den);
    num
}
