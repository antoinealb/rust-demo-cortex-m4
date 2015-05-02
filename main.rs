#![feature(no_std)]
#![feature(core)]
#![feature(lang_items)]
#![no_std]

#![crate_type="staticlib"]

extern crate core;

mod bitflags;
mod runtime;
mod sysctl;
mod led_driver;
mod gpio;
mod systick;

fn clock_init() {
    let clock_config = sysctl::SYSCTL_SYSDIV_2_5 + sysctl::SYSCTL_USE_PLL +
                       sysctl::SYSCTL_XTAL_16MHZ + sysctl::SYSCTL_OSC_MAIN;
    sysctl::clock_set(clock_config);
}

fn handler() {
}


#[no_mangle] pub fn main()
{
    clock_init();
    led_driver::init();
    systick::init();

    systick::set_period_us(10 * 1000);
    systick::set_handler(handler);
    systick::start();

    loop {
        let mut i = 0;
        while i < 1000000 {
            i += 1;
            led_driver::set_red(false);
        }

        i = 0;

        while i < 1000000 {
            i += 1;
            led_driver::set_red(true);
        }
    }
}

