pub trait InputPinType {}
pub trait OutputPinType {}

use stm32f4xx_hal::gpio::{Floating, OpenDrain, PullDown, PullUp, PushPull};
impl InputPinType for Floating {}
impl InputPinType for PullUp {}
impl InputPinType for PullDown {}

impl OutputPinType for OpenDrain {}
impl OutputPinType for PushPull {}

macro_rules! stm32f4xx_pin_types {
    ($name:ident, [
        $($input_pin_type:ident: ($input_pin_function_name:ident, [
        $(($output_pin_type:ident, $output_pin_function_name:ident),)+]),)+]) => {
        $(
            $(
                impl IoPin for $name<$input_pin_type, $output_pin_type> {
                    type InputPinError = Infallible;
                    type OutputPinError = Infallible;
                    type Input = Self;
                    type Output = Self;

                    fn into_input(&mut self) -> &mut Self::Input {
                        match self.pin {
                            Some(_) => {
                                return self;
                            },
                            None => {
                                self.pin = Some(self.opin.take().unwrap().$input_pin_function_name());
                                self
                            }
                        }
                    }
                    fn into_output(&mut self) -> &mut Self::Input {
                        match self.opin {
                            Some(_) => {
                                return self;
                            },
                            None => {
                                self.opin = Some(self.pin.take().unwrap().$output_pin_function_name());
                                self
                            }
                        }
                    }
                }
            )+
        )+
    };
}

macro_rules! stm32f4xx_io_pins {
    ($gpio_name:ident, $gpio_module:ident, [
        $($name:ident: ($pin:ident),)+]) => {
        #[allow(non_snake_case)]
        pub mod $gpio_module {

            use crate::IoPin;
            use core::convert::Infallible;

            use embedded_hal::digital::v2::InputPin;
            use embedded_hal::digital::v2::OutputPin;
            use stm32f4xx_hal::gpio::{PushPull, PullDown, PullUp, OpenDrain, Floating};
            use no_std_compat::marker::PhantomData;

            use crate::io::stm32f4xx::{InputPinType, OutputPinType};

            $(
                use stm32f4xx_hal::gpio::$gpio_module::$pin;

                pub struct $name<I: InputPinType, O: OutputPinType> {
                    pin: Option<$pin<stm32f4xx_hal::gpio::Input<I>>>,
                    opin: Option<$pin<stm32f4xx_hal::gpio::Output<O>>>,
                    _inputPinType: PhantomData<I>,
                    _outputPinType: PhantomData<O>,
                }

                impl<I, O> $name<I, O>
                where I: InputPinType, O: OutputPinType {
                    pub fn new(pin: Option<$pin<stm32f4xx_hal::gpio::Input<I>>>, output_pin: Option<$pin<stm32f4xx_hal::gpio::Output<O>>>) -> $name<I, O> {
                        $name { pin: pin, opin: output_pin, _inputPinType: PhantomData, _outputPinType: PhantomData }
                    }
                }

                impl<I, O> InputPin for $name<I, O>
                where I: InputPinType, O: OutputPinType {
                    type Error = Infallible;

                    fn is_high(&self) -> Result<bool, Self::Error> {
                        self.pin.as_ref().unwrap().is_high()
                    }
                    fn is_low(&self) -> Result<bool, Self::Error> {
                        self.pin.as_ref().unwrap().is_low()
                    }
                }

                impl<I, O> OutputPin for $name<I, O>
                where I: InputPinType, O: OutputPinType {
                    type Error = Infallible;

                    #[inline(always)]
                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        self.opin.as_mut().unwrap().set_low()
                    }
                    #[inline(always)]
                    fn set_high(&mut self) -> Result<(), Self::Error> {
                        self.opin.as_mut().unwrap().set_high()
                    }
                }

                stm32f4xx_pin_types!($name,
                    [
                        Floating: (into_floating_input, [(PushPull, into_push_pull_output), (OpenDrain, into_open_drain_output),]),
                        PullDown: (into_pull_down_input, [(PushPull, into_push_pull_output), (OpenDrain, into_open_drain_output),]),
                        PullUp: (into_pull_up_input, [(PushPull, into_push_pull_output), (OpenDrain, into_open_drain_output),]),
                    ]
                );

            )+

            pub struct $gpio_name {}

            impl $gpio_name {
                $(
                    pub fn $pin<I, O>(pin: $pin<stm32f4xx_hal::gpio::Input<I>>) -> $name<I, O>
                    where I: InputPinType , O: OutputPinType {
                         $name::<I, O>::new(Some(pin), None)
                    }
                )+
            }
        }
    };
}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
stm32f4xx_io_pins!(
    GPIOA,
    gpioa,
    [
        PA0IOPin: (PA0),
        PA1IOPin: (PA1),
        PA2IOPin: (PA2),
        PA3IOPin: (PA3),
        PA5IOPin: (PA5),
        PA6IOPin: (PA6),
        PA7IOPin: (PA7),
        PA8IOPin: (PA8),
        PA9IOPin: (PA9),
        PA10IOPin: (PA10),
        PA11IOPin: (PA11),
        PA12IOPin: (PA12),
        PA13IOPin: (PA13),
        PA14IOPin: (PA14),
        PA15IOPin: (PA15),
    ]
);

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
stm32f4xx_io_pins!(
    GPIOB,
    gpiob,
    [
        PB0IOPin: (PB0),
        PB1IOPin: (PB1),
        PB2IOPin: (PB2),
        PB5IOPin: (PB5),
        PB6IOPin: (PB6),
        PB7IOPin: (PB7),
        PB8IOPin: (PB8),
        PB9IOPin: (PB9),
        PB10IOPin: (PB10),
        PB11IOPin: (PB11),
        PB12IOPin: (PB12),
        PB13IOPin: (PB13),
        PB14IOPin: (PB14),
        PB15IOPin: (PB15),
    ]
);

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
stm32f4xx_io_pins!(
    GPIOC,
    gpioc,
    [
        PC0IOPin: (PC0),
        PC1IOPin: (PC1),
        PC2IOPin: (PC2),
        PC3IOPin: (PC3),
        PC4IOPin: (PC4),
        PC5IOPin: (PC5),
        PC6IOPin: (PC6),
        PC7IOPin: (PC7),
        PC8IOPin: (PC8),
        PC9IOPin: (PC9),
        PC10IOPin: (PC10),
        PC11IOPin: (PC11),
        PC12IOPin: (PC12),
        PC13IOPin: (PC13),
        PC14IOPin: (PC14),
        PC15IOPin: (PC15),
    ]
);

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
stm32f4xx_io_pins!(
    GPIOD,
    gpiod,
    [
        PD0IOPin: (PD0),
        PD1IOPin: (PD1),
        PD2IOPin: (PD2),
        PD3IOPin: (PD3),
        PD4IOPin: (PD4),
        PD5IOPin: (PD5),
        PD6IOPin: (PD6),
        PD7IOPin: (PD7),
        PD8IOPin: (PD8),
        PD9IOPin: (PD9),
        PD10IOPin: (PD10),
        PD11IOPin: (PD11),
        PD12IOPin: (PD12),
        PD13IOPin: (PD13),
        PD14IOPin: (PD14),
        PD15IOPin: (PD15),
    ]
);

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
stm32f4xx_io_pins!(
    GPIOE,
    gpioe,
    [
        PE0IOPin: (PE0),
        PE1IOPin: (PE1),
        PE2IOPin: (PE2),
        PE3IOPin: (PE3),
        PE4IOPin: (PE4),
        PE5IOPin: (PE5),
        PE6IOPin: (PE6),
        PE7IOPin: (PE7),
        PE8IOPin: (PE8),
        PE9IOPin: (PE9),
        PE10IOPin: (PE10),
        PE11IOPin: (PE11),
        PE12IOPin: (PE12),
        PE13IOPin: (PE13),
        PE14IOPin: (PE14),
        PE15IOPin: (PE15),
    ]
);

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
stm32f4xx_io_pins!(
    GPIOF,
    gpiof,
    [
        PF0IOPin: (PF0),
        PF1IOPin: (PF1),
        PF2IOPin: (PF2),
        PF3IOPin: (PF3),
        PF4IOPin: (PF4),
        PF5IOPin: (PF5),
        PF6IOPin: (PF6),
        PF7IOPin: (PF7),
        PF8IOPin: (PF8),
        PF9IOPin: (PF9),
        PF10IOPin: (PF10),
        PF11IOPin: (PF11),
        PF12IOPin: (PF12),
        PF13IOPin: (PF13),
        PF14IOPin: (PF14),
        PF15IOPin: (PF15),
    ]
);

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
stm32f4xx_io_pins!(
    GPIOG,
    gpiog,
    [
        PG0IOPin: (PG0),
        PG1IOPin: (PG1),
        PG2IOPin: (PG2),
        PG3IOPin: (PG3),
        PG4IOPin: (PG4),
        PG5IOPin: (PG5),
        PG6IOPin: (PG6),
        PG7IOPin: (PG7),
        PG8IOPin: (PG8),
        PG9IOPin: (PG9),
        PG10IOPin: (PG10),
        PG11IOPin: (PG11),
        PG12IOPin: (PG12),
        PG13IOPin: (PG13),
        PG14IOPin: (PG14),
        PG15IOPin: (PG15),
    ]
);

#[cfg(any(feature = "stm32f401"))]
stm32f4xx_io_pins!(GPIOH, gpioh, [PH0IOPin: (PH0), PH1IOPin: (PH1),]);

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
stm32f4xx_io_pins!(
    GPIOH,
    gpioh,
    [
        PH0IOPin: (PH0),
        PH1IOPin: (PH1),
        PH2IOPin: (PH2),
        PH3IOPin: (PH3),
        PH4IOPin: (PH4),
        PH5IOPin: (PH5),
        PH6IOPin: (PH6),
        PH7IOPin: (PH7),
        PH8IOPin: (PH8),
        PH9IOPin: (PH9),
        PH10IOPin: (PH10),
        PH11IOPin: (PH11),
        PH12IOPin: (PH12),
        PH13IOPin: (PH13),
        PH14IOPin: (PH14),
        PH15IOPin: (PH15),
    ]
);

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
stm32f4xx_io_pins!(
    GPIOI,
    gpioi,
    [
        PI0IOPin: (PI0),
        PI1IOPin: (PI1),
        PI2IOPin: (PI2),
        PI3IOPin: (PI3),
        PI4IOPin: (PI4),
        PI5IOPin: (PI5),
        PI6IOPin: (PI6),
        PI7IOPin: (PI7),
        PI8IOPin: (PI8),
        PI9IOPin: (PI9),
        PI10IOPin: (PI10),
        PI11IOPin: (PI11),
        PI12IOPin: (PI12),
        PI13IOPin: (PI13),
        PI14IOPin: (PI14),
        PI15IOPin: (PI15),
    ]
);

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
stm32f4xx_io_pins!(
    GPIOJ,
    gpioj,
    [
        PJ0IOPin: (PJ0),
        PJ1IOPin: (PJ1),
        PJ2IOPin: (PJ2),
        PJ3IOPin: (PJ3),
        PJ4IOPin: (PJ4),
        PJ5IOPin: (PJ5),
        PJ6IOPin: (PJ6),
        PJ7IOPin: (PJ7),
        PJ8IOPin: (PJ8),
        PJ9IOPin: (PJ9),
        PJ10IOPin: (PJ10),
        PJ11IOPin: (PJ11),
        PJ12IOPin: (PJ12),
        PJ13IOPin: (PJ13),
        PJ14IOPin: (PJ14),
        PJ15IOPin: (PJ15),
    ]
);

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
stm32f4xx_io_pins!(
    GPIOK,
    gpiok,
    [
        PK0IOPin: (PK0),
        PK1IOPin: (PK1),
        PK2IOPin: (PK2),
        PK3IOPin: (PK3),
        PK4IOPin: (PK4),
        PK5IOPin: (PK5),
        PK6IOPin: (PK6),
        PK7IOPin: (PK7),
        PK8IOPin: (PK8),
        PK9IOPin: (PK9),
        PK10IOPin: (PK10),
        PK11IOPin: (PK11),
        PK12IOPin: (PK12),
        PK13IOPin: (PK13),
        PK14IOPin: (PK14),
        PK15IOPin: (PK15),
    ]
);
