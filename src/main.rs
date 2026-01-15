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
    gpio::Speed,
    ltdc::{Layer, PixelFormat},
    rcc::{HSEClock, HSEClockMode}
};

use embedded_graphics::{
    prelude::*,
    pixelcolor::Rgb565,
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    text::Text,
};

const WIDTH: u16 = 480;
const HEIGHT: u16 = 272;

const FB_GRAPHICS_SIZE: usize = (WIDTH as usize) * (HEIGHT as usize);
static mut FB_LAYER1: [u16; FB_GRAPHICS_SIZE] = [0; FB_GRAPHICS_SIZE];


#[entry]
fn main() -> ! {
    defmt::println!("ZoomPunk booting up!");
    let device_peripherals = pac::Peripherals::take().unwrap();
    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let rcc = device_peripherals.RCC.constrain();

    let _gpioa = device_peripherals.GPIOA.split();
    let _gpiob = device_peripherals.GPIOB.split();
    let gpioe = device_peripherals.GPIOE.split();
    let gpiog = device_peripherals.GPIOG.split();
    let gpioh = device_peripherals.GPIOH.split();
    let gpioi = device_peripherals.GPIOI.split();
    let gpioj = device_peripherals.GPIOJ.split();
    let gpiok = device_peripherals.GPIOK.split();

    gpioe.pe4.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_B0

    gpiog.pg12.into_alternate::<9>().set_speed(Speed::VeryHigh); // LTCD_B4

    gpioi.pi9.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_VSYNC
    gpioi.pi10.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_HSYNC
    gpioi.pi13.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioi.pi14.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_CLK
    gpioi.pi15.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_R0

    gpioj.pj0.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_R1
    gpioj.pj1.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_R2
    gpioj.pj2.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_R3
    gpioj.pj3.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_R4
    gpioj.pj4.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_R5
    gpioj.pj5.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_R6
    gpioj.pj6.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_R7
    gpioj.pj7.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_G0
    gpioj.pj8.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_G1
    gpioj.pj9.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_G2
    gpioj.pj10.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_G3
    gpioj.pj11.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_G4
    gpioj.pj13.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_B1
    gpioj.pj14.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_B2
    gpioj.pj15.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_B3

    gpiok.pk0.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_G5
    gpiok.pk1.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_G6
    gpiok.pk2.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_G7
    gpiok.pk4.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_B5
    gpiok.pk5.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_B6
    gpiok.pk6.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_D7
    gpiok.pk7.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_E

    gpioh.ph1.into_floating_input();

    let clocks = rcc
        .cfgr
        .hse(HSEClock::new(25_000_000.Hz(), HSEClockMode::Bypass))
        .sysclk(216.MHz())
        .hclk(216.MHz())
        .freeze();

    let mut disp_on = gpioi.pi12.into_push_pull_output();
    disp_on.set_low();

    let mut backlight = gpiok.pk3.into_push_pull_output();
    backlight.set_high();

    let mut disp = display::Stm32F7DiscoDisplay::new(device_peripherals.LTDC, device_peripherals.DMA2D);
    disp
        .controller
        .config_layer(Layer::L1, unsafe {&mut *core::ptr::addr_of_mut!(FB_LAYER1)}, PixelFormat::RGB565);

    disp.controller.enable_layer(Layer::L1);
    disp.controller.reload();

    let disp = &mut disp;

    disp_on.set_high();

    let style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
    Text::new("ZoomPunk is coming...", Point::new(20, 30), style).draw(disp).unwrap();

    let mut delay = Delay::new(core_peripherals.SYST, clocks.sysclk().to_Hz());
    loop {
        delay.delay_ms(1000_u32);
    }
}
