use std::f64::consts::PI;
use std::ffi::c_int;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// Bus for Robotics Cape and BeagleboneBlue is 2, interrupt pin is on gpio3.21.
// Change these for your platform.
const I2C_BUS: c_int = 2;
const GPIO_INT_PIN_CHIP: c_int = 3;
const GPIO_INT_PIN_PIN: c_int = 21;
static mut DATA: Option<librobotcontrol_sys::rc_mpu_data_t> = None;

unsafe extern "C" fn print_data() {
    print!("\rheading: {:.1}", DATA.as_ref().unwrap().compass_heading * 360.0 / (PI * 2.0));
}

fn main() {
    unsafe { DATA = Some(librobotcontrol_sys::rc_mpu_data_t::default()); }

    let mut conf = unsafe { librobotcontrol_sys::rc_mpu_default_config() };
    conf.i2c_bus = I2C_BUS;
    conf.gpio_interrupt_pin_chip = GPIO_INT_PIN_CHIP;
    conf.gpio_interrupt_pin = GPIO_INT_PIN_PIN;
    conf.enable_magnetometer = 1;

    // now set up the imu for dmp interrupt operation
    let init_code = unsafe { librobotcontrol_sys::rc_mpu_initialize_dmp(DATA.as_mut().unwrap(), conf) };
    if init_code != 0 {
        println!("rc_mpu_initialize_dmp failed with code={}", init_code);
        return;
    }

    unsafe { librobotcontrol_sys::rc_mpu_set_dmp_callback(Some(print_data)) };

    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term)).unwrap();
    while !term.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_millis(100));
    }
    unsafe { librobotcontrol_sys::rc_mpu_power_off() };
}
