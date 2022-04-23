//! # Example: Hello
//!
//! A simple example displaying some graphics and text on the Ableton Push2 display.

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Bgr565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};
use push2_display::Push2Display;
use std::{error, thread, time};

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut display = Push2Display::new()?;
    let text_style = MonoTextStyle::new(&FONT_10X20, Bgr565::WHITE);

    let mut position = Point::new(0, 70);
    let mut step = 4;
    loop {
        display.clear(Bgr565::BLACK)?;

        Rectangle::new(Point::zero(), display.size())
            .into_styled(PrimitiveStyle::with_stroke(Bgr565::WHITE, 1))
            .draw(&mut display)?;

        position.x += step;
        if position.x >= display.size().width as i32 || position.x <= 0 {
            step *= -1;
        }

        Text::new("Hello!", position, text_style)
            .draw(&mut display)?;

        display.flush()?; // if no frame arrives in 2 seconds, the display is turned black
        thread::sleep(time::Duration::from_millis(1000 / 60));
    }
}
