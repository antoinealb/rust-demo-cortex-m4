#![allow(dead_code)]

extern {
    fn SysTickEnable();
    fn SysTickDisable();
    fn SysTickIntRegister(handler: *const u32);
    fn SysTickIntUnregister();
    fn SysTickIntEnable();
    fn SysTickIntDisable();
    fn SysTickPeriodSet(period: u32);
    fn SysTickPeriodGet() -> u32;
    fn SysTickValueGet() -> u32;
}

pub fn init() {
    unsafe {
        SysTickEnable();
    }
}

pub fn set_period_us(period: u32) {
    unsafe {
        SysTickPeriodSet(period);
    }
}

pub fn set_handler(handler: fn()) {
    unsafe {
        SysTickIntRegister(handler as *const u32);
    }
}

pub fn start() {
    unsafe {
        SysTickIntEnable();
    }
}

