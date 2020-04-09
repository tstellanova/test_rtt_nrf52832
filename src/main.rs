#![no_std]
#![no_main]

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use nrf52832_hal as p_hal;


use p_hal::nrf52832_pac as pac;
use p_hal::{delay::Delay};
use embedded_hal::blocking::delay::DelayMs;
use cortex_m_rt as rt;
use rt::entry;


#[entry]
fn main() -> ! {
    rtt_init_print!(NoBlockTrim);

    let cp = pac::CorePeripherals::take().unwrap();
    let mut delay_source = Delay::new(cp.SYST);

    let mut i = 0;
    loop {
        rprintln!("ratt {}", i);
        i += 1;
        delay_source.delay_ms(10u8);
    }
}
