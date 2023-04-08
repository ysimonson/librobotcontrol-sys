use std::f64::consts::PI;
use std::ffi::c_int;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use lazy_static::lazy_static;

// Bus for Robotics Cape and BeagleboneBlue is 2, interrupt pin is on gpio3.21.
// Change these for your platform.
const I2C_BUS: c_int = 2;
const GPIO_INT_PIN_CHIP: c_int = 3;
const GPIO_INT_PIN_PIN: c_int = 21;

lazy_static! {
    static ref DATA: Mutex<librobotcontrol_sys::mpu::rc_mpu_data_t> =
        Mutex::new(librobotcontrol_sys::mpu::rc_mpu_data_t::default());
}

unsafe extern "C" fn print_data() {
    let data = DATA.lock().unwrap();
    print!("\rheading: {:.1}", data.compass_heading * 360.0 / (PI * 2.0));
}

fn main() {
    let mut conf = unsafe { librobotcontrol_sys::mpu::rc_mpu_default_config() };
    conf.i2c_bus = I2C_BUS;
    conf.gpio_interrupt_pin_chip = GPIO_INT_PIN_CHIP;
    conf.gpio_interrupt_pin = GPIO_INT_PIN_PIN;
    conf.enable_magnetometer = 1;

    // now set up the imu for dmp interrupt operation
    let init_code = unsafe { librobotcontrol_sys::mpu::rc_mpu_initialize_dmp(&mut *DATA.lock().unwrap(), conf) };
    if init_code != 0 {
        println!("rc_mpu_initialize_dmp failed with code={}", init_code);
        return;
    }

    unsafe { librobotcontrol_sys::mpu::rc_mpu_set_dmp_callback(Some(print_data)) };

    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term)).unwrap();
    while !term.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_millis(100));
    }
    unsafe { librobotcontrol_sys::mpu::rc_mpu_power_off() };
}
