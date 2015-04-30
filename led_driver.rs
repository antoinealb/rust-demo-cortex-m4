use sysctl;
use gpio;

pub const RED: u32 = gpio::PIN1;
pub const BLUE: u32 = gpio::PIN2;
pub const GREEN: u32 = gpio::PIN3;


pub fn led_init()
{
    sysctl::peripheral_enable(sysctl::SYSCTL_PERIPH_GPIOF);
    gpio::make_output(gpio::PORTF_BASE, RED + BLUE + GREEN);
}

pub fn set_red(state: bool) {
    let value = if state { RED } else { 0 };
    gpio::write(gpio::PORTF_BASE, RED, value);
}

pub fn set_blue(state: bool) {
    let value = if state { BLUE } else { 0 };
    gpio::write(gpio::PORTF_BASE, BLUE, value);
}

