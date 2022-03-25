use packed_struct::prelude::*;
use register_derive::Register;

#[repr(u8)]
pub(crate) enum Address {
    Refresh,
    Ctrl,
    AccCount,
    // Accumulator outputs
    Vaccn = 0x03,
    // VBus
    Vbusn = 0x07,
    // Vsense
    Vsensen = 0x0B,
    // VBus rolling avg,
    VbusnAvg = 0x0F,
    // Vsense rolling avg,
    VsensenAvg = 0x13,
    // Powers (Vsense*Vbus)
    Vpowern = 0x17,
    // Controls
    SmbusSettings = 0x1C,
    NegPwrFsr,
    RefreshG,
    RefreshV,
    Slow,
    CtrlAct,
    NegPwrFsrAct,
    CtrlLat,
    NegPwrFsrLat,
    AccumConfig,
    AlterStatus,
    SlowAltert1,
    GpioAltert2,
    AccFullnessLimits,
    // Overcurrent Limits
    OcLimitn = 0x30,
    // Undercurrent Limits
    UcLimitn = 0x34,
    // Overpower Limits
    OpLimitn = 0x38,
    // Overvoltage Limits
    OvLimitn = 0x3C,
    // Undervoltage Limits
    UvLimitn = 0x40,
    // Limit Altert Thresholds
    OcLimitNSamples = 0x44,
    UcLimitNSamples,
    OpLimitNSamples,
    OvLimitNSamples,
    UvLimitNSamples,
    // More Control
    AlertEnable,
    AccumConfigAct,
    AccumConfigLat,
    ProductId = 0xFD,
    ManufacturerId,
    RevisionId,
}

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
/// These bits select one of the sampling modes listed below. These modes are
/// exclusive – that is, only one mode can be set at any given time. One of the sampling modes is Sleep,
/// when no sampling occurs.
pub enum SampleMode {
    _1024Adaptive,
    _256Adaptive,
    _64Adaptive,
    _8Adaptive,
    _1024,
    _256,
    _64,
    _8,
    SingleShot,
    SingleShot8X,
    Fast,
    Burst,
    Sleep = 0b1111,
}

/// Pin mode for GPIO/ALERT2 and SLOW/ALERT1
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum GpioAlert {
    Alert,
    Input,
    Output,
    Slow,
}

#[derive(PackedStruct, Default, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0")]
pub struct Channels {
    #[packed_field(bits = "4")]
    pub _1: bool,
    pub _2: bool,
    pub _3: bool,
    pub _4: bool,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "2", bit_numbering = "lsb0")]
pub struct Ctrl {
    #[packed_field(bits = "15:12", ty = "enum")]
    pub sample_mode: SampleMode,
    #[packed_field(bits = "11:10", ty = "enum")]
    /// Select the signals for the GPIO/ALERT2 pin. If the pin is configured as a GPIO
    /// pin, the R/W data for the pin are stored in Register 7-10.
    pub gpio_alert2: GpioAlert,
    #[packed_field(bits = "9:8", ty = "enum")]
    /// Select the signals for SLOW/ALERT1 pin. If the pin is configured as a GPIO
    /// pin, the R/W data for the pin are stored in Register 7-10
    pub slow_alert1: GpioAlert,
    #[packed_field(bits = "7:4")]
    /// Allow one or more channels to be disabled (bit value = 1) during the conversion cycle. A bit value = 0
    /// means the channel is active. These settings apply for normal continuous round robin conversion
    /// cycles or Single-Shot mode, if Single-Shot mode is selected. If a channel is set to inactive, the
    /// auto-incrementing address pointer will skip addresses associated with that channel unless the No Skip
    /// bit 1 in Register 7-10 is set.
    pub channel_n_off: Channels,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "4", bit_numbering = "lsb0")]
/// This register contains the count for each time a power result is summed in the
/// accumulator.
pub struct AccCount {
    #[packed_field(bits = "31:0", endian = "lsb")]
    pub count: u32,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "7", bit_numbering = "lsb0")]
/// This register contains the accumulated sum of V POWER samples, where n = 1 to 4,
/// depending on the device by default. It can also hold the accumulated values of V SENSE and VBUS if
/// bits are set in Register 7-19. These are 56-bit unsigned numbers, unless either VBUS or VSENSE is con-
/// figured to have a bipolar range. In that case, they will be 55 bits + sign (two’s complement) numbers.
/// Power is always calculated using signed numbers for V BUS and VSENSE, but if both VBUS and VSENSE
/// are in the default Unipolar mode, power is reported as an unsigned number. This can lead to very small
/// discrepancies between a manual comparison of the product of VBUS and VSENSE and the results that
/// the chip calculates and accumulates for VPOWER . The digital math in the chip uses more bits than the
/// reported results for VBUS and VSENSE, so the results registers for VPOWER and the accumulated power
/// will in some cases have a more accurate number than calculations using the results registers for
/// VSENSE and V POWER will provide.
pub struct Vaccn {
    #[packed_field(bits = "55:0", endian = "lsb")]
    pub sum: u64,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "7", bit_numbering = "lsb0")]
///  This register contains the most recent digitized value of a VBUS sample, where n = 1 to
/// 4, depending on the device. These are 16-bit unsigned numbers, unless VBUS is configured to have a
/// bipolar range. In that case, they will be 15 bits + sign (two’s complement) numbers.
pub struct Vbusn {
    #[packed_field(bits = "15:0", endian = "lsb")]
    pub voltage: u16,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "7", bit_numbering = "lsb0")]
/// This register contains the most recent digitized value of V SENSE samples, where n
/// = 1 to 4, depending on the device. These are 16-bit unsigned numbers, unless V SENSE is configured
/// to have a bipolar range. In that case, they will be 15 bits + sign (two’s complement) numbers
pub struct Vsensen {
    #[packed_field(bits = "15:0", endian = "lsb")]
    pub voltage: u16,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "7", bit_numbering = "lsb0")]
/// This register contain a rolling average of the eight most recent V BUS
/// measurements. It has the same format as the values in the VBUS registers.
pub struct VbusnAvg {
    #[packed_field(bits = "15:0", endian = "lsb")]
    pub voltage: u16,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "7", bit_numbering = "lsb0")]
/// This register contain a rolling average of the eight most recent V SENSE
/// measurements. It has the same format as the values in the V SENSE registers.
pub struct VsensenAvg {
    #[packed_field(bits = "15:0", endian = "lsb")]
    pub voltage: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_ctrl() {
        let ctrl = Ctrl {
            sample_mode: SampleMode::_256,
            gpio_alert2: GpioAlert::Input,
            slow_alert1: GpioAlert::Output,
            channel_n_off: Channels {
                _1: false,
                _2: true,
                _3: false,
                _4: false,
            },
        };
        let bytes = ctrl.pack().unwrap();
        assert_eq!(ctrl, Ctrl::unpack(&bytes).unwrap());
    }

    #[test]
    fn round_trip_acc_count() {
        let acc_count = AccCount { count: 2459526763 };
        let bytes = acc_count.pack().unwrap();
        assert_eq!(acc_count, AccCount::unpack(&bytes).unwrap());
    }
}
