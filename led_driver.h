#ifndef LED_DRIVER_H
#define LED_DRIVER_H

#include <stdint.h>
#include <stdbool.h>
#include "inc/hw_gpio.h"
#include "driverlib/gpio.h"

#define LED_RED GPIO_PIN_1
#define LED_BLUE GPIO_PIN_2
#define LED_GREEN GPIO_PIN_3

/** Setups the onboard LED for use. */
void led_setup(void);

/** Sets the onboard LED to a given value or combination of value.
 *
 * For example, to get purple use led_set(LED_RED+lED_BLUE).
 */
void led_set(int32_t value);

#endif
