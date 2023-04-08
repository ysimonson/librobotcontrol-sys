#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn rc_i2c_write_byte(bus: libc::c_int, regAddr: uint8_t, data: uint8_t) -> libc::c_int;
    fn rc_i2c_init(bus: libc::c_int, devAddr: uint8_t) -> libc::c_int;
    fn rc_i2c_set_device_address(bus: libc::c_int, devAddr: uint8_t) -> libc::c_int;
    fn rc_i2c_read_byte(bus: libc::c_int, regAddr: uint8_t, data: *mut uint8_t) -> libc::c_int;
    fn rc_i2c_read_bytes(bus: libc::c_int, regAddr: uint8_t, count: size_t, data: *mut uint8_t) -> libc::c_int;
    fn rc_i2c_lock_bus(bus: libc::c_int) -> libc::c_int;
    fn rc_i2c_unlock_bus(bus: libc::c_int) -> libc::c_int;
    fn rc_i2c_get_lock(bus: libc::c_int) -> libc::c_int;
    fn rc_usleep(us: libc::c_uint);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type rc_bmp_oversample_t = libc::c_uint;
pub const BMP_OVERSAMPLE_16: rc_bmp_oversample_t = 20;
pub const BMP_OVERSAMPLE_8: rc_bmp_oversample_t = 16;
pub const BMP_OVERSAMPLE_4: rc_bmp_oversample_t = 12;
pub const BMP_OVERSAMPLE_2: rc_bmp_oversample_t = 8;
pub const BMP_OVERSAMPLE_1: rc_bmp_oversample_t = 4;
pub type rc_bmp_filter_t = libc::c_uint;
pub const BMP_FILTER_16: rc_bmp_filter_t = 16;
pub const BMP_FILTER_8: rc_bmp_filter_t = 12;
pub const BMP_FILTER_4: rc_bmp_filter_t = 8;
pub const BMP_FILTER_2: rc_bmp_filter_t = 4;
pub const BMP_FILTER_OFF: rc_bmp_filter_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rc_bmp_data_t {
    pub temp_c: libc::c_double,
    pub alt_m: libc::c_double,
    pub pressure_pa: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmp280_cal_t {
    pub dig_T1: uint16_t,
    pub dig_T2: int16_t,
    pub dig_T3: int16_t,
    pub dig_P1: uint16_t,
    pub dig_P2: int16_t,
    pub dig_P3: int16_t,
    pub dig_P4: int16_t,
    pub dig_P5: int16_t,
    pub dig_P6: int16_t,
    pub dig_P7: int16_t,
    pub dig_P8: int16_t,
    pub dig_P9: int16_t,
    pub sea_level_pa: libc::c_double,
}
static mut rc_bmp280_cal: bmp280_cal_t = bmp280_cal_t {
    dig_T1: 0,
    dig_T2: 0,
    dig_T3: 0,
    dig_P1: 0,
    dig_P2: 0,
    dig_P3: 0,
    dig_P4: 0,
    dig_P5: 0,
    dig_P6: 0,
    dig_P7: 0,
    dig_P8: 0,
    dig_P9: 0,
    sea_level_pa: 0.,
};
static mut rc_bmp280_init_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn rc_bmp_init(mut oversample: rc_bmp_oversample_t, mut filter: rc_bmp_filter_t) -> libc::c_int {
    let mut buf: [uint8_t; 24] = [0; 24];
    let mut c: uint8_t = 0;
    let mut i: libc::c_int = 0;
    if rc_i2c_get_lock(2 as libc::c_int) != 0 {
        fprintf(
            stderr,
            b"WARNING in rc_bmp_init, i2c bus claimed by another thread\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(stderr, b"Continuing anyway.\n\0" as *const u8 as *const libc::c_char);
    }
    if rc_i2c_init(2 as libc::c_int, 0x76 as libc::c_int as uint8_t) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: in rc_bmp_init failed to initialize i2c bus\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_i2c_lock_bus(2 as libc::c_int);
    if rc_i2c_write_byte(
        2 as libc::c_int,
        0xe0 as libc::c_int as uint8_t,
        0xb6 as libc::c_int as uint8_t,
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"ERROR: in rc_bmp_init failed to send reset byte to barometer\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(2 as libc::c_int);
        return -(1 as libc::c_int);
    }
    if rc_i2c_read_byte(2 as libc::c_int, 0xd0 as libc::c_int as uint8_t, &mut c) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: in rc_bmp_init, failed to read chip_id register\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(2 as libc::c_int);
        return -(1 as libc::c_int);
    }
    if c as libc::c_int != 0x58 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: in rc_bmp_init, barometer returned wrong chip_id\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"received: %x  expected: %x\n\0" as *const u8 as *const libc::c_char,
            c as libc::c_int,
            0xd0 as libc::c_int,
        );
        rc_i2c_unlock_bus(2 as libc::c_int);
        return -(1 as libc::c_int);
    }
    c = 0x3 as libc::c_int as uint8_t;
    c = (c as libc::c_int | (0x1 as libc::c_int) << 5 as libc::c_int) as uint8_t;
    c = (c as libc::c_uint | oversample as libc::c_uint) as uint8_t;
    if rc_i2c_write_byte(2 as libc::c_int, 0xf4 as libc::c_int as uint8_t, c) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: in rc_bmp_init, can't write to measurement control register\n\0" as *const u8
                as *const libc::c_char,
        );
        rc_i2c_unlock_bus(2 as libc::c_int);
        return -(1 as libc::c_int);
    }
    c = 0 as libc::c_int as uint8_t;
    c = (c as libc::c_uint | filter as libc::c_uint) as uint8_t;
    if rc_i2c_write_byte(2 as libc::c_int, 0xf5 as libc::c_int as uint8_t, c) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: in rc_bmp_init, failed to write to bmp_config register\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(2 as libc::c_int);
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    c = 0x1 as libc::c_int as uint8_t;
    loop {
        rc_usleep(20000 as libc::c_int as libc::c_uint);
        if rc_i2c_read_byte(2 as libc::c_int, 0xf3 as libc::c_int as uint8_t, &mut c) < 0 as libc::c_int {
            fprintf(
                stderr,
                b"ERROR: in rc_bmp_init can't read status byte from barometer\n\0" as *const u8 as *const libc::c_char,
            );
            rc_i2c_unlock_bus(2 as libc::c_int);
            return -(1 as libc::c_int);
        }
        if i > 10 as libc::c_int {
            fprintf(
                stderr,
                b"ERROR: in rc_bmp_init factory NVM calibration not available yet\n\0" as *const u8
                    as *const libc::c_char,
            );
            rc_i2c_unlock_bus(2 as libc::c_int);
            return -(1 as libc::c_int);
        }
        i += 1;
        if !(c as libc::c_int & 0x1 as libc::c_int != 0) {
            break;
        }
    }
    if rc_i2c_read_bytes(
        2 as libc::c_int,
        0x88 as libc::c_int as uint8_t,
        24 as libc::c_int as size_t,
        buf.as_mut_ptr(),
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"ERROR: in rc_bmp_init failed to load factory calibration registers\n\0" as *const u8
                as *const libc::c_char,
        );
        rc_i2c_unlock_bus(2 as libc::c_int);
        return -(1 as libc::c_int);
    }
    rc_bmp280_cal.dig_T1 = ((buf[1 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
        | buf[0 as libc::c_int as usize] as libc::c_int) as uint16_t;
    rc_bmp280_cal.dig_T2 = ((buf[3 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
        | buf[2 as libc::c_int as usize] as libc::c_int) as uint16_t as int16_t;
    rc_bmp280_cal.dig_T3 = ((buf[5 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
        | buf[4 as libc::c_int as usize] as libc::c_int) as uint16_t as int16_t;
    rc_bmp280_cal.dig_P1 = ((buf[7 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
        | buf[6 as libc::c_int as usize] as libc::c_int) as uint16_t;
    rc_bmp280_cal.dig_P2 = ((buf[9 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
        | buf[8 as libc::c_int as usize] as libc::c_int) as uint16_t as int16_t;
    rc_bmp280_cal.dig_P3 = ((buf[11 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
        | buf[10 as libc::c_int as usize] as libc::c_int) as uint16_t as int16_t;
    rc_bmp280_cal.dig_P4 = ((buf[13 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
        | buf[12 as libc::c_int as usize] as libc::c_int) as uint16_t as int16_t;
    rc_bmp280_cal.dig_P5 = ((buf[15 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
        | buf[14 as libc::c_int as usize] as libc::c_int) as uint16_t as int16_t;
    rc_bmp280_cal.dig_P6 = ((buf[17 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
        | buf[16 as libc::c_int as usize] as libc::c_int) as uint16_t as int16_t;
    rc_bmp280_cal.dig_P7 = ((buf[19 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
        | buf[18 as libc::c_int as usize] as libc::c_int) as uint16_t as int16_t;
    rc_bmp280_cal.dig_P8 = ((buf[21 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
        | buf[20 as libc::c_int as usize] as libc::c_int) as uint16_t as int16_t;
    rc_bmp280_cal.dig_P9 = ((buf[23 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
        | buf[22 as libc::c_int as usize] as libc::c_int) as uint16_t as int16_t;
    rc_bmp280_cal.sea_level_pa = 101325 as libc::c_int as libc::c_double;
    rc_i2c_unlock_bus(2 as libc::c_int);
    rc_usleep(50000 as libc::c_int as libc::c_uint);
    rc_bmp280_init_flag = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_bmp_power_off() -> libc::c_int {
    if rc_i2c_get_lock(2 as libc::c_int) != 0 {
        fprintf(
            stderr,
            b"WARNING: in rc_bmp_power_off i2c bus claimed by another thread\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(stderr, b"Continuing anyway.\n\0" as *const u8 as *const libc::c_char);
    }
    if rc_i2c_set_device_address(2 as libc::c_int, 0x76 as libc::c_int as uint8_t) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: in rc_bmp_power_off failed to set the i2c device address\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(2 as libc::c_int);
        return -(1 as libc::c_int);
    }
    if rc_i2c_write_byte(
        2 as libc::c_int,
        0xf4 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"ERROR: in rc_bmp_power_off cannot write bmp control mode register\n\0" as *const u8
                as *const libc::c_char,
        );
        rc_i2c_unlock_bus(2 as libc::c_int);
        return -(1 as libc::c_int);
    }
    rc_i2c_unlock_bus(2 as libc::c_int);
    rc_bmp280_init_flag = 0 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_bmp_read(mut data: *mut rc_bmp_data_t) -> libc::c_int {
    let mut var1: int64_t = 0;
    let mut var2: int64_t = 0;
    let mut var3: int64_t = 0;
    let mut var4: int64_t = 0;
    let mut t_fine: int64_t = 0;
    let mut T: int64_t = 0;
    let mut p: int64_t = 0;
    let mut raw: [uint8_t; 6] = [0; 6];
    let mut adc_P: int32_t = 0;
    let mut adc_T: int32_t = 0;
    if rc_bmp280_init_flag == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_bmp_read, call rc_bmp_init first\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if data.is_null() {
        fprintf(
            stderr,
            b"ERROR in rc_bmp_read, received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_i2c_get_lock(2 as libc::c_int) != 0 {
        fprintf(
            stderr,
            b"WARNING: in rc_bmp_read, i2c bus is claimed by another thread, aborting\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_i2c_lock_bus(2 as libc::c_int);
    if rc_i2c_set_device_address(2 as libc::c_int, 0x76 as libc::c_int as uint8_t) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: in rc_bmp_read, failed to set the i2c device address\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(2 as libc::c_int);
        return -(1 as libc::c_int);
    }
    if rc_i2c_read_bytes(
        2 as libc::c_int,
        0xf7 as libc::c_int as uint8_t,
        6 as libc::c_int as size_t,
        raw.as_mut_ptr(),
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"ERROR: in rc_bmp_read, failed to read barometer data registers\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(2 as libc::c_int);
        return -(1 as libc::c_int);
    }
    rc_i2c_unlock_bus(2 as libc::c_int);
    adc_P = (raw[0 as libc::c_int as usize] as libc::c_int) << 12 as libc::c_int
        | (raw[1 as libc::c_int as usize] as libc::c_int) << 4 as libc::c_int
        | raw[2 as libc::c_int as usize] as libc::c_int >> 4 as libc::c_int;
    adc_T = (raw[3 as libc::c_int as usize] as libc::c_int) << 12 as libc::c_int
        | (raw[4 as libc::c_int as usize] as libc::c_int) << 4 as libc::c_int
        | raw[5 as libc::c_int as usize] as libc::c_int >> 4 as libc::c_int;
    var1 = (((adc_T >> 3 as libc::c_int) - ((rc_bmp280_cal.dig_T1 as int32_t) << 1 as libc::c_int))
        * rc_bmp280_cal.dig_T2 as int32_t
        >> 11 as libc::c_int) as int64_t;
    var2 = ((((adc_T >> 4 as libc::c_int) - rc_bmp280_cal.dig_T1 as int32_t)
        * ((adc_T >> 4 as libc::c_int) - rc_bmp280_cal.dig_T1 as int32_t)
        >> 12 as libc::c_int)
        * rc_bmp280_cal.dig_T3 as int32_t
        >> 14 as libc::c_int) as int64_t;
    t_fine = var1 + var2;
    T = t_fine * 5 as libc::c_int as libc::c_long + 128 as libc::c_int as libc::c_long >> 8 as libc::c_int;
    (*data).temp_c = T as libc::c_double / 100.0f64;
    var3 = t_fine - 128000 as libc::c_int as libc::c_long;
    var4 = var3 * var3 * rc_bmp280_cal.dig_P6 as int64_t;
    var4 = var4 + ((var3 * rc_bmp280_cal.dig_P5 as int64_t) << 17 as libc::c_int);
    var4 = var4 + ((rc_bmp280_cal.dig_P4 as int64_t) << 35 as libc::c_int);
    var3 = (var3 * var3 * rc_bmp280_cal.dig_P3 as int64_t >> 8 as libc::c_int)
        + ((var3 * rc_bmp280_cal.dig_P2 as int64_t) << 12 as libc::c_int);
    var3 = (((1 as libc::c_int as int64_t) << 47 as libc::c_int) + var3) * rc_bmp280_cal.dig_P1 as int64_t
        >> 33 as libc::c_int;
    if var3 == 0 as libc::c_int as libc::c_long {
        fprintf(
            stderr,
            b"ERROR in rc_bmp_read, invalid data read\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    p = (1048576 as libc::c_int - adc_P) as int64_t;
    p = ((p << 31 as libc::c_int) - var4) * 3125 as libc::c_int as libc::c_long / var3;
    var3 = rc_bmp280_cal.dig_P9 as int64_t * (p >> 13 as libc::c_int) * (p >> 13 as libc::c_int) >> 25 as libc::c_int;
    var4 = rc_bmp280_cal.dig_P8 as int64_t * p >> 19 as libc::c_int;
    p = (p + var3 + var4 >> 8 as libc::c_int) + ((rc_bmp280_cal.dig_P7 as int64_t) << 4 as libc::c_int);
    (*data).pressure_pa = p as libc::c_double / 256.0f64;
    (*data).alt_m = 44330.0f64 * (1.0f64 - pow((*data).pressure_pa / rc_bmp280_cal.sea_level_pa, 0.1903f64));
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_bmp_set_sea_level_pressure_pa(mut pa: libc::c_double) -> libc::c_int {
    if rc_bmp280_init_flag == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_set_sea_level_pressure_pa, call rc_bmp_init first\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if pa < 80000 as libc::c_int as libc::c_double || pa > 120000 as libc::c_int as libc::c_double {
        fprintf(
            stderr,
            b"ERROR: in rc_set_sea_level_pressure_pa, Please enter a reasonable\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b" pressure between 80,000 & 120,000 pascals.\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_bmp280_cal.sea_level_pa = pa;
    return 0 as libc::c_int;
}
