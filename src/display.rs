use embedded_graphics_core::{
    pixelcolor::{Bgr565, IntoStorage},
    geometry::Size,
    prelude::*,
};
use rusb::{Context, Device, DeviceDescriptor, DeviceHandle, UsbContext};
use std::convert::TryInto;
use thiserror::Error;

pub struct Push2Display {
    handle: DeviceHandle<Context>,
    frame_buffer: [u16; DISPLAY_WIDTH * DISPLAY_HEIGHT],
}

#[derive(Error, Debug)]
pub enum Push2DisplayError {
    #[error("Ableton Push2 Not found")]
    Push2NotFound,

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    USBError(#[from] rusb::Error),
}

pub const DISPLAY_WIDTH: usize = 960;
pub const DISPLAY_HEIGHT: usize = 160;


const PUSH2_BULK_EP_OUT: u8 = 0x01;
const BYTES_PER_LINE: usize = 2048; // 960 * 2 + 128 filler
const PUSH_2_VENDOR_ID: u16 = 0x2982;
const PUSH_2_PRODUCT_ID: u16 = 0x1967;

const HEADER: [u8; 16] = [
    0xff, 0xcc, 0xaa, 0x88,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00
];
const MASK: [u8; 4] = [
    0xe7, 0xf3, 0xe7, 0xff
];

impl Push2Display {
    /// Open the Push2 display. and init the frame buffer with black.
    /// the frame buffer is not send send until you call `flush`
    pub fn new() -> Result<Push2Display, Push2DisplayError> {
        let mut context = Context::new()?;
        let (_, _, mut handle) = open_device(&mut context, PUSH_2_VENDOR_ID, PUSH_2_PRODUCT_ID).ok_or( Push2DisplayError::Push2NotFound)?;

        handle.claim_interface(0)?;
        let buffer : [u16; DISPLAY_WIDTH * DISPLAY_HEIGHT] = [0; DISPLAY_WIDTH * DISPLAY_HEIGHT];

        Ok(Push2Display {
            handle,
            frame_buffer: buffer,
        })
    }

    /// Writes the frame buffer to the display. If no frame arrives in 2 seconds, the display is turned black
    pub fn flush(&self) -> Result<(), Push2DisplayError> {
        use std::time::Duration;
        let timeout = Duration::from_secs(1);

        let tranfer_buffer = self.masked_frame_buffer();
        self.handle.write_bulk(PUSH2_BULK_EP_OUT, &HEADER, timeout)?;
        self.handle.write_bulk(PUSH2_BULK_EP_OUT, &tranfer_buffer, timeout)?;

        Ok(())
    }

    fn masked_frame_buffer(&self) -> [u8;BYTES_PER_LINE * DISPLAY_HEIGHT] {

        let mut masked_buffer: [u8;BYTES_PER_LINE * DISPLAY_HEIGHT] = [0; BYTES_PER_LINE * DISPLAY_HEIGHT];
        for r in 0..DISPLAY_HEIGHT {
            for c in 0..DISPLAY_WIDTH {
                let i = r * DISPLAY_WIDTH + c;
                let b: [u8; 2] = u16::to_le_bytes(self.frame_buffer[i]);
                let di = r * BYTES_PER_LINE + c * 2;
                masked_buffer[di] = b[0] ^ MASK[di%4];
                masked_buffer[di+1] = b[1] ^ MASK[(di+1)%4];
            }
        }

        masked_buffer
    }
}

impl DrawTarget for Push2Display {
    type Color = Bgr565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error> where
        I: IntoIterator<Item=Pixel<Self::Color>> {
        for Pixel(point, color) in pixels.into_iter() {
            if let Ok((x @ 0..=959, y @ 0..=159)) = point.try_into() {
                let index: u32 = x + y * 960;
                self.frame_buffer[index as usize] = color.into_storage();
            }
        }

        Ok(())
    }
}

impl OriginDimensions for Push2Display {
    fn size(&self) -> Size {
        Size::new(DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32)
    }
}

fn open_device<T: UsbContext>(
    context: &mut T,
    vid: u16,
    pid: u16,
) -> Option<(Device<T>, DeviceDescriptor, DeviceHandle<T>)> {
    let devices = match context.devices() {
        Ok(d) => d,
        Err(_) => return None,
    };

    for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            match device.open() {
                Ok(handle) => return Some((device, device_desc, handle)),
                Err(_) => continue,
            }
        }
    }

    None
}




