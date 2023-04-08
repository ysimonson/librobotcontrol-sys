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
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
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
pub type rc_governor_t = libc::c_uint;
pub const RC_GOV_CONSERVATIVE: rc_governor_t = 4;
pub const RC_GOV_SCHEDUTIL: rc_governor_t = 3;
pub const RC_GOV_ONDEMAND: rc_governor_t = 2;
pub const RC_GOV_PERFORMANCE: rc_governor_t = 1;
pub const RC_GOV_POWERSAVE: rc_governor_t = 0;
#[no_mangle]
pub unsafe extern "C" fn rc_cpu_set_governor(mut gov: rc_governor_t) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    fd = open(
        b"/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor\0" as *const u8 as *const libc::c_char,
        0o1 as libc::c_int,
    );
    if fd == -(1 as libc::c_int) {
        perror(b"ERROR in rc_cpu_set_governor\0" as *const u8 as *const libc::c_char);
        if *__errno_location() == 1 as libc::c_int {
            fprintf(stderr, b"try running as root\n\0" as *const u8 as *const libc::c_char);
        }
        return -(1 as libc::c_int);
    }
    match gov as libc::c_uint {
        0 => {
            ret = write(
                fd,
                b"powersave\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                9 as libc::c_int as size_t,
            ) as libc::c_int;
        }
        1 => {
            ret = write(
                fd,
                b"performance\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                11 as libc::c_int as size_t,
            ) as libc::c_int;
        }
        2 => {
            ret = write(
                fd,
                b"ondemand\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                8 as libc::c_int as size_t,
            ) as libc::c_int;
        }
        3 => {
            ret = write(
                fd,
                b"schedutil\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                9 as libc::c_int as size_t,
            ) as libc::c_int;
        }
        4 => {
            ret = write(
                fd,
                b"conservative\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                12 as libc::c_int as size_t,
            ) as libc::c_int;
        }
        _ => {
            fprintf(
                stderr,
                b"ERROR in rc_cpu_set_governor, unknown governor enum\n\0" as *const u8 as *const libc::c_char,
            );
            close(fd);
            return -(1 as libc::c_int);
        }
    }
    if ret == -(1 as libc::c_int) {
        perror(b"ERROR in rc_cpu_set_governor\0" as *const u8 as *const libc::c_char);
    }
    close(fd);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_cpu_get_freq() -> libc::c_int {
    let mut freq: libc::c_int = 0;
    let mut fd: *mut FILE = fopen(
        b"/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if fd.is_null() {
        perror(b"ERROR in rc_cpu_get_freq\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if fscanf(
        fd,
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut freq as *mut libc::c_int,
    ) != 1 as libc::c_int
    {
        perror(b"ERROR in rc_cpu_get_freq\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"failed to read from CPU current frequency path\n\0" as *const u8 as *const libc::c_char,
        );
        fclose(fd);
        return -(1 as libc::c_int);
    }
    fclose(fd);
    return freq;
}
#[no_mangle]
pub unsafe extern "C" fn rc_cpu_print_freq() -> libc::c_int {
    let mut freq: libc::c_int = rc_cpu_get_freq();
    if freq == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    printf(
        b"%dmhz\0" as *const u8 as *const libc::c_char,
        freq / 1000 as libc::c_int,
    );
    return 0 as libc::c_int;
}
