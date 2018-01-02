#![feature(lang_items)]
#![no_std]

pub extern fn main() {
    loop {}
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() {}
