use embedded_graphics::{
    pixelcolor::raw::RawU16,
    Pixel,
    draw_target::DrawTarget,
    geometry::Size,
    pixelcolor::Rgb565,
    prelude::*,
};

use stm32f7xx_hal::{
    gpio::{Output, PushPull, Speed, gpioi::PI12, gpiok::PK3},
    ltdc::{DisplayConfig, DisplayController, Layer, PixelFormat},
    pac::{DMA2D, LTDC, GPIOE, GPIOG, GPIOH, GPIOI, GPIOJ, GPIOK},
    prelude::*,
    rcc::{HSEClock, HSEClockMode},
};

const WIDTH: u16 = 480;
const HEIGHT: u16 = 272;

const FB_SIZE: usize = (WIDTH as usize) * (HEIGHT as usize);
static mut INTERNAL_FB: [u16; FB_SIZE] = [0; FB_SIZE];

const DISCO_CONFIG: DisplayConfig = DisplayConfig {
    active_width: 480,
    active_height: 272,
    h_back_porch: 13,
    h_front_porch: 30,
    h_sync: 41,
    v_back_porch: 2,
    v_front_porch: 2,
    v_sync: 10,
    frame_rate: 60,
    h_sync_pol: false,
    v_sync_pol: false,
    no_data_enable_pol: false,
    pixel_clock_pol: false,
};

pub struct Display {
    controller: DisplayController<u16>,
    _disp_on: PI12<Output<PushPull>>,
    _backlight: PK3<Output<PushPull>>,
}

impl Display {
    pub fn new(
        ltdc: LTDC,
        dma2d: DMA2D,
        gpioe: GPIOE, gpiog: GPIOG, gpioh: GPIOH, 
        gpioi: GPIOI, gpioj: GPIOJ, gpiok: GPIOK
    ) -> Self {
        let (mut disp_on, mut backlight) = init_pins(gpioe, gpiog, gpioh, gpioi, gpioj, gpiok);

        let mut controller = DisplayController::new(
            ltdc,
            dma2d,
            PixelFormat::RGB565,
            DISCO_CONFIG,
            Some(&HSEClock::new(25_000_000.Hz(), HSEClockMode::Bypass)),
        );

        controller.config_layer(
            Layer::L1,
            unsafe { &mut *core::ptr::addr_of_mut!(INTERNAL_FB) },
            PixelFormat::RGB565,
        );

        controller.enable_layer(Layer::L1);
        controller.reload();

        disp_on.set_high();
        backlight.set_high();

        Self {
            controller,
            _disp_on: disp_on,
            _backlight: backlight,
        }
    }
}

impl DrawTarget for Display {
    type Color = Rgb565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels.into_iter() {
            if point.x >= 0 && point.x < WIDTH as i32 && point.y >= 0 && point.y < HEIGHT as i32 {
                let raw = RawU16::from(color).into_inner();
                self.controller.draw_pixel(Layer::L1, point.x as usize, point.y as usize, raw);
            }
        }
        Ok(())
    }
}

impl OriginDimensions for Display {
    fn size(&self) -> Size {
        Size::new(WIDTH as u32, HEIGHT as u32)
    }
}

fn init_pins(
    gpioe: GPIOE, gpiog: GPIOG, gpioh: GPIOH, 
    gpioi: GPIOI, gpioj: GPIOJ, gpiok: GPIOK
) -> (PI12<Output<PushPull>>, PK3<Output<PushPull>>) {
    let gpioe = gpioe.split();
    let gpiog = gpiog.split();
    let gpioh = gpioh.split();
    let gpioi = gpioi.split();
    let gpioj = gpioj.split();
    let gpiok = gpiok.split();

    gpioe.pe4.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpiog.pg12.into_alternate::<9>().set_speed(Speed::VeryHigh);
    gpioi.pi9.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioi.pi10.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioi.pi13.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioi.pi14.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioi.pi15.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj0.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj1.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj2.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj3.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj4.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj5.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj6.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj7.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj8.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj9.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj10.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj11.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj13.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj14.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj15.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpiok.pk0.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpiok.pk1.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpiok.pk2.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpiok.pk4.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpiok.pk5.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpiok.pk6.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpiok.pk7.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioh.ph1.into_floating_input();

    let mut disp_on = gpioi.pi12.into_push_pull_output();
    disp_on.set_low();
    let mut backlight = gpiok.pk3.into_push_pull_output();
    backlight.set_high();

    (disp_on, backlight)
}
