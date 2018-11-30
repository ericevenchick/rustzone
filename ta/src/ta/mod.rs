use types::c_types as c_types;

use optee;

pub fn CreateEntryPoint() -> optee::TEE_Result {
    ta_print!("Rust TA CreateEntryPoint");
    return optee::TEE_SUCCESS;
}

pub fn DestroyEntryPoint() {
    ta_print!("Rust TA DestroyEntryPoint");
}

pub fn OpenSessionEntryPoint(_paramTypes: u32, _params: *mut optee::TEE_Param,
                                           _sessionContext: *mut *mut c_types::c_void)
     -> optee::TEE_Result {
    ta_print!("Rust TA OpenSessionEntryPoint");
    return optee::TEE_SUCCESS;
}

pub fn CloseSessionEntryPoint(_sessionContext: *mut c_types::c_void) {
    ta_print!("Rust TA CloseSessionEntryPoint");
}

pub fn InvokeCommandEntryPoint(_sessionContext: *mut c_types::c_void,
                                             commandID: u32, _paramTypes: u32,
                                             params: &mut [optee::TEE_Param; 4]) -> optee::TEE_Result {
    ta_print!("Rust TA InvokeCommandEntryPoint");

    match commandID {
        0 => {
            unsafe {params[0].value.a += 1};
            ta_print!("Incremented Value from Rust!");
        },
        1 => {
            unsafe {params[0].value.a -= 1};
            ta_print!("Decremented Value from Rust!");
        },
        _ => {
            return optee::TEE_ERROR_BAD_PARAMETERS;
        }
    }

    return optee::TEE_SUCCESS;
}
