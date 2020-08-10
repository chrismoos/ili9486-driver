#![deny(unsafe_code)]
#![no_std]
#![no_main]

use embedded_graphics::image::Image;
use ili9486::gpio::GPIO8ParallelInterface;
use ili9486::io::stm32f1xx::PullDownInput;
use ili9486::io::stm32f1xx::PushPullOutput;
use tinytga::Tga;

use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::{Rgb565, Rgb888},
    prelude::*,
    primitives::Circle,
    style::{PrimitiveStyle, TextStyle},
};

use ili9486::color::PixelFormat;
use ili9486::io::shim::OutputOnlyIoPin;
use ili9486::io::stm32f1xx::gpioa::GPIOA;
use ili9486::io::stm32f1xx::gpiob::GPIOB;
use ili9486::io::stm32f1xx::gpioc::GPIOC;
use ili9486::{Command, Commands, ILI9486};

use display_interface_spi::SPIInterface;

extern crate panic_semihosting;

use core::cell::RefCell;
use cortex_m_rt::entry;

use stm32f1xx_hal::delay::Delay;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    spi::{Mode, Phase, Polarity, Spi},
};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(72.mhz())
        .pclk1(36.mhz())
        .pclk2(56.mhz())
        .freeze(&mut flash.acr);

    let mut delay = Delay::new(cp.SYST, clocks);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let pins = (
        gpiob.pb13.into_alternate_push_pull(&mut gpiob.crh),
        gpiob.pb14.into_floating_input(&mut gpiob.crh),
        gpiob.pb15.into_alternate_push_pull(&mut gpiob.crh),
    );

    let cs = gpiob.pb10.into_push_pull_output(&mut gpiob.crh);
    let dc = gpiob.pb9.into_push_pull_output(&mut gpiob.crh);
    let rst = gpiob.pb8.into_push_pull_output(&mut gpiob.crh);

    let spi_mode = Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    };
    let spi = Spi::spi2(dp.SPI2, pins, spi_mode, 100.khz(), clocks, &mut rcc.apb1);

    let display_spi = SPIInterface::new(spi, dc, cs);

    let mut lcd_driver = ILI9486::new(
        &mut delay,
        PixelFormat::Rgb565,
        display_spi,
        OutputOnlyIoPin::new(rst),
    )
    .unwrap();

    // reset
    lcd_driver.write_command(Command::Nop, &[]).unwrap();
    lcd_driver.write_command(Command::SleepOut, &[]).unwrap();

    lcd_driver
        .write_command(Command::DisplayInversionOff, &[])
        .unwrap();

    // MADCTL settings
    lcd_driver
        .write_command(Command::MemoryAccessControl, &[0b10001000])
        .unwrap();

    lcd_driver.clear_screen().unwrap();

    // turn on display
    lcd_driver
        .write_command(Command::NormalDisplayMode, &[])
        .unwrap();
    lcd_driver.write_command(Command::DisplayOn, &[]).unwrap();
    lcd_driver.write_command(Command::IdleModeOff, &[]).unwrap();

    let t = Text::new("Hello Rust (and ILI9486 display)!", Point::new(64, 175))
        .into_styled(TextStyle::new(Font6x8, Rgb888::GREEN));

    t.draw(&mut lcd_driver).unwrap();

    let tga = Tga::from_slice(include_bytes!("../test/rust-rle-bw-topleft.tga")).unwrap();

    let image: Image<Tga, Rgb888> = Image::new(
        &tga,
        Point::new(
            (320 / 2 - (tga.width() / 2)) as i32,
            ((350 / 2 - (tga.height() / 2)) + 64) as i32,
        ),
    );

    image.draw(&mut lcd_driver).unwrap();
    loop {}
}
