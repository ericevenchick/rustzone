#![allow(dead_code)]
use types::c_types as c_types;

// include optee bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

macro_rules! ta_print {
    ($msg:expr) => {
        unsafe {
            optee::printf("TA: %s\n\0".as_ptr() as *const c_types::c_char, 
            concat!($msg, "\0").as_ptr() as *const c_types::c_char);
        }
    }
}
