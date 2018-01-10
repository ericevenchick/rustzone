#![feature(lang_items, core)]
#![no_std]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

macro_rules! log_debug {
    ($fname:expr, $msg:expr) => {
        unsafe {
            trace_printf($fname.as_ptr() as *const c_types::c_char, 
                         line!() as c_types::c_int, 
                         TRACE_DEBUG as c_types::c_int, 
                         true, 
                         $msg.as_ptr() as *const c_types::c_char);
        }
    }
}

#[no_mangle]
pub extern "C" fn TA_CreateEntryPoint() -> TEE_Result {
    log_debug!(file!(), "has been called");

    return TEE_SUCCESS;
}

#[no_mangle]
pub extern "C" fn TA_DestroyEntryPoint() {
    log_debug!(file!(), "has been destroyed");

}

#[no_mangle]
pub extern "C" fn TA_OpenSessionEntryPoint(_paramTypes: u32, _params: *mut TEE_Param,
                                           _sessionContext: *mut *mut c_types::c_void)
     -> TEE_Result {
    log_debug!(file!(), "has been opened");
    return TEE_SUCCESS;
}

#[no_mangle]
pub extern "C" fn TA_CloseSessionEntryPoint(_sessionContext: *mut c_types::c_void) {
    log_debug!(file!(), "has been closed");
}

#[no_mangle]
pub extern "C" fn TA_InvokeCommandEntryPoint(_sessionContext: *mut c_types::c_void,
                                             _commandID: u32, _paramTypes: u32,
                                             mut params: [TEE_Param; 4]) -> TEE_Result {
    log_debug!(file!(), "has been invoked");

    unsafe {params[0].value.as_mut().a = 1337} ;
    return TEE_SUCCESS;
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() {}

mod c_types {
    pub type c_char = i8;
    pub type c_schar = i8;
    pub type c_uchar = u8;
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_long = i32;
    pub type c_ulong = u64;
    pub type c_longlong = i64;
    pub type c_ulonglong = u64;

    // unused
    //pub type c_float = f32;
    //pub type c_double = f64;
    
    pub enum c_void {}
}

#[no_mangle]
pub extern "C" fn __muloti4() {}
#[no_mangle]
pub extern "C" fn __umodti3() {}
#[no_mangle]
pub extern "C" fn __udivti3() {}
