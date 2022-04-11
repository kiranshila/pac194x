# Rust PAC194X Power Monitor Driver

A platform-agnostic driver crate for the Microchip [PAC194X](https://ww1.microchip.com/downloads/en/DeviceDoc/PAC194X-Data-Sheet-20006543.pdf) single/multi channel power monitor using the embedded-hal traits.

[![Build Status](https://github.com/kiranshila/pac194x/workflows/Main/badge.svg)](https://github.com/kiranshila/pac194x/actions)
[![Docs.rs](https://docs.rs/pac194x/badge.svg)](https://docs.rs/pac194x)
[![Crates.io](https://img.shields.io/crates/v/pac194x)](https://crates.io/crates/pac194x)

This driver allows you to:
- Read/Write every available register as a Rust data structure, allowing you to configure alerts, averaging, etc.
- Read the bus and sense voltages directly as `f32`s

<!-- TODO
[Introductory blog post]()
-->

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the appropriate device.

```rust
use linux_embedded_hal::I2cdev;
use pac194x::{PAC194X, AddrSelect};

const SENSE_RESISTOR: f32 = 0.5;

fn main() {
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = PAC194X::new(i2c, AddrSelect::GND).unwrap();
    loop {
        let bus_voltage_1 = sensor.read_bus_voltage_n(1).unwrap();
        let sense_voltage_1 = sensor.read_sense_voltage_n(1).unwrap();
        println!("Channel 1 has a bus voltage of: {:.2} V", bus_voltage_1);
        println!("Channel 1 is pulling a current of: {:.2} A", sense_voltage_1 / SENSE_RESISTOR);
    }
}
```

## Discussion

I wrote a blog post about the development of this crate [here](https://blog.kiranshila.com/blog/pac_rust_driver.md)

## License
Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
