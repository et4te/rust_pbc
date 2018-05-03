use element::{Element, Element_t};
use integer::{Integer, Integer_t};

// Rust

pub struct ElementPP {
    _ptr: *mut ElementPP_t,
}

impl Drop for ElementPP {
    fn drop(&mut self) {
        unsafe {
            _element_pp_clear(self._ptr);
            _element_pp_free(self._ptr);
        }
    }
}

impl ElementPP {
    pub fn new() -> ElementPP {
        ElementPP {
            _ptr: unsafe { _element_pp_new() },
        }
    }

    pub fn init(&mut self, base: &Element) {
        unsafe {
            _element_pp_init(self._ptr, base.as_mut_ptr());
        }
    }

    // element = base ^ pow
    pub fn pow(&mut self, dst: &mut Element, pow: &Integer) {
        unsafe {
            _element_pp_pow(dst.as_mut_ptr(), pow.as_ptr(), self._ptr);
        }
    }

    pub fn pow_zn(&mut self, dst: &mut Element, pow: &Element) {
        unsafe {
            _element_pp_pow_zn(dst.as_mut_ptr(), pow.as_ptr(), self._ptr);
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            _element_pp_clear(self._ptr);
        }
    }
}

// C

#[link(name = "pbc_binding")]
extern "C" {
    fn _element_pp_new() -> *mut ElementPP_t;
    fn _element_pp_free(pp: *mut ElementPP_t) -> ();
    fn _element_pp_init(pp: *mut ElementPP_t, e: *const Element_t) -> ();
    fn _element_pp_pow(e: *mut Element_t, si: *const Integer_t, pp: *mut ElementPP_t) -> ();
    fn _element_pp_pow_zn(dst: *mut Element_t, pow: *const Element_t, pp: *mut ElementPP_t) -> ();
    fn _element_pp_clear(e: *mut ElementPP_t) -> ();
}

#[repr(C)]
struct ElementPP_t {
    _void: [u8; 0],
}
