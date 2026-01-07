#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;
use stm32f7xx_hal:: {
    pac,
    prelude::*,
};

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

/// Terminates the application and makes a semihosting-capable debug tool exit
/// with status code 0.
pub fn exit() -> ! {
    semihosting::process::exit(0);
}

/// Hardfault handler.
///
/// Terminates the application and makes a semihosting-capable debug tool exit
/// with an error. This seems better than the default, which is to spin in a
/// loop.
#[cortex_m_rt::exception]
unsafe fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
    semihosting::process::exit(1);
}

#[entry]
fn main() -> ! {
    defmt::println!("ZoomPunk booting up!");
    let device_peripherals = pac::Peripherals::take().unwrap();
    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let rcc = device_peripherals.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(216.MHz()).freeze();
    let gpioi = device_peripherals.GPIOI.split();

    let mut led = gpioi.pi1.into_push_pull_output();
    let mut delay = Delay::new(core_peripherals.SYST, clocks.sysclk().to_Hz());
    loop {
        led.toggle();
        delay.delay_ms(1000_u32);
    }
}
