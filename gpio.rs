#![allow(dead_code)]
pub const PORTF_BASE: *const u32 = 0x40025000 as (*const u32);

pub const PIN0: u32 = (1 << 0);
pub const PIN1: u32 = (1 << 1);
pub const PIN2: u32 = (1 << 2);
pub const PIN3: u32 = (1 << 3);
pub const PIN4: u32 = (1 << 4);
pub const PIN5: u32 = (1 << 5);
pub const PIN6: u32 = (1 << 6);
pub const PIN7: u32 = (1 << 7);

extern {
    fn GPIOPinTypeGPIOOutput(base: *const u32, mask: u32);
    fn GPIOPinWrite(base: *const u32, mask: u32, value: u32);
}

pub fn make_output(base: *const u32, mask: u32) {
    unsafe {
        GPIOPinTypeGPIOOutput(base, mask);
    }
}

pub fn write(base: *const u32, mask: u32, value: u32) {
    unsafe {
        GPIOPinWrite(base, mask, value);
    }
}
