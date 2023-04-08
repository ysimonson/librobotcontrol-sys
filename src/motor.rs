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
    fn rc_model() -> rc_model_t;
    fn rc_gpio_init(chip: libc::c_int, pin: libc::c_int, handle_flags: libc::c_int) -> libc::c_int;
    fn rc_gpio_cleanup(chip: libc::c_int, pin: libc::c_int);
    fn rc_gpio_set_value(chip: libc::c_int, pin: libc::c_int, value: libc::c_int) -> libc::c_int;
    fn rc_pwm_init(ss: libc::c_int, frequency: libc::c_int) -> libc::c_int;
    fn rc_pwm_cleanup(ss: libc::c_int) -> libc::c_int;
    fn rc_pwm_set_duty(ss: libc::c_int, ch: libc::c_char, duty: libc::c_double) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub const MODEL_BB_POCKET: rc_model_t = 8;
pub type rc_model_t = libc::c_uint;
pub const MODEL_PC: rc_model_t = 18;
pub const MODEL_RPI_CM3: rc_model_t = 17;
pub const MODEL_RPI_CM: rc_model_t = 16;
pub const MODEL_RPI0_W: rc_model_t = 15;
pub const MODEL_RPI0: rc_model_t = 14;
pub const MODEL_RPI3_B_PLUS: rc_model_t = 13;
pub const MODEL_RPI3_B: rc_model_t = 12;
pub const MODEL_RPI2_B: rc_model_t = 11;
pub const MODEL_RPI_B_PLUS: rc_model_t = 10;
pub const MODEL_RPI_B: rc_model_t = 9;
pub const MODEL_BB_BLUE: rc_model_t = 7;
pub const MODEL_BB_GREEN_W: rc_model_t = 6;
pub const MODEL_BB_GREEN: rc_model_t = 5;
pub const MODEL_BB_BLACK_W_RC: rc_model_t = 4;
pub const MODEL_BB_BLACK_W: rc_model_t = 3;
pub const MODEL_BB_BLACK_RC: rc_model_t = 2;
pub const MODEL_BB_BLACK: rc_model_t = 1;
pub const MODEL_UNKNOWN: rc_model_t = 0;
static mut polarity: [libc::c_double; 4] = [1.0f64, -1.0f64, -1.0f64, 1.0f64];
static mut init_flag: libc::c_int = 0 as libc::c_int;
static mut stby_state: libc::c_int = 0 as libc::c_int;
static mut dirA_chip: [libc::c_int; 4] = [0; 4];
static mut dirA_pin: [libc::c_int; 4] = [0; 4];
static mut dirB_chip: [libc::c_int; 4] = [0; 4];
static mut dirB_pin: [libc::c_int; 4] = [0; 4];
static mut pwmss: [libc::c_int; 4] = [0; 4];
static mut pwmch: [libc::c_int; 4] = [0; 4];
static mut channels: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn rc_motor_init() -> libc::c_int {
    return rc_motor_init_freq(25000 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn rc_motor_init_freq(mut pwm_frequency_hz: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if rc_model() as libc::c_uint == MODEL_BB_POCKET as libc::c_int as libc::c_uint {
        channels = 2 as libc::c_int;
    } else {
        channels = 4 as libc::c_int;
    }
    if rc_model() as libc::c_uint == MODEL_BB_BLUE as libc::c_int as libc::c_uint {
        dirA_chip[0 as libc::c_int as usize] = 2 as libc::c_int;
        dirA_pin[0 as libc::c_int as usize] = 0 as libc::c_int;
    } else {
        dirA_chip[0 as libc::c_int as usize] = 1 as libc::c_int;
        dirA_pin[0 as libc::c_int as usize] = 28 as libc::c_int;
    }
    dirB_chip[0 as libc::c_int as usize] = 0 as libc::c_int;
    dirB_pin[0 as libc::c_int as usize] = 31 as libc::c_int;
    pwmss[0 as libc::c_int as usize] = 1 as libc::c_int;
    pwmch[0 as libc::c_int as usize] = 'A' as i32;
    if rc_model() as libc::c_uint == MODEL_BB_POCKET as libc::c_int as libc::c_uint {
        dirA_chip[1 as libc::c_int as usize] = 0 as libc::c_int;
        dirA_pin[1 as libc::c_int as usize] = 26 as libc::c_int;
    } else {
        dirA_chip[1 as libc::c_int as usize] = 1 as libc::c_int;
        dirA_pin[1 as libc::c_int as usize] = 16 as libc::c_int;
    }
    if rc_model() as libc::c_uint == MODEL_BB_BLUE as libc::c_int as libc::c_uint {
        dirB_chip[1 as libc::c_int as usize] = 0 as libc::c_int;
        dirB_pin[1 as libc::c_int as usize] = 10 as libc::c_int;
    } else if rc_model() as libc::c_uint == MODEL_BB_POCKET as libc::c_int as libc::c_uint {
        dirB_chip[1 as libc::c_int as usize] = 1 as libc::c_int;
        dirB_pin[1 as libc::c_int as usize] = 27 as libc::c_int;
    } else {
        dirB_chip[1 as libc::c_int as usize] = 2 as libc::c_int;
        dirB_pin[1 as libc::c_int as usize] = 17 as libc::c_int;
    }
    if rc_model() as libc::c_uint == MODEL_BB_POCKET as libc::c_int as libc::c_uint {
        pwmss[1 as libc::c_int as usize] = 0 as libc::c_int;
        pwmch[1 as libc::c_int as usize] = 'A' as i32;
    } else {
        pwmss[1 as libc::c_int as usize] = 1 as libc::c_int;
        pwmch[1 as libc::c_int as usize] = 'B' as i32;
    }
    dirA_chip[2 as libc::c_int as usize] = 2 as libc::c_int;
    dirA_pin[2 as libc::c_int as usize] = 9 as libc::c_int;
    dirB_chip[2 as libc::c_int as usize] = 2 as libc::c_int;
    dirB_pin[2 as libc::c_int as usize] = 8 as libc::c_int;
    pwmss[2 as libc::c_int as usize] = 2 as libc::c_int;
    pwmch[2 as libc::c_int as usize] = 'A' as i32;
    dirA_chip[3 as libc::c_int as usize] = 2 as libc::c_int;
    dirA_pin[3 as libc::c_int as usize] = 6 as libc::c_int;
    dirB_chip[3 as libc::c_int as usize] = 2 as libc::c_int;
    dirB_pin[3 as libc::c_int as usize] = 7 as libc::c_int;
    pwmss[3 as libc::c_int as usize] = 2 as libc::c_int;
    pwmch[3 as libc::c_int as usize] = 'B' as i32;
    if (rc_pwm_init(0 as libc::c_int, pwm_frequency_hz) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_motor_init, failed to initialize pwm subsystem 1\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_pwm_init(1 as libc::c_int, pwm_frequency_hz) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_motor_init, failed to initialize pwm subsystem 1\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_pwm_init(2 as libc::c_int, pwm_frequency_hz) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_motor_init, failed to initialize pwm subsystem 2\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_gpio_init(
        0 as libc::c_int,
        20 as libc::c_int,
        ((1 as libc::c_ulong) << 1 as libc::c_int) as libc::c_int,
    ) != 0) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"ERROR in rc_motor_init, failed to set up gpio %d,%d\n\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            20 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < channels {
        if (rc_gpio_init(
            dirA_chip[i as usize],
            dirA_pin[i as usize],
            ((1 as libc::c_ulong) << 1 as libc::c_int) as libc::c_int,
        ) != 0) as libc::c_int as libc::c_long
            != 0
        {
            fprintf(
                stderr,
                b"ERROR in rc_motor_init, failed to set up gpio %d,%d\n\0" as *const u8 as *const libc::c_char,
                dirA_chip[i as usize],
                dirA_pin[i as usize],
            );
            return -(1 as libc::c_int);
        }
        if (rc_gpio_init(
            dirB_chip[i as usize],
            dirB_pin[i as usize],
            ((1 as libc::c_ulong) << 1 as libc::c_int) as libc::c_int,
        ) != 0) as libc::c_int as libc::c_long
            != 0
        {
            fprintf(
                stderr,
                b"ERROR in rc_motor_init, failed to set up gpio %d,%d\n\0" as *const u8 as *const libc::c_char,
                dirB_chip[i as usize],
                dirB_pin[i as usize],
            );
            return -(1 as libc::c_int);
        }
        i += 1;
    }
    stby_state = 0 as libc::c_int;
    init_flag = 1 as libc::c_int;
    if (rc_motor_free_spin(0 as libc::c_int) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_motor_init\n\0" as *const u8 as *const libc::c_char,
        );
        init_flag = 0 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if (rc_gpio_set_value(0 as libc::c_int, 20 as libc::c_int, 1 as libc::c_int) != 0) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"ERROR in rc_motor_init, can't write to gpio %d,%d\n\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            20 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    stby_state = 0 as libc::c_int;
    init_flag = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_motor_cleanup() -> libc::c_int {
    let mut i: libc::c_int = 0;
    if init_flag == 0 {
        return 0 as libc::c_int;
    }
    rc_motor_free_spin(0 as libc::c_int);
    rc_pwm_cleanup(0 as libc::c_int);
    rc_pwm_cleanup(1 as libc::c_int);
    rc_pwm_cleanup(2 as libc::c_int);
    rc_gpio_cleanup(0 as libc::c_int, 20 as libc::c_int);
    i = 0 as libc::c_int;
    while i < channels {
        rc_gpio_cleanup(dirA_chip[i as usize], dirA_pin[i as usize]);
        rc_gpio_cleanup(dirB_chip[i as usize], dirB_pin[i as usize]);
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_motor_standby(mut standby_en: libc::c_int) -> libc::c_int {
    let mut val: libc::c_int = 0;
    if (init_flag == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_motor_standby, must call rc_motor_init first\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if standby_en != 0 {
        if stby_state != 0 {
            return 0 as libc::c_int;
        }
        val = 0 as libc::c_int;
        rc_motor_free_spin(0 as libc::c_int);
    } else {
        if stby_state == 0 {
            return 0 as libc::c_int;
        }
        val = 1 as libc::c_int;
    }
    if (rc_gpio_set_value(0 as libc::c_int, 20 as libc::c_int, val) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_motor_standby, unable to write to gpio %d,%d\n\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            20 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    stby_state = standby_en;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_motor_set(mut motor: libc::c_int, mut duty: libc::c_double) -> libc::c_int {
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if (motor < 0 as libc::c_int || motor > channels) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_motor_set, motor argument must be between 0 & %d\n\0" as *const u8 as *const libc::c_char,
            channels,
        );
        return -(1 as libc::c_int);
    }
    if (init_flag == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_motor_set, call rc_motor_init first\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if duty > 1.0f64 {
        duty = 1.0f64;
    } else if duty < -1.0f64 {
        duty = -1.0f64;
    }
    if motor == 0 as libc::c_int {
        i = 1 as libc::c_int;
        while i <= channels {
            if rc_motor_set(i, duty) == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            i += 1;
        }
        return 0 as libc::c_int;
    }
    duty = duty * polarity[(motor - 1 as libc::c_int) as usize];
    if duty >= 0.0f64 {
        a = 1 as libc::c_int;
        b = 0 as libc::c_int;
    } else {
        a = 0 as libc::c_int;
        b = 1 as libc::c_int;
        duty = -duty;
    }
    if (rc_gpio_set_value(
        dirA_chip[(motor - 1 as libc::c_int) as usize],
        dirA_pin[(motor - 1 as libc::c_int) as usize],
        a,
    ) != 0) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"ERROR in rc_motor_set, failed to write to gpio pin %d,%d\n\0" as *const u8 as *const libc::c_char,
            dirA_chip[(motor - 1 as libc::c_int) as usize],
            dirA_pin[(motor - 1 as libc::c_int) as usize],
        );
        return -(1 as libc::c_int);
    }
    if (rc_gpio_set_value(
        dirB_chip[(motor - 1 as libc::c_int) as usize],
        dirB_pin[(motor - 1 as libc::c_int) as usize],
        b,
    ) != 0) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"ERROR in rc_motor_set, failed to write to gpio pin %d,%d\n\0" as *const u8 as *const libc::c_char,
            dirB_chip[(motor - 1 as libc::c_int) as usize],
            dirB_pin[(motor - 1 as libc::c_int) as usize],
        );
        return -(1 as libc::c_int);
    }
    if (rc_pwm_set_duty(
        pwmss[(motor - 1 as libc::c_int) as usize],
        pwmch[(motor - 1 as libc::c_int) as usize] as libc::c_char,
        duty,
    ) != 0) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"ERROR in rc_motor_set, failed to write to pwm %d%c\n\0" as *const u8 as *const libc::c_char,
            pwmss[(motor - 1 as libc::c_int) as usize],
            pwmch[(motor - 1 as libc::c_int) as usize],
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_motor_free_spin(mut motor: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (motor < 0 as libc::c_int || motor > channels) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_motor_free_spin, motor argument must be between 0 & %d\n\0" as *const u8
                as *const libc::c_char,
            channels,
        );
        return -(1 as libc::c_int);
    }
    if (init_flag == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_motor_free_spin, call rc_motor_init first\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if motor == 0 as libc::c_int {
        i = 1 as libc::c_int;
        while i <= channels {
            if rc_motor_free_spin(i) == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            i += 1;
        }
        return 0 as libc::c_int;
    }
    if (rc_gpio_set_value(
        dirA_chip[(motor - 1 as libc::c_int) as usize],
        dirA_pin[(motor - 1 as libc::c_int) as usize],
        0 as libc::c_int,
    ) != 0) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"ERROR in rc_motor_free_spin, failed to write to gpio pin %d,%d\n\0" as *const u8 as *const libc::c_char,
            dirA_chip[(motor - 1 as libc::c_int) as usize],
            dirA_pin[(motor - 1 as libc::c_int) as usize],
        );
        return -(1 as libc::c_int);
    }
    if (rc_gpio_set_value(
        dirB_chip[(motor - 1 as libc::c_int) as usize],
        dirB_pin[(motor - 1 as libc::c_int) as usize],
        0 as libc::c_int,
    ) != 0) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"ERROR in rc_motor_free_spin, failed to write to gpio pin %d,%d\n\0" as *const u8 as *const libc::c_char,
            dirB_chip[(motor - 1 as libc::c_int) as usize],
            dirB_pin[(motor - 1 as libc::c_int) as usize],
        );
        return -(1 as libc::c_int);
    }
    if (rc_pwm_set_duty(
        pwmss[(motor - 1 as libc::c_int) as usize],
        pwmch[(motor - 1 as libc::c_int) as usize] as libc::c_char,
        0.0f64,
    ) != 0) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"ERROR in rc_motor_free_spin, failed to write to pwm %d%c\n\0" as *const u8 as *const libc::c_char,
            pwmss[(motor - 1 as libc::c_int) as usize],
            pwmch[(motor - 1 as libc::c_int) as usize],
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_motor_brake(mut motor: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (motor < 0 as libc::c_int || motor > channels) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_motor_brake, motor argument must be between 0 & %d\n\0" as *const u8 as *const libc::c_char,
            channels,
        );
        return -(1 as libc::c_int);
    }
    if (init_flag == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_motor_brake, call rc_motor_init first\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if motor == 0 as libc::c_int {
        i = 1 as libc::c_int;
        while i <= channels {
            if rc_motor_brake(i) == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            i += 1;
        }
        return 0 as libc::c_int;
    }
    if (rc_gpio_set_value(
        dirA_chip[(motor - 1 as libc::c_int) as usize],
        dirA_pin[(motor - 1 as libc::c_int) as usize],
        1 as libc::c_int,
    ) != 0) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"ERROR in rc_motor_brake, failed to write to gpio pin %d,%d\n\0" as *const u8 as *const libc::c_char,
            dirA_chip[(motor - 1 as libc::c_int) as usize],
            dirA_pin[(motor - 1 as libc::c_int) as usize],
        );
        return -(1 as libc::c_int);
    }
    if (rc_gpio_set_value(
        dirB_chip[(motor - 1 as libc::c_int) as usize],
        dirB_pin[(motor - 1 as libc::c_int) as usize],
        1 as libc::c_int,
    ) != 0) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"ERROR in rc_motor_brake, failed to write to gpio pin %d,%d\n\0" as *const u8 as *const libc::c_char,
            dirB_chip[(motor - 1 as libc::c_int) as usize],
            dirB_pin[(motor - 1 as libc::c_int) as usize],
        );
        return -(1 as libc::c_int);
    }
    if (rc_pwm_set_duty(
        pwmss[(motor - 1 as libc::c_int) as usize],
        pwmch[(motor - 1 as libc::c_int) as usize] as libc::c_char,
        0.0f64,
    ) != 0) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"ERROR in rc_motor_brake, failed to write to pwm %d%c\n\0" as *const u8 as *const libc::c_char,
            pwmss[(motor - 1 as libc::c_int) as usize],
            pwmch[(motor - 1 as libc::c_int) as usize],
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
