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
    fn lround(_: libc::c_double) -> libc::c_long;
    fn rc_pru_start(ch: libc::c_int, fw_name: *const libc::c_char) -> libc::c_int;
    fn rc_pru_shared_mem_ptr() -> *mut uint32_t;
    fn rc_pru_stop(ch: libc::c_int) -> libc::c_int;
    fn rc_gpio_init(chip: libc::c_int, pin: libc::c_int, handle_flags: libc::c_int) -> libc::c_int;
    fn rc_gpio_set_value(chip: libc::c_int, pin: libc::c_int, value: libc::c_int) -> libc::c_int;
    fn rc_gpio_cleanup(chip: libc::c_int, pin: libc::c_int);
    fn rc_usleep(us: libc::c_uint);
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
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
pub type uint32_t = __uint32_t;
static mut shared_mem_32bit_ptr: *mut libc::c_uint = 0 as *const libc::c_uint as *mut libc::c_uint;
static mut init_flag: libc::c_int = 0 as libc::c_int;
static mut esc_max_us: libc::c_int = 2000 as libc::c_int;
static mut esc_min_us: libc::c_int = 1000 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn rc_servo_init() -> libc::c_int {
    let mut i: libc::c_int = 0;
    if rc_gpio_init(
        2 as libc::c_int,
        16 as libc::c_int,
        ((1 as libc::c_ulong) << 1 as libc::c_int) as libc::c_int,
    ) == -(1 as libc::c_int)
    {
        fprintf(
            stderr,
            b"ERROR in rc_servo_init, failed to set up power rail GPIO pin\n\0" as *const u8 as *const libc::c_char,
        );
        init_flag = 0 as libc::c_int;
        return -(1 as libc::c_int);
    }
    shared_mem_32bit_ptr = rc_pru_shared_mem_ptr();
    if shared_mem_32bit_ptr.is_null() {
        fprintf(
            stderr,
            b"ERROR in rc_servo_init, failed to map shared memory pointer\n\0" as *const u8 as *const libc::c_char,
        );
        init_flag = 0 as libc::c_int;
        return -(1 as libc::c_int);
    }
    i = 1 as libc::c_int;
    while i <= 8 as libc::c_int {
        ::core::ptr::write_volatile(
            shared_mem_32bit_ptr.offset((i - 1 as libc::c_int) as isize),
            42 as libc::c_int as libc::c_uint,
        );
        i += 1;
    }
    if rc_pru_start(
        1 as libc::c_int,
        b"am335x-pru1-rc-servo-fw\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        fprintf(
            stderr,
            b"ERROR in rc_servo_init, failed to start PRU%d\n\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < 40 as libc::c_int {
        if *shared_mem_32bit_ptr.offset(0 as libc::c_int as isize) == 0 as libc::c_int as libc::c_uint {
            init_flag = 1 as libc::c_int;
            return 0 as libc::c_int;
        }
        rc_usleep(100000 as libc::c_int as libc::c_uint);
        i += 1;
    }
    fprintf(
        stderr,
        b"ERROR in rc_servo_init, %s failed to load\n\0" as *const u8 as *const libc::c_char,
        b"am335x-pru1-rc-servo-fw\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"attempting to stop PRU1\n\0" as *const u8 as *const libc::c_char,
    );
    rc_pru_stop(1 as libc::c_int);
    init_flag = 0 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn rc_servo_cleanup() {
    let mut i: libc::c_int = 0;
    if !shared_mem_32bit_ptr.is_null() {
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            ::core::ptr::write_volatile(
                shared_mem_32bit_ptr.offset(i as isize),
                0 as libc::c_int as libc::c_uint,
            );
            i += 1;
        }
    }
    if init_flag != 0 as libc::c_int {
        rc_gpio_set_value(2 as libc::c_int, 16 as libc::c_int, 0 as libc::c_int);
        rc_gpio_cleanup(2 as libc::c_int, 16 as libc::c_int);
    }
    rc_pru_stop(1 as libc::c_int);
    shared_mem_32bit_ptr = 0 as *mut libc::c_uint;
    init_flag = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_servo_power_rail_en(mut en: libc::c_int) -> libc::c_int {
    if init_flag == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_servo_power_rail_en, call rc_servo_init first\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_gpio_set_value(2 as libc::c_int, 16 as libc::c_int, en) == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"ERROR in rc_servo_power_rail_en, failed to write to GPIO pin\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_servo_set_esc_range(mut min: libc::c_int, mut max: libc::c_int) -> libc::c_int {
    if min < 1 as libc::c_int || max < 2 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_servo_set_esc_range, in and max values must be positive\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if min >= max {
        fprintf(
            stderr,
            b"ERROR in rc_servo_set_esc_range. max must be greater than min\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    esc_min_us = min;
    esc_max_us = max;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_servo_send_pulse_us(mut ch: libc::c_int, mut us: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut num_loops: uint32_t = 0;
    if ch < 0 as libc::c_int || ch > 8 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: in rc_servo_send_pulse_us, channel argument must be between 0&%d\n\0" as *const u8
                as *const libc::c_char,
            8 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if init_flag == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: in rc_servo_send_pulse_us, call rc_servo_init first\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    num_loops = (us as libc::c_double * 200.0f64 / 48 as libc::c_int as libc::c_double) as uint32_t;
    if ch != 0 as libc::c_int {
        if *shared_mem_32bit_ptr.offset((ch - 1 as libc::c_int) as isize) != 0 as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"ERROR: in rc_servo_send_pulse_us, tried to start a new pulse amidst another\n\0" as *const u8
                    as *const libc::c_char,
            );
            fprintf(
                stderr,
                b"PRU may need more time to start up before sending pulses\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        ::core::ptr::write_volatile(shared_mem_32bit_ptr.offset((ch - 1 as libc::c_int) as isize), num_loops);
        return 0 as libc::c_int;
    }
    ret = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= 8 as libc::c_int {
        if *shared_mem_32bit_ptr.offset((i - 1 as libc::c_int) as isize) != 0 as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"ERROR: in rc_servo_send_pulse_us, tried to start a new pulse amidst another on channel %d\n\0"
                    as *const u8 as *const libc::c_char,
                i,
            );
            fprintf(
                stderr,
                b"current val:%d\n\0" as *const u8 as *const libc::c_char,
                *shared_mem_32bit_ptr.offset((i - 1 as libc::c_int) as isize),
            );
            fprintf(
                stderr,
                b"this either means you are sending pulses too fast, or the PRU binary didn't load properly\n\0"
                    as *const u8 as *const libc::c_char,
            );
            ret = -(1 as libc::c_int);
        } else {
            ::core::ptr::write_volatile(shared_mem_32bit_ptr.offset((i - 1 as libc::c_int) as isize), num_loops);
        }
        i += 1;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn rc_servo_send_pulse_normalized(mut ch: libc::c_int, mut input: libc::c_double) -> libc::c_int {
    let mut us: libc::c_int = 0;
    if input < -1.5f64 - 0.01f64 || input > 1.5f64 + 0.01f64 {
        fprintf(
            stderr,
            b"ERROR in rc_servo_send_pulse_normalized, normalized input must be between -1.5 & 1.5\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    us = (1500 as libc::c_int as libc::c_long + lround(input * 600.0f64)) as libc::c_int;
    return rc_servo_send_pulse_us(ch, us);
}
#[no_mangle]
pub unsafe extern "C" fn rc_servo_send_esc_pulse_normalized(
    mut ch: libc::c_int,
    mut input: libc::c_double,
) -> libc::c_int {
    let mut us: libc::c_int = 0;
    if input < -0.1f64 - 0.01f64 || input > 1.0f64 + 0.01f64 {
        fprintf(
            stderr,
            b"ERROR in rc_servo_send_esc_pulse_normalized, normalized input must be between -0.1 & 1.0\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    us = (esc_min_us as libc::c_long + lround(input * (esc_max_us - esc_min_us) as libc::c_double)) as libc::c_int;
    return rc_servo_send_pulse_us(ch, us);
}
#[no_mangle]
pub unsafe extern "C" fn rc_servo_send_oneshot_pulse_normalized(
    mut ch: libc::c_int,
    mut input: libc::c_double,
) -> libc::c_int {
    let mut us: libc::c_int = 0;
    if input < -0.1f64 - 0.01f64 || input > 1.0f64 + 0.01f64 {
        fprintf(
            stderr,
            b"ERROR in rc_servo_send_oneshot_pulse_normalized, normalized input must be between -0.1 & 1.0\n\0"
                as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    us = (125 as libc::c_int as libc::c_long + lround(input * 125.0f64)) as libc::c_int;
    return rc_servo_send_pulse_us(ch, us);
}
