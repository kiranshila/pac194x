use embedded_hal_bus::i2c::RefCellDevice;
use linux_embedded_hal::I2cdev;
use pac194x::{AddrSelect, PAC194X};
use std::cell::RefCell;
use std::{thread, time::Duration};

const SENSE_RESISTORS: [f32; 8] = [0.005, 0.010, 0.010, 0.010, 0.002, 0.010, 0.010, 0.010];

fn main() {
    let i2c = RefCell::new(I2cdev::new("/dev/i2c-3").unwrap());

    let bus_handle1 = RefCellDevice::new(&i2c);
    let mut sensor1 = PAC194X::new(bus_handle1, AddrSelect::GND).unwrap();

    let bus_handle2 = RefCellDevice::new(&i2c);
    let mut sensor2 = PAC194X::new(bus_handle2, AddrSelect::_499).unwrap();

    loop {
        print!("Sensor 1 ");
        for channel in 1..5 {
            let bus_voltage = sensor1.read_bus_voltage_n(channel).unwrap();
            let sense_voltage = sensor1.read_sense_voltage_n(channel).unwrap();
            print!(
                "CH{} {:5.2}V, {:5.2}A, ",
                channel,
                bus_voltage,
                sense_voltage / SENSE_RESISTORS[(channel - 1) as usize]
            );
        }
        println!();
        print!("Sensor 2 ");
        for channel in 1..5 {
            let bus_voltage = sensor2.read_bus_voltage_n(channel).unwrap();
            let sense_voltage = sensor2.read_sense_voltage_n(channel).unwrap();
            print!(
                "CH{} {:5.2}V, {:5.2}A, ",
                channel,
                bus_voltage,
                sense_voltage / SENSE_RESISTORS[(channel - 1 + 4) as usize]
            );
        }
        println!();
        println!();
        sensor1.refresh().unwrap();
        thread::sleep(Duration::from_millis(100));
        sensor1.refresh_v().unwrap();
        thread::sleep(Duration::from_millis(100));
        sensor2.refresh().unwrap();
        thread::sleep(Duration::from_millis(100));
        sensor2.refresh_v().unwrap();
    }
}
