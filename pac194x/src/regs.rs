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

/// Sample Modes
///
/// `Sleep` is the odd one out where samples are not taken
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
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
    pub gpio_alert2: GpioAlert,
    #[packed_field(bits = "9:8", ty = "enum")]
    pub slow_alert1: GpioAlert,
    #[packed_field(bits = "7:4")]
    pub channel_n_off: Channels,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "4", bit_numbering = "lsb0")]
pub struct AccCount {
    #[packed_field(bits = "31:0", endian = "lsb")]
    pub count: u32,
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
