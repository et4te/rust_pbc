
use std::ffi::CString;
use libc::c_char;

// Rust

pub struct Pairing {
    _ptr: *mut Pairing_t,
}

impl Drop for Pairing {
    fn drop(&mut self) {
     	unsafe {
	    _pairing_free(self._ptr);
	}
    }
}

impl Pairing {
    pub fn new() -> Pairing {
     	Pairing { _ptr: unsafe { _pairing_new() } }
    }
    
    pub fn init(&self, param: String) {
        let param = CString::new(param).unwrap();
	unsafe {
	    _pairing_init(self._ptr, param.as_ptr())
	}
    }
    
    pub fn as_ptr(&self) -> *mut Pairing_t {
     	self._ptr     	      	 
    }
}

// C

#[link(name = "pbc_binding")]
extern {
    fn _pairing_new() -> *mut Pairing_t;
    fn _pairing_free(p: *mut Pairing_t) -> ();
    fn _pairing_init(p: *mut Pairing_t, param: *const c_char) -> ();
}

#[repr(C)]
pub struct Pairing_t {
    _void: [u8; 0],
}

