use crate::IoPin;
use display_interface::v2::*;
use display_interface::DisplayError;

use embedded_hal::digital::v2::{InputPin, OutputPin};
pub struct GPIO16ParallelInterface<
    DB0,
    DB1,
    DB2,
    DB3,
    DB4,
    DB5,
    DB6,
    DB7,
    DB8,
    DB9,
    DB10,
    DB11,
    DB12,
    DB13,
    DB14,
    DB15,
    CS,
    DCX,
    RDX,
    WRX,
> where
    DB0: IoPin,
    DB1: IoPin,
    DB2: IoPin,
    DB3: IoPin,
    DB4: IoPin,
    DB5: IoPin,
    DB6: IoPin,
    DB7: IoPin,
    DB8: IoPin,
    DB9: IoPin,
    DB10: IoPin,
    DB11: IoPin,
    DB12: IoPin,
    DB13: IoPin,
    DB14: IoPin,
    DB15: IoPin,
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
    db8: DB8,
    db9: DB9,
    db10: DB10,
    db11: DB11,
    db12: DB12,
    db13: DB13,
    db14: DB14,
    db15: DB15,
    cs: CS,
    dcx: DCX,
    rdx: RDX,
    wrx: WRX,
}

impl<
        DB0,
        DB1,
        DB2,
        DB3,
        DB4,
        DB5,
        DB6,
        DB7,
        DB8,
        DB9,
        DB10,
        DB11,
        DB12,
        DB13,
        DB14,
        DB15,
        CS,
        DCX,
        RDX,
        WRX,
    >
    GPIO16ParallelInterface<
        DB0,
        DB1,
        DB2,
        DB3,
        DB4,
        DB5,
        DB6,
        DB7,
        DB8,
        DB9,
        DB10,
        DB11,
        DB12,
        DB13,
        DB14,
        DB15,
        CS,
        DCX,
        RDX,
        WRX,
    >
where
    DB0: IoPin,
    DB1: IoPin,
    DB2: IoPin,
    DB3: IoPin,
    DB4: IoPin,
    DB5: IoPin,
    DB6: IoPin,
    DB7: IoPin,
    DB8: IoPin,
    DB9: IoPin,
    DB10: IoPin,
    DB11: IoPin,
    DB12: IoPin,
    DB13: IoPin,
    DB14: IoPin,
    DB15: IoPin,
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
        db8: DB8,
        db9: DB9,
        db10: DB10,
        db11: DB11,
        db12: DB12,
        db13: DB13,
        db14: DB14,
        db15: DB15,
        mut cs: CS,
        mut dcx: DCX,
        mut rdx: RDX,
        mut wrx: WRX,
    ) -> Result<
        GPIO16ParallelInterface<
            DB0,
            DB1,
            DB2,
            DB3,
            DB4,
            DB5,
            DB6,
            DB7,
            DB8,
            DB9,
            DB10,
            DB11,
            DB12,
            DB13,
            DB14,
            DB15,
            CS,
            DCX,
            RDX,
            WRX,
        >,
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

        Ok(GPIO16ParallelInterface {
            db0: db0,
            db1: db1,
            db2: db2,
            db3: db3,
            db4: db4,
            db5: db5,
            db6: db6,
            db7: db7,
            db8: db8,
            db9: db9,
            db10: db10,
            db11: db11,
            db12: db12,
            db13: db13,
            db14: db14,
            db15: db15,
            cs: cs,
            dcx: dcx,
            rdx: rdx,
            wrx: wrx,
        })
    }
}

impl<
        DB0,
        DB1,
        DB2,
        DB3,
        DB4,
        DB5,
        DB6,
        DB7,
        DB8,
        DB9,
        DB10,
        DB11,
        DB12,
        DB13,
        DB14,
        DB15,
        CS,
        DCX,
        RDX,
        WRX,
    > ReadInterface<u16>
    for GPIO16ParallelInterface<
        DB0,
        DB1,
        DB2,
        DB3,
        DB4,
        DB5,
        DB6,
        DB7,
        DB8,
        DB9,
        DB10,
        DB11,
        DB12,
        DB13,
        DB14,
        DB15,
        CS,
        DCX,
        RDX,
        WRX,
    >
where
    DB0: IoPin,
    DB1: IoPin,
    DB2: IoPin,
    DB3: IoPin,
    DB4: IoPin,
    DB5: IoPin,
    DB6: IoPin,
    DB7: IoPin,
    DB8: IoPin,
    DB9: IoPin,
    DB10: IoPin,
    DB11: IoPin,
    DB12: IoPin,
    DB13: IoPin,
    DB14: IoPin,
    DB15: IoPin,
    CS: IoPin,
    DCX: IoPin,
    RDX: IoPin,
    WRX: IoPin,
{
    fn read_stream(&mut self, f: &mut dyn FnMut(u16) -> bool) -> Result<(), DisplayError> {
        let b0_input = &mut self.db0.into_input();
        let b1_input = &mut self.db1.into_input();
        let b2_input = &mut self.db2.into_input();
        let b3_input = &mut self.db3.into_input();
        let b4_input = &mut self.db4.into_input();
        let b5_input = &mut self.db5.into_input();
        let b6_input = &mut self.db6.into_input();
        let b7_input = &mut self.db7.into_input();
        let b8_input = &mut self.db8.into_input();
        let b9_input = &mut self.db9.into_input();
        let b10_input = &mut self.db10.into_input();
        let b11_input = &mut self.db11.into_input();
        let b12_input = &mut self.db12.into_input();
        let b13_input = &mut self.db13.into_input();
        let b14_input = &mut self.db14.into_input();
        let b15_input = &mut self.db15.into_input();
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
            let mut byte: u16 = 0;

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
            if wrap_input_err!(b8_input.is_high())? {
                byte |= 1 << 8;
            }
            if wrap_input_err!(b9_input.is_high())? {
                byte |= 1 << 9;
            }
            if wrap_input_err!(b10_input.is_high())? {
                byte |= 1 << 10;
            }
            if wrap_input_err!(b11_input.is_high())? {
                byte |= 1 << 11;
            }
            if wrap_input_err!(b12_input.is_high())? {
                byte |= 1 << 12;
            }
            if wrap_input_err!(b13_input.is_high())? {
                byte |= 1 << 13;
            }
            if wrap_input_err!(b14_input.is_high())? {
                byte |= 1 << 14;
            }
            if wrap_input_err!(b15_input.is_high())? {
                byte |= 1 << 15;
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

impl<
        DB0,
        DB1,
        DB2,
        DB3,
        DB4,
        DB5,
        DB6,
        DB7,
        DB8,
        DB9,
        DB10,
        DB11,
        DB12,
        DB13,
        DB14,
        DB15,
        CS,
        DCX,
        RDX,
        WRX,
    > WriteInterface<u16>
    for GPIO16ParallelInterface<
        DB0,
        DB1,
        DB2,
        DB3,
        DB4,
        DB5,
        DB6,
        DB7,
        DB8,
        DB9,
        DB10,
        DB11,
        DB12,
        DB13,
        DB14,
        DB15,
        CS,
        DCX,
        RDX,
        WRX,
    >
where
    DB0: IoPin,
    DB1: IoPin,
    DB2: IoPin,
    DB3: IoPin,
    DB4: IoPin,
    DB5: IoPin,
    DB6: IoPin,
    DB7: IoPin,
    DB8: IoPin,
    DB9: IoPin,
    DB10: IoPin,
    DB11: IoPin,
    DB12: IoPin,
    DB13: IoPin,
    DB14: IoPin,
    DB15: IoPin,
    CS: IoPin,
    DCX: IoPin,
    RDX: IoPin,
    WRX: IoPin,
{
    #[inline(always)]
    fn write_stream<'a>(
        &mut self,
        mode: WriteMode,
        func: &mut dyn FnMut() -> Option<&'a u16>,
    ) -> Result<(), DisplayError> {
        let b0 = self.db0.into_output();
        let b1 = self.db1.into_output();
        let b2 = self.db2.into_output();
        let b3 = self.db3.into_output();
        let b4 = self.db4.into_output();
        let b5 = self.db5.into_output();
        let b6 = self.db6.into_output();
        let b7 = self.db7.into_output();
        let b8 = self.db8.into_output();
        let b9 = self.db9.into_output();
        let b10 = self.db10.into_output();
        let b11 = self.db11.into_output();
        let b12 = self.db12.into_output();
        let b13 = self.db13.into_output();
        let b14 = self.db14.into_output();
        let b15 = self.db15.into_output();
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
                    write_bit!(b8, (1 << 8) & *byte != 0);
                    write_bit!(b9, (1 << 9) & *byte != 0);
                    write_bit!(b10, (1 << 10) & *byte != 0);
                    write_bit!(b11, (1 << 11) & *byte != 0);
                    write_bit!(b12, (1 << 12) & *byte != 0);
                    write_bit!(b13, (1 << 13) & *byte != 0);
                    write_bit!(b14, (1 << 14) & *byte != 0);
                    write_bit!(b15, (1 << 15) & *byte != 0);
                    wrx.set_high();
                }
                None => break,
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
