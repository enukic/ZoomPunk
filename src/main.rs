#![no_std]
#![no_main]

mod hooks;
mod display;

use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use defmt_rtt as _;
use stm32f7xx_hal:: {
    pac,
    prelude::*,
    rcc::{HSEClock, HSEClockMode}
};

use embedded_graphics::{
    prelude::*,
    pixelcolor::Rgb565,
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    text::Text,
};

#[entry]
fn main() -> ! {
    defmt::println!("ZoomPunk booting up!");
    let device_peripherals = pac::Peripherals::take().unwrap();
    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let rcc = device_peripherals.RCC.constrain();
    let clocks = rcc
        .cfgr
        .hse(HSEClock::new(25_000_000.Hz(), HSEClockMode::Bypass))
        .sysclk(216.MHz())
        .hclk(216.MHz())
        .freeze();

    let mut disp = display::Display::new(
        device_peripherals.LTDC, 
        device_peripherals.DMA2D,
        device_peripherals.GPIOE,
        device_peripherals.GPIOG,
        device_peripherals.GPIOH,
        device_peripherals.GPIOI,
        device_peripherals.GPIOJ,
        device_peripherals.GPIOK,
    );

    let style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
    Text::new("ZoomPunk is coming...", Point::new(20, 30), style).draw(&mut disp).unwrap();

    let mut delay = Delay::new(core_peripherals.SYST, clocks.sysclk().to_Hz());
    loop {
        delay.delay_ms(1000_u32);
    }
}
