use crate::gpio::U18;
use display_interface::v2::*;
use display_interface::DisplayError;

pub enum PixelFormat {
    Rgb565,
    Rgb666,
}

/// (R, G, B)
pub type RGBPixel = (u8, u8, u8);

pub trait PixelWriter<T> {
    fn write_pixel_data(
        &mut self,
        pixel_format: &PixelFormat,
        pixel_a: &RGBPixel,
        pixel_b: Option<&RGBPixel>,
    ) -> Result<(), DisplayError>;

    fn write_repeated_pixel_data(
        &mut self,
        pixel_format: &PixelFormat,
        pixel: &RGBPixel,
        num: usize,
    ) -> Result<(), DisplayError> {
        for _ in 0..num {
            self.write_pixel_data(pixel_format, pixel, None)?;
        }
        Ok(())
    }
}

fn encode_rgb565_8bit(pixel: &RGBPixel) -> (u8, u8) {
    (
        ((pixel.0 & 0b11111) << 3) | ((pixel.1 >> 2) & 0b111),
        ((pixel.1 << 5) & 0b11100000) | (pixel.2 & 0b11111),
    )
}

impl<T> PixelWriter<u8> for T
where
    T: ReadWriteInterface<u8>,
{
    fn write_repeated_pixel_data(
        &mut self,
        pixel_format: &PixelFormat,
        pixel: &RGBPixel,
        num: usize,
    ) -> Result<(), DisplayError> {
        match pixel_format {
            PixelFormat::Rgb565 => {
                let (hi, lo) = encode_rgb565_8bit(pixel);
                let mut index = num * 2;
                self.write_stream(WriteMode::Data, &mut || {
                    if index == 0 {
                        None
                    } else {
                        index -= 1;
                        if index % 2 == 0 {
                            Some(&lo)
                        } else {
                            Some(&hi)
                        }
                    }
                })
            }
            PixelFormat::Rgb666 => {
                for _ in 0..num {
                    self.write_pixel_data(pixel_format, pixel, None)?;
                }
                Ok(())
            }
        }
    }
    fn write_pixel_data(
        &mut self,
        pixel_format: &PixelFormat,
        pixel_a: &RGBPixel,
        pixel_b: Option<&RGBPixel>,
    ) -> Result<(), DisplayError> {
        match pixel_format {
            PixelFormat::Rgb565 => {
                let (hi, lo) = encode_rgb565_8bit(pixel_a);
                self.write(WriteMode::Data, &[hi, lo])?;
                match pixel_b {
                    Some(pixel) => {
                        let (hi, lo) = encode_rgb565_8bit(pixel);
                        self.write(WriteMode::Data, &[hi, lo])?;
                    }
                    None => {}
                };
                Ok(())
            }
            PixelFormat::Rgb666 => {
                self.write(
                    WriteMode::Data,
                    &[pixel_a.0 << 2, pixel_a.1 << 2, pixel_a.2 << 2],
                )?;

                match pixel_b {
                    Some(pixel) => {
                        self.write(WriteMode::Data, &[pixel.0 << 2, pixel.1 << 2, pixel.2 << 2])?;
                    }
                    None => {}
                }
                Ok(())
            }
        }
    }
}

fn encode_rgb565_16bit(pixel: &RGBPixel) -> u16 {
    (((pixel.0 & 0b11111) as u16) << 10)
        | (((pixel.1 & 0b111111) as u16) << 4)
        | (pixel.2 & 0b11111) as u16
}

fn encode_rgb666_16bit(pixel_a: &RGBPixel, pixel_b: &RGBPixel) -> (u16, u16, u16) {
    let one = (((pixel_a.0 & 0b111111) as u16) << 10) | (((pixel_a.1 & 0b111111) as u16) << 2);
    let two = (((pixel_a.2 & 0b111111) as u16) << 10) | (((pixel_b.0 & 0b111111) as u16) << 2);
    let three = (((pixel_b.1 & 0b111111) as u16) << 10) | (((pixel_b.2 & 0b111111) as u16) << 2);
    (one, two, three)
}

impl<T> PixelWriter<u16> for T
where
    T: ReadWriteInterface<u16>,
{
    fn write_pixel_data(
        &mut self,
        pixel_format: &PixelFormat,
        pixel_a: &RGBPixel,
        pixel_b: Option<&RGBPixel>,
    ) -> Result<(), DisplayError> {
        match pixel_format {
            PixelFormat::Rgb565 => self.write(WriteMode::Data, &[encode_rgb565_16bit(pixel_a)]),
            PixelFormat::Rgb666 => match pixel_b {
                Some(b) => {
                    let (one, two, three) = encode_rgb666_16bit(pixel_a, b);
                    self.write(WriteMode::Data, &[one, two, three])
                }
                None => {
                    let (one, two, _three) = encode_rgb666_16bit(pixel_a, &(0, 0, 0));
                    self.write(WriteMode::Data, &[one, two])
                }
            },
        }
    }

    fn write_repeated_pixel_data(
        &mut self,
        pixel_format: &PixelFormat,
        pixel: &RGBPixel,
        num: usize,
    ) -> Result<(), DisplayError> {
        match pixel_format {
            PixelFormat::Rgb565 => {
                let data = encode_rgb565_16bit(pixel);
                let mut index = num;
                self.write_stream(WriteMode::Data, &mut || {
                    if index == 0 {
                        None
                    } else {
                        index -= 1;
                        Some(&data)
                    }
                })
            }
            PixelFormat::Rgb666 => {
                for _ in 0..num {
                    self.write_pixel_data(pixel_format, pixel, None)?;
                }
                Ok(())
            }
        }
    }
}

fn encode_rgb666_18bit(pixel: &RGBPixel) -> u32 {
    (((pixel.0 & 0b111111) as u32) << 12)
        | (((pixel.1 & 0b111111) as u32) << 6)
        | ((pixel.2 & 0b111111) as u32)
}

impl<T> PixelWriter<U18> for T
where
    T: ReadWriteInterface<U18>,
{
    fn write_pixel_data(
        &mut self,
        pixel_format: &PixelFormat,
        pixel_a: &RGBPixel,
        pixel_b: Option<&RGBPixel>,
    ) -> Result<(), DisplayError> {
        match pixel_format {
            PixelFormat::Rgb565 => Err(DisplayError::DataFormatNotImplemented),
            PixelFormat::Rgb666 => match pixel_b {
                Some(b) => self.write(
                    WriteMode::Data,
                    &[encode_rgb666_18bit(pixel_a), encode_rgb666_18bit(b)],
                ),
                None => self.write(WriteMode::Data, &[encode_rgb666_18bit(pixel_a)]),
            },
        }
    }
}
