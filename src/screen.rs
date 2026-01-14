use embedded_graphics::{
    Pixel,
    draw_target::DrawTarget,
    geometry::Size,
    pixelcolor::Rgb565,
    pixelcolor::raw::RawU16,
    prelude::*,
};

use stm32f7xx_hal::{
    ltdc::{DisplayConfig, DisplayController, Layer, PixelFormat, SupportedWord},
    pac::{DMA2D, LTDC},
    prelude::*,
    rcc::{HSEClock, HSEClockMode},
};

pub const DISCO_SCREEN_CONFIG: DisplayConfig = DisplayConfig {
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

pub struct Stm32F7DiscoDisplay<T: 'static + SupportedWord> {
    pub controller: DisplayController<T>,
}

impl <T: 'static + SupportedWord> Stm32F7DiscoDisplay<T> {
    pub fn new(ltdc: LTDC, dma2d: DMA2D) -> Stm32F7DiscoDisplay<T> {
        let controller = DisplayController::new(
            ltdc,
            dma2d,
            PixelFormat::RGB565,
            DISCO_SCREEN_CONFIG,
            Some(&HSEClock::new(25_000_000.Hz(), HSEClockMode::Bypass)),
        );

        Stm32F7DiscoDisplay{controller}
    }
}

impl DrawTarget for Stm32F7DiscoDisplay<u16> {
    type Color = Rgb565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels.into_iter() {
            if point.x >= 0 && point.x < DISCO_SCREEN_CONFIG.active_width as i32 && point.y >= 0 && point.y < DISCO_SCREEN_CONFIG.active_height as i32 {
                let raw_value = RawU16::from(color).into_inner();

                self.controller.draw_pixel(
                    Layer::L1, 
                    point.x as usize, 
                    point.y as usize, 
                    raw_value
                );
            }
        }
        Ok(())
    }
}

impl OriginDimensions for Stm32F7DiscoDisplay<u16> {
    fn size(&self) -> Size {
        Size::new(DISCO_SCREEN_CONFIG.active_width as u32, DISCO_SCREEN_CONFIG.active_height as u32)
    }
}
