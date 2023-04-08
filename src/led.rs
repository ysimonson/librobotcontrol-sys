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
    fn perror(__s: *const libc::c_char);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn rc_usleep(us: libc::c_uint);
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
pub type rc_led_t = libc::c_uint;
pub const RC_LED_WIFI: rc_led_t = 10;
pub const RC_LED_BAT100: rc_led_t = 9;
pub const RC_LED_BAT75: rc_led_t = 8;
pub const RC_LED_BAT50: rc_led_t = 7;
pub const RC_LED_BAT25: rc_led_t = 6;
pub const RC_LED_USR3: rc_led_t = 5;
pub const RC_LED_USR2: rc_led_t = 4;
pub const RC_LED_USR1: rc_led_t = 3;
pub const RC_LED_USR0: rc_led_t = 2;
pub const RC_LED_RED: rc_led_t = 1;
pub const RC_LED_GREEN: rc_led_t = 0;
static mut paths: [*const libc::c_char; 11] = [
    b"green\0" as *const u8 as *const libc::c_char,
    b"red\0" as *const u8 as *const libc::c_char,
    b"beaglebone:green:usr0\0" as *const u8 as *const libc::c_char,
    b"beaglebone:green:usr1\0" as *const u8 as *const libc::c_char,
    b"beaglebone:green:usr2\0" as *const u8 as *const libc::c_char,
    b"beaglebone:green:usr3\0" as *const u8 as *const libc::c_char,
    b"bat25\0" as *const u8 as *const libc::c_char,
    b"bat50\0" as *const u8 as *const libc::c_char,
    b"bat75\0" as *const u8 as *const libc::c_char,
    b"bat100\0" as *const u8 as *const libc::c_char,
    b"wifi\0" as *const u8 as *const libc::c_char,
];
static mut fd: [libc::c_int; 11] = [0; 11];
static mut blinking: [libc::c_int; 11] = [0; 11];
static mut stop_blinking_flag: [libc::c_int; 11] = [0; 11];
unsafe extern "C" fn init_led(mut led: rc_led_t) -> libc::c_int {
    if fd[led as libc::c_int as usize] != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut temp_fd: libc::c_int = 0;
    snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b"/sys/devices/platform/leds/leds/%s/brightness\0" as *const u8 as *const libc::c_char,
        paths[led as libc::c_int as usize],
    );
    temp_fd = open(buf.as_mut_ptr(), 0o2 as libc::c_int);
    if temp_fd == -(1 as libc::c_int) {
        perror(b"ERROR: in init_led, failed to open LED file handle\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    fd[led as libc::c_int as usize] = temp_fd;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_led_set(mut led: rc_led_t, mut value: libc::c_int) -> libc::c_int {
    if fd[led as libc::c_int as usize] == 0 as libc::c_int {
        if init_led(led) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    let mut ret: libc::c_int = 0;
    if value != 0 {
        ret = write(
            fd[led as libc::c_int as usize],
            b"1\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as size_t,
        ) as libc::c_int;
    } else {
        ret = write(
            fd[led as libc::c_int as usize],
            b"0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as size_t,
        ) as libc::c_int;
    }
    if ret == -(1 as libc::c_int) {
        perror(b"ERROR in rc_led_set writing to led fd\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_led_cleanup() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 11 as libc::c_int {
        close(fd[i as usize]);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rc_led_get(mut led: rc_led_t) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut ch: libc::c_char = 0;
    if fd[led as libc::c_int as usize] == 0 as libc::c_int {
        if init_led(led) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    ret = read(
        fd[led as libc::c_int as usize],
        &mut ch as *mut libc::c_char as *mut libc::c_void,
        1 as libc::c_int as size_t,
    ) as libc::c_int;
    if ret == -(1 as libc::c_int) {
        perror(b"ERROR in rc_led_get reading file descriptor\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if ch as libc::c_int == '0' as i32 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_led_blink(
    mut led: rc_led_t,
    mut hz: libc::c_float,
    mut duration: libc::c_float,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut toggle: libc::c_int = 0 as libc::c_int;
    if blinking[led as libc::c_int as usize] != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_led_blink(), led is already blinking!\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let delay_us: libc::c_int = (1000000.0f32 / (2.0f32 * hz)) as libc::c_int;
    let blinks: libc::c_int = (duration * 2.0f32 * hz) as libc::c_int;
    blinking[led as libc::c_int as usize] = 1 as libc::c_int;
    stop_blinking_flag[led as libc::c_int as usize] = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < blinks {
        if stop_blinking_flag[led as libc::c_int as usize] != 0 {
            rc_led_set(led, 0 as libc::c_int);
            blinking[led as libc::c_int as usize] = 0 as libc::c_int;
            return 1 as libc::c_int;
        }
        toggle = (toggle == 0) as libc::c_int;
        if rc_led_set(led, toggle) < 0 as libc::c_int {
            blinking[led as libc::c_int as usize] = 0 as libc::c_int;
            return -(1 as libc::c_int);
        }
        rc_usleep(delay_us as libc::c_uint);
        i += 1;
    }
    rc_led_set(led, 0 as libc::c_int);
    blinking[led as libc::c_int as usize] = 0 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_led_stop_blink(mut led: rc_led_t) {
    stop_blinking_flag[led as libc::c_int as usize] = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_led_stop_blink_all() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 11 as libc::c_int {
        stop_blinking_flag[i as usize] = 0 as libc::c_int;
        i += 1;
    }
}
