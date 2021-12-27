#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use stm32l4::stm32l4x5;

#[entry]
fn main() -> ! {
    loop {
        // give us references to the peripherals
        let peripherals = stm32l4x5::Peripherals::take().unwrap();
        let rcc = &peripherals.RCC;
        let gpiob = &peripherals.GPIOB;
        let gpioe = &peripherals.GPIOE;

        // Enable clocks for GPIOB and GPIOE
        rcc.ahb2enr.write(|w| w.gpioben().set_bit());
        rcc.ahb2enr.write(|w| w.gpioeen().set_bit());

        // Set GPIO pins to output
        gpiob.moder.modify(|_, w| w.moder2().output());
        gpioe.moder.modify(|_, w| w.moder8().output());

        // Set LEDs
        gpiob.odr.write(|w| w.odr2().set_bit()); // red LED on
        gpiob.odr.write(|w| w.odr2().clear_bit()); // red LED off
        gpioe.odr.write(|w| w.odr8().set_bit()); // green LED on
        gpioe.odr.write(|w| w.odr8().clear_bit()); // green LED off
    }
}
