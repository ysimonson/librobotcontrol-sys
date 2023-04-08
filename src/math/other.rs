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
    fn rand() -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
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
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union rc_int_float_t {
    pub i: uint32_t,
    pub f: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union rc_int_double_t {
    pub i: uint64_t,
    pub d: libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn rc_get_random_float() -> libc::c_float {
    let mut new: rc_int_float_t = rc_int_float_t { i: 0 };
    new.i = (rand() & 0x7fffff as libc::c_int | 0x40000000 as libc::c_int) as uint32_t;
    return new.f - 3.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn rc_get_random_double() -> libc::c_double {
    let mut new: rc_int_double_t = rc_int_double_t { i: 0 };
    new.i = ((rand() as uint64_t) << 32 as libc::c_int | rand() as libc::c_ulong)
        & 0xfffffffffffffi64 as libc::c_long as libc::c_ulong
        | 0x4000000000000000i64 as libc::c_long as libc::c_ulong;
    return new.d - 3.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn rc_saturate_float(
    mut val: *mut libc::c_float,
    mut min: libc::c_float,
    mut max: libc::c_float,
) -> libc::c_int {
    if (min > max) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_saturate_float, min must be less than max\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if *val > max {
        *val = max;
        return 1 as libc::c_int;
    } else {
        if *val < min {
            *val = min;
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_saturate_double(
    mut val: *mut libc::c_double,
    mut min: libc::c_double,
    mut max: libc::c_double,
) -> libc::c_int {
    if (min > max) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_saturate_double, min must be less than max\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if *val > max {
        *val = max;
        return 1 as libc::c_int;
    } else {
        if *val < min {
            *val = min;
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
