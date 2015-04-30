#![feature(no_std)]
#![feature(core)]
#![feature(lang_items)]
#![no_std]

#![crate_type="staticlib"]

extern crate core;
mod sysctl;
mod runtime;



#[no_mangle] pub fn main()
{
    let clock_config = sysctl::SYSCTL_SYSDIV_2_5 + sysctl::SYSCTL_USE_PLL +
                       sysctl::SYSCTL_XTAL_16MHZ + sysctl::SYSCTL_OSC_MAIN;
    sysctl::clock_set(clock_config);

}
