use linux_embedded_hal::I2cdev;
use pac194x::{AddrSelect, PAC194X};
use std::{thread, time::Duration};

const SENSE_RESISTORS: [f32; 4] = [0.005, 0.010, 0.010, 0.010];

fn main() {
    let i2c = I2cdev::new("/dev/i2c-3").unwrap();
    let mut sensor = PAC194X::new(i2c, AddrSelect::GND).unwrap();
    loop {
        for channel in 1..5 {
            let bus_voltage = sensor.read_bus_voltage_n(channel).unwrap();
            let sense_voltage = sensor.read_sense_voltage_n(channel).unwrap();
            print!(
                "CH{} {:.2}V, {:.2}A, ",
                channel,
                bus_voltage,
                sense_voltage / SENSE_RESISTORS[(channel - 1) as usize]
            );
        }
        println!();
        println!();

        sensor.refresh().unwrap();
        thread::sleep(Duration::from_millis(100));
        sensor.refresh_v().unwrap();
        thread::sleep(Duration::from_millis(100));
    }
}
