use crate::IoPin;
use display_interface::v2::*;
use display_interface::DisplayError;

use embedded_hal::digital::v2::{InputPin, OutputPin};
pub struct GPIO8ParallelInterface<DB0, DB1, DB2, DB3, DB4, DB5, DB6, DB7, CS, DCX, RDX, WRX>
where
    DB0: IoPin,
    DB1: IoPin,
    DB2: IoPin,
    DB3: IoPin,
    DB4: IoPin,
    DB5: IoPin,
    DB6: IoPin,
    DB7: IoPin,
    CS: IoPin,
    DCX: IoPin,
    RDX: IoPin,
    WRX: IoPin,
{
    db0: DB0,
    db1: DB1,
    db2: DB2,
    db3: DB3,
    db4: DB4,
    db5: DB5,
    db6: DB6,
    db7: DB7,
    cs: CS,
    dcx: DCX,
    rdx: RDX,
    wrx: WRX,
}

impl<DB0, DB1, DB2, DB3, DB4, DB5, DB6, DB7, CS, DCX, RDX, WRX>
    GPIO8ParallelInterface<DB0, DB1, DB2, DB3, DB4, DB5, DB6, DB7, CS, DCX, RDX, WRX>
where
    DB0: IoPin,
    DB1: IoPin,
    DB2: IoPin,
    DB3: IoPin,
    DB4: IoPin,
    DB5: IoPin,
    DB6: IoPin,
    DB7: IoPin,
    CS: IoPin,
    DCX: IoPin,
    RDX: IoPin,
    WRX: IoPin,
{
    pub fn new(
        db0: DB0,
        db1: DB1,
        db2: DB2,
        db3: DB3,
        db4: DB4,
        db5: DB5,
        db6: DB6,
        db7: DB7,
        mut cs: CS,
        mut dcx: DCX,
        mut rdx: RDX,
        mut wrx: WRX,
    ) -> Result<
        GPIO8ParallelInterface<DB0, DB1, DB2, DB3, DB4, DB5, DB6, DB7, CS, DCX, RDX, WRX>,
        DisplayError,
    > {
        let dcx_output = dcx.into_output();
        let cs_output = cs.into_output();
        let rdx_output = rdx.into_output();
        let wrx_output = wrx.into_output();

        wrap_output_err!(dcx_output.set_high())?;
        wrap_output_err!(cs_output.set_high())?;
        wrap_output_err!(rdx_output.set_high())?;
        wrap_output_err!(wrx_output.set_high())?;

        Ok(GPIO8ParallelInterface {
            db0: db0,
            db1: db1,
            db2: db2,
            db3: db3,
            db4: db4,
            db5: db5,
            db6: db6,
            db7: db7,
            cs: cs,
            dcx: dcx,
            rdx: rdx,
            wrx: wrx,
        })
    }
}

impl<DB0, DB1, DB2, DB3, DB4, DB5, DB6, DB7, CS, DCX, RDX, WRX> ReadInterface<u8>
    for GPIO8ParallelInterface<DB0, DB1, DB2, DB3, DB4, DB5, DB6, DB7, CS, DCX, RDX, WRX>
where
    DB0: IoPin,
    DB1: IoPin,
    DB2: IoPin,
    DB3: IoPin,
    DB4: IoPin,
    DB5: IoPin,
    DB6: IoPin,
    DB7: IoPin,
    CS: IoPin,
    DCX: IoPin,
    RDX: IoPin,
    WRX: IoPin,
{
    fn read_stream(&mut self, f: &mut dyn FnMut(u8) -> bool) -> Result<(), DisplayError> {
        let b0_input = &mut self.db0.into_input();
        let b1_input = &mut self.db1.into_input();
        let b2_input = &mut self.db2.into_input();
        let b3_input = &mut self.db3.into_input();
        let b4_input = &mut self.db4.into_input();
        let b5_input = &mut self.db5.into_input();
        let b6_input = &mut self.db6.into_input();
        let b7_input = &mut self.db7.into_input();
        let cs = self.cs.into_output();
        let rdx = self.rdx.into_output();
        let dcx = self.dcx.into_output();
        let wrx = self.wrx.into_output();

        wrap_output_err!(rdx.set_high())?;
        wrap_output_err!(wrx.set_high())?;
        wrap_output_err!(cs.set_low())?;

        wrap_output_err!(dcx.set_high())?;

        loop {
            wrap_output_err!(rdx.set_low())?;
            let mut byte: u8 = 0;

            if wrap_input_err!(b0_input.is_high())? {
                byte |= 1 << 0;
            }
            if wrap_input_err!(b1_input.is_high())? {
                byte |= 1 << 1;
            }
            if wrap_input_err!(b2_input.is_high())? {
                byte |= 1 << 2;
            }
            if wrap_input_err!(b3_input.is_high())? {
                byte |= 1 << 3;
            }
            if wrap_input_err!(b4_input.is_high())? {
                byte |= 1 << 4;
            }
            if wrap_input_err!(b5_input.is_high())? {
                byte |= 1 << 5;
            }
            if wrap_input_err!(b6_input.is_high())? {
                byte |= 1 << 6;
            }
            if wrap_input_err!(b7_input.is_high())? {
                byte |= 1 << 7;
            }

            wrap_output_err!(rdx.set_high())?;
            let read_more = f(byte);
            if !read_more {
                break;
            }
        }
        wrap_output_err!(dcx.set_low())?;
        wrap_output_err!(cs.set_high())?;

        Ok(())
    }
}

impl<DB0, DB1, DB2, DB3, DB4, DB5, DB6, DB7, CS, DCX, RDX, WRX> WriteInterface<u8>
    for GPIO8ParallelInterface<DB0, DB1, DB2, DB3, DB4, DB5, DB6, DB7, CS, DCX, RDX, WRX>
where
    DB0: IoPin,
    DB1: IoPin,
    DB2: IoPin,
    DB3: IoPin,
    DB4: IoPin,
    DB5: IoPin,
    DB6: IoPin,
    DB7: IoPin,
    CS: IoPin,
    DCX: IoPin,
    RDX: IoPin,
    WRX: IoPin,
{
    #[inline(always)]
    fn write_stream<'a>(
        &mut self,
        mode: WriteMode,
        func: &mut dyn FnMut() -> Option<&'a u8>,
    ) -> Result<(), DisplayError> {
        let b0 = self.db0.into_output();
        let b1 = self.db1.into_output();
        let b2 = self.db2.into_output();
        let b3 = self.db3.into_output();
        let b4 = self.db4.into_output();
        let b5 = self.db5.into_output();
        let b6 = self.db6.into_output();
        let b7 = self.db7.into_output();
        let cs = self.cs.into_output();
        let rdx = self.rdx.into_output();
        let dcx = self.dcx.into_output();
        let wrx = self.wrx.into_output();

        wrap_output_err!(rdx.set_high())?;
        wrap_output_err!(wrx.set_high())?;
        wrap_output_err!(cs.set_low())?;

        match mode {
            WriteMode::Command => {
                wrap_output_err!(dcx.set_low())?;
            }
            _ => {
                wrap_output_err!(dcx.set_high())?;
            }
        }

        loop {
            match func() {
                Some(byte) => {
                    wrx.set_low();
                    write_bit!(b0, (1 << 0) & *byte != 0);
                    write_bit!(b1, (1 << 1) & *byte != 0);
                    write_bit!(b2, (1 << 2) & *byte != 0);
                    write_bit!(b3, (1 << 3) & *byte != 0);
                    write_bit!(b4, (1 << 4) & *byte != 0);
                    write_bit!(b5, (1 << 5) & *byte != 0);
                    write_bit!(b6, (1 << 6) & *byte != 0);
                    write_bit!(b7, (1 << 7) & *byte != 0);
                    wrx.set_high();
                }
                None => {
                    break;
                }
            }
        }

        match mode {
            WriteMode::Command => {
                wrap_output_err!(dcx.set_high())?;
            }
            _ => {
                wrap_output_err!(dcx.set_low())?;
            }
        }

        wrap_output_err!(wrx.set_high())?;
        wrap_output_err!(cs.set_high())?;

        Ok(())
    }
}
