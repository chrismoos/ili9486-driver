
/*
 * Example for stm32f4xx
 *
 * $ cargo build --example stm32f4xx_hello_world --features=examples,stm32f4xx,stm32f4xx-hal,stm32f407 --target=thumbv7em-none-eabihf
 */
#![deny(unsafe_code)]
#![no_std]
#![no_main]

use embedded_graphics::image::Image;
use ili9486::gpio::GPIO8ParallelInterface;
use tinytga::Tga;

use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::{Rgb565, Rgb888},
    prelude::*,
    style::{PrimitiveStyle, TextStyle},
};

use ili9486::color::PixelFormat;
#[cfg(feature = "stm32f4xx")]
use ili9486::io::stm32f4xx::gpioa::GPIOA;
#[cfg(feature = "stm32f4xx")]
use ili9486::io::stm32f4xx::gpiob::GPIOB;
#[cfg(feature = "stm32f4xx")]
use ili9486::io::stm32f4xx::gpioc::GPIOC;
use ili9486::{Command, Commands, ILI9486};

extern crate panic_semihosting;

use cortex_m_rt::entry;

#[cfg(feature = "stm32f4xx")]
use stm32f4xx_hal::{
    delay::Delay,
    gpio::{PullDown, PushPull},
};
#[cfg(feature = "stm32f4xx")]
use stm32f4xx_hal::{pac, prelude::*};

#[entry]
#[cfg(not(feature = "stm32f4xx"))]
fn main() -> ! {
    loop {}
}

#[entry]
#[cfg(feature = "stm32f4xx")]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();

    let mut delay = Delay::new(cp.SYST, clocks);

    let pa5 = GPIOA::PA5::<PullDown, PushPull>(gpioa.pa5.into_pull_down_input());
    let pa6 = GPIOA::PA6::<PullDown, PushPull>(gpioa.pa6.into_pull_down_input());
    let pa7 = GPIOA::PA7::<PullDown, PushPull>(gpioa.pa7.into_pull_down_input());

    let pa8 = GPIOA::PA8::<PullDown, PushPull>(gpioa.pa8.into_pull_down_input());
    let pa9 = GPIOA::PA9::<PullDown, PushPull>(gpioa.pa9.into_pull_down_input());
    let pa10 = GPIOA::PA10::<PullDown, PushPull>(gpioa.pa10.into_pull_down_input());

    let pb10 = GPIOB::PB10::<PullDown, PushPull>(gpiob.pb10.into_pull_down_input());
    let pb5 = GPIOB::PB5::<PullDown, PushPull>(gpiob.pb5.into_pull_down_input());
    let pb6 = GPIOB::PB6::<PullDown, PushPull>(gpiob.pb6.into_pull_down_input());
    let pb7 = GPIOB::PB7::<PullDown, PushPull>(gpiob.pb7.into_pull_down_input());
    let pb8 = GPIOB::PB8::<PullDown, PushPull>(gpiob.pb8.into_pull_down_input());
    let pb9 = GPIOB::PB9::<PullDown, PushPull>(gpiob.pb9.into_pull_down_input());

    let pc7 = GPIOC::PC7::<PullDown, PushPull>(gpioc.pc7.into_pull_down_input());

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
