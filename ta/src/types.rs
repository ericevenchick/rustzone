// define our types for interfacing with C
pub mod c_types {
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
