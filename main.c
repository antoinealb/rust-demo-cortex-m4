#include <stdint.h>
#include <stdbool.h>

#include "inc/hw_memmap.h"
#include "inc/hw_sysctl.h"
#include "driverlib/sysctl.h"
#include "driverlib/systick.h"

#include "led_driver.h"

void systick_handler(void)
{
    static int i = 0;
    i++;
    if (i==1000)
        led_set(0);

    if (i==2000) {
        led_set(LED_RED);
        i = 0;
    }
}


void clock_setup(void)
{
    /* External Crystal, 16 Mhz.
     * The PLL elevates it to 400 Mhz (fixed).
     * We divide it by 2.5 (SYSCTL_SYSDIV_2_5) to obtain 80 Mhz (maximum for our chip).
     */
    SysCtlClockSet(SYSCTL_SYSDIV_2_5|SYSCTL_USE_PLL|SYSCTL_XTAL_16MHZ|SYSCTL_OSC_MAIN);

    /* System tick configuration.
     * This tick can be used for an RTOS for example.
     * Here it is used to toggle the LED out of the processor main loop.
     */
    SysTickEnable();
    SysTickPeriodSet(80 * 1000); /* every ms */
    SysTickIntEnable();
    SysTickIntRegister(systick_handler);
}



// main function.
int main(void)
{
    clock_setup();
    led_setup();


    for (;;) {
    }
}
