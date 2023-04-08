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
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
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
static mut init_flag: libc::c_int = 0 as libc::c_int;
static mut fd: [libc::c_int; 8] = [0; 8];
#[no_mangle]
pub unsafe extern "C" fn rc_adc_init() -> libc::c_int {
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    let mut temp_fd: libc::c_int = 0;
    if init_flag != 0 {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            b"/sys/bus/iio/devices/iio:device0/in_voltage%d_raw\0" as *const u8 as *const libc::c_char,
            i,
        );
        temp_fd = open(buf.as_mut_ptr(), 0 as libc::c_int);
        if temp_fd < 0 as libc::c_int {
            perror(b"ERROR in rc_adc_init, failed to open iio adc interface\n\0" as *const u8 as *const libc::c_char);
            fprintf(
                stderr,
                b"Perhaps kernel or device tree is too old\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        fd[i as usize] = temp_fd;
        i += 1;
    }
    init_flag = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_adc_cleanup() -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        close(fd[i as usize]);
        i += 1;
    }
    init_flag = 0 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_adc_read_raw(mut ch: libc::c_int) -> libc::c_int {
    let mut buf: [libc::c_char; 8] = [0; 8];
    let mut i: libc::c_int = 0;
    if (init_flag == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_adc_read_raw, please initialize with rc_adc_init() first\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (ch < 0 as libc::c_int || ch >= 8 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_adc_read_raw, adc channel must be between 0 & %d\n\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int - 1 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if (lseek(fd[ch as usize], 0 as libc::c_int as __off_t, 0 as libc::c_int) < 0 as libc::c_int as libc::c_long)
        as libc::c_int as libc::c_long
        != 0
    {
        perror(b"ERROR: in rc_adc_read_raw, failed to seek to beginning of FD\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if (read(
        fd[ch as usize],
        buf.as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
    ) == -(1 as libc::c_int) as libc::c_long) as libc::c_int as libc::c_long
        != 0
    {
        perror(b"ERROR in rc_adc_read_raw, can't read iio adc fd\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    i = atoi(buf.as_mut_ptr());
    if i > 4095 as libc::c_int || i < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: in rc_adc_read_raw, value out of bounds: %d\n\0" as *const u8 as *const libc::c_char,
            i,
        );
        return -(1 as libc::c_int);
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn rc_adc_read_volt(mut ch: libc::c_int) -> libc::c_double {
    let mut raw: libc::c_int = rc_adc_read_raw(ch);
    if raw < 0 as libc::c_int {
        return -(1 as libc::c_int) as libc::c_double;
    }
    return raw as libc::c_double * 1.8f64 / 4095.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn rc_adc_batt() -> libc::c_double {
    let mut v: libc::c_double = rc_adc_read_volt(6 as libc::c_int) * 11.0f64 + -0.10f64;
    if v < 1.0f64 {
        v = 0.0f64;
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn rc_adc_dc_jack() -> libc::c_double {
    let mut v: libc::c_double = rc_adc_read_volt(5 as libc::c_int) * 11.0f64 + -0.15f64;
    if v < 1.0f64 {
        v = 0.0f64;
    }
    return v;
}
