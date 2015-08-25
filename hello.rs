#![feature(no_std)]
#![feature(core)]
#![feature(lang_items)]
#![no_std]

#![crate_type="staticlib"]

extern crate core;

mod runtime;

extern {
    fn led_toggle() -> ();
    fn os_thread_sleep_ms(ms: u32) -> ();
}

#[no_mangle]
pub fn test_add(a: u32, b: u32) -> u32 {
    return a + b;
}

#[no_mangle]
pub fn rust_thread(arg: u32) -> () {
    loop {
        unsafe {
            led_toggle();
            os_thread_sleep_ms(500);
        }
    }
}
