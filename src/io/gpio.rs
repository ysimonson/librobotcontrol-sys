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
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
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
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type __u8 = libc::c_uchar;
pub type __u32 = libc::c_uint;
pub type __u64 = libc::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gpiohandle_request {
    pub lineoffsets: [__u32; 64],
    pub flags: __u32,
    pub default_values: [__u8; 64],
    pub consumer_label: [libc::c_char; 32],
    pub lines: __u32,
    pub fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gpiohandle_data {
    pub values: [__u8; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gpioevent_request {
    pub lineoffset: __u32,
    pub handleflags: __u32,
    pub eventflags: __u32,
    pub consumer_label: [libc::c_char; 32],
    pub fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gpioevent_data {
    pub timestamp: __u64,
    pub id: __u32,
}
pub type uint64_t = __uint64_t;
static mut chip_fd: [libc::c_int; 6] = [0; 6];
static mut handle_fd: [[libc::c_int; 64]; 6] = [[0; 64]; 6];
static mut event_fd: [[libc::c_int; 64]; 6] = [[0; 64]; 6];
unsafe extern "C" fn __open_gpiochip(mut chip: libc::c_int) -> libc::c_int {
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut temp_fd: libc::c_int = 0;
    snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        b"/dev/gpiochip%d\0" as *const u8 as *const libc::c_char,
        chip,
    );
    temp_fd = open(buf.as_mut_ptr(), 0o2 as libc::c_int);
    if temp_fd == -(1 as libc::c_int) {
        perror(b"ERROR opening gpiochip\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    chip_fd[chip as usize] = temp_fd;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_gpio_init(
    mut chip: libc::c_int,
    mut pin: libc::c_int,
    mut handle_flags: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut req: gpiohandle_request = gpiohandle_request {
        lineoffsets: [0; 64],
        flags: 0,
        default_values: [0; 64],
        consumer_label: [0; 32],
        lines: 0,
        fd: 0,
    };
    if chip < 0 as libc::c_int || chip >= 6 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_gpio_init, chip out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if pin < 0 as libc::c_int || pin >= 64 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_gpio_init, pin out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if chip_fd[chip as usize] == 0 as libc::c_int {
        if (__open_gpiochip(chip) != 0) as libc::c_int as libc::c_long != 0 {
            return -(1 as libc::c_int);
        }
    }
    memset(
        &mut req as *mut gpiohandle_request as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<gpiohandle_request>() as libc::c_ulong,
    );
    req.lineoffsets[0 as libc::c_int as usize] = pin as __u32;
    req.lines = 1 as libc::c_int as __u32;
    req.flags = handle_flags as __u32;
    *__errno_location() = 0 as libc::c_int;
    ret = ioctl(
        chip_fd[chip as usize],
        ((2 as libc::c_uint | 1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | ((0xb4 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((0x3 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::core::mem::size_of::<gpiohandle_request>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut req as *mut gpiohandle_request,
    );
    if (ret == -(1 as libc::c_int)) as libc::c_int as libc::c_long != 0 {
        perror(b"ERROR in rc_gpio_init\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if req.fd == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_gpio_init, ioctl gave NULL fd\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    handle_fd[chip as usize][pin as usize] = req.fd;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_gpio_set_value(
    mut chip: libc::c_int,
    mut pin: libc::c_int,
    mut value: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut data: gpiohandle_data = gpiohandle_data { values: [0; 64] };
    if chip < 0 as libc::c_int || chip >= 6 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_gpio_set_value, chip out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if pin < 0 as libc::c_int || pin >= 64 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_gpio_set_value, pin out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (handle_fd[chip as usize][pin as usize] == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR, pin %d not initialized yet\n\0" as *const u8 as *const libc::c_char,
            pin,
        );
        return -(1 as libc::c_int);
    }
    if value != 0 {
        data.values[0 as libc::c_int as usize] = 1 as libc::c_int as __u8;
    } else {
        data.values[0 as libc::c_int as usize] = 0 as libc::c_int as __u8;
    }
    ret = ioctl(
        handle_fd[chip as usize][pin as usize],
        ((2 as libc::c_uint | 1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | ((0xb4 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((0x9 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::core::mem::size_of::<gpiohandle_data>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut data as *mut gpiohandle_data,
    );
    if (ret == -(1 as libc::c_int)) as libc::c_int as libc::c_long != 0 {
        perror(b"ERROR in rc_gpio_set_value\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_gpio_get_value(mut chip: libc::c_int, mut pin: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut data: gpiohandle_data = gpiohandle_data { values: [0; 64] };
    if chip < 0 as libc::c_int || chip >= 6 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_gpio_get_value, chip out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if pin < 0 as libc::c_int || pin >= 64 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_gpio_get_value, pin out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (handle_fd[chip as usize][pin as usize] == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_gpio_get_value chip %d pin %d not initialized yet\n\0" as *const u8 as *const libc::c_char,
            chip,
            pin,
        );
        return -(1 as libc::c_int);
    }
    ret = ioctl(
        handle_fd[chip as usize][pin as usize],
        ((2 as libc::c_uint | 1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | ((0xb4 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((0x8 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::core::mem::size_of::<gpiohandle_data>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut data as *mut gpiohandle_data,
    );
    if (ret == -(1 as libc::c_int)) as libc::c_int as libc::c_long != 0 {
        perror(b"ERROR in rc_gpio_get_value\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return data.values[0 as libc::c_int as usize] as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_gpio_init_event(
    mut chip: libc::c_int,
    mut pin: libc::c_int,
    mut handle_flags: libc::c_int,
    mut event_flags: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut req: gpioevent_request = gpioevent_request {
        lineoffset: 0,
        handleflags: 0,
        eventflags: 0,
        consumer_label: [0; 32],
        fd: 0,
    };
    if chip < 0 as libc::c_int || chip >= 6 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_gpio_init_event, chip out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if pin < 0 as libc::c_int || pin >= 64 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_gpio_init_event, pin out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (handle_flags as libc::c_ulong & (1 as libc::c_ulong) << 1 as libc::c_int != 0) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"ERROR in rc_gpio_init_event, can't request OUTPUT and poll input events\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if chip_fd[chip as usize] == 0 as libc::c_int {
        if (__open_gpiochip(chip) != 0) as libc::c_int as libc::c_long != 0 {
            return -(1 as libc::c_int);
        }
    }
    req.lineoffset = pin as __u32;
    req.eventflags = event_flags as __u32;
    req.handleflags = handle_flags as __u32;
    ret = ioctl(
        chip_fd[chip as usize],
        ((2 as libc::c_uint | 1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | ((0xb4 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((0x4 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::core::mem::size_of::<gpioevent_request>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut req as *mut gpioevent_request,
    );
    if (ret == -(1 as libc::c_int)) as libc::c_int as libc::c_long != 0 {
        perror(b"ERROR in rc_gpio_init_event\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    event_fd[chip as usize][pin as usize] = req.fd;
    handle_fd[chip as usize][pin as usize] = req.fd;
    return req.fd;
}
#[no_mangle]
pub unsafe extern "C" fn rc_gpio_poll(
    mut chip: libc::c_int,
    mut pin: libc::c_int,
    mut timeout_ms: libc::c_int,
    mut event_time_ns: *mut uint64_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut event: gpioevent_data = gpioevent_data { timestamp: 0, id: 0 };
    let mut poll_fds: [pollfd; 1] = [pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    }; 1];
    if chip < 0 as libc::c_int || chip >= 6 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_gpio_poll, chip out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if pin < 0 as libc::c_int || pin >= 64 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_gpio_poll, pin out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    poll_fds[0 as libc::c_int as usize].fd = event_fd[chip as usize][pin as usize];
    poll_fds[0 as libc::c_int as usize].events = (0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_short;
    poll_fds[0 as libc::c_int as usize].revents = 0 as libc::c_int as libc::c_short;
    ret = poll(poll_fds.as_mut_ptr(), 1 as libc::c_int as nfds_t, timeout_ms);
    if (ret == -(1 as libc::c_int)) as libc::c_int as libc::c_long != 0 {
        perror(b"ERROR in rc_gpio_poll calling poll\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    } else {
        if ret == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    ret = read(
        event_fd[chip as usize][pin as usize],
        &mut event as *mut gpioevent_data as *mut libc::c_void,
        ::core::mem::size_of::<gpioevent_data>() as libc::c_ulong,
    ) as libc::c_int;
    if ret == -(1 as libc::c_int) {
        perror(b"ERROR in rc_gpio_poll while reading event\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if event.id != 0x1 as libc::c_int as libc::c_uint && event.id != 0x2 as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"ERROR in rc_gpio_poll, read unknown event ID\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if !event_time_ns.is_null() {
        *event_time_ns = event.timestamp as uint64_t;
    }
    if event.id == 0x1 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int;
    }
    return 2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_gpio_cleanup(mut chip: libc::c_int, mut pin: libc::c_int) {
    if chip < 0 as libc::c_int || chip >= 6 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_gpio_cleanup, chip out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if pin < 0 as libc::c_int || pin >= 64 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_gpio_cleanup, pin out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if handle_fd[chip as usize][pin as usize] != 0 as libc::c_int {
        close(handle_fd[chip as usize][pin as usize]);
        handle_fd[chip as usize][pin as usize] = 0 as libc::c_int;
    }
    if event_fd[chip as usize][pin as usize] != 0 as libc::c_int {
        close(event_fd[chip as usize][pin as usize]);
        event_fd[chip as usize][pin as usize] = 0 as libc::c_int;
    }
}
