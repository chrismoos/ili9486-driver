#![no_std]
//#![deny(warnings)]

//! This crate provides a driver for the ILI9486 LCD controller.
//!
//! The driver takes multiple [IoPin](IoPin) instances, which for now, can be created
//! with helpers in the [io](io) module.
//!

use crate::color::PixelFormat;
use crate::color::PixelWriter;
use core::convert::Infallible;
use core::marker::PhantomData;
use display_interface::v2::*;
use display_interface::DisplayError;

use embedded_graphics::prelude::Pixel;
use embedded_graphics::prelude::RgbColor;
use embedded_graphics::prelude::Size;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::style::PrimitiveStyle;
use embedded_graphics::style::Styled;
use embedded_graphics::DrawTarget;
use embedded_hal::digital::v2::OutputPin;

/// This module provides I/O abstractions.
///
/// The following devices are supported:
/// * STM32F1xx
///
/// The I/O pin abstraction will be removed when [this issue](https://github.com/rust-embedded/embedded-hal/issues/29)
/// is resolved in `embedded-hal`.
pub mod io;

#[macro_use]
pub mod gpio;

pub mod color;

use io::IoPin;

use embedded_hal::blocking::delay::DelayUs;

struct PixelStream<'a, T> {
    total: usize,
    index: usize,
    bytes_per_pixel: u8,
    pixel_data: &'a [T],
}

impl<'a, T> Iterator for PixelStream<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.total == 0 {
            None
        } else {
            let result: &T = &self.pixel_data[self.index];
            self.index += 1;
            if self.index == self.bytes_per_pixel as usize {
                self.index = 0;
                self.total -= 1;
            }
            Some(result)
        }
    }
}

/// A driver for the ILI9486 LCD controller.
pub struct ILI9486<RW, T>
where
    RW: ReadWriteInterface<T> + PixelWriter<T>,
{
    rw_interface: RW,
    color_mode: PixelFormat,
    _marker: PhantomData<T>,
}

pub enum Command {
    ColumnAddressSet = 0x2a,
    PageAddressSet = 0x2b,
}

pub trait Commands {
    fn set_interface_pixel_format(
        &mut self,
        pixel_format: &PixelFormat,
    ) -> Result<(), DisplayError>;
    fn clear_screen(&mut self) -> Result<(), DisplayError>;
    fn column_address_set(&mut self, start: u16, end: u16) -> Result<(), DisplayError>;
    fn page_address_set(&mut self, start: u16, end: u16) -> Result<(), DisplayError>;
}
impl<RW, T> Commands for ILI9486<RW, T>
where
    RW: ReadWriteInterface<T> + PixelWriter<T>,
    T: From<u8> + Default,
{
    fn set_interface_pixel_format(
        &mut self,
        pixel_format: &PixelFormat,
    ) -> Result<(), DisplayError> {
        match pixel_format {
            PixelFormat::Rgb565 => self.write_command(0x3A.into(), &[0b01010101.into()]),
            PixelFormat::Rgb666 => self.write_command(0x3A.into(), &[0b01100110.into()]),
            _ => Err(DisplayError::InvalidFormatError),
        }
    }

    fn column_address_set(&mut self, start: u16, end: u16) -> Result<(), DisplayError> {
        self.write_command(
            Command::ColumnAddressSet as u8,
            &[
                ((start >> 8) as u8).into(),
                ((start & 0xff) as u8).into(),
                ((end >> 8) as u8).into(),
                ((end & 0xff) as u8).into(),
            ],
        )
    }

    fn page_address_set(&mut self, start: u16, end: u16) -> Result<(), DisplayError> {
        self.write_command(
            Command::PageAddressSet as u8,
            &[
                ((start >> 8) as u8).into(),
                ((start & 0xff) as u8).into(),
                ((end >> 8) as u8).into(),
                ((end & 0xff) as u8).into(),
            ],
        )
    }

    fn clear_screen(&mut self) -> Result<(), DisplayError> {
        self._draw_rect(0, 0, 320, 480, 0, 0, 0)
    }
}

impl<RW, T> ILI9486<RW, T>
where
    RW: ReadWriteInterface<T> + PixelWriter<T>,
    T: From<u8> + Default,
{
    pub fn new<RST>(
        delay: &mut dyn DelayUs<u32>,
        color_mode: PixelFormat,
        rw_interface: RW,
        mut rst: RST,
    ) -> Result<ILI9486<RW, T>, DisplayError>
    where
        RST: IoPin,
    {
        let rst_output = rst.into_output();
        wrap_output_err!(rst_output.set_low())?;
        delay.delay_us(20);
        wrap_output_err!(rst_output.set_high())?;
        delay.delay_us(120_000_000);

        let mut driver = ILI9486 {
            rw_interface: rw_interface,
            color_mode: PixelFormat::Rgb565,
            _marker: PhantomData,
        };

        driver.set_interface_pixel_format(&color_mode)?;
        driver.color_mode = color_mode;

        Ok(driver)
    }

    fn _draw_pixel(&mut self, x: u16, y: u16, r: u8, g: u8, b: u8) -> Result<(), DisplayError> {
        self.column_address_set(x, x + 1)?;
        self.page_address_set(y, y + 1)?;

        self.rw_interface
            .write(WriteMode::Command, &[0x2c.into()])?;
        self.rw_interface
            .write_pixel_data(&self.color_mode, &(r, g, b), None)
    }

    fn _draw_rect(
        &mut self,
        x: u16,
        y: u16,
        width: u32,
        height: u32,
        r: u8,
        g: u8,
        b: u8,
    ) -> Result<(), DisplayError> {
        self.column_address_set(x, x + (width - 1) as u16)?;
        self.page_address_set(y, y + (height - 1) as u16)?;

        let n: usize = (width * height) as usize;
        self.rw_interface
            .write(WriteMode::Command, &[0x2c.into()])?;

        self.rw_interface
            .write(WriteMode::Command, &[0x2c.into()])?;

        for x in (0..n).step_by(2) {
            // last pixel
            if x + 1 == n {
                self.rw_interface
                    .write_pixel_data(&self.color_mode, &(r, g, b), None)?;
            } else {
                self.rw_interface.write_pixel_data(
                    &self.color_mode,
                    &(r, g, b),
                    Some(&(r, g, b)),
                )?;
            }
        }
        Ok(())
    }

    pub fn writer(&mut self) -> &mut RW {
        &mut self.rw_interface
    }

    pub fn reader(&mut self) -> &mut RW {
        &mut self.rw_interface
    }

    /// Sends a write command to the device, streaming data from `data_provider` to the device.
    ///
    /// # Arguments
    ///
    /// `command` - The command to send
    ///
    /// `data_provider` - Function that provides data to be sent. `None` should be returned from the function when transmission is complete.
    ///
    pub fn write_command(&mut self, command: u8, data: &[T]) -> Result<(), DisplayError> {
        self.rw_interface
            .write(WriteMode::Command, &mut [command.into()])?;
        self.rw_interface.write(WriteMode::Data, data)
    }

    /// Sends a read command to the device.
    ///
    /// # Arguments
    ///
    /// `command` - The command to send
    ///
    /// `output` - `output.len()` bytes will be read from the device.
    ///
    pub fn read_command(&mut self, command: u8, output: &mut [T]) -> Result<(), DisplayError> {
        self.rw_interface
            .write(WriteMode::Command, &mut [command.into()])?;
        self.rw_interface.read(output)
    }
}

impl<RW, RGBC> DrawTarget<RGBC> for ILI9486<RW, u8>
where
    RGBC: RgbColor,
    RW: ReadWriteInterface<u8> + PixelWriter<u8>,
{
    type Error = Infallible;

    fn draw_rectangle(
        &mut self,
        item: &Styled<Rectangle, PrimitiveStyle<RGBC>>,
    ) -> Result<(), Self::Error> {
        let color = item.style.fill_color.unwrap_or(RgbColor::BLACK);
        self._draw_rect(
            item.primitive.top_left.x as u16,
            item.primitive.top_left.y as u16,
            (item.primitive.bottom_right.x - item.primitive.top_left.x) as u32,
            (item.primitive.bottom_right.y - item.primitive.top_left.y) as u32,
            color.r(),
            color.g(),
            color.b(),
        )
        .unwrap();

        match item.style.stroke_color {
            Some(stroke_color) => {
                self._draw_rect(
                    item.primitive.top_left.x as u16,
                    item.primitive.top_left.y as u16,
                    (item.primitive.bottom_right.x - item.primitive.top_left.x) as u32,
                    item.style.stroke_width,
                    stroke_color.r(),
                    stroke_color.g(),
                    stroke_color.b(),
                )
                .unwrap();

                self._draw_rect(
                    item.primitive.top_left.x as u16,
                    (item.primitive.top_left.y as u32 + item.style.stroke_width) as u16,
                    item.style.stroke_width,
                    (item.primitive.bottom_right.y
                        - item.primitive.top_left.y
                        - item.style.stroke_width as i32) as u32,
                    stroke_color.r(),
                    stroke_color.g(),
                    stroke_color.b(),
                )
                .unwrap();

                self._draw_rect(
                    (item.primitive.bottom_right.x as u32 - item.style.stroke_width) as u16,
                    (item.primitive.top_left.y as u32 + item.style.stroke_width) as u16,
                    item.style.stroke_width,
                    (item.primitive.bottom_right.y
                        - item.primitive.top_left.y
                        - item.style.stroke_width as i32) as u32,
                    stroke_color.r(),
                    stroke_color.g(),
                    stroke_color.b(),
                )
                .unwrap();

                self._draw_rect(
                    (item.primitive.top_left.x as u32 + item.style.stroke_width) as u16,
                    (item.primitive.bottom_right.y as u32 - item.style.stroke_width) as u16,
                    (item.primitive.bottom_right.x - item.primitive.top_left.x) as u32
                        - item.style.stroke_width * 2,
                    item.style.stroke_width,
                    stroke_color.r(),
                    stroke_color.g(),
                    stroke_color.b(),
                )
                .unwrap();
            }
            None => {}
        };
        Ok(())
    }

    fn draw_pixel(&mut self, item: Pixel<RGBC>) -> Result<(), Self::Error> {
        self._draw_pixel(
            item.0.x as u16,
            item.0.y as u16,
            item.1.r(),
            item.1.g(),
            item.1.b(),
        )
        .unwrap();
        Ok(())
    }
    fn size(&self) -> Size {
        Size::new(320, 480)
    }
}

#[cfg(test)]
mod tests {}
