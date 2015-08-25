#include "ch.h"
#include "hal.h"

// rust functions
uint32_t test_add(uint32_t a, uint32_t b);
extern void rust_thread(void *p);

static THD_WORKING_AREA(rust_thread_wa, 256);

int main(void) {
    halInit();
    chSysInit();

    sdStart(&SD2, NULL);
    palSetPadMode(GPIOA, 2, PAL_MODE_ALTERNATE(7));
    palSetPadMode(GPIOA, 3, PAL_MODE_ALTERNATE(7));

    test_add(23, 42);

    chThdCreateStatic(rust_thread_wa, sizeof(rust_thread_wa), NORMALPRIO, rust_thread, NULL);

    while (true) {
        palTogglePad(GPIOD, GPIOD_LED4);
        chThdSleepMilliseconds(100);
    }
}


void led_toggle(void)
{
    palTogglePad(GPIOD, GPIOD_LED3);
}

void os_thread_sleep_ms(uint32_t ms)
{
    chThdSleepMilliseconds(ms);
}

