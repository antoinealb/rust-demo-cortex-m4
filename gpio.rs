#![allow(dead_code)]

pub enum Port {
    PortF = 0x40025000,
}

pub enum Pin {
    Pin0 = (1 << 0),
    Pin1 = (1 << 1),
    Pin2 = (1 << 2),
    Pin3 = (1 << 3),
    Pin4 = (1 << 4),
    Pin5 = (1 << 5),
    Pin6 = (1 << 6),
    Pin7 = (1 << 7),
}

extern {
    fn GPIOPinTypeGPIOOutput(base: *const u32, mask: u32);
    fn GPIOPinWrite(base: *const u32, mask: u32, value: u32);
}

pub fn make_output(port: Port, pin: Pin) {
    let mask = pin as u32;
    let base = port as u32;
    unsafe {
        GPIOPinTypeGPIOOutput(base as *const u32, mask);
    }
}

pub fn write(port: Port, pin: Pin, value: bool) {
    let base = port as u32;
    let shifted_val = pin as u32;
    unsafe {
        if value {
            GPIOPinWrite(base as *const u32, shifted_val, shifted_val);
        } else {
            GPIOPinWrite(base as *const u32, shifted_val, 0);
        }
    }
}
