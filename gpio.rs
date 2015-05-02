#![allow(dead_code)]

pub enum Port {
    PortF = 0x40025000,
}

bitflags! {
    flags Pin: u32 {
        const PIN0 = (1 << 0),
        const PIN1 = (1 << 1),
        const PIN2 = (1 << 2),
        const PIN3 = (1 << 3),
        const PIN4 = (1 << 4),
        const PIN5 = (1 << 5),
        const PIN6 = (1 << 6),
        const PIN7 = (1 << 7),
    }
}

extern {
    fn GPIOPinTypeGPIOOutput(base: *const u32, mask: u32);
    fn GPIOPinWrite(base: *const u32, mask: u32, value: u32);
}

pub fn make_output(port: Port, pin: Pin) {
    let mask = pin.bits();
    let base = port as u32;
    unsafe {
        GPIOPinTypeGPIOOutput(base as *const u32, mask);
    }
}

pub fn write(port: Port, pin: Pin, value: bool) {
    let base = port as u32;
    let shifted_val: u32 = pin.bits();
    let output_val = if value { pin.bits() } else { 0 };
    unsafe {
        GPIOPinWrite(base as *const u32, shifted_val, output_val);
    }
}
