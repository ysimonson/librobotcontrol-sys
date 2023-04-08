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
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn rc_model() -> rc_model_t;
    fn rc_model_category() -> rc_model_category_t;
    fn rc_gpio_init(chip: libc::c_int, pin: libc::c_int, handle_flags: libc::c_int) -> libc::c_int;
    fn rc_gpio_set_value(chip: libc::c_int, pin: libc::c_int, value: libc::c_int) -> libc::c_int;
    fn rc_gpio_cleanup(chip: libc::c_int, pin: libc::c_int);
    fn rc_pinmux_set(pin: libc::c_int, mode: rc_pinmux_mode_t) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type size_t = libc::c_ulong;
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
pub const MODEL_BB_BLUE: rc_model_t = 7;
pub const MODEL_BB_GREEN_W: rc_model_t = 6;
pub const MODEL_BB_GREEN: rc_model_t = 5;
pub const MODEL_BB_BLACK_W_RC: rc_model_t = 4;
pub const MODEL_BB_BLACK_W: rc_model_t = 3;
pub const MODEL_BB_BLACK_RC: rc_model_t = 2;
pub const MODEL_BB_BLACK: rc_model_t = 1;
pub const MODEL_UNKNOWN: rc_model_t = 0;
pub type rc_model_category_t = libc::c_uint;
pub const CATEGORY_PC: rc_model_category_t = 3;
pub const CATEGORY_RPI: rc_model_category_t = 2;
pub const CATEGORY_BEAGLEBONE: rc_model_category_t = 1;
pub const CATEGORY_UNKNOWN: rc_model_category_t = 0;
pub type rc_pinmux_mode_t = libc::c_uint;
pub const PINMUX_CAN: rc_pinmux_mode_t = 6;
pub const PINMUX_UART: rc_pinmux_mode_t = 5;
pub const PINMUX_SPI: rc_pinmux_mode_t = 4;
pub const PINMUX_PWM: rc_pinmux_mode_t = 3;
pub const PINMUX_GPIO_PD: rc_pinmux_mode_t = 2;
pub const PINMUX_GPIO_PU: rc_pinmux_mode_t = 1;
pub const PINMUX_GPIO: rc_pinmux_mode_t = 0;
pub type __u8 = libc::c_uchar;
pub type __u16 = libc::c_ushort;
pub type __u32 = libc::c_uint;
pub type __u64 = libc::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spi_ioc_transfer {
    pub tx_buf: __u64,
    pub rx_buf: __u64,
    pub len: __u32,
    pub speed_hz: __u32,
    pub delay_usecs: __u16,
    pub bits_per_word: __u8,
    pub cs_change: __u8,
    pub tx_nbits: __u8,
    pub rx_nbits: __u8,
    pub word_delay_usecs: __u8,
    pub pad: __u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rc_spi_state_t {
    pub init: [libc::c_uchar; 12],
    pub ss_mode: [libc::c_uchar; 12],
    pub chip: [libc::c_uchar; 12],
    pub pin: [libc::c_uchar; 12],
    pub speed: [libc::c_int; 12],
    pub fd: [libc::c_int; 12],
}
static mut state: [rc_spi_state_t; 6] = [rc_spi_state_t {
    init: [0; 12],
    ss_mode: [0; 12],
    chip: [0; 12],
    pin: [0; 12],
    speed: [0; 12],
    fd: [0; 12],
}; 6];
unsafe extern "C" fn __open_fd(mut bus: libc::c_int, mut slave: libc::c_int) -> libc::c_int {
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut ret: libc::c_int = 0;
    snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        b"/dev/spidev%d.%d\0" as *const u8 as *const libc::c_char,
        bus,
        slave,
    );
    ret = open(buf.as_mut_ptr(), 0o2 as libc::c_int);
    if ret == -(1 as libc::c_int)
        && bus == 1 as libc::c_int
        && (slave == 0 as libc::c_int || slave == 1 as libc::c_int)
        && rc_model_category() as libc::c_uint == CATEGORY_BEAGLEBONE as libc::c_int as libc::c_uint
    {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"/dev/spidev%d.%d\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int,
            slave,
        );
        ret = open(buf.as_mut_ptr(), 0o2 as libc::c_int);
    }
    if ret == -(1 as libc::c_int) {
        perror(b"ERROR in rc_spi_init, failed to open /dev/spidev device\0" as *const u8 as *const libc::c_char);
        if *__errno_location() != 1 as libc::c_int {
            fprintf(
                stderr,
                b"likely SPI is not enabled in the device tree or kernel\n\0" as *const u8 as *const libc::c_char,
            );
        }
        return -(1 as libc::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn rc_spi_init_auto_slave(
    mut bus: libc::c_int,
    mut slave: libc::c_int,
    mut bus_mode: libc::c_int,
    mut speed_hz: libc::c_int,
) -> libc::c_int {
    let mut bits: libc::c_int = 8 as libc::c_int;
    let mut model: rc_model_t = rc_model();
    let mut category: rc_model_category_t = rc_model_category();
    let mut fd: libc::c_int = 0;
    if bus < 0 as libc::c_int || bus > 5 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_init_auto_slave, bus must be between 0 and %d\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if slave < 0 as libc::c_int || slave >= 12 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_init_auto_slave, slave must be between 0 and %d\n\0" as *const u8 as *const libc::c_char,
            12 as libc::c_int - 1 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if speed_hz > 24000000 as libc::c_int || speed_hz < 1000 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_init_auto_slave, speed_hz must be between %d & %d\n\0" as *const u8
                as *const libc::c_char,
            1000 as libc::c_int,
            24000000 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if bus_mode != 0 as libc::c_int | 0 as libc::c_int
        && bus_mode as libc::c_ulong != 0 as libc::c_int as libc::c_ulong | (1 as libc::c_ulong) << 0 as libc::c_int
        && bus_mode as libc::c_ulong != (1 as libc::c_ulong) << 1 as libc::c_int | 0 as libc::c_int as libc::c_ulong
        && bus_mode as libc::c_ulong
            != (1 as libc::c_ulong) << 1 as libc::c_int | (1 as libc::c_ulong) << 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"ERROR in rc_spi_init_auto_slave, bus_mode must be SPI_MODE_0, 1, 2, or 3\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if bus == 1 as libc::c_int
        && slave == 1 as libc::c_int
        && (model as libc::c_uint == MODEL_BB_BLACK_RC as libc::c_int as libc::c_uint
            || model as libc::c_uint == MODEL_BB_BLACK_W_RC as libc::c_int as libc::c_uint)
    {
        fprintf(
            stderr,
            b"ERROR in rc_spi_init_auto_slave, auto slave mode not available on slave 2 with Robotics Cape\n\0"
                as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if bus > 1 as libc::c_int && category as libc::c_uint == CATEGORY_BEAGLEBONE as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"ERROR in rc_spi_init_auto_slave, can only use spi bus 0 and 1 on BeagleBones\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if slave > 1 as libc::c_int && category as libc::c_uint == CATEGORY_BEAGLEBONE as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"ERROR in rc_spi_init_auto_slave, can only use slave 0 and 1 on BeagleBones\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    fd = __open_fd(bus, slave);
    if fd == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if ioctl(
        fd,
        ((1 as libc::c_uint) << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('k' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::core::mem::size_of::<__u8>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut bus_mode as *mut libc::c_int,
    ) == -(1 as libc::c_int)
    {
        perror(b"ERROR in rc_spi_init_auto_slave setting spi mode\0" as *const u8 as *const libc::c_char);
        close(fd);
        return -(1 as libc::c_int);
    }
    if ioctl(
        fd,
        ((1 as libc::c_uint) << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('k' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((3 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::core::mem::size_of::<__u8>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut bits as *mut libc::c_int,
    ) == -(1 as libc::c_int)
    {
        perror(b"ERROR in rc_spi_init_auto_slave setting bits per word\0" as *const u8 as *const libc::c_char);
        close(fd);
        return -(1 as libc::c_int);
    }
    if ioctl(
        fd,
        ((1 as libc::c_uint) << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('k' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((4 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::core::mem::size_of::<__u32>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut speed_hz as *mut libc::c_int,
    ) == -(1 as libc::c_int)
    {
        perror(b"ERROR in rc_spi_init_auto_slave setting max speed hz\0" as *const u8 as *const libc::c_char);
        close(fd);
        return -(1 as libc::c_int);
    }
    if bus == 1 as libc::c_int
        && slave == 0 as libc::c_int
        && model as libc::c_uint == MODEL_BB_BLUE as libc::c_int as libc::c_uint
    {
        if rc_pinmux_set(29 as libc::c_int, PINMUX_SPI) != 0 {
            fprintf(
                stderr,
                b"ERROR in rc_spi_init_auto_slave, failed to set slave select pinmux to SPI mode\n\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else if bus == 1 as libc::c_int
        && slave == 1 as libc::c_int
        && model as libc::c_uint == MODEL_BB_BLUE as libc::c_int as libc::c_uint
    {
        if rc_pinmux_set(7 as libc::c_int, PINMUX_SPI) != 0 {
            fprintf(
                stderr,
                b"ERROR in rc_spi_init_auto_slave, failed to set slave select pinmux to SPI mode\n\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else if bus == 1 as libc::c_int
        && slave == 1 as libc::c_int
        && (model as libc::c_uint == MODEL_BB_BLACK_RC as libc::c_int as libc::c_uint
            || model as libc::c_uint == MODEL_BB_BLACK_W_RC as libc::c_int as libc::c_uint)
    {
        if rc_pinmux_set(49 as libc::c_int, PINMUX_SPI) != 0 {
            fprintf(
                stderr,
                b"ERROR in rc_spi_init_auto_slave, failed to set slave select pinmux to SPI mode\n\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    state[bus as usize].init[slave as usize] = 1 as libc::c_int as libc::c_uchar;
    state[bus as usize].ss_mode[slave as usize] = 1 as libc::c_int as libc::c_uchar;
    state[bus as usize].chip[slave as usize] = -(1 as libc::c_int) as libc::c_uchar;
    state[bus as usize].pin[slave as usize] = -(1 as libc::c_int) as libc::c_uchar;
    state[bus as usize].fd[slave as usize] = fd;
    state[bus as usize].speed[slave as usize] = speed_hz;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_spi_init_manual_slave(
    mut bus: libc::c_int,
    mut slave: libc::c_int,
    mut bus_mode: libc::c_int,
    mut speed_hz: libc::c_int,
    mut chip: libc::c_int,
    mut pin: libc::c_int,
) -> libc::c_int {
    let mut bits: libc::c_int = 8 as libc::c_int;
    let mut model: rc_model_t = rc_model();
    let mut category: rc_model_category_t = rc_model_category();
    let mut fd: libc::c_int = 0;
    if bus < 0 as libc::c_int || bus > 5 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_init_manual_slave, bus must be between 0 and %d\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if slave < 0 as libc::c_int || slave >= 12 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_init_manual_slave, slave must be between 0 and %d\n\0" as *const u8
                as *const libc::c_char,
            12 as libc::c_int - 1 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if speed_hz > 24000000 as libc::c_int || speed_hz < 1000 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_init_manual_slave, speed_hz must be between %d & %d\n\0" as *const u8
                as *const libc::c_char,
            1000 as libc::c_int,
            24000000 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if bus_mode != 0 as libc::c_int | 0 as libc::c_int
        && bus_mode as libc::c_ulong != 0 as libc::c_int as libc::c_ulong | (1 as libc::c_ulong) << 0 as libc::c_int
        && bus_mode as libc::c_ulong != (1 as libc::c_ulong) << 1 as libc::c_int | 0 as libc::c_int as libc::c_ulong
        && bus_mode as libc::c_ulong
            != (1 as libc::c_ulong) << 1 as libc::c_int | (1 as libc::c_ulong) << 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"ERROR in rc_spi_init_manual_slave, bus_mode must be SPI_MODE_0, 1, 2, or 3\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if bus == 1 as libc::c_int
        && slave == 1 as libc::c_int
        && (model as libc::c_uint == MODEL_BB_BLACK_RC as libc::c_int as libc::c_uint
            || model as libc::c_uint == MODEL_BB_BLACK_W_RC as libc::c_int as libc::c_uint)
    {
        fprintf(
            stderr,
            b"ERROR in rc_spi_init_manual_slave, auto slave mode not available on slave 2 with Robotics Cape\n\0"
                as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if bus > 1 as libc::c_int && category as libc::c_uint == CATEGORY_BEAGLEBONE as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"ERROR in rc_spi_init_manual_slave, can only use spi bus 0 and 1 on BeagleBones\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if slave > 1 as libc::c_int && category as libc::c_uint == CATEGORY_BEAGLEBONE as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"ERROR in rc_spi_init_manual_slave, can only use slave 0 and 1 on BeagleBones\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if state[bus as usize].fd[0 as libc::c_int as usize] == 0 as libc::c_int {
        fd = __open_fd(bus, 0 as libc::c_int);
        if fd == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        if ioctl(
            fd,
            ((1 as libc::c_uint) << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
                | (('k' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
                | ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
                | (::core::mem::size_of::<__u8>() as libc::c_ulong)
                    << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
            &mut bus_mode as *mut libc::c_int,
        ) == -(1 as libc::c_int)
        {
            perror(b"ERROR in rc_spi_init_manual_slave setting spi mode\0" as *const u8 as *const libc::c_char);
            close(fd);
            return -(1 as libc::c_int);
        }
        if ioctl(
            fd,
            ((1 as libc::c_uint) << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
                | (('k' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
                | ((3 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
                | (::core::mem::size_of::<__u8>() as libc::c_ulong)
                    << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
            &mut bits as *mut libc::c_int,
        ) == -(1 as libc::c_int)
        {
            perror(b"ERROR in rc_spi_init_manual_slave setting bits per word\0" as *const u8 as *const libc::c_char);
            close(fd);
            return -(1 as libc::c_int);
        }
        if ioctl(
            fd,
            ((1 as libc::c_uint) << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
                | (('k' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
                | ((4 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
                | (::core::mem::size_of::<__u32>() as libc::c_ulong)
                    << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
            &mut speed_hz as *mut libc::c_int,
        ) == -(1 as libc::c_int)
        {
            perror(b"ERROR in rc_spi_init_manual_slave setting max speed hz\0" as *const u8 as *const libc::c_char);
            close(fd);
            return -(1 as libc::c_int);
        }
    } else {
        fd = state[bus as usize].fd[0 as libc::c_int as usize];
    }
    if bus == 1 as libc::c_int
        && slave == 0 as libc::c_int
        && model as libc::c_uint == MODEL_BB_BLUE as libc::c_int as libc::c_uint
    {
        if rc_pinmux_set(29 as libc::c_int, PINMUX_GPIO) != 0 {
            fprintf(
                stderr,
                b"ERROR in rc_spi_init_manual_slave, failed to set slave select pinmux to GPIO mode\n\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else if bus == 1 as libc::c_int
        && slave == 1 as libc::c_int
        && model as libc::c_uint == MODEL_BB_BLUE as libc::c_int as libc::c_uint
    {
        if rc_pinmux_set(7 as libc::c_int, PINMUX_GPIO) != 0 {
            fprintf(
                stderr,
                b"ERROR in rc_spi_init_manual_slave, failed to set slave select pinmux to GPIO mode\n\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else if bus == 1 as libc::c_int
        && slave == 0 as libc::c_int
        && (model as libc::c_uint == MODEL_BB_BLACK_RC as libc::c_int as libc::c_uint
            || model as libc::c_uint == MODEL_BB_BLACK_W_RC as libc::c_int as libc::c_uint)
    {
        if rc_pinmux_set(113 as libc::c_int, PINMUX_GPIO) != 0 {
            fprintf(
                stderr,
                b"ERROR in rc_spi_init_manual_slave, failed to set slave select pinmux to GPIO mode\n\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else if bus == 1 as libc::c_int
        && slave == 1 as libc::c_int
        && (model as libc::c_uint == MODEL_BB_BLACK_RC as libc::c_int as libc::c_uint
            || model as libc::c_uint == MODEL_BB_BLACK_W_RC as libc::c_int as libc::c_uint)
    {
        if rc_pinmux_set(49 as libc::c_int, PINMUX_GPIO) != 0 {
            fprintf(
                stderr,
                b"ERROR in rc_spi_init_manual_slave, failed to set slave select pinmux to GPIO mode\n\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    if rc_gpio_init(chip, pin, ((1 as libc::c_ulong) << 1 as libc::c_int) as libc::c_int) != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_spi_init_manual_slave failed to initialize slave select gpio pin\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_gpio_set_value(chip, pin, 1 as libc::c_int) != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_spi_init_manual_slave, failed to write to gpio slave select pin\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    state[bus as usize].init[slave as usize] = 1 as libc::c_int as libc::c_uchar;
    state[bus as usize].ss_mode[slave as usize] = 2 as libc::c_int as libc::c_uchar;
    state[bus as usize].chip[slave as usize] = chip as libc::c_uchar;
    state[bus as usize].pin[slave as usize] = pin as libc::c_uchar;
    state[bus as usize].fd[slave as usize] = fd;
    state[bus as usize].fd[0 as libc::c_int as usize] = fd;
    state[bus as usize].speed[slave as usize] = speed_hz;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_spi_get_fd(mut bus: libc::c_int, mut slave: libc::c_int) -> libc::c_int {
    if bus < 0 as libc::c_int || bus > 5 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_get_fd, bus must be between 0 and %d\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if slave < 0 as libc::c_int || slave >= 12 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_get_fd, slave must be between 0 and %d\n\0" as *const u8 as *const libc::c_char,
            12 as libc::c_int - 1 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if state[bus as usize].init[slave as usize] as libc::c_int == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_get_fd, need to initialize first\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return state[bus as usize].fd[slave as usize];
}
#[no_mangle]
pub unsafe extern "C" fn rc_spi_close(mut bus: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if bus < 0 as libc::c_int || bus > 5 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_close, bus must be between 0 and %d\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < 12 as libc::c_int {
        if state[bus as usize].init[i as usize] as libc::c_int == 2 as libc::c_int {
            if rc_gpio_set_value(
                state[bus as usize].chip[i as usize] as libc::c_int,
                state[bus as usize].pin[i as usize] as libc::c_int,
                1 as libc::c_int,
            ) != 0
            {
                fprintf(
                    stderr,
                    b"WARNING in rc_spi_close, failed to write to gpio slave select pin\n\0" as *const u8
                        as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            rc_gpio_cleanup(
                state[bus as usize].chip[i as usize] as libc::c_int,
                state[bus as usize].pin[i as usize] as libc::c_int,
            );
            if i == 0 as libc::c_int {
                close(state[bus as usize].fd[i as usize]);
            }
        }
        if state[bus as usize].init[i as usize] as libc::c_int == 1 as libc::c_int {
            close(state[bus as usize].fd[i as usize]);
        }
        state[bus as usize].init[i as usize] = 0 as libc::c_int as libc::c_uchar;
        state[bus as usize].ss_mode[i as usize] = 0 as libc::c_int as libc::c_uchar;
        state[bus as usize].chip[i as usize] = 0 as libc::c_int as libc::c_uchar;
        state[bus as usize].pin[i as usize] = 0 as libc::c_int as libc::c_uchar;
        state[bus as usize].fd[i as usize] = 0 as libc::c_int;
        state[bus as usize].speed[i as usize] = 0 as libc::c_int;
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_spi_manual_select(
    mut bus: libc::c_int,
    mut slave: libc::c_int,
    mut select: libc::c_int,
) -> libc::c_int {
    if bus < 0 as libc::c_int || bus > 5 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_manual_select, bus must be between 0 and %d\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if slave < 0 as libc::c_int || slave >= 12 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_manual_select, slave must be between 0 and %d\n\0" as *const u8 as *const libc::c_char,
            12 as libc::c_int - 1 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if state[bus as usize].init[slave as usize] as libc::c_int == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_manual_select, need to initialize first\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if state[bus as usize].ss_mode[slave as usize] as libc::c_int != 2 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_manual_select, slave not configured in manual mode\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_gpio_set_value(
        state[bus as usize].chip[slave as usize] as libc::c_int,
        state[bus as usize].pin[slave as usize] as libc::c_int,
        (select == 0) as libc::c_int,
    ) == -(1 as libc::c_int)
    {
        fprintf(
            stderr,
            b"ERROR in rc_spi_manual_select writing to gpio pin\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_spi_transfer(
    mut bus: libc::c_int,
    mut slave: libc::c_int,
    mut tx_data: *mut uint8_t,
    mut tx_bytes: size_t,
    mut rx_data: *mut uint8_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut xfer: spi_ioc_transfer = {
        let mut init = spi_ioc_transfer {
            tx_buf: 0 as libc::c_int as __u64,
            rx_buf: 0,
            len: 0,
            speed_hz: 0,
            delay_usecs: 0,
            bits_per_word: 0,
            cs_change: 0,
            tx_nbits: 0,
            rx_nbits: 0,
            word_delay_usecs: 0,
            pad: 0,
        };
        init
    };
    if bus < 0 as libc::c_int || bus > 5 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_transfer, bus must be between 0 and %d\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if slave < 0 as libc::c_int || slave >= 12 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_transfer, slave must be between 0 and %d\n\0" as *const u8 as *const libc::c_char,
            12 as libc::c_int - 1 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if state[bus as usize].init[slave as usize] as libc::c_int == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_transfer, need to initialize first\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if tx_bytes < 1 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"ERROR: in rc_spi_transfer, tx_bytes must be >=1\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    xfer.tx_buf = tx_data as libc::c_ulong as __u64;
    xfer.rx_buf = rx_data as libc::c_ulong as __u64;
    xfer.len = tx_bytes as __u32;
    xfer.speed_hz = state[bus as usize].speed[slave as usize] as __u32;
    xfer.delay_usecs = 0 as libc::c_int as __u16;
    xfer.bits_per_word = 8 as libc::c_int as __u8;
    xfer.cs_change = 1 as libc::c_int as __u8;
    ret = ioctl(
        state[bus as usize].fd[slave as usize],
        ((1 as libc::c_uint) << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('k' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((0 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut xfer as *mut spi_ioc_transfer,
    );
    if ret == -(1 as libc::c_int) {
        perror(b"ERROR in rc_spi_transfer\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn rc_spi_write(
    mut bus: libc::c_int,
    mut slave: libc::c_int,
    mut data: *mut uint8_t,
    mut bytes: size_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut xfer: spi_ioc_transfer = {
        let mut init = spi_ioc_transfer {
            tx_buf: 0 as libc::c_int as __u64,
            rx_buf: 0,
            len: 0,
            speed_hz: 0,
            delay_usecs: 0,
            bits_per_word: 0,
            cs_change: 0,
            tx_nbits: 0,
            rx_nbits: 0,
            word_delay_usecs: 0,
            pad: 0,
        };
        init
    };
    if bus < 0 as libc::c_int || bus > 5 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_write, bus must be between 0 and %d\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if slave < 0 as libc::c_int || slave >= 12 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_write, slave must be between 0 and %d\n\0" as *const u8 as *const libc::c_char,
            12 as libc::c_int - 1 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if state[bus as usize].init[slave as usize] as libc::c_int == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_write, need to initialize first\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if bytes < 1 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"ERROR: in rc_spi_write, bytes must be >=1\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    xfer.tx_buf = data as libc::c_ulong as __u64;
    xfer.rx_buf = 0 as libc::c_int as __u64;
    xfer.len = bytes as __u32;
    xfer.speed_hz = state[bus as usize].speed[slave as usize] as __u32;
    xfer.delay_usecs = 0 as libc::c_int as __u16;
    xfer.bits_per_word = 8 as libc::c_int as __u8;
    xfer.cs_change = 1 as libc::c_int as __u8;
    ret = ioctl(
        state[bus as usize].fd[slave as usize],
        ((1 as libc::c_uint) << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('k' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((0 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut xfer as *mut spi_ioc_transfer,
    );
    if ret == -(1 as libc::c_int) {
        perror(b"ERROR in rc_spi_write\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn rc_spi_read(
    mut bus: libc::c_int,
    mut slave: libc::c_int,
    mut data: *mut uint8_t,
    mut bytes: size_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut xfer: spi_ioc_transfer = {
        let mut init = spi_ioc_transfer {
            tx_buf: 0 as libc::c_int as __u64,
            rx_buf: 0,
            len: 0,
            speed_hz: 0,
            delay_usecs: 0,
            bits_per_word: 0,
            cs_change: 0,
            tx_nbits: 0,
            rx_nbits: 0,
            word_delay_usecs: 0,
            pad: 0,
        };
        init
    };
    if bus < 0 as libc::c_int || bus > 5 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_read, bus must be between 0 and %d\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if slave < 0 as libc::c_int || slave >= 12 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_read, slave must be between 0 and %d\n\0" as *const u8 as *const libc::c_char,
            12 as libc::c_int - 1 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if state[bus as usize].init[slave as usize] as libc::c_int == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_spi_read, need to initialize first\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if bytes < 1 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"ERROR: in rc_spi_read, bytes must be >=1\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    xfer.tx_buf = 0 as libc::c_int as __u64;
    xfer.rx_buf = data as libc::c_ulong as __u64;
    xfer.len = bytes as __u32;
    xfer.speed_hz = state[bus as usize].speed[slave as usize] as __u32;
    xfer.delay_usecs = 0 as libc::c_int as __u16;
    xfer.bits_per_word = 8 as libc::c_int as __u8;
    xfer.cs_change = 1 as libc::c_int as __u8;
    ret = ioctl(
        state[bus as usize].fd[slave as usize],
        ((1 as libc::c_uint) << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('k' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((0 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut xfer as *mut spi_ioc_transfer,
    );
    if ret == -(1 as libc::c_int) {
        perror(b"ERROR in rc_spi_read\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return ret;
}
