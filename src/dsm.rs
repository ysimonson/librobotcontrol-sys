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
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getchar() -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(__th: pthread_t, __thread_return: *mut *mut libc::c_void) -> libc::c_int;
    fn rc_pthread_create(
        thread: *mut pthread_t,
        func: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        arg: *mut libc::c_void,
        policy: libc::c_int,
        priority: libc::c_int,
    ) -> libc::c_int;
    fn rc_pthread_timed_join(
        thread: pthread_t,
        retval: *mut *mut libc::c_void,
        timeout_sec: libc::c_float,
    ) -> libc::c_int;
    fn rc_pinmux_set(pin: libc::c_int, mode: rc_pinmux_mode_t) -> libc::c_int;
    fn rc_usleep(us: libc::c_uint);
    fn rc_nanos_since_boot() -> uint64_t;
    fn rc_uart_init(
        bus: libc::c_int,
        baudrate: libc::c_int,
        timeout: libc::c_float,
        canonical_en: libc::c_int,
        stop_bits: libc::c_int,
        parity_en: libc::c_int,
    ) -> libc::c_int;
    fn rc_uart_flush(bus: libc::c_int) -> libc::c_int;
    fn rc_uart_read_bytes(bus: libc::c_int, buf: *mut uint8_t, bytes: size_t) -> libc::c_int;
    fn rc_gpio_init(chip: libc::c_int, pin: libc::c_int, handle_flags: libc::c_int) -> libc::c_int;
    fn rc_gpio_set_value(chip: libc::c_int, pin: libc::c_int, value: libc::c_int) -> libc::c_int;
    fn rc_gpio_get_value(chip: libc::c_int, pin: libc::c_int) -> libc::c_int;
    fn rc_gpio_cleanup(chip: libc::c_int, pin: libc::c_int);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
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
pub type int64_t = __int64_t;
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type rc_pinmux_mode_t = libc::c_uint;
pub const PINMUX_CAN: rc_pinmux_mode_t = 6;
pub const PINMUX_UART: rc_pinmux_mode_t = 5;
pub const PINMUX_SPI: rc_pinmux_mode_t = 4;
pub const PINMUX_PWM: rc_pinmux_mode_t = 3;
pub const PINMUX_GPIO_PD: rc_pinmux_mode_t = 2;
pub const PINMUX_GPIO_PU: rc_pinmux_mode_t = 1;
pub const PINMUX_GPIO: rc_pinmux_mode_t = 0;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
static mut running: libc::c_int = 0;
static mut channels: [libc::c_int; 9] = [0; 9];
static mut maxes: [libc::c_int; 9] = [0; 9];
static mut mins: [libc::c_int; 9] = [0; 9];
static mut centers: [libc::c_int; 9] = [0; 9];
static mut range_up: [libc::c_double; 9] = [0.; 9];
static mut range_down: [libc::c_double; 9] = [0.; 9];
static mut num_channels: libc::c_int = 0;
static mut resolution: libc::c_int = 0;
static mut new_dsm_flag: libc::c_int = 0;
static mut dsm_frame_rate: libc::c_int = 0;
static mut last_time: uint64_t = 0;
static mut parse_thread: pthread_t = 0;
static mut listening: libc::c_int = 0;
static mut new_data_callback: Option<unsafe extern "C" fn() -> ()> = None;
static mut disconnect_callback: Option<unsafe extern "C" fn() -> ()> = None;
static mut active_flag: libc::c_int = 0 as libc::c_int;
static mut init_flag: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn __continue_or_quit() -> libc::c_int {
    fflush(stdin);
    if system(b"stty raw\0" as *const u8 as *const libc::c_char) != 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in continue_or_quit setting stty raw\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut c: libc::c_int = getchar();
    let mut ret: libc::c_int = 0;
    if c == 3 as libc::c_int {
        ret = -(1 as libc::c_int);
    } else if c == '\r' as i32 || c == '\n' as i32 {
        ret = 1 as libc::c_int;
    } else {
        ret = 0 as libc::c_int;
    }
    if system(b"stty cooked\0" as *const u8 as *const libc::c_char) != 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in continue_or_quit setting stty cooked\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    return ret;
}
unsafe extern "C" fn __parser_func(mut _ptr: *mut libc::c_void) -> *mut libc::c_void {
    let mut buf: [uint8_t; 16] = [0; 16];
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut new_values: [libc::c_int; 9] = [0; 9];
    let mut detection_packets_left: libc::c_int = 0;
    let mut ch_id: libc::c_uchar = 0;
    let mut value: int16_t = 0;
    let mut is_complete: libc::c_int = 0;
    let mut max_channel_id_1024: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut max_channel_id_2048: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut channels_detected_1024: [libc::c_char; 9] = [0; 9];
    let mut channels_detected_2048: [libc::c_char; 9] = [0; 9];
    new_dsm_flag = 0 as libc::c_int;
    init_flag = 1 as libc::c_int;
    '_DETECTION_START: loop {
        rc_uart_flush(4 as libc::c_int);
        detection_packets_left = 4 as libc::c_int;
        max_channel_id_1024 = 0 as libc::c_int as libc::c_uchar;
        max_channel_id_2048 = 0 as libc::c_int as libc::c_uchar;
        memset(
            channels_detected_1024.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            9 as libc::c_int as libc::c_ulong,
        );
        memset(
            channels_detected_2048.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            9 as libc::c_int as libc::c_ulong,
        );
        while detection_packets_left > 0 as libc::c_int && running != 0 {
            memset(
                buf.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                16 as libc::c_int as libc::c_ulong,
            );
            ret = rc_uart_read_bytes(4 as libc::c_int, buf.as_mut_ptr(), 16 as libc::c_int as size_t);
            if ret != 16 as libc::c_int {
                rc_uart_flush(4 as libc::c_int);
            } else {
                if buf[1 as libc::c_int as usize] as libc::c_int == 0xa2 as libc::c_int
                    && buf[3 as libc::c_int as usize] as libc::c_int == 0xa2 as libc::c_int
                    && buf[5 as libc::c_int as usize] as libc::c_int == 0xa2 as libc::c_int
                    && buf[7 as libc::c_int as usize] as libc::c_int == 0xa2 as libc::c_int
                    && buf[9 as libc::c_int as usize] as libc::c_int == 0xa2 as libc::c_int
                    && buf[11 as libc::c_int as usize] as libc::c_int == 0xa2 as libc::c_int
                {
                    continue;
                }
                i = 1 as libc::c_int;
                while i < 8 as libc::c_int {
                    if buf[(2 as libc::c_int * i) as usize] as libc::c_int != 0xff as libc::c_int
                        || buf[(2 as libc::c_int * i + 1 as libc::c_int) as usize] as libc::c_int != 0xff as libc::c_int
                    {
                        ch_id = ((buf[(i * 2 as libc::c_int) as usize] as libc::c_int & 0x7c as libc::c_int)
                            >> 2 as libc::c_int) as libc::c_uchar;
                        if ch_id as libc::c_int > max_channel_id_1024 as libc::c_int {
                            max_channel_id_1024 = ch_id;
                        }
                        if (ch_id as libc::c_int) < 9 as libc::c_int {
                            channels_detected_1024[ch_id as usize] = 1 as libc::c_int as libc::c_char;
                        }
                    }
                    i += 1;
                }
                i = 1 as libc::c_int;
                while i < 8 as libc::c_int {
                    if buf[(2 as libc::c_int * i) as usize] as libc::c_int != 0xff as libc::c_int
                        || buf[(2 as libc::c_int * i + 1 as libc::c_int) as usize] as libc::c_int != 0xff as libc::c_int
                    {
                        ch_id = ((buf[(i * 2 as libc::c_int) as usize] as libc::c_int & 0x78 as libc::c_int)
                            >> 3 as libc::c_int) as libc::c_uchar;
                        if ch_id as libc::c_int > max_channel_id_2048 as libc::c_int {
                            max_channel_id_2048 = ch_id;
                        }
                        if (ch_id as libc::c_int) < 9 as libc::c_int {
                            channels_detected_2048[ch_id as usize] = 1 as libc::c_int as libc::c_char;
                        }
                    }
                    i += 1;
                }
                detection_packets_left -= 1;
            }
        }
        if running == 0 {
            return 0 as *mut libc::c_void;
        }
        if max_channel_id_1024 as libc::c_int >= 9 as libc::c_int {
            resolution = 2048 as libc::c_int;
            if max_channel_id_2048 as libc::c_int >= 9 as libc::c_int {
                continue;
            }
            num_channels = max_channel_id_2048 as libc::c_int + 1 as libc::c_int;
            i = 0 as libc::c_int;
            while i < num_channels {
                if channels_detected_2048[i as usize] as libc::c_int == 0 as libc::c_int {
                    continue '_DETECTION_START;
                }
                i += 1;
            }
        } else {
            resolution = 1024 as libc::c_int;
            if max_channel_id_1024 as libc::c_int >= 9 as libc::c_int {
                fprintf(
                    stderr,
                    b"WARNING: too many DSM channels detected, trying again\n\0" as *const u8 as *const libc::c_char,
                );
                continue;
            } else {
                num_channels = max_channel_id_1024 as libc::c_int + 1 as libc::c_int;
                i = 0 as libc::c_int;
                while i < num_channels {
                    if channels_detected_1024[i as usize] as libc::c_int == 0 as libc::c_int {
                        fprintf(
                            stderr,
                            b"WARNING: Missing DSM channel, trying again\n\0" as *const u8 as *const libc::c_char,
                        );
                        continue '_DETECTION_START;
                    } else {
                        i += 1;
                    }
                }
            }
        }
        if !(num_channels < 2 as libc::c_int) {
            break;
        }
    }
    's_283: while running != 0 {
        if active_flag != 0 as libc::c_int
            && rc_dsm_nanos_since_last_packet() > 300000000 as libc::c_int as libc::c_long
        {
            active_flag = 0 as libc::c_int;
            if disconnect_callback.is_some() {
                ::core::mem::transmute::<_, fn()>(disconnect_callback.expect("non-null function pointer"))();
            }
        }
        memset(
            buf.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            16 as libc::c_int as libc::c_ulong,
        );
        rc_uart_flush(4 as libc::c_int);
        ret = rc_uart_read_bytes(4 as libc::c_int, buf.as_mut_ptr(), 16 as libc::c_int as size_t);
        if ret != 16 as libc::c_int {
            rc_uart_flush(4 as libc::c_int);
        } else {
            if buf[1 as libc::c_int as usize] as libc::c_int == 0xa2 as libc::c_int
                && buf[3 as libc::c_int as usize] as libc::c_int == 0xa2 as libc::c_int
                && buf[5 as libc::c_int as usize] as libc::c_int == 0xa2 as libc::c_int
                && buf[7 as libc::c_int as usize] as libc::c_int == 0xa2 as libc::c_int
                && buf[9 as libc::c_int as usize] as libc::c_int == 0xa2 as libc::c_int
                && buf[11 as libc::c_int as usize] as libc::c_int == 0xa2 as libc::c_int
            {
                continue;
            }
            i = 1 as libc::c_int;
            while i <= 7 as libc::c_int {
                if buf[(2 as libc::c_int * i) as usize] as libc::c_int != 0xff as libc::c_int
                    || buf[(2 as libc::c_int * i + 1 as libc::c_int) as usize] as libc::c_int != 0xff as libc::c_int
                {
                    if resolution == 1024 as libc::c_int {
                        ch_id = ((buf[(i * 2 as libc::c_int) as usize] as libc::c_int & 0x7c as libc::c_int)
                            >> 2 as libc::c_int) as libc::c_uchar;
                        value = (((buf[(i * 2 as libc::c_int) as usize] as libc::c_int & 0x3 as libc::c_int)
                            << 8 as libc::c_int)
                            + buf[(2 as libc::c_int * i + 1 as libc::c_int) as usize] as libc::c_int)
                            as int16_t;
                        value = (value as libc::c_int + 989 as libc::c_int) as int16_t;
                    } else if resolution == 2048 as libc::c_int {
                        ch_id = ((buf[(i * 2 as libc::c_int) as usize] as libc::c_int & 0x78 as libc::c_int)
                            >> 3 as libc::c_int) as libc::c_uchar;
                        value = (((buf[(i * 2 as libc::c_int) as usize] as libc::c_int & 0x7 as libc::c_int)
                            << 8 as libc::c_int)
                            + buf[(2 as libc::c_int * i + 1 as libc::c_int) as usize] as libc::c_int)
                            as int16_t;
                        value = (value as libc::c_int / 2 as libc::c_int + 989 as libc::c_int) as int16_t;
                    } else {
                        fprintf(
                            stderr,
                            b"ERROR: dsm resolution incorrect\n\0" as *const u8 as *const libc::c_char,
                        );
                        return 0 as *mut libc::c_void;
                    }
                    if ch_id as libc::c_int >= num_channels {
                        continue 's_283;
                    }
                    new_values[ch_id as usize] = value as libc::c_int;
                }
                i += 1;
            }
            is_complete = 1 as libc::c_int;
            i = 0 as libc::c_int;
            while i < num_channels {
                if new_values[i as usize] == 0 as libc::c_int {
                    is_complete = 0 as libc::c_int;
                    if num_channels > 7 as libc::c_int {
                        break;
                    }
                    i = 0 as libc::c_int;
                    while i < num_channels {
                        new_values[i as usize] = 0 as libc::c_int;
                        i += 1;
                    }
                }
                i += 1;
            }
            if is_complete != 0 {
                new_dsm_flag = 1 as libc::c_int;
                active_flag = 1 as libc::c_int;
                last_time = rc_nanos_since_boot();
                i = 0 as libc::c_int;
                while i < num_channels {
                    channels[i as usize] = new_values[i as usize];
                    new_values[i as usize] = 0 as libc::c_int;
                    i += 1;
                }
                if new_data_callback.is_some() {
                    ::core::mem::transmute::<_, fn()>(new_data_callback.expect("non-null function pointer"))();
                }
            }
        }
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn __calibration_listen_func(mut _ptr: *mut libc::c_void) -> *mut libc::c_void {
    let mut j: libc::c_int = 0;
    let mut raw: libc::c_int = 0;
    printf(b"waiting for dsm connection\0" as *const u8 as *const libc::c_char);
    new_dsm_flag = 0 as libc::c_int;
    while rc_dsm_is_new_data() == 0 {
        if listening == 0 as libc::c_int {
            return 0 as *mut libc::c_void;
        }
        rc_usleep(5000 as libc::c_int as libc::c_uint);
    }
    j = 0 as libc::c_int;
    while j < 9 as libc::c_int {
        mins[j as usize] = channels[j as usize];
        maxes[j as usize] = channels[j as usize];
        j += 1;
    }
    while listening != 0 {
        printf(b"\r\0" as *const u8 as *const libc::c_char);
        if rc_dsm_is_new_data() != 0 {
            j = 0 as libc::c_int;
            while j < 9 as libc::c_int {
                raw = channels[j as usize];
                if raw > 0 as libc::c_int {
                    if raw > maxes[j as usize] {
                        maxes[j as usize] = raw;
                    } else if raw < mins[j as usize] {
                        mins[j as usize] = raw;
                    }
                    printf(
                        b"%d:%d \0" as *const u8 as *const libc::c_char,
                        j + 1 as libc::c_int,
                        raw,
                    );
                }
                j += 1;
            }
            fflush(stdout);
        }
        rc_usleep(10000 as libc::c_int as libc::c_uint);
    }
    j = 0 as libc::c_int;
    while j < 9 as libc::c_int {
        raw = channels[j as usize];
        if raw > 0 as libc::c_int {
            centers[j as usize] = raw;
        }
        j += 1;
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn rc_dsm_init() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut fd: *mut FILE = 0 as *mut FILE;
    fd = fopen(
        b"/var/lib/robotcontrol/dsm.cal\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if fd.is_null() {
        fprintf(
            stderr,
            b"\ndsm Calibration File Doesn't Exist Yet\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"Run calibrate_dsm example to create one\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"Using default values for now\n\0" as *const u8 as *const libc::c_char,
        );
        i = 0 as libc::c_int;
        while i < 9 as libc::c_int {
            mins[i as usize] = 1142 as libc::c_int;
            maxes[i as usize] = 1858 as libc::c_int;
            centers[i as usize] = 1500 as libc::c_int;
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < 9 as libc::c_int {
            if fscanf(
                fd,
                b"%d %d %d\0" as *const u8 as *const libc::c_char,
                &mut *mins.as_mut_ptr().offset(i as isize) as *mut libc::c_int,
                &mut *maxes.as_mut_ptr().offset(i as isize) as *mut libc::c_int,
                &mut *centers.as_mut_ptr().offset(i as isize) as *mut libc::c_int,
            ) != 3 as libc::c_int
            {
                fprintf(
                    stderr,
                    b"ERROR in rc_dsm_init reading calibration data\n\0" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    stderr,
                    b"Malformed calibration file, deleting and using defaults\n\0" as *const u8 as *const libc::c_char,
                );
                remove(b"/var/lib/robotcontrol/dsm.cal\0" as *const u8 as *const libc::c_char);
                i = 0 as libc::c_int;
                while i < 9 as libc::c_int {
                    mins[i as usize] = 1142 as libc::c_int;
                    maxes[i as usize] = 1858 as libc::c_int;
                    centers[i as usize] = 1500 as libc::c_int;
                    i += 1;
                }
                break;
            } else {
                i += 1;
            }
        }
        fclose(fd);
    }
    i = 0 as libc::c_int;
    while i < 9 as libc::c_int {
        if centers[i as usize] < mins[i as usize] + 50 as libc::c_int
            && centers[i as usize] > mins[i as usize] - 50 as libc::c_int
            || centers[i as usize] < maxes[i as usize] + 50 as libc::c_int
                && centers[i as usize] > maxes[i as usize] - 50 as libc::c_int
        {
            centers[i as usize] = mins[i as usize];
            range_up[i as usize] = (maxes[i as usize] - mins[i as usize]) as libc::c_double;
            range_down[i as usize] = (maxes[i as usize] - mins[i as usize]) as libc::c_double;
        } else {
            range_up[i as usize] = (maxes[i as usize] - centers[i as usize]) as libc::c_double;
            range_down[i as usize] = (centers[i as usize] - mins[i as usize]) as libc::c_double;
        }
        i += 1;
    }
    if rc_pinmux_set(30 as libc::c_int, PINMUX_UART) != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_dsm_init, failed to set pinmux\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    dsm_frame_rate = 0 as libc::c_int;
    running = 1 as libc::c_int;
    num_channels = 0 as libc::c_int;
    last_time = 0 as libc::c_int as uint64_t;
    active_flag = 0 as libc::c_int;
    new_data_callback = None;
    disconnect_callback = None;
    new_dsm_flag = 0 as libc::c_int;
    if rc_uart_init(
        4 as libc::c_int,
        115200 as libc::c_int,
        0.2f64 as libc::c_float,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    ) != 0
    {
        fprintf(
            stderr,
            b"ERROR in rc_dsm_init, failed to init uart bus\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_pthread_create(
        &mut parse_thread,
        Some(__parser_func as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
        0 as libc::c_int,
        0 as libc::c_int,
    ) != 0
    {
        fprintf(
            stderr,
            b"ERROR in rc_dsm_init, failed to start thread\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        rc_usleep(20000 as libc::c_int as libc::c_uint);
        if init_flag != 0 {
            break;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_dsm_cleanup() -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if running == 0 {
        init_flag = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    running = 0 as libc::c_int;
    ret = rc_pthread_timed_join(parse_thread, 0 as *mut *mut libc::c_void, 1.0f64 as libc::c_float);
    if ret == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"ERORR in rc_dsm_cleanup, problem joining thread for pin\n\0" as *const u8 as *const libc::c_char,
        );
    } else if ret == 1 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_dsm_cleanup, thread exit timeout\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"most likely cause is your callback function is stuck and didn't return\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    init_flag = 0 as libc::c_int;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn rc_dsm_ch_raw(mut ch: libc::c_int) -> libc::c_int {
    if init_flag == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_dsm_ch_raw, call rc_dsm_init first\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if ch < 1 as libc::c_int || ch > 9 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_dsm_ch_raw channel must be between 1 & %d\0" as *const u8 as *const libc::c_char,
            9 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    new_dsm_flag = 0 as libc::c_int;
    return channels[(ch - 1 as libc::c_int) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn rc_dsm_ch_normalized(mut ch: libc::c_int) -> libc::c_double {
    if init_flag == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_dsm_ch_normalized, call rc_dsm_init first\n\0" as *const u8 as *const libc::c_char,
        );
        return -1.0f64;
    }
    if ch < 1 as libc::c_int || ch > 9 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_dsm_ch_raw channel must be between 1 & %d\0" as *const u8 as *const libc::c_char,
            9 as libc::c_int,
        );
        return -1.0f64;
    }
    if fabs(range_up[(ch - 1 as libc::c_int) as usize]) < 0.0001f64
        || fabs(range_down[(ch - 1 as libc::c_int) as usize]) < 0.0001f64
        || channels[(ch - 1 as libc::c_int) as usize] == 0 as libc::c_int
    {
        return 0.0f32 as libc::c_double;
    }
    new_dsm_flag = 0 as libc::c_int;
    if channels[(ch - 1 as libc::c_int) as usize] == centers[(ch - 1 as libc::c_int) as usize] {
        return 0.0f64;
    }
    if channels[(ch - 1 as libc::c_int) as usize] > centers[(ch - 1 as libc::c_int) as usize] {
        return (channels[(ch - 1 as libc::c_int) as usize] - centers[(ch - 1 as libc::c_int) as usize])
            as libc::c_double
            / range_up[(ch - 1 as libc::c_int) as usize];
    }
    return (channels[(ch - 1 as libc::c_int) as usize] - centers[(ch - 1 as libc::c_int) as usize]) as libc::c_double
        / range_down[(ch - 1 as libc::c_int) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn rc_dsm_is_new_data() -> libc::c_int {
    if init_flag == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_dsm_is_new_data, call rc_dsm_init first\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    return new_dsm_flag;
}
#[no_mangle]
pub unsafe extern "C" fn rc_dsm_set_callback(mut func: Option<unsafe extern "C" fn() -> ()>) {
    if init_flag == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_dsm_set_callback, call rc_dsm_init first\n\0" as *const u8 as *const libc::c_char,
        );
    }
    new_data_callback =
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, Option<unsafe extern "C" fn() -> ()>>(func);
}
#[no_mangle]
pub unsafe extern "C" fn rc_dsm_set_disconnect_callback(mut func: Option<unsafe extern "C" fn() -> ()>) {
    if init_flag == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_dsm_set_disconnect_callback, call rc_dsm_init first\n\0" as *const u8 as *const libc::c_char,
        );
    }
    disconnect_callback =
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, Option<unsafe extern "C" fn() -> ()>>(func);
}
#[no_mangle]
pub unsafe extern "C" fn rc_dsm_is_connection_active() -> libc::c_int {
    if init_flag == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_dsm_is_connection_active, call rc_dsm_init first\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    return active_flag;
}
#[no_mangle]
pub unsafe extern "C" fn rc_dsm_nanos_since_last_packet() -> int64_t {
    if init_flag == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_dsm_nanos_since_last_packet, call rc_dsm_init first\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int) as int64_t;
    }
    if last_time == 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int) as int64_t;
    }
    return (rc_nanos_since_boot()).wrapping_sub(last_time) as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn rc_dsm_resolution() -> libc::c_int {
    if init_flag == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_dsm_resolution, call rc_dsm_init first\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return resolution;
}
#[no_mangle]
pub unsafe extern "C" fn rc_dsm_channels() -> libc::c_int {
    if init_flag == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_dsm_channels, call rc_dsm_init first\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return num_channels;
}
#[no_mangle]
pub unsafe extern "C" fn rc_dsm_bind_routine() -> libc::c_int {
    let mut value: libc::c_int = 0;
    let mut delay: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut c: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut pulses: libc::c_int = 9 as libc::c_int;
    if rc_pinmux_set(30 as libc::c_int, PINMUX_GPIO_PD) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_dsm_bind_routine, pinmux helper not enabled for P9_11\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_gpio_init(
        0 as libc::c_int,
        30 as libc::c_int,
        ((1 as libc::c_ulong) << 0 as libc::c_int) as libc::c_int,
    ) == -(1 as libc::c_int)
    {
        fprintf(
            stderr,
            b"ERROR in rc_dsm_bind_routine initializing gpio pin as input\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    printf(
        b"\n\nYou must choose which DSM mode to request from your transmitter\n\0" as *const u8 as *const libc::c_char,
    );
    printf(b"Note that your transmitter may actually bind in a different mode\n\0" as *const u8 as *const libc::c_char);
    printf(b"depending on how it is configured.\n\0" as *const u8 as *const libc::c_char);
    printf(b"We suggest option 1 for 6-channel dsm radios,\n\0" as *const u8 as *const libc::c_char);
    printf(b"option 4 for 7-9 channel DSMX radios\n\0" as *const u8 as *const libc::c_char);
    printf(b"and option 5 for 6-channel Orange and JR radios\n\0" as *const u8 as *const libc::c_char);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"1: Spektrum  DSM2 10-bit 22ms framerate\n\0" as *const u8 as *const libc::c_char);
    printf(b"2: Spektrum  DSM2 11-bit 11ms framerate\n\0" as *const u8 as *const libc::c_char);
    printf(b"3: Spektrum  DSMX 10-bit 22ms framerate\n\0" as *const u8 as *const libc::c_char);
    printf(b"4: Spektrum  DSMX 11-bit 11ms framerate\n\0" as *const u8 as *const libc::c_char);
    printf(b"5: Orange/JR DSM2 10-bit 22ms framerate\n\0" as *const u8 as *const libc::c_char);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"Enter mode 1-5: \0" as *const u8 as *const libc::c_char);
    loop {
        c = getchar() as libc::c_char;
        match c as libc::c_int {
            49 => {
                pulses = 3 as libc::c_int;
                delay = 200000 as libc::c_int;
                break;
            }
            50 => {
                pulses = 5 as libc::c_int;
                delay = 200000 as libc::c_int;
                break;
            }
            51 => {
                pulses = 7 as libc::c_int;
                delay = 200000 as libc::c_int;
                break;
            }
            52 => {
                pulses = 9 as libc::c_int;
                delay = 200000 as libc::c_int;
                break;
            }
            53 => {
                pulses = 9 as libc::c_int;
                delay = 50000 as libc::c_int;
                break;
            }
            10 => {}
            _ => {
                fprintf(
                    stderr,
                    b"invalid entry, try again\n\0" as *const u8 as *const libc::c_char,
                );
                getchar();
            }
        }
    }
    printf(
        b"Using mode %c\n\0" as *const u8 as *const libc::c_char,
        c as libc::c_int,
    );
    printf(
        b"\nDisconnect your dsm satellite receiver if it is still connected\n\0" as *const u8 as *const libc::c_char,
    );
    printf(b"Plug it into the board quickly and firmly to begin binding.\n\0" as *const u8 as *const libc::c_char);
    value = 1 as libc::c_int;
    while value == 1 as libc::c_int {
        value = rc_gpio_get_value(0 as libc::c_int, 30 as libc::c_int);
        if value == -(1 as libc::c_int) {
            fprintf(
                stderr,
                b"ERROR in rc_dsm_bind_routine, failed to read gpio value\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        rc_usleep(500 as libc::c_int as libc::c_uint);
    }
    rc_usleep(100000 as libc::c_int as libc::c_uint);
    while value == 0 as libc::c_int {
        value = rc_gpio_get_value(0 as libc::c_int, 30 as libc::c_int);
        rc_usleep(500 as libc::c_int as libc::c_uint);
    }
    rc_gpio_cleanup(0 as libc::c_int, 30 as libc::c_int);
    if rc_gpio_init(
        0 as libc::c_int,
        30 as libc::c_int,
        ((1 as libc::c_ulong) << 1 as libc::c_int) as libc::c_int,
    ) == -(1 as libc::c_int)
    {
        fprintf(
            stderr,
            b"ERROR in rc_dsm_bind_routine initializing gpio pin as output\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_gpio_set_value(0 as libc::c_int, 30 as libc::c_int, 1 as libc::c_int) == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"ERROR in rc_dsm_bind_routine, failed to write to gpio\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_usleep(delay as libc::c_uint);
    i = 0 as libc::c_int;
    while i < pulses {
        rc_gpio_set_value(0 as libc::c_int, 30 as libc::c_int, 0 as libc::c_int);
        rc_usleep(115 as libc::c_int as libc::c_uint);
        rc_gpio_set_value(0 as libc::c_int, 30 as libc::c_int, 1 as libc::c_int);
        rc_usleep(115 as libc::c_int as libc::c_uint);
        i += 1;
    }
    rc_usleep(1000000 as libc::c_int as libc::c_uint);
    if rc_pinmux_set(30 as libc::c_int, PINMUX_UART) != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_dsm_bind_routine, failed to put pin back to UART mode\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    printf(b"\n\n\nYour receiver should now be blinking. If not try again.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"This is a finicky process and may require a few attempts, don't be discouraged!\n\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"If the receiver LED is blinking, turn on your transmitter in bind mode.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"After binding completes use rc_test_dsm to confirm functionality.\n\n\0" as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_dsm_calibrate_routine() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut fd: *mut FILE = 0 as *mut FILE;
    dsm_frame_rate = 0 as libc::c_int;
    running = 1 as libc::c_int;
    num_channels = 0 as libc::c_int;
    last_time = 0 as libc::c_int as uint64_t;
    active_flag = 0 as libc::c_int;
    new_data_callback = None;
    disconnect_callback = None;
    ret = mkdir(
        b"/var/lib/robotcontrol/\0" as *const u8 as *const libc::c_char,
        0o777 as libc::c_int as __mode_t,
    );
    if ret == -(1 as libc::c_int) && *__errno_location() != 17 as libc::c_int {
        perror(
            b"ERROR in rc_dsm_calibration_routine making calibration file directory\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_uart_init(
        4 as libc::c_int,
        115200 as libc::c_int,
        0.5f64 as libc::c_float,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    ) != 0
    {
        fprintf(
            stderr,
            b"ERROR in rc_dsm_calibrate_routine, failed to init uart bus\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    pthread_create(
        &mut parse_thread,
        0 as *const pthread_attr_t,
        Some(__parser_func as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );
    i = 0 as libc::c_int;
    while init_flag == 0 as libc::c_int {
        rc_usleep(10000 as libc::c_int as libc::c_uint);
        if i > 10 as libc::c_int {
            fprintf(
                stderr,
                b"ERROR in rc_dsm_calibrate_routine, timeout waiting for parser thread to start\n\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        i += 1;
    }
    printf(b"\nRaw dsm data should display below if the transmitter and\n\0" as *const u8 as *const libc::c_char);
    printf(b"receiver are paired and working. Move all channels through\n\0" as *const u8 as *const libc::c_char);
    printf(b"their range of motion and the minimum and maximum values will\n\0" as *const u8 as *const libc::c_char);
    printf(b"be recorded. When you are finished moving all channels,\n\0" as *const u8 as *const libc::c_char);
    printf(b"return 3-position switches and sticks to their natural\n\0" as *const u8 as *const libc::c_char);
    printf(b"zero-position which will be recorded.\n\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Two position switches can be left in either position, and sliding\n\0" as *const u8 as *const libc::c_char,
    );
    printf(b"throttle sticks should be left at the bottom of their travel.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"If there is a RATE switch, make sure it's in the HIGH position.\n\n\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"If there is a DISARM switch which fixes the throttle position, leave\n\0" as *const u8 as *const libc::c_char,
    );
    printf(b"it in the ARMED state and DO NOT TOUCH IT during calibration\n\0" as *const u8 as *const libc::c_char);
    printf(b"Press ENTER to save data or any other key to abort.\n\n\0" as *const u8 as *const libc::c_char);
    listening = 1 as libc::c_int;
    let mut listening_thread: pthread_t = 0;
    pthread_create(
        &mut listening_thread,
        0 as *const pthread_attr_t,
        Some(__calibration_listen_func as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );
    ret = __continue_or_quit();
    listening = 0 as libc::c_int;
    pthread_join(listening_thread, 0 as *mut *mut libc::c_void);
    if ret < 0 as libc::c_int {
        fprintf(
            stderr,
            b"aborting calibrate_dsm routine\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if mins[0 as libc::c_int as usize] == 0 as libc::c_int
        || mins[0 as libc::c_int as usize] == maxes[0 as libc::c_int as usize]
    {
        fprintf(
            stderr,
            b"no new data recieved, exiting\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    fd = fopen(
        b"/var/lib/robotcontrol/dsm.cal\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if fd.is_null() {
        perror(
            b"ERROR in rc_dsm_calibration_routine opening calibration file for writing\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < 9 as libc::c_int {
        if mins[i as usize] == 0 as libc::c_int || mins[i as usize] == maxes[i as usize] {
            fprintf(
                fd,
                b"%d %d %d\n\0" as *const u8 as *const libc::c_char,
                1142 as libc::c_int,
                1858 as libc::c_int,
                1500 as libc::c_int,
            );
        } else {
            fprintf(
                fd,
                b"%d %d %d\n\0" as *const u8 as *const libc::c_char,
                mins[i as usize],
                maxes[i as usize],
                centers[i as usize],
            );
        }
        i += 1;
    }
    fclose(fd);
    printf(b"New calibration file written\n\0" as *const u8 as *const libc::c_char);
    printf(b"use rc_test_dsm to confirm\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
