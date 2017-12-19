#![feature(lang_items, asm)]
#![crate_type = "staticlib"]
#![no_std]

#[link(name = "libpayload", kind = "static")]
extern {
    fn putchar(c: u32);
}

#[no_mangle]
pub extern fn main() {
    loop{
        unsafe { putchar(65) };
    }
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() {}

