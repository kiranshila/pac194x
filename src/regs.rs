//! Contains the registers and associated types for the PAC194X

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
    AlertStatus,
    SlowAlert1,
    GpioAlert2,
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
    // Limit Alert Thresholds
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

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
/// Pin mode for GPIO/ALERT2 and SLOW/ALERT1
pub enum GpioAlert {
    Alert,
    Input,
    Output,
    Slow,
}

#[derive(PackedStruct, Default, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0")]
/// Channels to be potentially disabled
pub struct Channels {
    #[packed_field(bits = "4")]
    pub _1: bool,
    pub _2: bool,
    pub _3: bool,
    pub _4: bool,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "2", bit_numbering = "lsb0")]
/// Primary control registeer
pub struct Ctrl {
    #[packed_field(bits = "15:12", ty = "enum")]
    pub sample_mode: SampleMode,
    #[packed_field(bits = "11:10", ty = "enum")]
    /// Select the signals for the GPIO/ALERT2 pin. If the pin is configured as a GPIO
    /// pin, the R/W data for the pin are stored in [`SmbusSettings`].
    pub gpio_alert2: GpioAlert,
    #[packed_field(bits = "9:8", ty = "enum")]
    /// Select the signals for SLOW/ALERT1 pin. If the pin is configured as a GPIO
    /// pin, the R/W data for the pin are stored in [`SmbusSettings`]
    pub slow_alert1: GpioAlert,
    #[packed_field(bits = "7:4")]
    /// Allow one or more channels to be disabled (bit value = 1) during the conversion cycle. A bit value = 0
    /// means the channel is active. These settings apply for normal continuous round robin conversion
    /// cycles or Single-Shot mode, if Single-Shot mode is selected. If a channel is set to inactive, the
    /// auto-incrementing address pointer will skip addresses associated with that channel unless the No Skip
    /// bit 1 in Register [`SmbusSettings`] is set.
    pub channel_n_off: Channels,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "4", bit_numbering = "lsb0")]
/// This register contains the count for each time a power result is summed in the
/// accumulator.
pub struct AccCount {
    #[packed_field(bits = "31:0", endian = "msb")]
    pub count: u32,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "7", bit_numbering = "lsb0")]
/// This register contains the accumulated sum of V POWER samples, where n = 1 to 4,
/// depending on the device by default. It can also hold the accumulated values of V SENSE and VBUS if
/// bits are set in [`AccumConfig`]. These are 56-bit unsigned numbers, unless either VBUS or VSENSE is con-
/// figured to have a bipolar range. In that case, they will be 55 bits + sign (two’s complement) numbers.
/// Power is always calculated using signed numbers for V BUS and VSENSE, but if both VBUS and VSENSE
/// are in the default Unipolar mode, power is reported as an unsigned number. This can lead to very small
/// discrepancies between a manual comparison of the product of VBUS and VSENSE and the results that
/// the chip calculates and accumulates for VPOWER . The digital math in the chip uses more bits than the
/// reported results for VBUS and VSENSE, so the results registers for VPOWER and the accumulated power
/// will in some cases have a more accurate number than calculations using the results registers for
/// VSENSE and V POWER will provide.
pub struct Vaccn {
    #[packed_field(bits = "55:0", endian = "msb")]
    pub sum: u64,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "2", bit_numbering = "lsb0")]
///  This register contains the most recent digitized value of a VBUS sample, where n = 1 to
/// 4, depending on the device. These are 16-bit unsigned numbers, unless VBUS is configured to have a
/// bipolar range. In that case, they will be 15 bits + sign (two’s complement) numbers.
pub struct Vbusn {
    #[packed_field(bits = "15:0", endian = "msb")]
    pub voltage: u16,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "2", bit_numbering = "lsb0")]
/// This register contains the most recent digitized value of V SENSE samples, where n
/// = 1 to 4, depending on the device. These are 16-bit unsigned numbers, unless V SENSE is configured
/// to have a bipolar range. In that case, they will be 15 bits + sign (two’s complement) numbers
pub struct Vsensen {
    #[packed_field(bits = "15:0", endian = "msb")]
    pub voltage: u16,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "2", bit_numbering = "lsb0")]
/// This register contain a rolling average of the eight most recent V BUS
/// measurements. It has the same format as the values in the VBUS registers.
pub struct VbusnAvg {
    #[packed_field(bits = "15:0", endian = "msb")]
    pub voltage: u16,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "2", bit_numbering = "lsb0")]
/// This register contain a rolling average of the eight most recent V SENSE
/// measurements. It has the same format as the values in the V SENSE registers.
pub struct VsensenAvg {
    #[packed_field(bits = "15:0", endian = "msb")]
    pub voltage: u16,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "4", bit_numbering = "lsb0")]
/// This register contains the product of V BUS (14 MSBs) and V SENSE, which
/// represents the proportional power for each channel. These are 30-bit unsigned numbers unless either
/// VBUS or VSENSE is configured to have a bipolar range. In that case, they will be 29 bits + sign (two’s
/// complement) numbers. These are the numbers that are accumulated in the accumulators. Power is
/// always calculated using signed numbers for VBUS and VSENSE, but if both VBUS and VSENSE are in the
/// default Unipolar mode, power is reported as an unsigned number. This can lead to very small discrep-
/// ancies between a manual comparison of the product of V BUS and VSENSE and the results that the chip
/// calculates for VPOWER. The digital math in the chip uses more bits than the reported results for VBUS
/// and VSENSE, so the results registers for VPOWER and the accumulated power will in some cases have
/// a more accurate number than calculations using the results registers for V SENSE and V POWER will
/// provide.
pub struct Vpowern {
    #[packed_field(bits = "31:2", endian = "msb")]
    pub power: u32,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "1", bit_numbering = "lsb0")]
/// Bits in this register may be written or read at any time and are active immediately. Refresh is not required to activate
/// them or update them.
pub struct SmbusSettings {
    #[packed_field(bits = "7")]
    /// R/W data for the pin GPIO/ALERT2 if the pin is configured as a GPIO pin. See
    /// Register 7-2 to configure GPIO/ALERT2 as a GPIO pin.
    ///
    /// - false - Pull the GPIO pin low when configured to be an output (default)
    /// - true - PIO pin pulled to external V IO via an external resistor when configured to be an output
    pub gpio_data2: bool,
    #[packed_field(bits = "6")]
    /// R/W data for the pin SLOW/ALERT1 if the pin is configured as a GPIO pin. See
    /// Register 7-2 to configure SLOW/ALERT1 as a GPIO pin.
    ///
    /// - false - Pull the GPIO pin low when configured to be an output (default)
    /// - true - PIO pin pulled to external V IO via an external resistor when configured to be an output
    pub gpio_data1: bool,
    #[packed_field(bits = "5")]
    /// This bit is set by any of the active ALERT functions being triggered, except ALERT_CC.
    /// This bit is cleared when the ALERT function that set the bit is cleared.
    ///
    /// - false - No ALERT condition has occurred (default)
    /// - true - An ALERT condition has occurred
    pub any_alert: bool,
    #[packed_field(bits = "4")]
    /// The POR bit is for the purpose of enabling the system designer to learn if the chip is reset after
    /// it is programmed. The user can clear this bit after POR and then monitor it to detect if the device was
    /// powered cycled or somehow reset since the POR. If the reset is detected in this manner, any
    /// non-default programming can be reprogrammed. This bit is only reset by the internal POR, which can
    /// occur from power cycling or the PWRDN pin going low.
    ///
    /// - false = This bit has been cleared over I2C since the last POR occurred
    /// - true = Default. This bit has the POR default value of `1` and has not been cleared since the last reset occurred
    pub por: bool,
    #[packed_field(bits = "3")]
    /// TIMEOUT enable bit. The SMBus time-out is disabled by default and is enabled by setting this bit.
    ///
    /// - false = No SMBus time-out feature (default)
    /// - true = SMBus time-out feature is available
    pub timeout: bool,
    #[packed_field(bits = "2")]
    /// This bit causes Byte Count data to be included in the response to the SMBus Block
    /// Read command for each register read. This functionality is disabled by default and Block Read
    /// corresponds to the I2 C protocol.
    ///
    /// - false = No Byte Count in response to a Block Read command (default)
    /// - true = Data in response to a Block Read command include the Byte Count data
    pub byte_count: bool,
    #[packed_field(bits = "1")]
    /// This bit controls the auto-incrementing of the address pointer for channels that are inactive
    ///
    /// - false = The auto-incrementing pointer will skip over addresses used by/for channels that are inactive (default)
    /// - true = he auto-incrementing pointer will not skip over addresses used by/for channels that are inactive.
    /// When these channels are disabled, if a read is performed, it will read FF.
    pub no_skip: bool,
    #[packed_field(bits = "0")]
    /// Setting this bit enables the 3.4 MHz I2 C operation by changing the pulse-width
    /// parameters of the Pulse Gobbler. Default = false
    pub i2c_hispeed: bool,
}

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
/// Full scale range (FSR) for the sense voltage
pub enum VSenseFSR {
    /// Unipolar range of +100 mV to 0V FSR
    Unipolar = 0,
    /// Bipolar range of +100 mV to -100mV FSR
    BipolarHV = 1,
    /// Bipolar range of +50 mV to -50mV FSR
    BipolarLV = 2,
}

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
/// Full scale range (FSR) for the bus voltage
pub enum VBusFSR {
    /// Unipolar range of +9 V to 0V FSR
    Unipolar = 0,
    /// Bipolar range of +9 V to -9V FSR
    BipolarHV = 1,
    /// Bipolar range of +4.5 V to -4.5 V FSR
    BipolarLV = 2,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "2", bit_numbering = "lsb0")]
/// Configures the nth channel FSR for Vsense and Vbus measurement
pub struct NegPwrFsr {
    #[packed_field(bits = "15:14", ty = "enum")]
    pub cfg_vs1: VSenseFSR,
    #[packed_field(bits = "13:12", ty = "enum")]
    pub cfg_vs2: VSenseFSR,
    #[packed_field(bits = "11:10", ty = "enum")]
    pub cfg_vs3: VSenseFSR,
    #[packed_field(bits = "9:8", ty = "enum")]
    pub cfg_vs4: VSenseFSR,
    #[packed_field(bits = "7:6", ty = "enum")]
    pub cfg_vb1: VBusFSR,
    #[packed_field(bits = "5:4", ty = "enum")]
    pub cfg_vb2: VBusFSR,
    #[packed_field(bits = "3:2", ty = "enum")]
    pub cfg_vb3: VBusFSR,
    #[packed_field(bits = "1:0", ty = "enum")]
    pub cfg_vb4: VBusFSR,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "1", bit_numbering = "lsb0")]
/// This register tracks the state of the SLOW pin, tracks transitions on the SLOW pin and controls the type of limited
/// REFRESH command (if any) that will occur on a SLOW pin transition. This allows software to monitor the state of the
/// SLOW pin and its transitions over the I2C even though the SLOW pin is asynchronous to the I2C pins and may have
/// a different controller. As such, no REFRESH or REFRESH_V command is required to activate new written values or
/// to update readable values. On a transition of the SLOW pin, a limited REFRESH function may be executed if bits 1-4
/// are set. These limited REFRESH and REFRESH_V functions update all of the readable results registers. The limited
/// REFRESH does not update the CTRL_LAT, NEG_PWR_FSR_LAT or the ACCUM_CONFIG_LAT registers. For the
/// limited REFRESH function only, it also resets the accumulators and accumulator count. These are called limited
/// REFRESH and limited REFRESH_V functions because there is no activation of any pending changes to the control
/// registers. If limited REFRESH and limited REFRESH_V are both enabled for a certain SLOW pin transition,
/// REFRESH will be executed (REFRESH wins over REFRESH_V).
pub struct Slow {
    #[packed_field(bits = "7")]
    /// - false = the current status is not active
    /// - true = the current status is active
    pub slow: bool,
    #[packed_field(bits = "6")]
    /// - false = The SLOW pin has not transitioned low to high since the last REFRESH command
    /// - true = The SLOW pin has transitioned low to high since the last REFRESH command
    /// The bit is reset to ‘0’ by a REFRESH or REFRESH_G command.
    pub slow_lh: bool,
    #[packed_field(bits = "5")]
    /// - false = The SLOW pin has not transitioned low to high since the last REFRESH command
    /// - true = The SLOW pin has transitioned low to high since the last REFRESH command
    /// The bit is reset to ‘0’ by a REFRESH or REFRESH_G command.
    pub slow_hl: bool,
    #[packed_field(bits = "4")]
    /// - false = Disables limited REFRESH function to take place on the rising edge of the SLOW pin
    /// - true = Enables limited REFRESH function to take place on the rising edge of the SLOW pin
    /// The bit is not reset automatically, it must be written to be changed.
    pub r_rise: bool,
    #[packed_field(bits = "3")]
    /// - false = Disables limited REFRESH_V function to take place on the rising edge of the SLOW pin
    /// - true = Enables limited REFRESH_V function to take place on the rising edge of the SLOW pin
    /// The bit is not reset automatically, it must be written to be changed.
    pub r_v_rise: bool,
    #[packed_field(bits = "2")]
    /// - false = Disables limited REFRESH function to take place on the rising edge of the SLOW pin
    /// - true = Enables limited REFRESH function to take place on the rising edge of the SLOW pin
    /// The bit is not reset automatically, it must be written to be changed.
    pub r_fall: bool,
    #[packed_field(bits = "1")]
    /// - false = Disables limited REFRESH_V function to take place on the rising edge of the SLOW pin
    /// - true = Enables limited REFRESH_V function to take place on the rising edge of the SLOW pin
    /// The bit is not reset automatically, it must be written to be changed.
    pub r_v_fall: bool,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "2", bit_numbering = "lsb0")]
/// This register contains an image of [`Ctrl`] and reflects the current active value of these settings, whereas the
/// values in register 01h may be programmed but not activated by one of the REFRESH commands. This register
/// allows software to determine the actual active settings. This register is valid when the results registers are valid, 1 ms
/// after a REFRESH/_V/_G command, in most cases. However, if you program a conversion rate change followed by
/// REFRESH, the new conversion rate will not become effective until the current conversion cycle is complete. This can
/// cause a delay in some cases before the conversion cycle (and the [`CtrlAct`] register) is updated. This delay can be
/// variable, depending on where you are in the conversion cycle when the REFRESH command is sent.
pub struct CtrlAct {
    #[packed_field(bits = "15:12", ty = "enum")]
    pub sample_mode: SampleMode,
    #[packed_field(bits = "11:10", ty = "enum")]
    /// Select the signals for the GPIO/ALERT2 pin. If the pin is configured as a GPIO
    /// pin, the R/W data for the pin are stored in [`SmbusSettings`].
    pub gpio_alert2: GpioAlert,
    #[packed_field(bits = "9:8", ty = "enum")]
    /// Select the signals for SLOW/ALERT1 pin. If the pin is configured as a GPIO
    /// pin, the R/W data for the pin are stored in [`SmbusSettings`]
    pub slow_alert1: GpioAlert,
    #[packed_field(bits = "7:4")]
    /// Allow one or more channels to be disabled (bit value = 1) during the conversion cycle. A bit value = 0
    /// means the channel is active. These settings apply for normal continuous round robin conversion
    /// cycles or Single-Shot mode, if Single-Shot mode is selected. If a channel is set to inactive, the
    /// auto-incrementing address pointer will skip addresses associated with that channel unless the No Skip
    /// bit 1 in Register [`SmbusSettings`] is set.
    pub channel_n_off: Channels,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "2", bit_numbering = "lsb0")]
/// This register contains an image of [`NegPwrFsr`]. The bits in this register reflect the current active value of these set-
/// tings, whereas the values in register 1Dh may be programmed but not activated by one of the REFRESH commands.
/// This register allows software to determine the actual active setting. This register is valid when the results registers are
/// valid, 1 ms after a REFRESH/_V/_G command.
pub struct NegPwrFsrAct {
    #[packed_field(bits = "15:14", ty = "enum")]
    pub cfg_vs1: VSenseFSR,
    #[packed_field(bits = "13:12", ty = "enum")]
    pub cfg_vs2: VSenseFSR,
    #[packed_field(bits = "11:10", ty = "enum")]
    pub cfg_vs3: VSenseFSR,
    #[packed_field(bits = "9:8", ty = "enum")]
    pub cfg_vs4: VSenseFSR,
    #[packed_field(bits = "7:6", ty = "enum")]
    pub cfg_vb1: VBusFSR,
    #[packed_field(bits = "5:4", ty = "enum")]
    pub cfg_vb2: VBusFSR,
    #[packed_field(bits = "3:2", ty = "enum")]
    pub cfg_vb3: VBusFSR,
    #[packed_field(bits = "1:0", ty = "enum")]
    pub cfg_vb4: VBusFSR,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "2", bit_numbering = "lsb0")]
/// This register contains an image of [`Ctrl`]. The bits in this register reflect the value of these settings, that was
/// active before the most recent REFRESH command (including REFRESH_V and/or REFRESH_G). The values in
/// register 01h may be programmed but not activated by one of the REFRESH commands and the values in 21h are
/// currently active. This register allows software to determine the actual active setting that was active prior to the most
/// recent REFRESH command and therefore corresponds to the dataset that is held in the readable registers. This reg-
/// ister is valid when the results registers are valid, 1 ms after a REFRESH/_V/_G command. The CTRL_LAT register is
/// not valid until the first REFRESH is sent after a POR event.
pub struct CtrlLat {
    #[packed_field(bits = "15:12", ty = "enum")]
    pub sample_mode: SampleMode,
    #[packed_field(bits = "11:10", ty = "enum")]
    /// Select the signals for the GPIO/ALERT2 pin. If the pin is configured as a GPIO
    /// pin, the R/W data for the pin are stored in [`SmbusSettings`].
    pub gpio_alert2: GpioAlert,
    #[packed_field(bits = "9:8", ty = "enum")]
    /// Select the signals for SLOW/ALERT1 pin. If the pin is configured as a GPIO
    /// pin, the R/W data for the pin are stored in [`SmbusSettings`]
    pub slow_alert1: GpioAlert,
    #[packed_field(bits = "7:4")]
    /// Allow one or more channels to be disabled (bit value = 1) during the conversion cycle. A bit value = 0
    /// means the channel is active. These settings apply for normal continuous round robin conversion
    /// cycles or Single-Shot mode, if Single-Shot mode is selected. If a channel is set to inactive, the
    /// auto-incrementing address pointer will skip addresses associated with that channel unless the No Skip
    /// bit 1 in Register [`SmbusSettings`] is set.
    pub channel_n_off: Channels,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "2", bit_numbering = "lsb0")]
/// This register contains an image of [`NegPwrFsr`].
/// The bits in this register reflect the settings that were active before the
/// most recent REFRESH command (including REFRESH_V and/or REFRESH_G). The values in register 1Dh may be
/// programmed but not activated by one of the REFRESH commands. This register shows the settings that were active
/// prior to the most recent REFRESH command and therefore correspond to the dataset that is held in the readable reg-
/// isters. This register is valid when the results registers are valid, 1 ms after a REFRESH/_V/_G command.
pub struct NegPwrFsrLat {
    #[packed_field(bits = "15:14", ty = "enum")]
    pub cfg_vs1: VSenseFSR,
    #[packed_field(bits = "13:12", ty = "enum")]
    pub cfg_vs2: VSenseFSR,
    #[packed_field(bits = "11:10", ty = "enum")]
    pub cfg_vs3: VSenseFSR,
    #[packed_field(bits = "9:8", ty = "enum")]
    pub cfg_vs4: VSenseFSR,
    #[packed_field(bits = "7:6", ty = "enum")]
    pub cfg_vb1: VBusFSR,
    #[packed_field(bits = "5:4", ty = "enum")]
    pub cfg_vb2: VBusFSR,
    #[packed_field(bits = "3:2", ty = "enum")]
    pub cfg_vb3: VBusFSR,
    #[packed_field(bits = "1:0", ty = "enum")]
    pub cfg_vb4: VBusFSR,
}

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
/// The voltage an accumulator accumulates
pub enum AccumSetting {
    VPower = 0,
    VSense = 1,
    VBus = 2,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "1", bit_numbering = "lsb0")]
/// This register is used to configure the Accumulator for one of the alternate modes: V SENSE Accumulation (Coulomb
/// Counting) or V BUS Accumulation (VBUS integration). All bits default to zero, which is the V POWER Accumulation mode
/// for the Accumulator, useful for energy measurements.
pub struct AccumConfig {
    #[packed_field(bits = "7:6", ty = "enum")]
    pub acc1_config: AccumSetting,
    #[packed_field(bits = "5:4", ty = "enum")]
    pub acc2_config: AccumSetting,
    #[packed_field(bits = "3:2", ty = "enum")]
    pub acc3_config: AccumSetting,
    #[packed_field(bits = "1:0", ty = "enum")]
    pub acc4_config: AccumSetting,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "3", bit_numbering = "msb0")]
/// Read this register to determine the cause of ALERT being tripped.
/// This register is cleared when read and another conversion cycle completes. If the
/// condition that set the ALERT is still present when the conversion cycle completes, the bit remains set. The register
/// does not require a REFRESH to update the readable register value. The OC, UC, OP, OV and UV ALERTs are
/// disabled by default. To enable the ones you want, set the appropriate bits in [`AlertEnable`].
pub struct AlertStatus {
    #[packed_field(bit = "0")]
    pub ch1_oc: bool,
    pub ch2_oc: bool,
    pub ch3_oc: bool,
    pub ch4_oc: bool,
    pub ch1_uc: bool,
    pub ch2_uc: bool,
    pub ch3_uc: bool,
    pub ch4_uc: bool,
    pub ch1_ov: bool,
    pub ch2_ov: bool,
    pub ch3_ov: bool,
    pub ch4_ov: bool,
    pub ch1_uv: bool,
    pub ch2_uv: bool,
    pub ch3_uv: bool,
    pub ch4_uv: bool,
    pub ch1_op: bool,
    pub ch2_op: bool,
    pub ch3_op: bool,
    pub ch4_op: bool,
    /// This bit signals when the accumulator for any channel overflows or exceeds its fullness limit
    /// specified in [`AccFullnessLimits`]
    ///
    /// - false = No accumulated full related ALERT for this channel
    /// - true = ALERT triggered by accumulator limit
    pub acc_ovf: bool,
    /// This bit signals whn the accumulator count overflows or exceeds its fullness limit specified in [`AccFullnessLimits`]
    ///
    /// - false = No accumulator full related ALERT for this channl
    /// - true = ALERT triggered by Accumulator Count fullness limit exceded
    pub acc_count: bool,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "3", bit_numbering = "msb0")]
/// Write to this register to assign a specific ALERT signal to the SLOW/ALERT1 pin. The SLOW/ALERT1 pin must be
/// configured for the ALERT function in [`Ctrl`] for this register to control the pin.
/// ALERTs must be enabled in [`AlertEnable`] before you can route them to a pin. Disable ALERTs in
/// [`AlertEnable`] before changing any limit to avoid false triggers.
pub struct SlowAlert1 {
    #[packed_field(bit = "0")]
    pub ch1_oc: bool,
    pub ch2_oc: bool,
    pub ch3_oc: bool,
    pub ch4_oc: bool,
    pub ch1_uc: bool,
    pub ch2_uc: bool,
    pub ch3_uc: bool,
    pub ch4_uc: bool,
    pub ch1_ov: bool,
    pub ch2_ov: bool,
    pub ch3_ov: bool,
    pub ch4_ov: bool,
    pub ch1_uv: bool,
    pub ch2_uv: bool,
    pub ch3_uv: bool,
    pub ch4_uv: bool,
    pub ch1_op: bool,
    pub ch2_op: bool,
    pub ch3_op: bool,
    pub ch4_op: bool,
    /// This bit signals when the accumulator for any channel overflows or exceeds its fullness limit
    /// specified in [`AccFullnessLimits`]
    ///
    /// - false = No accumulated full related ALERT for this channel
    /// - true = ALERT triggered by accumulator limit
    pub acc_ovf: bool,
    /// This bit signals whn the accumulator count overflows or exceeds its fullness limit specified in [`AccFullnessLimits`]
    ///
    /// - false = No accumulator full related ALERT for this channl
    /// - true = ALERT triggered by Accumulator Count fullness limit exceded
    pub acc_count: bool,
    /// Setting this bit to ‘1’ causes the SLOW/ALERT1 pin to be asserted for 5 μs at the end
    /// of each conversion cycle. This pin must be configured as an ALERT pin for this function to trigger the
    /// SLOW/ALERT1 pin. The SLOW function is not available on this pin when the pin is used as an ALERT
    /// pin.
    ///
    /// - false = No ALERT on SLOW/ALERT1 pin at each conversion cycle complete event
    /// - true = ALERT function on SLOW/ALERT1 pin asserted for 5 μs on each completion of the conversion cycle
    pub alert_cc1: bool,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "3", bit_numbering = "msb0")]
/// Write to this register to assign a specific ALERT signal to the GPIO/ALERT2 pin. The GPIO/ALERT2 pin must be
/// configured for the ALERT function in [`Ctrl`] for this register to control the pin.
/// ALERTs must be enabled in [`AlertEnable`] before you can route them to a pin. Disable ALERTs in
/// [`AlertEnable`] before changing any limit to avoid false triggers.
pub struct GpioAlert2 {
    #[packed_field(bit = "0")]
    pub ch1_oc: bool,
    pub ch2_oc: bool,
    pub ch3_oc: bool,
    pub ch4_oc: bool,
    pub ch1_uc: bool,
    pub ch2_uc: bool,
    pub ch3_uc: bool,
    pub ch4_uc: bool,
    pub ch1_ov: bool,
    pub ch2_ov: bool,
    pub ch3_ov: bool,
    pub ch4_ov: bool,
    pub ch1_uv: bool,
    pub ch2_uv: bool,
    pub ch3_uv: bool,
    pub ch4_uv: bool,
    pub ch1_op: bool,
    pub ch2_op: bool,
    pub ch3_op: bool,
    pub ch4_op: bool,
    /// This bit signals when the accumulator for any channel overflows or exceeds its fullness limit
    /// specified in [`AccFullnessLimits`]
    ///
    /// - false = No accumulated full related ALERT for this channel
    /// - true = ALERT triggered by accumulator limit
    pub acc_ovf: bool,
    /// This bit signals whn the accumulator count overflows or exceeds its fullness limit specified in [`AccFullnessLimits`]
    ///
    /// - false = No accumulator full related ALERT for this channel
    /// - true = ALERT triggered by Accumulator Count fullness limit exceeded
    pub acc_count: bool,
    /// Setting this bit to ‘1’ causes the GPIO/ALERT2 pin to be asserted for 5 μs at the end
    /// of each conversion cycle. This pin must be configured as an ALERT pin for this function to trigger the
    /// GPIO/ALERT2 pin. The SLOW function is not available on this pin when the pin is used as an ALERT
    /// pin.
    ///
    /// - false = No ALERT on GPIO/ALERT2 pin at each conversion cycle complete event
    /// - true = ALERT function on GPIO/ALERT2 pin asserted for 5 μs on each completion of the conversion cycle
    pub alert_cc2: bool,
}

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum AccFullness {
    Full = 0,
    /// 15/16 Full (Default)
    Mostly = 1,
    /// 7/8 Full
    Somewhat = 2,
    /// 3/4 Full
    Partially = 3,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "2", bit_numbering = "lsb0")]
/// These limits are used to set a limit for how full the Accumulators and Accumulator Count registers can be before the
/// Accumulator Full and Accumulator Count full limits are tripped. This allows an ALERT to be registered when the
/// Accumulator and Accumulator Count are approaching 100% full. Disable ALERTs in [`AlertEnable`] before changing
/// the value to avoid false triggers.
pub struct AccFullnessLimits {
    #[packed_field(bits = "15:14", ty = "enum")]
    pub ch1_acc_full: AccFullness,
    #[packed_field(bits = "13:12", ty = "enum")]
    pub ch2_acc_full: AccFullness,
    #[packed_field(bits = "11:10", ty = "enum")]
    pub ch3_acc_full: AccFullness,
    #[packed_field(bits = "9:8", ty = "enum")]
    pub ch4_acc_full: AccFullness,
    #[packed_field(bits = "7:6", ty = "enum")]
    pub acc_count_full: AccFullness,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "2", bit_numbering = "lsb0")]
/// Overcurrent (OC) limit for each channel. This limit is a two’s complement number for
/// all modes. Disable ALERTs in [`AlertEnable`] before changing the value to avoid false triggers. Each
/// channel has its own limit and addressable register.
pub struct OcLimitn {
    #[packed_field(bits = "15:0", endian = "msb")]
    pub limit: i16,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "2", bit_numbering = "lsb0")]
/// Undercurrent (UC) limit for each channel. This limit is a two’s complement number for
/// all modes. Disable ALERTs in [`AlertEnable`] before changing the value to avoid false triggers. Each
/// channel has its own limit and addressable register.
pub struct UcLimitn {
    #[packed_field(bits = "15:0", endian = "msb")]
    pub limit: i16,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "3", bit_numbering = "lsb0")]
/// Overpower (OP) limit for each channel. This limit is a two’s complement number for
/// all modes. These 24 bits correspond to the upper 24 MSBs in the VPOWER number. The OP limit (only)
/// is magnitude based, an OP trigger occurs when the result is more positive or more negative than the
/// limit. Disable ALERTs in [`AlertEnable`] before changing the value to avoid false triggers. Each channel
/// has its own limit and addressable register.
pub struct OpLimitn {
    #[packed_field(bits = "23:0", endian = "msb")]
    pub limit: i32,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "2", bit_numbering = "lsb0")]
/// Overvoltage (OV) limit for each channel. This limit is a two’s complement number for
/// all modes. Disable ALERTs in [`AlertEnable`] before changing the value to avoid false triggers. Each
/// channel has its own limit and addressable register.
pub struct OvLimitn {
    #[packed_field(bits = "15:0", endian = "msb")]
    pub limit: i16,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "2", bit_numbering = "lsb0")]
/// Undervoltage (UV) limit for each channel. This limit is a two’s complement number for
/// all modes. Disable ALERTs in [`AlertEnable`] before changing the value to avoid false triggers. Each
/// channel has its own limit and addressable register.
pub struct UvLimitn {
    #[packed_field(bits = "15:0", endian = "msb")]
    pub limit: i16,
}

/// The consecutive sample count to trigger an alert
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum SampleCount {
    /// Default
    _1 = 0,
    _4 = 1,
    _8 = 2,
    _16 = 3,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "1", bit_numbering = "lsb0")]
/// Number of consecutive samples exceeding the overcurrent limit that are required to trigger the ALERT function for
/// each channel. The default is 1 sample [`SampleCount::_1`]. The sample counter is not reset until a conversion is completed to con-
/// firm that the ALERT condition is no longer present. A single conversion immediately after the ALERT is cleared will
/// reset the ALERT. Disable ALERTs in [`AlertEnable`] before changing the value to avoid false triggers.
pub struct OcLimitNSamples {
    #[packed_field(bits = "7:6", ty = "enum")]
    pub n_samples_ch1: SampleCount,
    #[packed_field(bits = "5:4", ty = "enum")]
    pub n_samples_ch2: SampleCount,
    #[packed_field(bits = "3:2", ty = "enum")]
    pub n_samples_ch3: SampleCount,
    #[packed_field(bits = "1:0", ty = "enum")]
    pub n_samples_ch4: SampleCount,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "1", bit_numbering = "lsb0")]
/// Number of consecutive samples exceeding the undercurrent limit that are required to trigger the ALERT function for
/// each channel. The default is 1 sample [`SampleCount::_1`].
/// Disable ALERTs in [`AlertEnable`] before changing the value to avoid false triggers.
pub struct UcLimitNSamples {
    #[packed_field(bits = "7:6", ty = "enum")]
    pub n_samples_ch1: SampleCount,
    #[packed_field(bits = "5:4", ty = "enum")]
    pub n_samples_ch2: SampleCount,
    #[packed_field(bits = "3:2", ty = "enum")]
    pub n_samples_ch3: SampleCount,
    #[packed_field(bits = "1:0", ty = "enum")]
    pub n_samples_ch4: SampleCount,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "1", bit_numbering = "lsb0")]
/// Number of consecutive samples exceeding the overpower limit that are required to trigger the ALERT function for
/// each channel. The default is 1 sample [`SampleCount::_1`].
/// Disable ALERTs in [`AlertEnable`] before changing the value to avoid false triggers.
pub struct OpLimitNSamples {
    #[packed_field(bits = "7:6", ty = "enum")]
    pub n_samples_ch1: SampleCount,
    #[packed_field(bits = "5:4", ty = "enum")]
    pub n_samples_ch2: SampleCount,
    #[packed_field(bits = "3:2", ty = "enum")]
    pub n_samples_ch3: SampleCount,
    #[packed_field(bits = "1:0", ty = "enum")]
    pub n_samples_ch4: SampleCount,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "1", bit_numbering = "lsb0")]
/// Number of consecutive samples exceeding the overvoltage limit that are required to trigger the ALERT function for
/// each channel. The default is 1 sample [`SampleCount::_1`].
/// Disable ALERTs in [`AlertEnable`] before changing the value to avoid false triggers.
pub struct OvLimitNSamples {
    #[packed_field(bits = "7:6", ty = "enum")]
    pub n_samples_ch1: SampleCount,
    #[packed_field(bits = "5:4", ty = "enum")]
    pub n_samples_ch2: SampleCount,
    #[packed_field(bits = "3:2", ty = "enum")]
    pub n_samples_ch3: SampleCount,
    #[packed_field(bits = "1:0", ty = "enum")]
    pub n_samples_ch4: SampleCount,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "1", bit_numbering = "lsb0")]
/// Number of consecutive samples exceeding the undervoltage limit that are required to trigger the ALERT function for
/// each channel. The default is 1 sample [`SampleCount::_1`].
/// Disable ALERTs in [`AlertEnable`] before changing the value to avoid false triggers.
pub struct UvLimitNSamples {
    #[packed_field(bits = "7:6", ty = "enum")]
    pub n_samples_ch1: SampleCount,
    #[packed_field(bits = "5:4", ty = "enum")]
    pub n_samples_ch2: SampleCount,
    #[packed_field(bits = "3:2", ty = "enum")]
    pub n_samples_ch3: SampleCount,
    #[packed_field(bits = "1:0", ty = "enum")]
    pub n_samples_ch4: SampleCount,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "3", bit_numbering = "msb0")]
/// Write to these bits to enable ALERT functions.
/// To enable OC, UC, OP, OV, UV ALERTs, write ‘1’ to the appropriate bit. ALERTs must be enabled in this reg-
/// ALERTs must be enabled in this register before they can be routed to an ALERT pin.
/// A REFRESH (or REFRESH_V/G) is required to activate the enabled ALERTs.
pub struct AlertEnable {
    #[packed_field(bit = "0")]
    pub ch1_oc: bool,
    pub ch2_oc: bool,
    pub ch3_oc: bool,
    pub ch4_oc: bool,
    pub ch1_uc: bool,
    pub ch2_uc: bool,
    pub ch3_uc: bool,
    pub ch4_uc: bool,
    pub ch1_ov: bool,
    pub ch2_ov: bool,
    pub ch3_ov: bool,
    pub ch4_ov: bool,
    pub ch1_uv: bool,
    pub ch2_uv: bool,
    pub ch3_uv: bool,
    pub ch4_uv: bool,
    pub ch1_op: bool,
    pub ch2_op: bool,
    pub ch3_op: bool,
    pub ch4_op: bool,
    pub acc_ovf: bool,
    pub acc_count: bool,
    pub alert_cc: bool,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "1", bit_numbering = "lsb0")]
/// This register contains an image of [`AccumConfig`] and reflects the current active values of these settings, whereas the
/// values in register 25h may be programmed but not activated by one of the REFRESH commands. This register
/// allows software to determine the actual active settings.
pub struct AccumConfigAct {
    #[packed_field(bits = "7:6", ty = "enum")]
    pub acc1_config: AccumSetting,
    #[packed_field(bits = "5:4", ty = "enum")]
    pub acc2_config: AccumSetting,
    #[packed_field(bits = "3:2", ty = "enum")]
    pub acc3_config: AccumSetting,
    #[packed_field(bits = "1:0", ty = "enum")]
    pub acc4_config: AccumSetting,
}

#[derive(PackedStruct, Debug, PartialEq, Register)]
#[packed_struct(size_bytes = "1", bit_numbering = "lsb0")]
/// This register contains an image of [`AccumConfig`]
/// The bits in this register reflect the value of these settings that were
/// active before the most recent REFRESH command (including REFRESH_V and/of REFRESH_G). The values in
/// register 25h may be programmed but not activated by one of the REFRESH commands and the values in register
/// 4Ah are currently active. This register allows software to determine the actual setting that was active prior to the most
/// recent REFRESH command and, therefore, corresponds to the dataset that is held in the readable registers. This
/// register is valid when the results registers are valid, 1 ms after a REFRESH/_V/_G command
pub struct AccumConfigLat {
    #[packed_field(bits = "7:6", ty = "enum")]
    pub acc1_config: AccumSetting,
    #[packed_field(bits = "5:4", ty = "enum")]
    pub acc2_config: AccumSetting,
    #[packed_field(bits = "3:2", ty = "enum")]
    pub acc3_config: AccumSetting,
    #[packed_field(bits = "1:0", ty = "enum")]
    pub acc4_config: AccumSetting,
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
