# Rust on Cortex-M4

This project implements a blinking LED on a Texas Instrument Tiva C microcontroller.

[Setup tutorial ](http://antoinealb.net/programming/2015/05/01/rust-on-arm-microcontroller.html)


To compile rust libraries (only needed once) :
    
    make libs

To compile :

    make

To send to board using OpenOCD :

    make flash

## Hardfloat ABI
There is a second branch using the hardfloat ABI. See issue #1 for discussion

