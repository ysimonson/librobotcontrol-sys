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
    fn __errno_location() -> *mut libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn rc_model() -> rc_model_t;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
pub type rc_pinmux_mode_t = libc::c_uint;
pub const PINMUX_CAN: rc_pinmux_mode_t = 6;
pub const PINMUX_UART: rc_pinmux_mode_t = 5;
pub const PINMUX_SPI: rc_pinmux_mode_t = 4;
pub const PINMUX_PWM: rc_pinmux_mode_t = 3;
pub const PINMUX_GPIO_PD: rc_pinmux_mode_t = 2;
pub const PINMUX_GPIO_PU: rc_pinmux_mode_t = 1;
pub const PINMUX_GPIO: rc_pinmux_mode_t = 0;
pub const MODEL_BB_BLUE: rc_model_t = 7;
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
pub const MODEL_BB_POCKET: rc_model_t = 8;
pub const MODEL_BB_GREEN_W: rc_model_t = 6;
pub const MODEL_BB_GREEN: rc_model_t = 5;
pub const MODEL_BB_BLACK_W_RC: rc_model_t = 4;
pub const MODEL_BB_BLACK_W: rc_model_t = 3;
pub const MODEL_BB_BLACK_RC: rc_model_t = 2;
pub const MODEL_BB_BLACK: rc_model_t = 1;
pub const MODEL_UNKNOWN: rc_model_t = 0;
#[no_mangle]
pub unsafe extern "C" fn rc_pinmux_set(mut pin: libc::c_int, mut mode: rc_pinmux_mode_t) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut blue_only: libc::c_int = 0 as libc::c_int;
    match pin {
        30 => {
            if mode as libc::c_uint != PINMUX_GPIO as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PU as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PD as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_UART as libc::c_int as libc::c_uint
            {
                fprintf(
                    stderr,
                    b"ERROR in rc_pinmux_set, DSM pairing pin can only be put in GPIO or UART mode\n\0" as *const u8
                        as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            path = b"/sys/devices/platform/ocp/ocp:P9_11_pinmux/state\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        2 => {
            if mode as libc::c_uint != PINMUX_GPIO as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PU as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PD as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_PWM as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_UART as libc::c_int as libc::c_uint
            {
                fprintf(
                    stderr,
                    b"ERROR in rc_pinmux_set, GPS_HEADER_PIN_3 can only be put in GPIO, UART, or PWM modes\n\0"
                        as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            path = b"/sys/devices/platform/ocp/ocp:P9_22_pinmux/state\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        3 => {
            if mode as libc::c_uint != PINMUX_GPIO as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PU as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PD as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_PWM as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_UART as libc::c_int as libc::c_uint
            {
                fprintf(
                    stderr,
                    b"ERROR in rc_pinmux_set, GPS_HEADER_PIN_4 can only be put in GPIO, UART, or PWM modes\n\0"
                        as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            path = b"/sys/devices/platform/ocp/ocp:P9_21_pinmux/state\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        14 => {
            if mode as libc::c_uint != PINMUX_GPIO as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PU as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PD as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_CAN as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_UART as libc::c_int as libc::c_uint
            {
                fprintf(
                    stderr,
                    b"ERROR in rc_pinmux_set, UART1_HEADER_PIN_3 can only be put in GPIO, UART, CAN modes\n\0"
                        as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            path = b"/sys/devices/platform/ocp/ocp:P9_26_pinmux/state\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        15 => {
            if mode as libc::c_uint != PINMUX_GPIO as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PU as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PD as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_CAN as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_UART as libc::c_int as libc::c_uint
            {
                fprintf(
                    stderr,
                    b"ERROR in rc_pinmux_set, UART1_HEADER_PIN_3 can only be put in GPIO, UART, CAN modes\n\0"
                        as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            path = b"/sys/devices/platform/ocp/ocp:P9_24_pinmux/state\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        112 => {
            if mode as libc::c_uint != PINMUX_GPIO as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PU as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PD as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_SPI as libc::c_int as libc::c_uint
            {
                fprintf(
                    stderr,
                    b"ERROR in rc_pinmux_set, SPI_HEADER_PIN_3 can only be put in GPIO, or SPI modes\n\0" as *const u8
                        as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            path = b"/sys/devices/platform/ocp/ocp:P9_30_pinmux/state\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        111 => {
            if mode as libc::c_uint != PINMUX_GPIO as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PU as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PD as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_SPI as libc::c_int as libc::c_uint
            {
                fprintf(
                    stderr,
                    b"ERROR in rc_pinmux_set, SPI_HEADER_PIN_4 can only be put in GPIO, or SPI modes\n\0" as *const u8
                        as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            path = b"/sys/devices/platform/ocp/ocp:P9_29_pinmux/state\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        110 => {
            if mode as libc::c_uint != PINMUX_GPIO as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PU as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PD as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_SPI as libc::c_int as libc::c_uint
            {
                fprintf(
                    stderr,
                    b"ERROR in rc_pinmux_set, SPI_HEADER_PIN_5 can only be put in GPIO, or SPI modes\n\0" as *const u8
                        as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            path = b"/sys/devices/platform/ocp/ocp:P9_31_pinmux/state\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        113 => {
            if mode as libc::c_uint != PINMUX_GPIO as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PU as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PD as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_SPI as libc::c_int as libc::c_uint
            {
                if rc_model() as libc::c_uint == MODEL_BB_BLUE as libc::c_int as libc::c_uint {
                    fprintf(
                        stderr,
                        b"ERROR in rc_pinmux_set, BLUE_GP0_PIN_6 can only be put in GPIO modes\n\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    fprintf(
                        stderr,
                        b"ERROR in rc_pinmux_set, SPI_HEADER_PIN_6_SS1 can only be put in GPIO or SPI modes\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                return -(1 as libc::c_int);
            }
            path = b"/sys/devices/platform/ocp/ocp:P9_28_pinmux/state\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        49 => {
            if mode as libc::c_uint != PINMUX_GPIO as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PU as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PD as libc::c_int as libc::c_uint
            {
                if rc_model() as libc::c_uint == MODEL_BB_BLUE as libc::c_int as libc::c_uint {
                    fprintf(
                        stderr,
                        b"ERROR in rc_pinmux_set, BLUE_GP0_PIN_4 can only be put in GPIO modes\n\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    fprintf(
                        stderr,
                        b"ERROR in rc_pinmux_set, SPI_HEADER_PIN_6_SS2 can only be put in GPIO modes\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                return -(1 as libc::c_int);
            }
            path = b"/sys/devices/platform/ocp/ocp:P9_23_pinmux/state\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        29 => {
            if mode as libc::c_uint != PINMUX_GPIO as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PU as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PD as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_SPI as libc::c_int as libc::c_uint
            {
                fprintf(
                    stderr,
                    b"ERROR in rc_pinmux_set, BLUE_SPI_PIN_6_SS1 can only be put in GPIO, or SPI modes\n\0" as *const u8
                        as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            path = b"/sys/devices/platform/ocp/ocp:H18_pinmux/state\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            blue_only = 1 as libc::c_int;
        }
        7 => {
            if mode as libc::c_uint != PINMUX_GPIO as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PU as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PD as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_SPI as libc::c_int as libc::c_uint
            {
                fprintf(
                    stderr,
                    b"ERROR in rc_pinmux_set, BLUE_SPI_PIN_6_SS2 can only be put in GPIO, or SPI modes\n\0" as *const u8
                        as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            path = b"/sys/devices/platform/ocp/ocp:C18_pinmux/state\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            blue_only = 1 as libc::c_int;
        }
        57 => {
            if mode as libc::c_uint != PINMUX_GPIO as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PU as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PD as libc::c_int as libc::c_uint
            {
                fprintf(
                    stderr,
                    b"ERROR in rc_pinmux_set, BLUE_GP0_PIN_3 can only be put in GPIO modes\n\0" as *const u8
                        as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            path = b"/sys/devices/platform/ocp/ocp:U16_pinmux/state\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            blue_only = 1 as libc::c_int;
        }
        116 => {
            if mode as libc::c_uint != PINMUX_GPIO as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PU as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PD as libc::c_int as libc::c_uint
            {
                fprintf(
                    stderr,
                    b"ERROR in rc_pinmux_set, BLUE_GP0_PIN_5 can only be put in GPIO modes\n\0" as *const u8
                        as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            path = b"/sys/devices/platform/ocp/ocp:D13_pinmux/state\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            blue_only = 1 as libc::c_int;
        }
        98 => {
            if mode as libc::c_uint != PINMUX_GPIO as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PU as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PD as libc::c_int as libc::c_uint
            {
                fprintf(
                    stderr,
                    b"ERROR in rc_pinmux_set, BLUE_GP1_PIN_3 can only be put in GPIO modes\n\0" as *const u8
                        as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            path = b"/sys/devices/platform/ocp/ocp:J15_pinmux/state\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            blue_only = 1 as libc::c_int;
        }
        97 => {
            if mode as libc::c_uint != PINMUX_GPIO as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PU as libc::c_int as libc::c_uint
                && mode as libc::c_uint != PINMUX_GPIO_PD as libc::c_int as libc::c_uint
            {
                fprintf(
                    stderr,
                    b"ERROR in rc_pinmux_set, BLUE_GP1_PIN_4 can only be put in GPIO modes\n\0" as *const u8
                        as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            path = b"/sys/devices/platform/ocp/ocp:H17_pinmux/state\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            blue_only = 1 as libc::c_int;
        }
        _ => {
            fprintf(
                stderr,
                b"ERROR in rc_pinmux_set, Pinmuxing on pin %d is not supported\n\0" as *const u8 as *const libc::c_char,
                pin,
            );
            return -(1 as libc::c_int);
        }
    }
    if blue_only != 0 && rc_model() as libc::c_uint != MODEL_BB_BLUE as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"ERROR in rc_pinmux_set, Trying to set pinmux on pin that should only used on BB Blue\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    *__errno_location() = 0 as libc::c_int;
    fd = open(path, 0o1 as libc::c_int);
    if (fd == -(1 as libc::c_int)) as libc::c_int as libc::c_long != 0 {
        perror(b"ERORR opening pinmux driver\0" as *const u8 as *const libc::c_char);
        fprintf(stderr, b"can't open: %s\n\0" as *const u8 as *const libc::c_char, path);
        return -(1 as libc::c_int);
    }
    match mode as libc::c_uint {
        0 => {
            ret = write(
                fd,
                b"gpio\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                4 as libc::c_int as size_t,
            ) as libc::c_int;
        }
        1 => {
            ret = write(
                fd,
                b"gpio_pu\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                7 as libc::c_int as size_t,
            ) as libc::c_int;
        }
        2 => {
            ret = write(
                fd,
                b"gpio_pd\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                7 as libc::c_int as size_t,
            ) as libc::c_int;
        }
        3 => {
            ret = write(
                fd,
                b"pwm\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                3 as libc::c_int as size_t,
            ) as libc::c_int;
        }
        4 => {
            ret = write(
                fd,
                b"spi\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                3 as libc::c_int as size_t,
            ) as libc::c_int;
        }
        5 => {
            ret = write(
                fd,
                b"uart\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                4 as libc::c_int as size_t,
            ) as libc::c_int;
        }
        6 => {
            ret = write(
                fd,
                b"can\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                3 as libc::c_int as size_t,
            ) as libc::c_int;
        }
        _ => {
            fprintf(
                stderr,
                b"ERROR in rc_pinmux_set, unknown PINMUX mode\n\0" as *const u8 as *const libc::c_char,
            );
            close(fd);
            return -(1 as libc::c_int);
        }
    }
    if ret < 0 as libc::c_int {
        perror(b"ERROR in rc_pinmux_set, failed to write to pinmux driver\0" as *const u8 as *const libc::c_char);
        close(fd);
        return -(1 as libc::c_int);
    }
    close(fd);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_pinmux_set_default() -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if rc_model() as libc::c_uint == MODEL_BB_BLUE as libc::c_int as libc::c_uint {
        ret |= rc_pinmux_set(29 as libc::c_int, PINMUX_SPI);
        ret |= rc_pinmux_set(7 as libc::c_int, PINMUX_SPI);
        ret |= rc_pinmux_set(57 as libc::c_int, PINMUX_GPIO_PU);
        ret |= rc_pinmux_set(49 as libc::c_int, PINMUX_GPIO_PU);
        ret |= rc_pinmux_set(116 as libc::c_int, PINMUX_GPIO_PU);
        ret |= rc_pinmux_set(113 as libc::c_int, PINMUX_GPIO_PU);
        ret |= rc_pinmux_set(98 as libc::c_int, PINMUX_GPIO_PU);
        ret |= rc_pinmux_set(97 as libc::c_int, PINMUX_GPIO_PU);
    } else {
        ret |= rc_pinmux_set(113 as libc::c_int, PINMUX_SPI);
        ret |= rc_pinmux_set(49 as libc::c_int, PINMUX_GPIO);
    }
    ret |= rc_pinmux_set(30 as libc::c_int, PINMUX_UART);
    ret |= rc_pinmux_set(2 as libc::c_int, PINMUX_UART);
    ret |= rc_pinmux_set(3 as libc::c_int, PINMUX_UART);
    ret |= rc_pinmux_set(14 as libc::c_int, PINMUX_UART);
    ret |= rc_pinmux_set(15 as libc::c_int, PINMUX_UART);
    ret |= rc_pinmux_set(112 as libc::c_int, PINMUX_SPI);
    ret |= rc_pinmux_set(111 as libc::c_int, PINMUX_SPI);
    ret |= rc_pinmux_set(110 as libc::c_int, PINMUX_SPI);
    if ret != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_pinmux_set_default\n\0" as *const u8 as *const libc::c_char,
        );
        printf(b"You probbaly just need a newer kernel\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
