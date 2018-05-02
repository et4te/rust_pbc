use libc::{c_long, c_ulong};

// Rust

pub struct Integer {
    _ptr: *mut Integer_t,
}

impl Drop for Integer {
    fn drop(&mut self) {
        unsafe {
            _integer_clear(self._ptr);
            _integer_free(self._ptr);
        }
    }
}

impl Integer {
    pub fn new() -> Integer {
        Integer {
            _ptr: unsafe { _integer_new() },
        }
    }

    pub fn as_ptr(&self) -> *const Integer_t {
        self._ptr
    }

    pub fn as_mut_ptr(&mut self) -> *mut Integer_t {
        self._ptr
    }

    pub fn init(&mut self) {
        unsafe {
            _integer_init(self._ptr);
        }
    }

    pub fn get_ui(&self) -> u64 {
        unsafe { _integer_get_ui(self._ptr) as u64 }
    }

    pub fn get_si(&self) -> i64 {
        unsafe { _integer_get_si(self._ptr) as i64 }
    }

    pub fn set_si(&mut self, si: i64) {
        unsafe {
            _integer_set_si(self._ptr, si as c_long);
        }
    }
}

// C

#[link(name = "pbc_binding")]
extern "C" {
    fn _integer_new() -> *mut Integer_t;
    fn _integer_free(i: *mut Integer_t) -> ();
    fn _integer_init(i: *mut Integer_t) -> ();
    fn _integer_clear(i: *mut Integer_t) -> ();
    fn _integer_get_ui(i: *mut Integer_t) -> c_ulong;
    fn _integer_get_si(i: *mut Integer_t) -> c_long;
    fn _integer_set_si(i: *mut Integer_t, si: c_long) -> ();
}

#[repr(C)]
pub struct Integer_t {
    _void: [u8; 0],
}
