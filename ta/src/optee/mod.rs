#![allow(dead_code)]
use types::c_types as c_types;

// include optee bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

macro_rules! log_debug {
    ($fname:expr, $msg:expr) => {
        unsafe {
            optee::trace_printf($fname.as_ptr() as *const c_types::c_char, 
                        line!() as c_types::c_int, 
                        optee::TRACE_DEBUG as c_types::c_int, 
                        true, 
                        $msg.as_ptr() as *const c_types::c_char);
        }
    }
}
