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

extern crate panic_semihosting;

use core::cell::RefCell;
use cortex_m_rt::entry;

use stm32f1xx_hal::delay::Delay;
use stm32f1xx_hal::{pac, prelude::*};

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

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
    let mut delay = Delay::new(cp.SYST, clocks);

    let pa5 = OutputOnlyIoPin::new(gpioa.pa5.into_push_pull_output(&mut gpioa.crl));
    let pa6 = OutputOnlyIoPin::new(gpioa.pa6.into_push_pull_output(&mut gpioa.crl));
    let pa7 = OutputOnlyIoPin::new(gpioa.pa7.into_push_pull_output(&mut gpioa.crl));
    let pa8 = OutputOnlyIoPin::new(gpioa.pa8.into_push_pull_output(&mut gpioa.crh));
    let pa9 = OutputOnlyIoPin::new(gpioa.pa9.into_push_pull_output(&mut gpioa.crh));
    let pa10 = OutputOnlyIoPin::new(gpioa.pa10.into_push_pull_output(&mut gpioa.crh));
    let pa11 = OutputOnlyIoPin::new(gpioa.pa11.into_push_pull_output(&mut gpioa.crh));
    let pa12 = OutputOnlyIoPin::new(gpioa.pa12.into_push_pull_output(&mut gpioa.crh));

    let pb10 = OutputOnlyIoPin::new(gpiob.pb10.into_push_pull_output(&mut gpiob.crh));
    let pb5 = OutputOnlyIoPin::new(gpiob.pb5.into_push_pull_output(&mut gpiob.crl));
    let pb6 = OutputOnlyIoPin::new(gpiob.pb6.into_push_pull_output(&mut gpiob.crl));
    let pb7 = OutputOnlyIoPin::new(gpiob.pb7.into_push_pull_output(&mut gpiob.crl));
    let pb8 = OutputOnlyIoPin::new(gpiob.pb8.into_push_pull_output(&mut gpiob.crh));
    let pb9 = OutputOnlyIoPin::new(gpiob.pb9.into_push_pull_output(&mut gpiob.crh));
    let pc7 = OutputOnlyIoPin::new(gpioc.pc7.into_push_pull_output(&mut gpioc.crl));

    let parallel_gpio =
        GPIO8ParallelInterface::new(pa5, pa6, pa7, pa8, pa9, pa10, pc7, pb10, pb6, pb8, pb7, pb9)
            .unwrap();
    let mut lcd_driver = ILI9486::new(&mut delay, PixelFormat::Rgb565, parallel_gpio, pb5).unwrap();

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
