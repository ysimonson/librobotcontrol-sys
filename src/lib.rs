#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(arithmetic_overflow)]
#![feature(extern_types)]

pub mod bmp;
pub mod button;
pub mod cpu;
pub mod dsm;
pub mod io {
    pub mod adc;
    pub mod encoder_eqep;
    pub mod gpio;
    pub mod i2c;
    pub mod pwm;
    pub mod spi;
    pub mod uart;
} // mod io
pub mod led;
pub mod math {
    pub mod algebra;
    pub mod algebra_common;
    pub mod filter;
    pub mod matrix;
    pub mod other;
    pub mod polynomial;
    pub mod quaternion;
    pub mod ring_buffer;
    pub mod vector;
} // mod math
pub mod model;
pub mod motor;
pub mod mpu;
pub mod pinmux;
pub mod pru {
    pub mod encoder_pru;
    pub mod pru;
    pub mod servo;
} // mod pru
pub mod pthread;
pub mod start_stop;
pub mod time;
