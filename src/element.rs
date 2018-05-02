use group::Group;
use integer::{Integer, Integer_t};
use libc::{c_char, c_int, c_long, c_uchar};
use pairing::{Pairing, Pairing_t};
use sha2::{Digest, Sha256};
use std::ffi::CString;

// Rust

pub struct Element {
    pub group: Group,
    _ptr: *mut Element_t,
}

impl Drop for Element {
    fn drop(&mut self) {
        unsafe {
            _element_free(self._ptr);
        }
    }
}

impl Element {
    pub fn new(g: Group, p: &Pairing) -> Element {
        let mut e = Element {
            group: g.clone(),
            _ptr: unsafe { _element_new() },
        };
        match g {
            Group::Zr => {
                e.init_zr(p);
                e
            }
            Group::G1 => {
                e.init_g1(p);
                e
            }
            Group::G2 => {
                e.init_g2(p);
                e
            }
            Group::GT => {
                e.init_gt(p);
                e
            }
        }
    }

    // Always decode element in G1
    pub fn from_bytes(pairing: &Pairing, mut data: Vec<u8>) -> Element {
        let element = Element::new(Group::G1, pairing);
        unsafe {
            let _nbytes = _element_from_bytes(element._ptr, data.as_mut_ptr());
        }
        element
    }

    pub fn as_ptr(&self) -> *const Element_t {
        self._ptr
    }

    pub fn as_mut_ptr(&self) -> *mut Element_t {
        self._ptr
    }

    // Initialisation

    pub fn init_g1(&mut self, pairing: &Pairing) {
        unsafe {
            _element_init_g1(self._ptr, pairing.as_ptr());
        }
    }

    pub fn init_g2(&mut self, pairing: &Pairing) {
        unsafe {
            _element_init_g2(self._ptr, pairing.as_ptr());
        }
    }

    pub fn init_gt(&mut self, pairing: &Pairing) {
        unsafe {
            _element_init_gt(self._ptr, pairing.as_ptr());
        }
    }

    pub fn init_zr(&mut self, pairing: &Pairing) {
        unsafe {
            _element_init_zr(self._ptr, pairing.as_ptr());
        }
    }

    pub fn set(&mut self, rhs: &Element) {
        unsafe {
            _element_set(self._ptr, rhs._ptr);
        }
    }

    pub fn set0(&mut self) {
        unsafe {
            _element_set0(self._ptr);
        }
    }

    pub fn set1(&mut self) {
        unsafe {
            _element_set1(self._ptr);
        }
    }

    pub fn set_si(&mut self, si: i64) {
        unsafe {
            _element_set_si(self._ptr, si as c_long);
        }
    }

    // Pairing

    // TODO assert! self must be in G1 and rhs must be in G2, result is in GT
    pub fn pair(&self, pairing: &Pairing, rhs: &Element) -> Element {
        let element = Element::new(Group::GT, pairing);
        unsafe {
            _element_pairing(element._ptr, self._ptr, rhs._ptr);
        }
        element
    }

    // Arithmetic Operations

    pub fn add(&mut self, rhs: &Element) {
        unsafe {
            _element_add(self._ptr, self._ptr, rhs._ptr);
        }
    }

    pub fn sub(&mut self, rhs: &Element) {
        unsafe {
            _element_sub(self._ptr, self._ptr, rhs._ptr);
        }
    }

    pub fn mul(&mut self, rhs: &Element) {
        unsafe {
            _element_mul(self._ptr, self._ptr, rhs._ptr);
        }
    }

    pub fn mul_si(&mut self, rhs: i64) {
        unsafe {
            _element_mul_si(self._ptr, self._ptr, rhs as c_long);
        }
    }

    pub fn mul_zn(&mut self, rhs: &Element) {
        unsafe {
            _element_mul_zn(self._ptr, self._ptr, rhs._ptr);
        }
    }

    pub fn div(&mut self, rhs: &Element) {
        unsafe {
            _element_div(self._ptr, self._ptr, rhs._ptr);
        }
    }

    pub fn double(&mut self, rhs: &Element) {
        unsafe {
            _element_double(self._ptr, rhs._ptr);
        }
    }

    pub fn halve(&mut self, rhs: &Element) {
        unsafe {
            _element_halve(self._ptr, rhs._ptr);
        }
    }

    pub fn square(&mut self, rhs: &Element) {
        unsafe {
            _element_square(self._ptr, rhs._ptr);
        }
    }

    pub fn neg(&mut self, rhs: &Element) {
        unsafe {
            _element_neg(self._ptr, rhs._ptr);
        }
    }

    pub fn invert(&mut self, rhs: &Element) {
        unsafe {
            _element_invert(self._ptr, rhs._ptr);
        }
    }

    pub fn to_integer(&self) -> Integer {
        let mut i = Integer::new();
        unsafe {
            _element_to_integer(i.as_mut_ptr(), self._ptr);
        }
        i
    }

    // Comparison

    pub fn cmp(&self, rhs: &Element) -> c_int {
        unsafe { _element_cmp(self._ptr, rhs._ptr) }
    }

    // Random

    pub fn random(&mut self) {
        unsafe {
            _element_random(self._ptr);
        }
    }

    // Hashing

    pub fn set_from_hash(&mut self, h: Vec<u8>) {
        unsafe {
            _element_from_hash(self._ptr, h.as_ptr(), h.len() as c_int);
        }
    }

    pub fn hash_g(&self) -> Vec<u8> {
        let element_bytes = self.to_bytes();
        let mut hasher = Sha256::default();
        hasher.input(&element_bytes);
        hasher.result().to_vec()
    }

    pub fn hash_h(&self, pairing: &Pairing, bytes: Vec<u8>) -> Element {
        assert_eq!(bytes.len(), 32);

        let element_bytes = self.to_bytes();
        let combined_bytes = [element_bytes, bytes].concat();

        let mut h = Element::new(Group::G2, pairing);
        h.set_from_hash(combined_bytes);
        h
    }

    // Serialisation

    pub fn to_bytes(&self) -> Vec<u8> {
        unsafe {
            let nbytes = _element_length_in_bytes(self._ptr);
            let mut c_vec = vec![0u8; nbytes as usize];
            let n = _element_to_bytes(c_vec.as_mut_ptr(), self._ptr);
            assert_eq!(n, nbytes);
            c_vec
        }
    }

    // Printing

    pub fn print(&self, prefix: String) {
        let c_str = CString::new(format!("{}%B\n", prefix)).unwrap();
        unsafe {
            _element_printf(c_str.as_ptr(), self._ptr);
        }
    }
}

// C

#[link(name = "pbc_binding")]
extern "C" {
    // Initialisation

    fn _element_new() -> *mut Element_t;
    fn _element_free(e: *mut Element_t) -> ();
    fn _element_init_g1(e: *mut Element_t, p: *mut Pairing_t) -> ();
    fn _element_init_g2(e: *mut Element_t, p: *mut Pairing_t) -> ();
    fn _element_init_gt(e: *mut Element_t, p: *mut Pairing_t) -> ();
    fn _element_init_zr(e: *mut Element_t, p: *mut Pairing_t) -> ();
    fn _element_set(dst: *mut Element_t, src: *mut Element_t) -> ();
    fn _element_set0(e: *mut Element_t) -> ();
    fn _element_set1(e: *mut Element_t) -> ();
    fn _element_set_si(e: *mut Element_t, si: c_long) -> ();

    // Pairing

    fn _element_pairing(out_gt: *mut Element_t, in_g1: *mut Element_t, in_g2: *mut Element_t)
        -> ();

    // Arithmetic Operations

    fn _element_add(e0: *mut Element_t, e1: *mut Element_t, e2: *mut Element_t) -> ();
    fn _element_sub(e0: *mut Element_t, e1: *mut Element_t, e2: *mut Element_t) -> ();
    fn _element_mul(e0: *mut Element_t, e1: *mut Element_t, e2: *mut Element_t) -> ();
    fn _element_mul_si(e0: *mut Element_t, e1: *mut Element_t, si: c_long) -> ();
    fn _element_mul_zn(e0: *mut Element_t, e1: *mut Element_t, e2: *mut Element_t) -> ();
    fn _element_div(e0: *mut Element_t, e1: *mut Element_t, e2: *mut Element_t) -> ();
    fn _element_double(e0: *mut Element_t, e1: *mut Element_t) -> ();
    fn _element_halve(e0: *mut Element_t, e1: *mut Element_t) -> ();
    fn _element_square(e0: *mut Element_t, e1: *mut Element_t) -> ();
    fn _element_neg(e0: *mut Element_t, e1: *mut Element_t) -> ();
    fn _element_invert(e0: *mut Element_t, e1: *mut Element_t) -> ();
    fn _element_cmp(e0: *mut Element_t, e1: *mut Element_t) -> c_int;

    // Hashing

    fn _element_from_hash(e: *mut Element_t, data: *const u8, len: c_int) -> ();
    fn _element_to_integer(i: *mut Integer_t, e: *mut Element_t) -> ();

    // Serialisation

    fn _element_to_bytes(s: *mut c_uchar, e: *mut Element_t) -> c_int;
    fn _element_from_bytes(e: *mut Element_t, data: *mut c_uchar) -> c_int;
    fn _element_length_in_bytes(e: *mut Element_t) -> c_int;

    // RNG

    fn _element_random(e: *mut Element_t) -> ();

    // Printing

    fn _element_printf(s: *const c_char, e: *mut Element_t) -> ();
}

#[repr(C)]
pub struct Element_t {
    _void: [u8; 0],
}
