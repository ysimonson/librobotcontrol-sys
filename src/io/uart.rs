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
    fn perror(__s: *const libc::c_char);
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn cfsetospeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
    fn cfsetispeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    fn tcsetattr(__fd: libc::c_int, __optional_actions: libc::c_int, __termios_p: *const termios) -> libc::c_int;
    fn tcflush(__fd: libc::c_int, __queue_selector: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
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
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
pub type uint8_t = __uint8_t;
static mut rc_uart_fd: [libc::c_int; 17] = [0; 17];
static mut rc_uart_bus_timeout_s: [libc::c_float; 17] = [0.; 17];
static mut rc_uart_shutdown_flag: [libc::c_int; 17] = [0; 17];
#[no_mangle]
pub unsafe extern "C" fn rc_uart_init(
    mut bus: libc::c_int,
    mut baudrate: libc::c_int,
    mut timeout_s: libc::c_float,
    mut canonical_en: libc::c_int,
    mut stop_bits: libc::c_int,
    mut parity_en: libc::c_int,
) -> libc::c_int {
    let mut tmpfd: libc::c_int = 0;
    let mut tenths: libc::c_int = 0;
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut config: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    let mut speed: speed_t = 0;
    if bus < 0 as libc::c_int || bus > 16 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_uart_init, bus must be between 0 & %d\n\0" as *const u8 as *const libc::c_char,
            16 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if timeout_s < 0.1f32 {
        fprintf(
            stderr,
            b"ERROR in rc_uart_init, timeout must be >=0.1 seconds\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if stop_bits != 1 as libc::c_int && stop_bits != 2 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_uart_init, stop bits must be 1 or 2\n\0" as *const u8 as *const libc::c_char,
        );
    }
    match baudrate {
        230400 => {
            speed = 0o10003 as libc::c_int as speed_t;
        }
        115200 => {
            speed = 0o10002 as libc::c_int as speed_t;
        }
        57600 => {
            speed = 0o10001 as libc::c_int as speed_t;
        }
        38400 => {
            speed = 0o17 as libc::c_int as speed_t;
        }
        19200 => {
            speed = 0o16 as libc::c_int as speed_t;
        }
        9600 => {
            speed = 0o15 as libc::c_int as speed_t;
        }
        4800 => {
            speed = 0o14 as libc::c_int as speed_t;
        }
        2400 => {
            speed = 0o13 as libc::c_int as speed_t;
        }
        1800 => {
            speed = 0o12 as libc::c_int as speed_t;
        }
        1200 => {
            speed = 0o11 as libc::c_int as speed_t;
        }
        600 => {
            speed = 0o10 as libc::c_int as speed_t;
        }
        300 => {
            speed = 0o7 as libc::c_int as speed_t;
        }
        200 => {
            speed = 0o6 as libc::c_int as speed_t;
        }
        150 => {
            speed = 0o5 as libc::c_int as speed_t;
        }
        134 => {
            speed = 0o4 as libc::c_int as speed_t;
        }
        110 => {
            speed = 0o3 as libc::c_int as speed_t;
        }
        75 => {
            speed = 0o2 as libc::c_int as speed_t;
        }
        50 => {
            speed = 0o1 as libc::c_int as speed_t;
        }
        _ => {
            fprintf(
                stderr,
                b"ERROR: int rc_uart_init, invalid baudrate. Please use a standard baudrate\n\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    rc_uart_close(bus);
    snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        b"/dev/ttyO%d\0" as *const u8 as *const libc::c_char,
        bus,
    );
    tmpfd = open(
        buf.as_mut_ptr(),
        0o2 as libc::c_int | 0o400 as libc::c_int | 0o4000 as libc::c_int,
    );
    if tmpfd == -(1 as libc::c_int) {
        perror(b"ERROR: int rc_uart_init while opening file descriptor\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"device tree probably isn't loaded\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if tcgetattr(tmpfd, &mut config) == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"ERROR: int rc_uart_init, Cannot get uart attributes\n\0" as *const u8 as *const libc::c_char,
        );
        close(tmpfd);
        return -(1 as libc::c_int);
    }
    memset(
        &mut config as *mut termios as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<termios>() as libc::c_ulong,
    );
    if canonical_en != 0 {
        config.c_lflag |= 0o2 as libc::c_int as libc::c_uint;
    } else {
        config.c_lflag &= !(0o2 as libc::c_int) as libc::c_uint;
    }
    if parity_en != 0 {
        config.c_cflag |= 0o400 as libc::c_int as libc::c_uint;
    } else {
        config.c_cflag &= !(0o400 as libc::c_int) as libc::c_uint;
    }
    if stop_bits == 1 as libc::c_int {
        config.c_cflag &= !(0o100 as libc::c_int) as libc::c_uint;
    } else {
        config.c_cflag |= 0o100 as libc::c_int as libc::c_uint;
    }
    config.c_cflag &= !(0o60 as libc::c_int) as libc::c_uint;
    config.c_cflag |= 0o60 as libc::c_int as libc::c_uint;
    config.c_cflag |= 0o200 as libc::c_int as libc::c_uint;
    config.c_cflag |= 0o4000 as libc::c_int as libc::c_uint;
    tenths = (timeout_s * 10 as libc::c_int as libc::c_float) as libc::c_int;
    config.c_cc[6 as libc::c_int as usize] = 128 as libc::c_int as cc_t;
    config.c_cc[5 as libc::c_int as usize] = (tenths + 1 as libc::c_int) as cc_t;
    if cfsetispeed(&mut config, speed) == -(1 as libc::c_int) {
        perror(b"ERROR: in rc_uart_init calling cfsetispeed\0" as *const u8 as *const libc::c_char);
        close(tmpfd);
        return -(1 as libc::c_int);
    }
    if cfsetospeed(&mut config, speed) == -(1 as libc::c_int) {
        perror(b"ERROR: in rc_uart_init calling cfsetospeed\0" as *const u8 as *const libc::c_char);
        close(tmpfd);
        return -(1 as libc::c_int);
    }
    if tcflush(tmpfd, 2 as libc::c_int) == -(1 as libc::c_int) {
        perror(b"ERROR: in rc_uart_init calling tcflush\0" as *const u8 as *const libc::c_char);
        close(tmpfd);
    }
    if tcsetattr(tmpfd, 0 as libc::c_int, &mut config) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"cannot set uart%d attributes\n\0" as *const u8 as *const libc::c_char,
            bus,
        );
        close(rc_uart_fd[bus as usize]);
        return -(1 as libc::c_int);
    }
    if tcflush(tmpfd, 2 as libc::c_int) == -(1 as libc::c_int) {
        perror(b"ERROR: in rc_uart_init calling tcflush\0" as *const u8 as *const libc::c_char);
        close(tmpfd);
        return -(1 as libc::c_int);
    }
    if fcntl(tmpfd, 4 as libc::c_int, 0 as libc::c_int) == -(1 as libc::c_int) {
        perror(b"ERROR: in rc_uart_init calling fcntl\0" as *const u8 as *const libc::c_char);
        close(tmpfd);
        return -(1 as libc::c_int);
    }
    if tcflush(tmpfd, 2 as libc::c_int) == -(1 as libc::c_int) {
        perror(b"ERROR: in rc_uart_init calling tcflush\0" as *const u8 as *const libc::c_char);
        close(tmpfd);
        return -(1 as libc::c_int);
    }
    rc_uart_fd[bus as usize] = tmpfd;
    rc_uart_bus_timeout_s[bus as usize] = timeout_s;
    rc_uart_shutdown_flag[bus as usize] = 0 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_uart_close(mut bus: libc::c_int) -> libc::c_int {
    if bus < 0 as libc::c_int || bus > 16 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: uart bus must be between 0 & %d\n\0" as *const u8 as *const libc::c_char,
            16 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    rc_uart_shutdown_flag[bus as usize] = 1 as libc::c_int;
    if rc_uart_fd[bus as usize] == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    tcflush(rc_uart_fd[bus as usize], 2 as libc::c_int);
    close(rc_uart_fd[bus as usize]);
    rc_uart_fd[bus as usize] = 0 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_uart_get_fd(mut bus: libc::c_int) -> libc::c_int {
    if bus < 0 as libc::c_int || bus > 16 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: in rc_uart_get_fd, bus must be between 0 & %d\n\0" as *const u8 as *const libc::c_char,
            16 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if rc_uart_fd[bus as usize] == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: in rc_uart_get_fd, uart%d not initialized yet\n\0" as *const u8 as *const libc::c_char,
            bus,
        );
        return -(1 as libc::c_int);
    }
    return rc_uart_fd[bus as usize];
}
#[no_mangle]
pub unsafe extern "C" fn rc_uart_flush(mut bus: libc::c_int) -> libc::c_int {
    if bus < 0 as libc::c_int || bus > 16 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: in rc_uart_flush, bus must be between 0 & %d\n\0" as *const u8 as *const libc::c_char,
            16 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if rc_uart_fd[bus as usize] == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: in rc_uart_flush, uart%d must be initialized first\n\0" as *const u8 as *const libc::c_char,
            bus,
        );
        return -(1 as libc::c_int);
    }
    if tcflush(rc_uart_fd[bus as usize], 2 as libc::c_int) == -(1 as libc::c_int) {
        perror(b"ERROR in rc_uart_flush:\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_uart_write(mut bus: libc::c_int, mut data: *mut uint8_t, mut bytes: size_t) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if bus < 0 as libc::c_int || bus > 16 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: uart bus must be between 0 & %d\n\0" as *const u8 as *const libc::c_char,
            16 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if bytes < 1 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"ERROR: number of bytes to send must be >1\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_uart_fd[bus as usize] == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: uart%d must be initialized first\n\0" as *const u8 as *const libc::c_char,
            bus,
        );
        return -(1 as libc::c_int);
    }
    ret = write(rc_uart_fd[bus as usize], data as *const libc::c_void, bytes) as libc::c_int;
    if ret == -(1 as libc::c_int) {
        perror(b"ERROR in rc_uart_write\0" as *const u8 as *const libc::c_char);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn rc_uart_read_bytes(
    mut bus: libc::c_int,
    mut buf: *mut uint8_t,
    mut bytes: size_t,
) -> libc::c_int {
    let mut bytes_to_read: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if bus < 0 as libc::c_int || bus > 16 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: uart bus must be between 0 & %d\n\0" as *const u8 as *const libc::c_char,
            16 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if bytes < 1 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"ERROR: number of bytes to read must be >=1\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_uart_fd[bus as usize] == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: uart%d must be initialized first\n\0" as *const u8 as *const libc::c_char,
            bus,
        );
        return -(1 as libc::c_int);
    }
    let mut set: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut timeout: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut bytes_read: libc::c_int = 0;
    let mut bytes_left: libc::c_int = 0;
    bytes_read = 0 as libc::c_int;
    bytes_left = bytes as libc::c_int;
    timeout.tv_sec = rc_uart_bus_timeout_s[bus as usize] as libc::c_int as __time_t;
    timeout.tv_usec = (1000000 as libc::c_int as libc::c_double
        * fmod(
            rc_uart_bus_timeout_s[bus as usize] as libc::c_double,
            1 as libc::c_int as libc::c_double,
        )) as libc::c_int as __suseconds_t;
    while bytes_left > 0 as libc::c_int && rc_uart_shutdown_flag[bus as usize] == 0 as libc::c_int {
        let mut __i: libc::c_uint = 0;
        let mut __arr: *mut fd_set = &mut set;
        __i = 0 as libc::c_int as libc::c_uint;
        while (__i as libc::c_ulong)
            < (::core::mem::size_of::<fd_set>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<__fd_mask>() as libc::c_ulong)
        {
            (*__arr).__fds_bits[__i as usize] = 0 as libc::c_int as __fd_mask;
            __i = __i.wrapping_add(1);
        }
        set.__fds_bits[(rc_uart_fd[bus as usize]
            / (8 as libc::c_int * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize] |= ((1 as libc::c_ulong)
            << rc_uart_fd[bus as usize]
                % (8 as libc::c_int * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as __fd_mask;
        ret = select(
            rc_uart_fd[bus as usize] + 1 as libc::c_int,
            &mut set,
            0 as *mut fd_set,
            0 as *mut fd_set,
            &mut timeout,
        );
        if ret == -(1 as libc::c_int) {
            if *__errno_location() != 4 as libc::c_int {
                perror(b"ERROR in rc_uart_read_bytes calling select\0" as *const u8 as *const libc::c_char);
                return -(1 as libc::c_int);
            }
            return bytes_read;
        } else {
            if ret == 0 as libc::c_int {
                return bytes_read;
            } else {
                if bytes_left > 128 as libc::c_int {
                    bytes_to_read = 128 as libc::c_int;
                } else {
                    bytes_to_read = bytes_left;
                }
                ret = read(
                    rc_uart_fd[bus as usize],
                    buf.offset(bytes_read as isize) as *mut libc::c_void,
                    bytes_to_read as size_t,
                ) as libc::c_int;
                if ret < 0 as libc::c_int {
                    perror(b"ERROR: in uart_read_bytes\0" as *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int);
                } else {
                    if ret > 0 as libc::c_int {
                        bytes_read += ret;
                        bytes_left -= ret;
                    }
                }
            }
        }
    }
    return bytes_read;
}
#[no_mangle]
pub unsafe extern "C" fn rc_uart_read_line(
    mut bus: libc::c_int,
    mut buf: *mut uint8_t,
    mut max_bytes: size_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut temp: libc::c_char = 0;
    let mut set: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut timeout: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut bytes_read: libc::c_int = 0 as libc::c_int;
    if bus < 0 as libc::c_int || bus > 16 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: in rc_uart_read_line, bus must be between 0 & %d\n\0" as *const u8 as *const libc::c_char,
            16 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if max_bytes < 1 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"ERROR: in rc_uart_read_line, max_bytes must be >=1\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_uart_fd[bus as usize] == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: in rc_uart_read_line, uart%d must be initialized first\n\0" as *const u8 as *const libc::c_char,
            bus,
        );
        return -(1 as libc::c_int);
    }
    timeout.tv_sec = rc_uart_bus_timeout_s[bus as usize] as libc::c_int as __time_t;
    timeout.tv_usec = (1000000 as libc::c_int as libc::c_double
        * fmod(
            rc_uart_bus_timeout_s[bus as usize] as libc::c_double,
            1 as libc::c_int as libc::c_double,
        )) as libc::c_int as __suseconds_t;
    while bytes_read < max_bytes as libc::c_int && rc_uart_shutdown_flag[bus as usize] == 0 as libc::c_int {
        let mut __i: libc::c_uint = 0;
        let mut __arr: *mut fd_set = &mut set;
        __i = 0 as libc::c_int as libc::c_uint;
        while (__i as libc::c_ulong)
            < (::core::mem::size_of::<fd_set>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<__fd_mask>() as libc::c_ulong)
        {
            (*__arr).__fds_bits[__i as usize] = 0 as libc::c_int as __fd_mask;
            __i = __i.wrapping_add(1);
        }
        set.__fds_bits[(rc_uart_fd[bus as usize]
            / (8 as libc::c_int * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize] |= ((1 as libc::c_ulong)
            << rc_uart_fd[bus as usize]
                % (8 as libc::c_int * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as __fd_mask;
        ret = select(
            rc_uart_fd[bus as usize] + 1 as libc::c_int,
            &mut set,
            0 as *mut fd_set,
            0 as *mut fd_set,
            &mut timeout,
        );
        if ret == -(1 as libc::c_int) {
            if *__errno_location() != 4 as libc::c_int {
                perror(b"ERROR in rc_uart_read_line calling select\0" as *const u8 as *const libc::c_char);
                return -(1 as libc::c_int);
            }
            return bytes_read;
        } else {
            if ret == 0 as libc::c_int {
                return bytes_read;
            } else {
                ret = read(
                    rc_uart_fd[bus as usize],
                    &mut temp as *mut libc::c_char as *mut libc::c_void,
                    1 as libc::c_int as size_t,
                ) as libc::c_int;
                if ret < 0 as libc::c_int {
                    perror(b"ERROR in rc_uart_read_line calling read\0" as *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int);
                } else {
                    if ret == 1 as libc::c_int {
                        if temp as libc::c_int == '\n' as i32 {
                            return bytes_read;
                        } else {
                            *buf.offset(bytes_read as isize) = temp as uint8_t;
                            bytes_read += 1;
                        }
                    }
                }
            }
        }
    }
    return bytes_read;
}
#[no_mangle]
pub unsafe extern "C" fn rc_uart_bytes_available(mut bus: libc::c_int) -> libc::c_int {
    let mut out: libc::c_int = 0;
    if bus < 0 as libc::c_int || bus > 16 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: uart bus must be between 0 & %d\n\0" as *const u8 as *const libc::c_char,
            16 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if rc_uart_fd[bus as usize] == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: uart%d must be initialized first\n\0" as *const u8 as *const libc::c_char,
            bus,
        );
        return -(1 as libc::c_int);
    }
    if ioctl(
        rc_uart_fd[bus as usize],
        0x541b as libc::c_int as libc::c_ulong,
        &mut out as *mut libc::c_int,
    ) == -(1 as libc::c_int)
    {
        perror(b"ERROR in rc_uart_bytes_available calling ioctl\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return out;
}
