use linux_embedded_hal::I2cdev;
use pac194x::{PAC194X, AddrSelect};

const SENSE_RESISTOR: f32 = 0.5;

fn main() {
    let i2c = I2cdev::new("/dev/i2c-3").unwrap();
    let mut sensor1 = PAC194X::new(i2c, AddrSelect::GND);
    loop {
        let bus_voltage_1 = sensor.read_bus_voltage_n(1).unwrap();
        let sense_voltage_1 = sensor.read_sense_voltage_n(1).unwrap();
        println!("Channel 1 has a bus voltage of: {:.2} V", bus_voltage_1);
        println!("Channel 1 is pulling a current of: {:.2} A", sense_voltage_1 / SENSE_RESISTOR);
    }
}
