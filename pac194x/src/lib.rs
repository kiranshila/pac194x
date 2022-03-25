#![cfg_attr(not(test), no_std)]

mod regs;

use embedded_hal::blocking::i2c;
use packed_struct::prelude::*;
use paste::paste;
use regs::*;

#[repr(u8)]
/// Address select resistor value.
///
/// `GND` is a zero-ohm resistor where `ADDRSEL` is connected to ground.
/// `VDD` is when `ADDRSEL` is connected directly to the power rail.
pub enum AddrSelect {
    GND = 0b10000,
    _499 = 0b10001,
    _806 = 0b10010,
    _1270 = 0b10011,
    _2050 = 0b10100,
    _3240 = 0b10101,
    _5230 = 0b10110,
    _8450 = 0b10111,
    _13300 = 0b11000,
    _21500 = 0b11001,
    _34000 = 0b11010,
    _54900 = 0b11011,
    _88700 = 0b11100,
    _140000 = 0b11101,
    _226000 = 0b11110,
    VDD = 0b11111,
}

/// A PAC194X power monitor on the I2C bus `I`.
pub struct PAC194X<I>
where
    I: i2c::Read + i2c::Write,
{
    i2c: I,
    address: u8,
}

/// Driver errors.
#[derive(Debug, PartialEq)]
pub enum Error<E> {
    /// I2C bus error
    I2c(E),
    /// Errors such as overflowing the stack.
    Internal,
}

macro_rules! read_fn {
    ($var:ident, $type:ty) => {
        paste! {
            #[doc = stringify!(Reads the $type register and deserializes into the appropriate struct)]
            pub fn [<read_ $var>](&mut self) -> Result<$type, Error<E>> {
                Ok($type::unpack(&self.block_read($type::addr())?).unwrap())
            }
        }
    };
}

macro_rules! write_fn {
    ($var:ident,$type:ty) => {
        paste! {
            #[doc = stringify!(Writes out the $type register)]
            pub fn [<write_ $var>](&mut self, $var: $type) -> Result<(), Error<E>> {
                self.block_write($type::addr(),&$var.pack().unwrap())
            }
        }
    };
}

macro_rules! read_write {
    ($var:ident,$type:ty) => {
        write_fn!($var, $type);
        read_fn!($var, $type);
    };
}

impl<E, I> PAC194X<I>
where
    I: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    /// Initializes the driver.
    ///
    /// This consumes the I2C bus `I`. To use this driver with other I2C crates, check out [shared-bus](https://github.com/Rahix/shared-bus)
    pub fn new(i2c: I, addr_sel: AddrSelect) -> Self {
        Self {
            i2c,
            address: addr_sel as u8,
        }
    }

    /// The send byte protocol is used to set the internal address register pointer to the correct address
    /// location. No data is transferred.
    fn send_byte(&mut self, addr: Address) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[addr as u8])
            .map_err(Error::I2c)?;
        Ok(())
    }

    /// The receive byte protocol is used to read data from a register where the internal register addr pointer is
    /// known to be at the right location (e.g. set via `send_byte`)
    fn receive_byte(&mut self) -> Result<u8, Error<E>> {
        let mut buf = [0u8; 1];
        self.i2c.read(self.address, &mut buf).map_err(Error::I2c)?;
        Ok(buf[0])
    }

    /// Block write is used to write multiple data bytes from a register that contains more than one byte of data
    /// of from a group of contiguous registers
    fn block_write(&mut self, addr: Address, bytes: &[u8]) -> Result<(), Error<E>> {
        self.i2c.write(self.address, bytes).map_err(Error::I2c)?;
        Ok(())
    }

    /// Block read is used to read multiple data bytes from a register that contains more than one byte of data or from a group
    /// of contiguous registers
    fn block_read<const N: usize>(&mut self, addr: Address) -> Result<[u8; N], Error<E>> {
        let mut buf = [0u8; N];
        self.i2c
            .write_read(self.address, &[addr as u8], &mut buf)
            .map_err(Error::I2c)?;
        Ok(buf)
    }

    /// Refreshes the device
    ///
    /// The accumulator data, accumulator count, Vbus and Vsense measurements are all refreshed and
    /// the accumulators are reset. The host must wait 1ms before reading accumulator or Vbus/Vsense data
    pub fn refresh(&mut self) -> Result<(), Error<E>> {
        self.send_byte(Address::Refresh)
    }

    /// Refreshes the device without resetting the accumulators
    ///
    /// Same behavior as `refresh`, but without resetting the accumulators.
    pub fn refresh_v(&mut self) -> Result<(), Error<E>> {
        self.send_byte(Address::RefreshV)
    }

    read_write!(ctrl, Ctrl);
    read_write!(acc_count, AccCount);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn addr_values() {
        assert_eq!(Address::AlertEnable as u8, 0x49);
        assert_eq!(Address::RevisionId as u8, 0xFF);
    }
}
