use types::c_types as c_types;

use optee;

pub fn CreateEntryPoint() -> optee::TEE_Result {
    log_debug!(file!(), "has been called");

    return optee::TEE_SUCCESS;
}

pub fn DestroyEntryPoint() {
    log_debug!(file!(), "has been destroyed");

}

pub fn OpenSessionEntryPoint(_paramTypes: u32, _params: *mut optee::TEE_Param,
                                           _sessionContext: *mut *mut c_types::c_void)
     -> optee::TEE_Result {
    log_debug!(file!(), "has been opened");
    return optee::TEE_SUCCESS;
}

pub fn CloseSessionEntryPoint(_sessionContext: *mut c_types::c_void) {
    log_debug!(file!(), "has been closed");
}

pub fn InvokeCommandEntryPoint(_sessionContext: *mut c_types::c_void,
                                             _commandID: u32, _paramTypes: u32,
                                             params: &mut [optee::TEE_Param; 4]) -> optee::TEE_Result {
    log_debug!(file!(), "has been invoked");

    unsafe {params[0].value.as_mut().a = 1234} ;
    return optee::TEE_SUCCESS;
}
