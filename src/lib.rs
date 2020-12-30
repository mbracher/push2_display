//! This Rust library implements the Embedded-graphics-core `DrawTarget` trait,
//! making it easy to draw 2D graphics primitives on the Push2 display.
//!
//! Ableton Push2 is a MIDI instrument with a 960x160 RGB LCD display.
//! Push2 is a USB composite device with a MIDI interface and a generic bulk data interface used to drive the display.
//!
//! # Examples
//!
//! ```rust
//! let mut display = Push2Display::new()?;
//!
//! Text::new("Hello!", Point::new(400, 70))
//!     .into_styled(TextStyle::new(Font6x8, Bgr565::BLUE))
//!     .draw(&mut display)?;
//!
//! display.flush()?;
//! ```
//!
//! ```bash
//! git clone https://github.com/mbracher/push2_display
//! cd push2_display
//!
//! cargo run --examples hello
//! ```
//!
//! ![Photo of hello example on Push2 device](https://raw.githubusercontent.com/mbracher/push2_display/master/doc/assets/push2hello.jpg)
//!
//! # References
//! [Ableton Push Interface](https://github.com/Ableton/push-interface)
//!
//! [Embedded graphics](https://github.com/embedded-graphics/embedded-graphics)

pub mod display;
pub use display::*;



