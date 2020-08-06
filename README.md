# ILI9486 driver for Rust

![crate](https://img.shields.io/crates/v/ili9486) ![docs](https://docs.rs/ili9486-driver/badge.svg)

This crate contains a driver for the ILI9486 LCD controller.

See the [full example](examples/full.rs) for usage, or check out the documentation.

<img src="./examples/hello_world.jpg" width="200" />

## Tasks

- [x] GPIO 8-bit Parallel Interface
- [ ] GPIO 16-bit Parallel Interface (Needs testing, needs Rgb666 support)
- [ ] Serial Interface (3 and 4-wire)

## Example

Setup the LCD with the 8-bit parallel interface, and draw some text and an image. See this whole example [here](./examples/hello_world.rs).

```rust
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
```

## License

This project uses the Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0).