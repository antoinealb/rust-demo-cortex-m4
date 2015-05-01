#![allow(dead_code)]
use sysctl;
use gpio;
use gpio::Pin;

pub const RED: Pin = Pin::Pin1;
pub const BLUE: Pin = Pin::Pin2;


pub fn init() {
    sysctl::peripheral_enable(sysctl::Peripheral::GPIOF);
    gpio::make_output(gpio::Port::PortF, RED);
    gpio::make_output(gpio::Port::PortF, BLUE);
}

pub fn set_red(state: bool) {
    gpio::write(gpio::Port::PortF, RED, state);
}

pub fn set_blue(state: bool) {
    gpio::write(gpio::Port::PortF, BLUE, state);
}
