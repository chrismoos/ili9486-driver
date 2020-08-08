use crate::IoPin;
use core::fmt::Debug;
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;

#[derive(Debug)]
pub enum Error {
    InputNotAvailableError,
}

pub struct OutputOnlyIoPin<P>
where
    P: OutputPin,
    P::Error: Debug,
{
    pin: P,
}

impl<P> OutputOnlyIoPin<P>
where
    P: OutputPin,
    P::Error: Debug,
{
    pub fn new(pin: P) -> OutputOnlyIoPin<P> {
        OutputOnlyIoPin { pin: pin }
    }
}

impl<P> IoPin for OutputOnlyIoPin<P>
where
    P: OutputPin,
    P::Error: Debug,
{
    type InputPinError = Error;
    type OutputPinError = P::Error;
    type Input = Self;
    type Output = Self;

    fn into_input(&mut self) -> &mut Self::Input {
        self
    }

    fn into_output(&mut self) -> &mut Self::Output {
        self
    }
}
impl<'a, P> InputPin for OutputOnlyIoPin<P>
where
    P: OutputPin,
    P::Error: Debug,
{
    type Error = Error;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Err(Error::InputNotAvailableError)
    }
    fn is_low(&self) -> Result<bool, Self::Error> {
        Err(Error::InputNotAvailableError)
    }
}

impl<'a, P> OutputPin for OutputOnlyIoPin<P>
where
    P: OutputPin,
    P::Error: Debug,
{
    type Error = P::Error;

    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }
}
