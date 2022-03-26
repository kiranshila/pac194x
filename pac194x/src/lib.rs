#![cfg_attr(not(test), no_std)]

pub mod regs;

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

/// The Product ID of the connected part
pub enum ProductId {
    PAC1941_1,
    PAC1942_1,
    PAC1943_1,
    PAC1944_1,
    PAC1941_2,
    PAC1942_2,
}

/// A PAC194X power monitor on the I2C bus `I`.
pub struct PAC194X<I>
where
    I: i2c::Read + i2c::Write + i2c::WriteRead,
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
    ($var:ident: $type:ty) => {
        paste! {
            #[doc = stringify!(Reads the $type register and deserializes into the appropriate struct)]
            pub fn [<read_ $var>](&mut self) -> Result<$type, Error<E>> {
                Ok($type::unpack(&self.block_read($type::addr())?).unwrap())
            }
        }
    };
}

macro_rules! read_n_fn {
    ($var:ident: $type:ty) => {
        paste! {
            #[doc = stringify!(Reads the $type register and deserializes into the appropriate struct)]
            pub fn [<read_ $var>](&mut self, n: u8) -> Result<$type, Error<E>> {
                assert!((1..=4).contains(&n),"Channel n must be between 1 and 4");
                Ok($type::unpack(&self.block_read_n($type::addr(),n)?).unwrap())
            }
        }
    };
}

macro_rules! write_fn {
    ($var:ident: $type:ty) => {
        paste! {
            #[doc = stringify!(Writes out the $type register)]
            pub fn [<write_ $var>](&mut self, $var: $type) -> Result<(), Error<E>> {
                const PACKED_SIZE_WITH_ADDR: usize = core::mem::size_of::<<$type as PackedStruct>::ByteArray>() + 1;
                let mut bytes = [0u8; PACKED_SIZE_WITH_ADDR];
                bytes[0] = $type::addr() as u8;
                $var.pack_to_slice(&mut bytes[1..]).unwrap();
                self.block_write(&bytes)?;
                Ok(())
            }
        }
    };
}

macro_rules! write_n_fn {
    ($var:ident: $type:ty) => {
        paste! {
            #[doc = stringify!(Writes out the $type register)]
            pub fn [<write_ $var>](&mut self, $var: $type, n: u8) -> Result<(), Error<E>> {
                assert!((1..=4).contains(&n),"Channel n must be between 1 and 4");
                const PACKED_SIZE_WITH_ADDR: usize = core::mem::size_of::<<$type as PackedStruct>::ByteArray>() + 1;
                let mut bytes = [0u8; PACKED_SIZE_WITH_ADDR];
                bytes[0] = ($type::addr() as u8) + n - 1;
                $var.pack_to_slice(&mut bytes[1..]).unwrap();
                self.block_write(&bytes)?;
                Ok(())
            }
        }
    };
}

macro_rules! read_write {
    ($var:ident: $type:ty) => {
        write_fn!($var: $type);
        read_fn!($var: $type);
    };
}

macro_rules! read_write_n {
    ($var:ident: $type:ty) => {
        write_n_fn!($var: $type);
        read_n_fn!($var: $type);
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
    fn block_write(&mut self, bytes: &[u8]) -> Result<(), Error<E>> {
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

    /// Same behavior as `block_read` but adds the channel offset to the address
    fn block_read_n<const N: usize>(&mut self, addr: Address, n: u8) -> Result<[u8; N], Error<E>> {
        let mut buf = [0u8; N];
        self.i2c
            .write_read(self.address, &[(addr as u8) + (n - 1)], &mut buf)
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

    /// Refreshes every PAC194X device on the bus by transmitting REFRESH_G to the
    /// general call address of 0
    pub fn regresh_g(&mut self) -> Result<(), Error<E>> {
        self.i2c
            .write(0u8, &[Address::RefreshG as u8])
            .map_err(Error::I2c)?;
        Ok(())
    }

    /// Retrieves the Product ID of the connected component
    pub fn product_id(&mut self) -> Result<ProductId, Error<E>> {
        self.send_byte(regs::Address::ProductId)?;
        Ok(match self.receive_byte()? {
            0b0110_1000 => ProductId::PAC1941_1,
            0b0110_1001 => ProductId::PAC1942_1,
            0b0110_1010 => ProductId::PAC1943_1,
            0b0110_1011 => ProductId::PAC1944_1,
            0b0110_1100 => ProductId::PAC1941_2,
            0b0110_1101 => ProductId::PAC1942_2,
            _ => unreachable!(),
        })
    }

    /// The Manufacturer ID register identifies Microchip as the manufacturer of the PAC194X.
    /// This should return 0x54
    pub fn manufacturer_id(&mut self) -> Result<u8, Error<E>> {
        self.send_byte(regs::Address::ManufacturerId)?;
        self.receive_byte()
    }

    /// The Revision register identifies the die revision.
    /// This should return 0b00000010
    pub fn revision_id(&mut self) -> Result<u8, Error<E>> {
        self.send_byte(regs::Address::RevisionId)?;
        self.receive_byte()
    }

    // Auto generated functions for reading and writing all of our registers
    read_write!(ctrl: Ctrl);
    read_write!(acc_count: AccCount);
    read_n_fn!(vaccn: Vaccn);
    read_n_fn!(vbusn: Vbusn);
    read_n_fn!(vsensen: Vsensen);
    read_n_fn!(vbusn_avg: VbusnAvg);
    read_n_fn!(vsensen_avg: VsensenAvg);
    read_n_fn!(vpowern: Vpowern);
    read_write!(smub_settings: SmbusSettings);
    read_write!(neg_pwr_fsr: NegPwrFsr);
    read_fn!(slow: Slow);
    read_fn!(ctrl_act: CtrlAct);
    read_write!(neg_pwr_fsr_act: NegPwrFsrAct);
    read_fn!(ctrl_lat: CtrlLat);
    read_write!(neg_pwr_fsr_lat: NegPwrFsrLat);
    read_write!(accum_config: AccumConfig);
    read_fn!(alert_statuc: AlertStatus);
    read_write!(slow_alert1: SlowAlert1);
    read_write!(gpio_alert2: GpioAlert2);
    read_write!(acc_fullness_limits: AccFullnessLimits);
    read_write_n!(oc_limitn: OcLimitn);
    read_write_n!(uc_limitn: UcLimitn);
    read_write_n!(op_limitn: OpLimitn);
    read_write_n!(ov_limitn: OvLimitn);
    read_write_n!(uv_limitn: UvLimitn);
    read_write!(oc_limit_n_samples: OcLimitNSamples);
    read_write!(uc_limit_n_samples: UcLimitNSamples);
    read_write!(op_limit_n_samples: OpLimitNSamples);
    read_write!(ov_limit_n_samples: OvLimitNSamples);
    read_write!(uv_limit_n_samples: UvLimitNSamples);
    read_write!(alert_enable: AlertEnable);
    read_write!(accum_config_act: AccumConfigAct);
    read_write!(accum_config_lat: AccumConfigLat);
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
