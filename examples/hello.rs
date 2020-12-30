//! # Example: Hello
//!
//! A simple example displaying some graphics and text on the Ableton Push2 display.

use embedded_graphics::{
    fonts::{Font12x16, Text},
    pixelcolor::{Bgr565},

    primitives::Rectangle,
    style:: {PrimitiveStyle, MonoTextStyle},
    prelude::*,
};
use push2_display::Push2Display;
use std::{thread, time, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut display = Push2Display::new()?;

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

        Text::new("Hello!", position)
            .into_styled(MonoTextStyle::new(Font12x16, Bgr565::BLUE))
            .draw(&mut display)?;

        display.flush()?; // if no frame arrives in 2 seconds, the display is turned black
        thread::sleep(time::Duration::from_millis(1000/60));
    }
}