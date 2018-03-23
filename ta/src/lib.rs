#![feature(lang_items, core)]
#![no_std]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod types;
use types::c_types as c_types;

#[macro_use]
mod optee;
mod ta;

// define functions required for no_std
#[lang = "eh_personality"]
extern fn eh_personality() {}
#[lang = "panic_fmt"]
extern fn panic_fmt() {}

// define ARM floating point functions to prevent linker error
#[no_mangle]
pub extern "C" fn __muloti4() {}
#[no_mangle]
pub extern "C" fn __umodti3() {}
#[no_mangle]
pub extern "C" fn __udivti3() {}

// define the five EntryPoint functions called by OPTEE
#[no_mangle]
pub extern "C" fn TA_CreateEntryPoint() -> optee::TEE_Result {
    return ta::CreateEntryPoint();
}

#[no_mangle]
pub extern "C" fn TA_DestroyEntryPoint() {
    ta::DestroyEntryPoint();
}

#[no_mangle]
pub extern "C" fn TA_OpenSessionEntryPoint(paramTypes: u32, params: *mut optee::TEE_Param,
                                           sessionContext: *mut *mut c_types::c_void)
     -> optee::TEE_Result {
    return ta::OpenSessionEntryPoint(paramTypes, params, sessionContext);
}

#[no_mangle]
pub extern "C" fn TA_CloseSessionEntryPoint(sessionContext: *mut c_types::c_void) {
    ta::CloseSessionEntryPoint(sessionContext);
}

#[no_mangle]
pub extern "C" fn TA_InvokeCommandEntryPoint(sessionContext: *mut c_types::c_void,
                                             commandID: u32, paramTypes: u32,
                                             mut params: &mut [optee::TEE_Param; 4]) -> optee::TEE_Result {
    return ta::InvokeCommandEntryPoint(sessionContext, commandID, paramTypes, &mut params);
}
