#![feature(used)]
#![no_std]

extern crate panic_abort;

extern crate stm32f767_hal as hal;
use hal::delay::Delay;
use hal::prelude::*;
use hal::stm32f767;

extern crate cortex_m;
use cortex_m::peripheral::Peripherals;

fn main() {
    if let (Some(p), Some(cp)) = (stm32f767::Peripherals::take(), Peripherals::take()) {
        let gpiob = p.GPIOB.split();

        // (Re-)configure Pb0 as output
        let mut led = gpiob.pb0.into_push_pull_output();

        // Constrain clocking registers
        let mut rcc = p.RCC.constrain();

        // Configure clock to and freeze it
        let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();

        /* Get delay provider */
        let mut delay = Delay::new(cp.SYST, clocks);

        loop {
            led.set_high();
            delay.delay_ms(500_u16);

            led.set_low();
            delay.delay_ms(500_u16);
        }
    }
}
