pub trait InputPinType {}
pub trait OutputPinType {}

pub struct FloatingInput {}
pub struct PullDownInput {}
pub struct PullUpInput {}
impl InputPinType for FloatingInput {}
impl InputPinType for PullUpInput {}
impl InputPinType for PullDownInput {}

pub struct OpenDrainOutput {}
pub struct PushPullOutput {}
impl OutputPinType for OpenDrainOutput {}
impl OutputPinType for PushPullOutput {}

macro_rules! stm32f1xx_pin_types {
    ($name:ident, [
        $($input_pin_type:ident: ($input_pin_function_name:ident, [
        $(($output_pin_type:ident, $output_pin_function_name:ident),)+]),)+]) => {
        $(
            $(
                impl<'a> IoPin for $name<'a, $input_pin_type, $output_pin_type> {
                    type InputPinError = PinModeError;
                    type OutputPinError = PinModeError;
                    type Input = Self;
                    type Output = Self;

                    fn into_input(&mut self) -> &mut Self::Input {
                        self.pin.$input_pin_function_name(&mut self.cr.borrow_mut());
                        self
                    }
                    fn into_output(&mut self) -> &mut Self::Input {
                        self.pin.$output_pin_function_name(&mut self.cr.borrow_mut());
                        self
                    }
                }
            )+
        )+
    };
}

macro_rules! stm32f1xx_io_pins {
    ($gpio_name:ident, $gpio_module:ident, [
        $($name:ident: ($pin:ident, $crTy:ty),)+]) => {
        #[allow(non_snake_case)]
        pub mod $gpio_module {

            use crate::IoPin;

            use core::cell::RefCell;
            use embedded_hal::digital::v2::InputPin;
            use embedded_hal::digital::v2::OutputPin;
            use stm32f1xx_hal::gpio::Dynamic;
            use stm32f1xx_hal::gpio::{PinModeError};
            use no_std_compat::marker::PhantomData;

            use crate::io::stm32f1xx::{InputPinType, OutputPinType, OpenDrainOutput, PullDownInput, PullUpInput, FloatingInput, PushPullOutput};

            $(
                use stm32f1xx_hal::gpio::$gpio_module::$pin;

                pub struct $name<'a, I: InputPinType, O: OutputPinType> {
                    pin: $pin<Dynamic>,
                    cr: &'a RefCell<$crTy>,
                    _inputPinType: PhantomData<I>,
                    _outputPinType: PhantomData<O>,
                }

                impl<'a, I, O> $name<'a, I, O>
                where I: InputPinType, O: OutputPinType {
                    pub fn new(cr: &'a RefCell<$crTy>, pin: $pin<Dynamic>) -> $name<'a, I, O> {
                        $name { pin: pin, cr: cr, _inputPinType: PhantomData, _outputPinType: PhantomData }
                    }
                }

                impl<'a, I, O> InputPin for $name<'a, I, O>
                where I: InputPinType, O: OutputPinType {
                    type Error = PinModeError;

                    fn is_high(&self) -> Result<bool, Self::Error> {
                        self.pin.is_high()
                    }
                    fn is_low(&self) -> Result<bool, Self::Error> {
                        self.pin.is_low()
                    }
                }

                impl<'a, I, O> OutputPin for $name<'a, I, O>
                where I: InputPinType, O: OutputPinType {
                    type Error = PinModeError;

                    #[inline(always)]
                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        self.pin.set_low()
                    }
                    #[inline(always)]
                    fn set_high(&mut self) -> Result<(), Self::Error> {
                        self.pin.set_high()
                    }
                }

                stm32f1xx_pin_types!($name,
                    [
                        FloatingInput: (make_floating_input, [(PushPullOutput, make_push_pull_output), (OpenDrainOutput, make_open_drain_output),]),
                        PullDownInput: (make_pull_down_input, [(PushPullOutput, make_push_pull_output), (OpenDrainOutput, make_open_drain_output),]),
                        PullUpInput: (make_pull_up_input, [(PushPullOutput, make_push_pull_output), (OpenDrainOutput, make_open_drain_output),]),
                    ]
                );

            )+

            pub struct $gpio_name {}

            impl<'a> $gpio_name {
                $(
                    pub fn $pin<I, O>(pin: $pin<Dynamic>, cr: &'a RefCell<$crTy>) -> $name<'a, I, O>
                    where I: InputPinType , O: OutputPinType {
                         $name::<'a, I, O>::new(cr, pin)
                    }
                )+
            }
        }
    };
}

stm32f1xx_io_pins!(
    GPIOA,
    gpioa,
    [
        PA0IOPin: (PA0, stm32f1xx_hal::gpio::gpioa::CRL),
        PA1IOPin: (PA1, stm32f1xx_hal::gpio::gpioa::CRL),
        PA2IOPin: (PA2, stm32f1xx_hal::gpio::gpioa::CRL),
        PA3IOPin: (PA3, stm32f1xx_hal::gpio::gpioa::CRL),
        PA5IOPin: (PA5, stm32f1xx_hal::gpio::gpioa::CRL),
        PA6IOPin: (PA6, stm32f1xx_hal::gpio::gpioa::CRL),
        PA7IOPin: (PA7, stm32f1xx_hal::gpio::gpioa::CRL),
        PA8IOPin: (PA8, stm32f1xx_hal::gpio::gpioa::CRH),
        PA9IOPin: (PA9, stm32f1xx_hal::gpio::gpioa::CRH),
        PA10IOPin: (PA10, stm32f1xx_hal::gpio::gpioa::CRH),
        PA11IOPin: (PA11, stm32f1xx_hal::gpio::gpioa::CRH),
        PA12IOPin: (PA12, stm32f1xx_hal::gpio::gpioa::CRH),
    ]
);

stm32f1xx_io_pins!(
    GPIOB,
    gpiob,
    [
        PB0IOPin: (PB0, stm32f1xx_hal::gpio::gpiob::CRL),
        PB1IOPin: (PB1, stm32f1xx_hal::gpio::gpiob::CRL),
        PB2IOPin: (PB2, stm32f1xx_hal::gpio::gpiob::CRL),
        PB5IOPin: (PB5, stm32f1xx_hal::gpio::gpiob::CRL),
        PB6IOPin: (PB6, stm32f1xx_hal::gpio::gpiob::CRL),
        PB7IOPin: (PB7, stm32f1xx_hal::gpio::gpiob::CRL),
        PB8IOPin: (PB8, stm32f1xx_hal::gpio::gpiob::CRH),
        PB9IOPin: (PB9, stm32f1xx_hal::gpio::gpiob::CRH),
        PB10IOPin: (PB10, stm32f1xx_hal::gpio::gpiob::CRH),
        PB11IOPin: (PB11, stm32f1xx_hal::gpio::gpiob::CRH),
        PB12IOPin: (PB12, stm32f1xx_hal::gpio::gpiob::CRH),
        PB13IOPin: (PB13, stm32f1xx_hal::gpio::gpiob::CRH),
        PB14IOPin: (PB14, stm32f1xx_hal::gpio::gpiob::CRH),
        PB15IOPin: (PB15, stm32f1xx_hal::gpio::gpiob::CRH),
    ]
);

stm32f1xx_io_pins!(
    GPIOC,
    gpioc,
    [
        PC0IOPin: (PC0, stm32f1xx_hal::gpio::gpioc::CRL),
        PC1IOPin: (PC1, stm32f1xx_hal::gpio::gpioc::CRL),
        PC2IOPin: (PC2, stm32f1xx_hal::gpio::gpioc::CRL),
        PC3IOPin: (PC3, stm32f1xx_hal::gpio::gpioc::CRL),
        PC4IOPin: (PC4, stm32f1xx_hal::gpio::gpioc::CRL),
        PC5IOPin: (PC5, stm32f1xx_hal::gpio::gpioc::CRL),
        PC6IOPin: (PC6, stm32f1xx_hal::gpio::gpioc::CRL),
        PC7IOPin: (PC7, stm32f1xx_hal::gpio::gpioc::CRL),
        PC8IOPin: (PC8, stm32f1xx_hal::gpio::gpioc::CRH),
        PC9IOPin: (PC9, stm32f1xx_hal::gpio::gpioc::CRH),
        PC10IOPin: (PC10, stm32f1xx_hal::gpio::gpioc::CRH),
        PC11IOPin: (PC11, stm32f1xx_hal::gpio::gpioc::CRH),
        PC12IOPin: (PC12, stm32f1xx_hal::gpio::gpioc::CRH),
        PC13IOPin: (PC13, stm32f1xx_hal::gpio::gpioc::CRH),
        PC14IOPin: (PC14, stm32f1xx_hal::gpio::gpioc::CRH),
        PC15IOPin: (PC15, stm32f1xx_hal::gpio::gpioc::CRH),
    ]
);
