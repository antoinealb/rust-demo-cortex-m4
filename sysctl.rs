
#![allow(dead_code)]

extern crate core;

//*****************************************************************************
//
// The following are values that can be passed to the
// SysCtlPeripheralPresent(), SysCtlPeripheralEnable(),
// SysCtlPeripheralDisable(), and SysCtlPeripheralReset() APIs as the
// uu32Peripheral parameter.  The peripherals in the fourth group (upper nibble
// is 3) can only be used with the SysCtlPeripheralPresent() API.
//
//*****************************************************************************
pub const SYSCTL_PERIPH_ADC0      : u32 = 0xf0003800;  // ADC 0
pub const SYSCTL_PERIPH_ADC1      : u32 = 0xf0003801;  // ADC 1
pub const SYSCTL_PERIPH_CAN0      : u32 = 0xf0003400;  // CAN 0
pub const SYSCTL_PERIPH_CAN1      : u32 = 0xf0003401;  // CAN 1
pub const SYSCTL_PERIPH_COMP0     : u32 = 0xf0003c00;  // Analog Comparator Module 0
pub const SYSCTL_PERIPH_EMAC0     : u32 = 0xf0009c00;  // Ethernet MAC0
pub const SYSCTL_PERIPH_EPHY0     : u32 = 0xf0003000;  // Ethernet PHY0
pub const SYSCTL_PERIPH_EPI0      : u32 = 0xf0001000;  // EPI0
pub const SYSCTL_PERIPH_GPIOA     : u32 = 0xf0000800;  // GPIO A
pub const SYSCTL_PERIPH_GPIOB     : u32 = 0xf0000801;  // GPIO B
pub const SYSCTL_PERIPH_GPIOC     : u32 = 0xf0000802;  // GPIO C
pub const SYSCTL_PERIPH_GPIOD     : u32 = 0xf0000803;  // GPIO D
pub const SYSCTL_PERIPH_GPIOE     : u32 = 0xf0000804;  // GPIO E
pub const SYSCTL_PERIPH_GPIOF     : u32 = 0xf0000805;  // GPIO F
pub const SYSCTL_PERIPH_GPIOG     : u32 = 0xf0000806;  // GPIO G
pub const SYSCTL_PERIPH_GPIOH     : u32 = 0xf0000807;  // GPIO H
pub const SYSCTL_PERIPH_GPIOJ     : u32 = 0xf0000808;  // GPIO J
pub const SYSCTL_PERIPH_HIBERNATE : u32 = 0xf0001400;  // Hibernation module
pub const SYSCTL_PERIPH_CCM0      : u32 = 0xf0007400;  // CCM 0
pub const SYSCTL_PERIPH_EEPROM0   : u32 = 0xf0005800;  // EEPROM 0
pub const SYSCTL_PERIPH_FAN0      : u32 = 0xf0005400;  // FAN 0
pub const SYSCTL_PERIPH_FAN1      : u32 = 0xf0005401;  // FAN 1
pub const SYSCTL_PERIPH_GPIOK     : u32 = 0xf0000809;  // GPIO K
pub const SYSCTL_PERIPH_GPIOL     : u32 = 0xf000080a;  // GPIO L
pub const SYSCTL_PERIPH_GPIOM     : u32 = 0xf000080b;  // GPIO M
pub const SYSCTL_PERIPH_GPION     : u32 = 0xf000080c;  // GPIO N
pub const SYSCTL_PERIPH_GPIOP     : u32 = 0xf000080d;  // GPIO P
pub const SYSCTL_PERIPH_GPIOQ     : u32 = 0xf000080e;  // GPIO Q
pub const SYSCTL_PERIPH_GPIOR     : u32 = 0xf000080f;  // GPIO R
pub const SYSCTL_PERIPH_GPIOS     : u32 = 0xf0000810;  // GPIO S
pub const SYSCTL_PERIPH_GPIOT     : u32 = 0xf0000811;  // GPIO T
pub const SYSCTL_PERIPH_I2C0      : u32 = 0xf0002000;  // I2C 0
pub const SYSCTL_PERIPH_I2C1      : u32 = 0xf0002001;  // I2C 1
pub const SYSCTL_PERIPH_I2C2      : u32 = 0xf0002002;  // I2C 2
pub const SYSCTL_PERIPH_I2C3      : u32 = 0xf0002003;  // I2C 3
pub const SYSCTL_PERIPH_I2C4      : u32 = 0xf0002004;  // I2C 4
pub const SYSCTL_PERIPH_I2C5      : u32 = 0xf0002005;  // I2C 5
pub const SYSCTL_PERIPH_I2C6      : u32 = 0xf0002006;  // I2C 6
pub const SYSCTL_PERIPH_I2C7      : u32 = 0xf0002007;  // I2C 7
pub const SYSCTL_PERIPH_I2C8      : u32 = 0xf0002008;  // I2C 8
pub const SYSCTL_PERIPH_I2C9      : u32 = 0xf0002009;  // I2C 9
pub const SYSCTL_PERIPH_LCD0      : u32 = 0xf0009000;  // LCD 0
pub const SYSCTL_PERIPH_ONEWIRE0  : u32 = 0xf0009800;  // One Wire 0
pub const SYSCTL_PERIPH_PWM0      : u32 = 0xf0004000;  // PWM 0
pub const SYSCTL_PERIPH_PWM1      : u32 = 0xf0004001;  // PWM 1
pub const SYSCTL_PERIPH_QEI0      : u32 = 0xf0004400;  // QEI 0
pub const SYSCTL_PERIPH_QEI1      : u32 = 0xf0004401;  // QEI 1
pub const SYSCTL_PERIPH_SSI0      : u32 = 0xf0001c00;  // SSI 0
pub const SYSCTL_PERIPH_SSI1      : u32 = 0xf0001c01;  // SSI 1
pub const SYSCTL_PERIPH_SSI2      : u32 = 0xf0001c02;  // SSI 2
pub const SYSCTL_PERIPH_SSI3      : u32 = 0xf0001c03;  // SSI 3
pub const SYSCTL_PERIPH_TIMER0    : u32 = 0xf0000400;  // Timer 0
pub const SYSCTL_PERIPH_TIMER1    : u32 = 0xf0000401;  // Timer 1
pub const SYSCTL_PERIPH_TIMER2    : u32 = 0xf0000402;  // Timer 2
pub const SYSCTL_PERIPH_TIMER3    : u32 = 0xf0000403;  // Timer 3
pub const SYSCTL_PERIPH_TIMER4    : u32 = 0xf0000404;  // Timer 4
pub const SYSCTL_PERIPH_TIMER5    : u32 = 0xf0000405;  // Timer 5
pub const SYSCTL_PERIPH_TIMER6    : u32 = 0xf0000406;  // Timer 6
pub const SYSCTL_PERIPH_TIMER7    : u32 = 0xf0000407;  // Timer 7
pub const SYSCTL_PERIPH_UART0     : u32 = 0xf0001800;  // UART 0
pub const SYSCTL_PERIPH_UART1     : u32 = 0xf0001801;  // UART 1
pub const SYSCTL_PERIPH_UART2     : u32 = 0xf0001802;  // UART 2
pub const SYSCTL_PERIPH_UART3     : u32 = 0xf0001803;  // UART 3
pub const SYSCTL_PERIPH_UART4     : u32 = 0xf0001804;  // UART 4
pub const SYSCTL_PERIPH_UART5     : u32 = 0xf0001805;  // UART 5
pub const SYSCTL_PERIPH_UART6     : u32 = 0xf0001806;  // UART 6
pub const SYSCTL_PERIPH_UART7     : u32 = 0xf0001807;  // UART 7
pub const SYSCTL_PERIPH_UDMA      : u32 = 0xf0000c00;  // uDMA
pub const SYSCTL_PERIPH_USB0      : u32 = 0xf0002800;  // USB 0
pub const SYSCTL_PERIPH_WDOG0     : u32 = 0xf0000000;  // Watchdog 0
pub const SYSCTL_PERIPH_WDOG1     : u32 = 0xf0000001;  // Watchdog 1
pub const SYSCTL_PERIPH_WTIMER0   : u32 = 0xf0005c00;  // Wide Timer 0
pub const SYSCTL_PERIPH_WTIMER1   : u32 = 0xf0005c01;  // Wide Timer 1
pub const SYSCTL_PERIPH_WTIMER2   : u32 = 0xf0005c02;  // Wide Timer 2
pub const SYSCTL_PERIPH_WTIMER3   : u32 = 0xf0005c03;  // Wide Timer 3
pub const SYSCTL_PERIPH_WTIMER4   : u32 = 0xf0005c04;  // Wide Timer 4
pub const SYSCTL_PERIPH_WTIMER5   : u32 = 0xf0005c05;  // Wide Timer 5

//*****************************************************************************
//
// The following are values that can be passed to the SysCtlLDOSleepSet() and
// SysCtlLDODeepSleepSet() APIs as the uu32Voltage value, or returned by the
// SysCtlLDOSleepGet() and SysCtlLDODeepSleepGet() APIs.
//
//*****************************************************************************
pub const SYSCTL_LDO_0_90V        : u32 = 0x80000012;  // LDO output of 0.90V
pub const SYSCTL_LDO_0_95V        : u32 = 0x80000013;  // LDO output of 0.95V
pub const SYSCTL_LDO_1_00V        : u32 = 0x80000014;  // LDO output of 1.00V
pub const SYSCTL_LDO_1_05V        : u32 = 0x80000015;  // LDO output of 1.05V
pub const SYSCTL_LDO_1_10V        : u32 = 0x80000016;  // LDO output of 1.10V
pub const SYSCTL_LDO_1_15V        : u32 = 0x80000017;  // LDO output of 1.15V
pub const SYSCTL_LDO_1_20V        : u32 = 0x80000018;  // LDO output of 1.20V

//*****************************************************************************
//
// The following are values that can be passed to the SysCtlIntEnable(),
// SysCtlIntDisable(), and SysCtlIntClear() APIs, or returned in the bit mask
// by the SysCtlIntStatus() API.
//
//*****************************************************************************
pub const SYSCTL_INT_BOR0         : u32 = 0x00000800; // VDD under BOR0
pub const SYSCTL_INT_VDDA_OK      : u32 = 0x00000400; // VDDA Power OK
pub const SYSCTL_INT_MOSC_PUP     : u32 = 0x00000100; // MOSC power-up interrupt
pub const SYSCTL_INT_USBPLL_LOCK  : u32 = 0x00000080; // USB PLL lock interrupt
pub const SYSCTL_INT_PLL_LOCK     : u32 = 0x00000040; // PLL lock interrupt
pub const SYSCTL_INT_MOSC_FAIL    : u32 = 0x00000008; // Main oscillator failure int
pub const SYSCTL_INT_BOR1         : u32 = 0x00000002; // VDD under BOR1
pub const SYSCTL_INT_BOR          : u32 = 0x00000002; // Brown out interrupt

//*****************************************************************************
//
// The following are values that can be passed to the SysCtlResetCauseClear()
// API or returned by the SysCtlResetCauseGet() API.
//
//*****************************************************************************
pub const SYSCTL_CAUSE_HSRVREQ    : u32 = 0x00001000;  // Hardware System Service Request
pub const SYSCTL_CAUSE_HIB        : u32 = 0x00000040;  // Hibernate reset
pub const SYSCTL_CAUSE_WDOG1      : u32 = 0x00000020;  // Watchdog 1 reset
pub const SYSCTL_CAUSE_SW         : u32 = 0x00000010;  // Software reset
pub const SYSCTL_CAUSE_WDOG0      : u32 = 0x00000008;  // Watchdog 0 reset
pub const SYSCTL_CAUSE_BOR        : u32 = 0x00000004;  // Brown-out reset
pub const SYSCTL_CAUSE_POR        : u32 = 0x00000002;  // Power on reset
pub const SYSCTL_CAUSE_EXT        : u32 = 0x00000001;  // External reset

//*****************************************************************************
//
// The following are values that can be passed to the SysCtlBrownOutConfigSet()
// API as the uu32Config parameter.
//
//*****************************************************************************
pub const SYSCTL_BOR_RESET        : u32 = 0x00000002;  // Reset instead of interrupting
pub const SYSCTL_BOR_RESAMPLE     : u32 = 0x00000001;  // Resample BOR before asserting

//*****************************************************************************
//
// The following are values that can be passed to the SysCtlPWMClockSet() API
// as the uu32Config parameter, and can be returned by the SysCtlPWMClockGet()
// API.
//
//*****************************************************************************
pub const SYSCTL_PWMDIV_1         : u32 = 0x00000000;  // PWM clock is processor clock /1
pub const SYSCTL_PWMDIV_2         : u32 = 0x00100000;  // PWM clock is processor clock /2
pub const SYSCTL_PWMDIV_4         : u32 = 0x00120000;  // PWM clock is processor clock /4
pub const SYSCTL_PWMDIV_8         : u32 = 0x00140000;  // PWM clock is processor clock /8
pub const SYSCTL_PWMDIV_16        : u32 = 0x00160000;  // PWM clock is processor clock /16
pub const SYSCTL_PWMDIV_32        : u32 = 0x00180000;  // PWM clock is processor clock /32
pub const SYSCTL_PWMDIV_64        : u32 = 0x001A0000;  // PWM clock is processor clock /64

//*****************************************************************************
//
// The following are values that can be passed to the SysCtlClockSet() API as
// the uu32Config parameter.
//
//*****************************************************************************
pub const SYSCTL_SYSDIV_1         : u32 = 0x07800000;  // Processor clock is osc/pll /1
pub const SYSCTL_SYSDIV_2         : u32 = 0x00C00000;  // Processor clock is osc/pll /2
pub const SYSCTL_SYSDIV_3         : u32 = 0x01400000;  // Processor clock is osc/pll /3
pub const SYSCTL_SYSDIV_4         : u32 = 0x01C00000;  // Processor clock is osc/pll /4
pub const SYSCTL_SYSDIV_5         : u32 = 0x02400000;  // Processor clock is osc/pll /5
pub const SYSCTL_SYSDIV_6         : u32 = 0x02C00000;  // Processor clock is osc/pll /6
pub const SYSCTL_SYSDIV_7         : u32 = 0x03400000;  // Processor clock is osc/pll /7
pub const SYSCTL_SYSDIV_8         : u32 = 0x03C00000;  // Processor clock is osc/pll /8
pub const SYSCTL_SYSDIV_9         : u32 = 0x04400000;  // Processor clock is osc/pll /9
pub const SYSCTL_SYSDIV_10        : u32 = 0x04C00000;  // Processor clock is osc/pll /10
pub const SYSCTL_SYSDIV_11        : u32 = 0x05400000;  // Processor clock is osc/pll /11
pub const SYSCTL_SYSDIV_12        : u32 = 0x05C00000;  // Processor clock is osc/pll /12
pub const SYSCTL_SYSDIV_13        : u32 = 0x06400000;  // Processor clock is osc/pll /13
pub const SYSCTL_SYSDIV_14        : u32 = 0x06C00000;  // Processor clock is osc/pll /14
pub const SYSCTL_SYSDIV_15        : u32 = 0x07400000;  // Processor clock is osc/pll /15
pub const SYSCTL_SYSDIV_16        : u32 = 0x07C00000;  // Processor clock is osc/pll /16
pub const SYSCTL_SYSDIV_17        : u32 = 0x88400000;  // Processor clock is osc/pll /17
pub const SYSCTL_SYSDIV_18        : u32 = 0x88C00000;  // Processor clock is osc/pll /18
pub const SYSCTL_SYSDIV_19        : u32 = 0x89400000;  // Processor clock is osc/pll /19
pub const SYSCTL_SYSDIV_20        : u32 = 0x89C00000;  // Processor clock is osc/pll /20
pub const SYSCTL_SYSDIV_21        : u32 = 0x8A400000;  // Processor clock is osc/pll /21
pub const SYSCTL_SYSDIV_22        : u32 = 0x8AC00000;  // Processor clock is osc/pll /22
pub const SYSCTL_SYSDIV_23        : u32 = 0x8B400000;  // Processor clock is osc/pll /23
pub const SYSCTL_SYSDIV_24        : u32 = 0x8BC00000;  // Processor clock is osc/pll /24
pub const SYSCTL_SYSDIV_25        : u32 = 0x8C400000;  // Processor clock is osc/pll /25
pub const SYSCTL_SYSDIV_26        : u32 = 0x8CC00000;  // Processor clock is osc/pll /26
pub const SYSCTL_SYSDIV_27        : u32 = 0x8D400000;  // Processor clock is osc/pll /27
pub const SYSCTL_SYSDIV_28        : u32 = 0x8DC00000;  // Processor clock is osc/pll /28
pub const SYSCTL_SYSDIV_29        : u32 = 0x8E400000;  // Processor clock is osc/pll /29
pub const SYSCTL_SYSDIV_30        : u32 = 0x8EC00000;  // Processor clock is osc/pll /30
pub const SYSCTL_SYSDIV_31        : u32 = 0x8F400000;  // Processor clock is osc/pll /31
pub const SYSCTL_SYSDIV_32        : u32 = 0x8FC00000;  // Processor clock is osc/pll /32
pub const SYSCTL_SYSDIV_33        : u32 = 0x90400000;  // Processor clock is osc/pll /33
pub const SYSCTL_SYSDIV_34        : u32 = 0x90C00000;  // Processor clock is osc/pll /34
pub const SYSCTL_SYSDIV_35        : u32 = 0x91400000;  // Processor clock is osc/pll /35
pub const SYSCTL_SYSDIV_36        : u32 = 0x91C00000;  // Processor clock is osc/pll /36
pub const SYSCTL_SYSDIV_37        : u32 = 0x92400000;  // Processor clock is osc/pll /37
pub const SYSCTL_SYSDIV_38        : u32 = 0x92C00000;  // Processor clock is osc/pll /38
pub const SYSCTL_SYSDIV_39        : u32 = 0x93400000;  // Processor clock is osc/pll /39
pub const SYSCTL_SYSDIV_40        : u32 = 0x93C00000;  // Processor clock is osc/pll /40
pub const SYSCTL_SYSDIV_41        : u32 = 0x94400000;  // Processor clock is osc/pll /41
pub const SYSCTL_SYSDIV_42        : u32 = 0x94C00000;  // Processor clock is osc/pll /42
pub const SYSCTL_SYSDIV_43        : u32 = 0x95400000;  // Processor clock is osc/pll /43
pub const SYSCTL_SYSDIV_44        : u32 = 0x95C00000;  // Processor clock is osc/pll /44
pub const SYSCTL_SYSDIV_45        : u32 = 0x96400000;  // Processor clock is osc/pll /45
pub const SYSCTL_SYSDIV_46        : u32 = 0x96C00000;  // Processor clock is osc/pll /46
pub const SYSCTL_SYSDIV_47        : u32 = 0x97400000;  // Processor clock is osc/pll /47
pub const SYSCTL_SYSDIV_48        : u32 = 0x97C00000;  // Processor clock is osc/pll /48
pub const SYSCTL_SYSDIV_49        : u32 = 0x98400000;  // Processor clock is osc/pll /49
pub const SYSCTL_SYSDIV_50        : u32 = 0x98C00000;  // Processor clock is osc/pll /50
pub const SYSCTL_SYSDIV_51        : u32 = 0x99400000;  // Processor clock is osc/pll /51
pub const SYSCTL_SYSDIV_52        : u32 = 0x99C00000;  // Processor clock is osc/pll /52
pub const SYSCTL_SYSDIV_53        : u32 = 0x9A400000;  // Processor clock is osc/pll /53
pub const SYSCTL_SYSDIV_54        : u32 = 0x9AC00000;  // Processor clock is osc/pll /54
pub const SYSCTL_SYSDIV_55        : u32 = 0x9B400000;  // Processor clock is osc/pll /55
pub const SYSCTL_SYSDIV_56        : u32 = 0x9BC00000;  // Processor clock is osc/pll /56
pub const SYSCTL_SYSDIV_57        : u32 = 0x9C400000;  // Processor clock is osc/pll /57
pub const SYSCTL_SYSDIV_58        : u32 = 0x9CC00000;  // Processor clock is osc/pll /58
pub const SYSCTL_SYSDIV_59        : u32 = 0x9D400000;  // Processor clock is osc/pll /59
pub const SYSCTL_SYSDIV_60        : u32 = 0x9DC00000;  // Processor clock is osc/pll /60
pub const SYSCTL_SYSDIV_61        : u32 = 0x9E400000;  // Processor clock is osc/pll /61
pub const SYSCTL_SYSDIV_62        : u32 = 0x9EC00000;  // Processor clock is osc/pll /62
pub const SYSCTL_SYSDIV_63        : u32 = 0x9F400000;  // Processor clock is osc/pll /63
pub const SYSCTL_SYSDIV_64        : u32 = 0x9FC00000;  // Processor clock is osc/pll /64
pub const SYSCTL_SYSDIV_2_5       : u32 = 0xC1000000;  // Processor clock is pll / 2.5
pub const SYSCTL_SYSDIV_3_5       : u32 = 0xC1800000;  // Processor clock is pll / 3.5
pub const SYSCTL_SYSDIV_4_5       : u32 = 0xC2000000;  // Processor clock is pll / 4.5
pub const SYSCTL_SYSDIV_5_5       : u32 = 0xC2800000;  // Processor clock is pll / 5.5
pub const SYSCTL_SYSDIV_6_5       : u32 = 0xC3000000;  // Processor clock is pll / 6.5
pub const SYSCTL_SYSDIV_7_5       : u32 = 0xC3800000;  // Processor clock is pll / 7.5
pub const SYSCTL_SYSDIV_8_5       : u32 = 0xC4000000;  // Processor clock is pll / 8.5
pub const SYSCTL_SYSDIV_9_5       : u32 = 0xC4800000;  // Processor clock is pll / 9.5
pub const SYSCTL_SYSDIV_10_5      : u32 = 0xC5000000;  // Processor clock is pll / 10.5
pub const SYSCTL_SYSDIV_11_5      : u32 = 0xC5800000;  // Processor clock is pll / 11.5
pub const SYSCTL_SYSDIV_12_5      : u32 = 0xC6000000;  // Processor clock is pll / 12.5
pub const SYSCTL_SYSDIV_13_5      : u32 = 0xC6800000;  // Processor clock is pll / 13.5
pub const SYSCTL_SYSDIV_14_5      : u32 = 0xC7000000;  // Processor clock is pll / 14.5
pub const SYSCTL_SYSDIV_15_5      : u32 = 0xC7800000;  // Processor clock is pll / 15.5
pub const SYSCTL_SYSDIV_16_5      : u32 = 0xC8000000;  // Processor clock is pll / 16.5
pub const SYSCTL_SYSDIV_17_5      : u32 = 0xC8800000;  // Processor clock is pll / 17.5
pub const SYSCTL_SYSDIV_18_5      : u32 = 0xC9000000;  // Processor clock is pll / 18.5
pub const SYSCTL_SYSDIV_19_5      : u32 = 0xC9800000;  // Processor clock is pll / 19.5
pub const SYSCTL_SYSDIV_20_5      : u32 = 0xCA000000;  // Processor clock is pll / 20.5
pub const SYSCTL_SYSDIV_21_5      : u32 = 0xCA800000;  // Processor clock is pll / 21.5
pub const SYSCTL_SYSDIV_22_5      : u32 = 0xCB000000;  // Processor clock is pll / 22.5
pub const SYSCTL_SYSDIV_23_5      : u32 = 0xCB800000;  // Processor clock is pll / 23.5
pub const SYSCTL_SYSDIV_24_5      : u32 = 0xCC000000;  // Processor clock is pll / 24.5
pub const SYSCTL_SYSDIV_25_5      : u32 = 0xCC800000;  // Processor clock is pll / 25.5
pub const SYSCTL_SYSDIV_26_5      : u32 = 0xCD000000;  // Processor clock is pll / 26.5
pub const SYSCTL_SYSDIV_27_5      : u32 = 0xCD800000;  // Processor clock is pll / 27.5
pub const SYSCTL_SYSDIV_28_5      : u32 = 0xCE000000;  // Processor clock is pll / 28.5
pub const SYSCTL_SYSDIV_29_5      : u32 = 0xCE800000;  // Processor clock is pll / 29.5
pub const SYSCTL_SYSDIV_30_5      : u32 = 0xCF000000;  // Processor clock is pll / 30.5
pub const SYSCTL_SYSDIV_31_5      : u32 = 0xCF800000;  // Processor clock is pll / 31.5
pub const SYSCTL_SYSDIV_32_5      : u32 = 0xD0000000;  // Processor clock is pll / 32.5
pub const SYSCTL_SYSDIV_33_5      : u32 = 0xD0800000;  // Processor clock is pll / 33.5
pub const SYSCTL_SYSDIV_34_5      : u32 = 0xD1000000;  // Processor clock is pll / 34.5
pub const SYSCTL_SYSDIV_35_5      : u32 = 0xD1800000;  // Processor clock is pll / 35.5
pub const SYSCTL_SYSDIV_36_5      : u32 = 0xD2000000;  // Processor clock is pll / 36.5
pub const SYSCTL_SYSDIV_37_5      : u32 = 0xD2800000;  // Processor clock is pll / 37.5
pub const SYSCTL_SYSDIV_38_5      : u32 = 0xD3000000;  // Processor clock is pll / 38.5
pub const SYSCTL_SYSDIV_39_5      : u32 = 0xD3800000;  // Processor clock is pll / 39.5
pub const SYSCTL_SYSDIV_40_5      : u32 = 0xD4000000;  // Processor clock is pll / 40.5
pub const SYSCTL_SYSDIV_41_5      : u32 = 0xD4800000;  // Processor clock is pll / 41.5
pub const SYSCTL_SYSDIV_42_5      : u32 = 0xD5000000;  // Processor clock is pll / 42.5
pub const SYSCTL_SYSDIV_43_5      : u32 = 0xD5800000;  // Processor clock is pll / 43.5
pub const SYSCTL_SYSDIV_44_5      : u32 = 0xD6000000;  // Processor clock is pll / 44.5
pub const SYSCTL_SYSDIV_45_5      : u32 = 0xD6800000;  // Processor clock is pll / 45.5
pub const SYSCTL_SYSDIV_46_5      : u32 = 0xD7000000;  // Processor clock is pll / 46.5
pub const SYSCTL_SYSDIV_47_5      : u32 = 0xD7800000;  // Processor clock is pll / 47.5
pub const SYSCTL_SYSDIV_48_5      : u32 = 0xD8000000;  // Processor clock is pll / 48.5
pub const SYSCTL_SYSDIV_49_5      : u32 = 0xD8800000;  // Processor clock is pll / 49.5
pub const SYSCTL_SYSDIV_50_5      : u32 = 0xD9000000;  // Processor clock is pll / 50.5
pub const SYSCTL_SYSDIV_51_5      : u32 = 0xD9800000;  // Processor clock is pll / 51.5
pub const SYSCTL_SYSDIV_52_5      : u32 = 0xDA000000;  // Processor clock is pll / 52.5
pub const SYSCTL_SYSDIV_53_5      : u32 = 0xDA800000;  // Processor clock is pll / 53.5
pub const SYSCTL_SYSDIV_54_5      : u32 = 0xDB000000;  // Processor clock is pll / 54.5
pub const SYSCTL_SYSDIV_55_5      : u32 = 0xDB800000;  // Processor clock is pll / 55.5
pub const SYSCTL_SYSDIV_56_5      : u32 = 0xDC000000;  // Processor clock is pll / 56.5
pub const SYSCTL_SYSDIV_57_5      : u32 = 0xDC800000;  // Processor clock is pll / 57.5
pub const SYSCTL_SYSDIV_58_5      : u32 = 0xDD000000;  // Processor clock is pll / 58.5
pub const SYSCTL_SYSDIV_59_5      : u32 = 0xDD800000;  // Processor clock is pll / 59.5
pub const SYSCTL_SYSDIV_60_5      : u32 = 0xDE000000;  // Processor clock is pll / 60.5
pub const SYSCTL_SYSDIV_61_5      : u32 = 0xDE800000;  // Processor clock is pll / 61.5
pub const SYSCTL_SYSDIV_62_5      : u32 = 0xDF000000;  // Processor clock is pll / 62.5
pub const SYSCTL_SYSDIV_63_5      : u32 = 0xDF800000;  // Processor clock is pll / 63.5
pub const SYSCTL_CFG_VCO_480      : u32 = 0xF1000000;  // VCO is 480 MHz
pub const SYSCTL_CFG_VCO_320      : u32 = 0xF0000000;  // VCO is 320 MHz
pub const SYSCTL_USE_PLL          : u32 = 0x00000000;  // System clock is the PLL clock
pub const SYSCTL_USE_OSC          : u32 = 0x00003800;  // System clock is the osc clock
pub const SYSCTL_XTAL_1MHZ        : u32 = 0x00000000;  // External crystal is 1MHz
pub const SYSCTL_XTAL_1_84MHZ     : u32 = 0x00000040;  // External crystal is 1.8432MHz
pub const SYSCTL_XTAL_2MHZ        : u32 = 0x00000080;  // External crystal is 2MHz
pub const SYSCTL_XTAL_2_45MHZ     : u32 = 0x000000C0;  // External crystal is 2.4576MHz
pub const SYSCTL_XTAL_3_57MHZ     : u32 = 0x00000100;  // External crystal is 3.579545MHz
pub const SYSCTL_XTAL_3_68MHZ     : u32 = 0x00000140;  // External crystal is 3.6864MHz
pub const SYSCTL_XTAL_4MHZ        : u32 = 0x00000180;  // External crystal is 4MHz
pub const SYSCTL_XTAL_4_09MHZ     : u32 = 0x000001C0;  // External crystal is 4.096MHz
pub const SYSCTL_XTAL_4_91MHZ     : u32 = 0x00000200;  // External crystal is 4.9152MHz
pub const SYSCTL_XTAL_5MHZ        : u32 = 0x00000240;  // External crystal is 5MHz
pub const SYSCTL_XTAL_5_12MHZ     : u32 = 0x00000280;  // External crystal is 5.12MHz
pub const SYSCTL_XTAL_6MHZ        : u32 = 0x000002C0;  // External crystal is 6MHz
pub const SYSCTL_XTAL_6_14MHZ     : u32 = 0x00000300;  // External crystal is 6.144MHz
pub const SYSCTL_XTAL_7_37MHZ     : u32 = 0x00000340;  // External crystal is 7.3728MHz
pub const SYSCTL_XTAL_8MHZ        : u32 = 0x00000380;  // External crystal is 8MHz
pub const SYSCTL_XTAL_8_19MHZ     : u32 = 0x000003C0;  // External crystal is 8.192MHz
pub const SYSCTL_XTAL_10MHZ       : u32 = 0x00000400;  // External crystal is 10 MHz
pub const SYSCTL_XTAL_12MHZ       : u32 = 0x00000440;  // External crystal is 12 MHz
pub const SYSCTL_XTAL_12_2MHZ     : u32 = 0x00000480;  // External crystal is 12.288 MHz
pub const SYSCTL_XTAL_13_5MHZ     : u32 = 0x000004C0;  // External crystal is 13.56 MHz
pub const SYSCTL_XTAL_14_3MHZ     : u32 = 0x00000500;  // External crystal is 14.31818 MHz
pub const SYSCTL_XTAL_16MHZ       : u32 = 0x00000540;  // External crystal is 16 MHz
pub const SYSCTL_XTAL_16_3MHZ     : u32 = 0x00000580;  // External crystal is 16.384 MHz
pub const SYSCTL_XTAL_18MHZ       : u32 = 0x000005C0;  // External crystal is 18.0 MHz
pub const SYSCTL_XTAL_20MHZ       : u32 = 0x00000600;  // External crystal is 20.0 MHz
pub const SYSCTL_XTAL_24MHZ       : u32 = 0x00000640;  // External crystal is 24.0 MHz
pub const SYSCTL_XTAL_25MHZ       : u32 = 0x00000680;  // External crystal is 25.0 MHz
pub const SYSCTL_OSC_MAIN         : u32 = 0x00000000;  // Osc source is main osc
pub const SYSCTL_OSC_INT          : u32 = 0x00000010;  // Osc source is int. osc
pub const SYSCTL_OSC_INT4         : u32 = 0x00000020;  // Osc source is int. osc /4
pub const SYSCTL_OSC_INT30        : u32 = 0x00000030;  // Osc source is int. 30 KHz
pub const SYSCTL_OSC_EXT32        : u32 = 0x80000038;  // Osc source is ext. 32 KHz
pub const SYSCTL_INT_OSC_DIS      : u32 = 0x00000002;  // Disable internal oscillator
pub const SYSCTL_MAIN_OSC_DIS     : u32 = 0x00000001;  // Disable main oscillator

//*****************************************************************************
//
// The following are values that can be passed to the SysCtlDeepSleepClockSet()
// API as the uu32Config parameter.
//
//*****************************************************************************
pub const SYSCTL_DSLP_DIV_1       : u32 = 0x00000000;  // Deep-sleep clock is osc /1
pub const SYSCTL_DSLP_DIV_2       : u32 = 0x00800000;  // Deep-sleep clock is osc /2
pub const SYSCTL_DSLP_DIV_3       : u32 = 0x01000000;  // Deep-sleep clock is osc /3
pub const SYSCTL_DSLP_DIV_4       : u32 = 0x01800000;  // Deep-sleep clock is osc /4
pub const SYSCTL_DSLP_DIV_5       : u32 = 0x02000000;  // Deep-sleep clock is osc /5
pub const SYSCTL_DSLP_DIV_6       : u32 = 0x02800000;  // Deep-sleep clock is osc /6
pub const SYSCTL_DSLP_DIV_7       : u32 = 0x03000000;  // Deep-sleep clock is osc /7
pub const SYSCTL_DSLP_DIV_8       : u32 = 0x03800000;  // Deep-sleep clock is osc /8
pub const SYSCTL_DSLP_DIV_9       : u32 = 0x04000000;  // Deep-sleep clock is osc /9
pub const SYSCTL_DSLP_DIV_10      : u32 = 0x04800000;  // Deep-sleep clock is osc /10
pub const SYSCTL_DSLP_DIV_11      : u32 = 0x05000000;  // Deep-sleep clock is osc /11
pub const SYSCTL_DSLP_DIV_12      : u32 = 0x05800000;  // Deep-sleep clock is osc /12
pub const SYSCTL_DSLP_DIV_13      : u32 = 0x06000000;  // Deep-sleep clock is osc /13
pub const SYSCTL_DSLP_DIV_14      : u32 = 0x06800000;  // Deep-sleep clock is osc /14
pub const SYSCTL_DSLP_DIV_15      : u32 = 0x07000000;  // Deep-sleep clock is osc /15
pub const SYSCTL_DSLP_DIV_16      : u32 = 0x07800000;  // Deep-sleep clock is osc /16
pub const SYSCTL_DSLP_DIV_17      : u32 = 0x08000000;  // Deep-sleep clock is osc /17
pub const SYSCTL_DSLP_DIV_18      : u32 = 0x08800000;  // Deep-sleep clock is osc /18
pub const SYSCTL_DSLP_DIV_19      : u32 = 0x09000000;  // Deep-sleep clock is osc /19
pub const SYSCTL_DSLP_DIV_20      : u32 = 0x09800000;  // Deep-sleep clock is osc /20
pub const SYSCTL_DSLP_DIV_21      : u32 = 0x0A000000;  // Deep-sleep clock is osc /21
pub const SYSCTL_DSLP_DIV_22      : u32 = 0x0A800000;  // Deep-sleep clock is osc /22
pub const SYSCTL_DSLP_DIV_23      : u32 = 0x0B000000;  // Deep-sleep clock is osc /23
pub const SYSCTL_DSLP_DIV_24      : u32 = 0x0B800000;  // Deep-sleep clock is osc /24
pub const SYSCTL_DSLP_DIV_25      : u32 = 0x0C000000;  // Deep-sleep clock is osc /25
pub const SYSCTL_DSLP_DIV_26      : u32 = 0x0C800000;  // Deep-sleep clock is osc /26
pub const SYSCTL_DSLP_DIV_27      : u32 = 0x0D000000;  // Deep-sleep clock is osc /27
pub const SYSCTL_DSLP_DIV_28      : u32 = 0x0D800000;  // Deep-sleep clock is osc /28
pub const SYSCTL_DSLP_DIV_29      : u32 = 0x0E000000;  // Deep-sleep clock is osc /29
pub const SYSCTL_DSLP_DIV_30      : u32 = 0x0E800000;  // Deep-sleep clock is osc /30
pub const SYSCTL_DSLP_DIV_31      : u32 = 0x0F000000;  // Deep-sleep clock is osc /31
pub const SYSCTL_DSLP_DIV_32      : u32 = 0x0F800000;  // Deep-sleep clock is osc /32
pub const SYSCTL_DSLP_DIV_33      : u32 = 0x10000000;  // Deep-sleep clock is osc /33
pub const SYSCTL_DSLP_DIV_34      : u32 = 0x10800000;  // Deep-sleep clock is osc /34
pub const SYSCTL_DSLP_DIV_35      : u32 = 0x11000000;  // Deep-sleep clock is osc /35
pub const SYSCTL_DSLP_DIV_36      : u32 = 0x11800000;  // Deep-sleep clock is osc /36
pub const SYSCTL_DSLP_DIV_37      : u32 = 0x12000000;  // Deep-sleep clock is osc /37
pub const SYSCTL_DSLP_DIV_38      : u32 = 0x12800000;  // Deep-sleep clock is osc /38
pub const SYSCTL_DSLP_DIV_39      : u32 = 0x13000000;  // Deep-sleep clock is osc /39
pub const SYSCTL_DSLP_DIV_40      : u32 = 0x13800000;  // Deep-sleep clock is osc /40
pub const SYSCTL_DSLP_DIV_41      : u32 = 0x14000000;  // Deep-sleep clock is osc /41
pub const SYSCTL_DSLP_DIV_42      : u32 = 0x14800000;  // Deep-sleep clock is osc /42
pub const SYSCTL_DSLP_DIV_43      : u32 = 0x15000000;  // Deep-sleep clock is osc /43
pub const SYSCTL_DSLP_DIV_44      : u32 = 0x15800000;  // Deep-sleep clock is osc /44
pub const SYSCTL_DSLP_DIV_45      : u32 = 0x16000000;  // Deep-sleep clock is osc /45
pub const SYSCTL_DSLP_DIV_46      : u32 = 0x16800000;  // Deep-sleep clock is osc /46
pub const SYSCTL_DSLP_DIV_47      : u32 = 0x17000000;  // Deep-sleep clock is osc /47
pub const SYSCTL_DSLP_DIV_48      : u32 = 0x17800000;  // Deep-sleep clock is osc /48
pub const SYSCTL_DSLP_DIV_49      : u32 = 0x18000000;  // Deep-sleep clock is osc /49
pub const SYSCTL_DSLP_DIV_50      : u32 = 0x18800000;  // Deep-sleep clock is osc /50
pub const SYSCTL_DSLP_DIV_51      : u32 = 0x19000000;  // Deep-sleep clock is osc /51
pub const SYSCTL_DSLP_DIV_52      : u32 = 0x19800000;  // Deep-sleep clock is osc /52
pub const SYSCTL_DSLP_DIV_53      : u32 = 0x1A000000;  // Deep-sleep clock is osc /53
pub const SYSCTL_DSLP_DIV_54      : u32 = 0x1A800000;  // Deep-sleep clock is osc /54
pub const SYSCTL_DSLP_DIV_55      : u32 = 0x1B000000;  // Deep-sleep clock is osc /55
pub const SYSCTL_DSLP_DIV_56      : u32 = 0x1B800000;  // Deep-sleep clock is osc /56
pub const SYSCTL_DSLP_DIV_57      : u32 = 0x1C000000;  // Deep-sleep clock is osc /57
pub const SYSCTL_DSLP_DIV_58      : u32 = 0x1C800000;  // Deep-sleep clock is osc /58
pub const SYSCTL_DSLP_DIV_59      : u32 = 0x1D000000;  // Deep-sleep clock is osc /59
pub const SYSCTL_DSLP_DIV_60      : u32 = 0x1D800000;  // Deep-sleep clock is osc /60
pub const SYSCTL_DSLP_DIV_61      : u32 = 0x1E000000;  // Deep-sleep clock is osc /61
pub const SYSCTL_DSLP_DIV_62      : u32 = 0x1E800000;  // Deep-sleep clock is osc /62
pub const SYSCTL_DSLP_DIV_63      : u32 = 0x1F000000;  // Deep-sleep clock is osc /63
pub const SYSCTL_DSLP_DIV_64      : u32 = 0x1F800000;  // Deep-sleep clock is osc /64
pub const SYSCTL_DSLP_OSC_MAIN    : u32 = 0x00000000;  // Osc source is main osc
pub const SYSCTL_DSLP_OSC_INT     : u32 = 0x00000010;  // Osc source is int. osc
pub const SYSCTL_DSLP_OSC_INT30   : u32 = 0x00000030;  // Osc source is int. 30 KHz
pub const SYSCTL_DSLP_OSC_EXT32   : u32 = 0x00000070;  // Osc source is ext. 32 KHz
pub const SYSCTL_DSLP_PIOSC_PD    : u32 = 0x00000002;  // Power down PIOSC in deep-sleep
pub const SYSCTL_DSLP_MOSC_PD     : u32 = 0x40000000;  // Power down MOSC in deep-sleep

//*****************************************************************************
//
// The following are values that can be passed to the SysCtlPIOSCCalibrate()
// API as the uu32Type parameter.
//
//*****************************************************************************
pub const SYSCTL_PIOSC_CAL_AUTO   : u32 = 0x00000200;  // Automatic calibration
pub const SYSCTL_PIOSC_CAL_FACT   : u32 = 0x00000100;  // Factory calibration
pub const SYSCTL_PIOSC_CAL_USER   : u32 = 0x80000100;  // User-supplied calibration

//*****************************************************************************
//
// The following are values that can be passed to the SysCtlMOSCConfigSet() API
// as the uu32Config parameter.
//
//*****************************************************************************
pub const SYSCTL_MOSC_VALIDATE    : u32 = 0x00000001;  // Enable MOSC validation
pub const SYSCTL_MOSC_INTERRUPT   : u32 = 0x00000002;  // Generate interrupt on MOSC fail
pub const SYSCTL_MOSC_NO_XTAL     : u32 = 0x00000004;  // No crystal is attached to MOSC
pub const SYSCTL_MOSC_PWR_DIS     : u32 = 0x00000008;  // Power down the MOSC.
pub const SYSCTL_MOSC_LOWFREQ     : u32 = 0x00000000;  // MOSC is less than 10MHz
pub const SYSCTL_MOSC_HIGHFREQ    : u32 = 0x00000010;  // MOSC is greater than 10MHz
pub const SYSCTL_MOSC_SESRC       : u32 = 0x00000020;  // Singled ended oscillator source.

//*****************************************************************************
//
// The following are values that can be passed to the SysCtlSleepPowerSet() and
// SysCtlDeepSleepPowerSet() APIs as the uu32Config parameter.
//
//*****************************************************************************
pub const SYSCTL_LDO_SLEEP        : u32 = 0x00000200;  // LDO in sleep mode
                                            // (Deep Sleep Only)
pub const SYSCTL_TEMP_LOW_POWER   : u32 = 0x00000100;  // Temp sensor in low power mode
                                            // (Deep Sleep Only)
pub const SYSCTL_FLASH_NORMAL     : u32 = 0x00000000;  // Flash in normal mode
pub const SYSCTL_FLASH_LOW_POWER  : u32 = 0x00000020;  // Flash in low power mode
pub const SYSCTL_SRAM_NORMAL      : u32 = 0x00000000;  // SRAM in normal mode
pub const SYSCTL_SRAM_STANDBY     : u32 = 0x00000001;  // SRAM in standby mode
pub const SYSCTL_SRAM_LOW_POWER   : u32 = 0x00000003;  // SRAM in low power mode

//*****************************************************************************
//
// Defines for the SysCtlResetBehaviorSet() and SysCtlResetBehaviorGet() APIs.
//
//*****************************************************************************
pub const SYSCTL_ONRST_WDOG0_POR  : u32 = 0x00000030;
pub const SYSCTL_ONRST_WDOG0_SYS  : u32 = 0x00000020;
pub const SYSCTL_ONRST_WDOG1_POR  : u32 = 0x000000C0;
pub const SYSCTL_ONRST_WDOG1_SYS  : u32 = 0x00000080;
pub const SYSCTL_ONRST_BOR_POR    : u32 = 0x0000000C;
pub const SYSCTL_ONRST_BOR_SYS    : u32 = 0x00000008;
pub const SYSCTL_ONRST_EXT_POR    : u32 = 0x00000003;
pub const SYSCTL_ONRST_EXT_SYS    : u32 = 0x00000002;

//*****************************************************************************
//
// Values used with the SysCtlVoltageEventConfig() API.
//
//*****************************************************************************
pub const SYSCTL_VEVENT_VDDABO_NONE : u32 = 0x00000000;
pub const SYSCTL_VEVENT_VDDABO_INT : u32 = 0x00000100;
pub const SYSCTL_VEVENT_VDDABO_NMI : u32 = 0x00000200;
pub const SYSCTL_VEVENT_VDDABO_RST : u32 = 0x00000300;
pub const SYSCTL_VEVENT_VDDBO_NONE : u32 = 0x00000000;
pub const SYSCTL_VEVENT_VDDBO_INT : u32 = 0x00000001;
pub const SYSCTL_VEVENT_VDDBO_NMI : u32 = 0x00000002;
pub const SYSCTL_VEVENT_VDDBO_RST : u32 = 0x00000003;

//*****************************************************************************
//
// Values used with the SysCtlVoltageEventStatus() and
// SysCtlVoltageEventClear() APIs.
//
//*****************************************************************************
pub const SYSCTL_VESTAT_VDDBOR    : u32 = 0x00000040;
pub const SYSCTL_VESTAT_VDDABOR   : u32 = 0x00000010;

//*****************************************************************************
//
// Values used with the SysCtlNMIStatus() API.
//
//*****************************************************************************
pub const SYSCTL_NMI_MOSCFAIL     : u32 = 0x00010000;
pub const SYSCTL_NMI_TAMPER       : u32 = 0x00000200;
pub const SYSCTL_NMI_WDT1         : u32 = 0x00000020;
pub const SYSCTL_NMI_WDT0         : u32 = 0x00000008;
pub const SYSCTL_NMI_POWER        : u32 = 0x00000004;
pub const SYSCTL_NMI_EXTERNAL     : u32 = 0x00000001;

//*****************************************************************************
//
// The defines for the SysCtlClockOutConfig() API.
//
//*****************************************************************************
pub const SYSCTL_CLKOUT_EN        : u32 = 0x80000000;
pub const SYSCTL_CLKOUT_DIS       : u32 = 0x00000000;
pub const SYSCTL_CLKOUT_SYSCLK    : u32 = 0x00000000;
pub const SYSCTL_CLKOUT_PIOSC     : u32 = 0x00010000;
pub const SYSCTL_CLKOUT_MOSC      : u32 = 0x00020000;

//*****************************************************************************
//
// The following defines are used with the SysCtlAltClkConfig() function.
//
//*****************************************************************************
pub const SYSCTL_ALTCLK_PIOSC     : u32 = 0x00000000;
pub const SYSCTL_ALTCLK_RTCOSC    : u32 = 0x00000003;
pub const SYSCTL_ALTCLK_LFIOSC    : u32 = 0x00000004;

extern {
    fn SysCtlClockSet(config: u32);
}

pub fn clock_set(config: u32)
{
    unsafe {
        SysCtlClockSet(config);
    }
}

//extern uint32_t SysCtlSRAMSizeGet(void);
//extern uint32_t SysCtlFlashSizeGet(void);
//extern uint32_t SysCtlFlashSectorSizeGet(void);
//extern bool SysCtlPeripheralPresent(uint32_t ui32Peripheral);
//extern bool SysCtlPeripheralReady(uint32_t ui32Peripheral);
//extern void SysCtlPeripheralPowerOn(uint32_t ui32Peripheral);
//extern void SysCtlPeripheralPowerOff(uint32_t ui32Peripheral);
//extern void SysCtlPeripheralReset(uint32_t ui32Peripheral);
//extern void SysCtlPeripheralEnable(uint32_t ui32Peripheral);
//extern void SysCtlPeripheralDisable(uint32_t ui32Peripheral);
//extern void SysCtlPeripheralSleepEnable(uint32_t ui32Peripheral);
//extern void SysCtlPeripheralSleepDisable(uint32_t ui32Peripheral);
//extern void SysCtlPeripheralDeepSleepEnable(uint32_t ui32Peripheral);
//extern void SysCtlPeripheralDeepSleepDisable(uint32_t ui32Peripheral);
//extern void SysCtlPeripheralClockGating(bool bEnable);
//extern void SysCtlIntRegister(void (*pfnHandler)(void));
//extern void SysCtlIntUnregister(void);
//extern void SysCtlIntEnable(uint32_t ui32Ints);
//extern void SysCtlIntDisable(uint32_t ui32Ints);
//extern void SysCtlIntClear(uint32_t ui32Ints);
//extern uint32_t SysCtlIntStatus(bool bMasked);
//extern void SysCtlLDOSleepSet(uint32_t ui32Voltage);
//extern uint32_t SysCtlLDOSleepGet(void);
//extern void SysCtlLDODeepSleepSet(uint32_t ui32Voltage);
//extern uint32_t SysCtlLDODeepSleepGet(void);
//extern void SysCtlSleepPowerSet(uint32_t ui32Config);
//extern void SysCtlDeepSleepPowerSet(uint32_t ui32Config);
//extern void SysCtlReset(void);
//extern void SysCtlSleep(void);
//extern void SysCtlDeepSleep(void);
//extern uint32_t SysCtlResetCauseGet(void);
//extern void SysCtlResetCauseClear(uint32_t ui32Causes);
//extern void SysCtlBrownOutConfigSet(uint32_t ui32Config, uint32_t ui32Delay);
//extern void SysCtlDelay(uint32_t ui32Count);
//extern void SysCtlMOSCConfigSet(uint32_t ui32Config);
//extern uint32_t SysCtlPIOSCCalibrate(uint32_t ui32Type);
//extern uint32_t SysCtlClockGet(void);
//extern void SysCtlDeepSleepClockSet(uint32_t ui32Config);
//extern void SysCtlDeepSleepClockConfigSet(uint32_t ui32Div, uint32_t ui32Config);
//extern void SysCtlPWMClockSet(uint32_t ui32Config);
//extern uint32_t SysCtlPWMClockGet(void);
//extern void SysCtlIOSCVerificationSet(bool bEnable);
//extern void SysCtlMOSCVerificationSet(bool bEnable);
//extern void SysCtlPLLVerificationSet(bool bEnable);
//extern void SysCtlClkVerificationClear(void);
//extern void SysCtlGPIOAHBEnable(uint32_t ui32GPIOPeripheral);
//extern void SysCtlGPIOAHBDisable(uint32_t ui32GPIOPeripheral);
//extern void SysCtlUSBPLLEnable(void);
//extern void SysCtlUSBPLLDisable(void);
//extern uint32_t SysCtlClockFreqSet(uint32_t ui32Config, uint32_t ui32SysClock);
//extern void SysCtlResetBehaviorSet(uint32_t ui32Behavior);
//extern uint32_t SysCtlResetBehaviorGet(void);
//extern void SysCtlClockOutConfig(uint32_t ui32Config, uint32_t ui32Div);
//extern void SysCtlAltClkConfig(uint32_t ui32Config);
//extern uint32_t SysCtlNMIStatus(void);
//extern void SysCtlNMIClear(uint32_t ui32Status);
//extern void SysCtlVoltageEventConfig(uint32_t ui32Config);
//extern uint32_t SysCtlVoltageEventStatus(void);
//extern void SysCtlVoltageEventClear(uint32_t ui32Status);
//
//*/
