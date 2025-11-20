# Rust PAC194X Power Monitor Driver

A platform-agnostic driver crate for the Microchip [PAC194X](https://ww1.microchip.com/downloads/en/DeviceDoc/PAC194X-Data-Sheet-20006543.pdf) and [PAC195X](https://www.mouser.com/datasheet/2/268/PAC195X_Family_Data_Sheet_20006539B-2933208.pdf) single/multi channel power monitors using the embedded-hal traits.

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
then instantiate the appropriate device. See the `examples` folder for example code.

Run it on Linux with `cargo build --examples linux && sudo ./target/debug/examples/linux`.
It's hardcoded to bus `/dev/i2c-3` and I2C address 0b10000 (grounded).

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
