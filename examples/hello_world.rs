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
use ili9486::io::stm32f1xx::gpioa::GPIOA;
use ili9486::io::stm32f1xx::gpiob::GPIOB;
use ili9486::io::stm32f1xx::gpioc::GPIOC;
use ili9486::{Commands, ILI9486};

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

    let gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let gpioc = dp.GPIOC.split(&mut rcc.apb2);

    let gpioa_crl_ref = RefCell::new(gpioa.crl);
    let gpioa_crh_ref = RefCell::new(gpioa.crh);
    let gpiob_crl_ref = RefCell::new(gpiob.crl);
    let gpiob_crh_ref = RefCell::new(gpiob.crh);
    let gpioc_crl_ref = RefCell::new(gpioc.crl);
    let mut delay = Delay::new(cp.SYST, clocks);

    let pa5 = GPIOA::PA5::<PullDownInput, PushPullOutput>(
        gpioa.pa5.into_dynamic(&mut gpioa_crl_ref.borrow_mut()),
        &gpioa_crl_ref,
    );
    let pa6 = GPIOA::PA6::<PullDownInput, PushPullOutput>(
        gpioa.pa6.into_dynamic(&mut gpioa_crl_ref.borrow_mut()),
        &gpioa_crl_ref,
    );
    let pa7 = GPIOA::PA7::<PullDownInput, PushPullOutput>(
        gpioa.pa7.into_dynamic(&mut gpioa_crl_ref.borrow_mut()),
        &gpioa_crl_ref,
    );
    let pa8 = GPIOA::PA8::<PullDownInput, PushPullOutput>(
        gpioa.pa8.into_dynamic(&mut gpioa_crh_ref.borrow_mut()),
        &gpioa_crh_ref,
    );
    let pa9 = GPIOA::PA9::<PullDownInput, PushPullOutput>(
        gpioa.pa9.into_dynamic(&mut gpioa_crh_ref.borrow_mut()),
        &gpioa_crh_ref,
    );
    let pa10 = GPIOA::PA10::<PullDownInput, PushPullOutput>(
        gpioa.pa10.into_dynamic(&mut gpioa_crh_ref.borrow_mut()),
        &gpioa_crh_ref,
    );
    let _pa11 = GPIOA::PA11::<PullDownInput, PushPullOutput>(
        gpioa.pa11.into_dynamic(&mut gpioa_crh_ref.borrow_mut()),
        &gpioa_crh_ref,
    );
    let _pa12 = GPIOA::PA12::<PullDownInput, PushPullOutput>(
        gpioa.pa12.into_dynamic(&mut gpioa_crh_ref.borrow_mut()),
        &gpioa_crh_ref,
    );
    let pb10 = GPIOB::PB10::<PullDownInput, PushPullOutput>(
        gpiob.pb10.into_dynamic(&mut gpiob_crh_ref.borrow_mut()),
        &gpiob_crh_ref,
    );
    let pb5 = GPIOB::PB5::<PullDownInput, PushPullOutput>(
        gpiob.pb5.into_dynamic(&mut gpiob_crl_ref.borrow_mut()),
        &gpiob_crl_ref,
    );
    let pb6 = GPIOB::PB6::<PullDownInput, PushPullOutput>(
        gpiob.pb6.into_dynamic(&mut gpiob_crl_ref.borrow_mut()),
        &gpiob_crl_ref,
    );
    let pb7 = GPIOB::PB7::<PullDownInput, PushPullOutput>(
        gpiob.pb7.into_dynamic(&mut gpiob_crl_ref.borrow_mut()),
        &gpiob_crl_ref,
    );
    let pb8 = GPIOB::PB8::<PullDownInput, PushPullOutput>(
        gpiob.pb8.into_dynamic(&mut gpiob_crh_ref.borrow_mut()),
        &gpiob_crh_ref,
    );
    let pb9 = GPIOB::PB9::<PullDownInput, PushPullOutput>(
        gpiob.pb9.into_dynamic(&mut gpiob_crh_ref.borrow_mut()),
        &gpiob_crh_ref,
    );
    let pc7 = GPIOC::PC7::<PullDownInput, PushPullOutput>(
        gpioc.pc7.into_dynamic(&mut gpioc_crl_ref.borrow_mut()),
        &gpioc_crl_ref,
    );

    let parallel_gpio =
        GPIO8ParallelInterface::new(pa5, pa6, pa7, pa8, pa9, pa10, pc7, pb10, pb6, pb8, pb7, pb9)
            .unwrap();
    let mut lcd_driver = ILI9486::new(&mut delay, PixelFormat::Rgb565, parallel_gpio, pb5).unwrap();

    // reset
    lcd_driver.write_command(0x01, &[]).unwrap();
    lcd_driver.write_command(0x11, &[]).unwrap();

    lcd_driver.write_command(0x20, &[]).unwrap();

    // MADCTL settings
    lcd_driver.write_command(0x36, &[0b10001000]).unwrap();

    lcd_driver.clear_screen().unwrap();

    // turn on display
    lcd_driver.write_command(0x13, &[]).unwrap();
    lcd_driver.write_command(0x29, &[]).unwrap();
    lcd_driver.write_command(0x38, &[]).unwrap();

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
