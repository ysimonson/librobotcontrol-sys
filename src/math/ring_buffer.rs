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
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rc_ringbuf_t {
    pub d: *mut libc::c_double,
    pub size: libc::c_int,
    pub index: libc::c_int,
    pub initialized: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn rc_ringbuf_empty() -> rc_ringbuf_t {
    let mut out: rc_ringbuf_t = {
        let mut init = rc_ringbuf_t {
            d: 0 as *mut libc::c_double,
            size: 0 as libc::c_int,
            index: 0 as libc::c_int,
            initialized: 0 as libc::c_int,
        };
        init
    };
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn rc_ringbuf_alloc(mut buf: *mut rc_ringbuf_t, mut size: libc::c_int) -> libc::c_int {
    if buf.is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_ringbuf_alloc, received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (size < 2 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_ringbuf_alloc, size must be >=2\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*buf).initialized != 0 && (*buf).size == size && !((*buf).d).is_null() {
        return 0 as libc::c_int;
    }
    (*buf).size = 0 as libc::c_int;
    (*buf).index = 0 as libc::c_int;
    (*buf).initialized = 0 as libc::c_int;
    free((*buf).d as *mut libc::c_void);
    (*buf).d = calloc(
        size as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    if ((*buf).d).is_null() {
        fprintf(
            stderr,
            b"ERROR in rc_ringbuf_alloc, failed to allocate memory\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*buf).size = size;
    (*buf).initialized = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_ringbuf_free(mut buf: *mut rc_ringbuf_t) -> libc::c_int {
    let mut new: rc_ringbuf_t = {
        let mut init = rc_ringbuf_t {
            d: 0 as *mut libc::c_double,
            size: 0 as libc::c_int,
            index: 0 as libc::c_int,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if buf.is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_ringbuf_free, received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*buf).initialized != 0 {
        free((*buf).d as *mut libc::c_void);
    }
    *buf = new;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_ringbuf_reset(mut buf: *mut rc_ringbuf_t) -> libc::c_int {
    if buf.is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_ringbuf_reset, received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if ((*buf).initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR rc_ringbuf_reset, ringbuf uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    memset(
        (*buf).d as *mut libc::c_void,
        0 as libc::c_int,
        ((*buf).size as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    (*buf).index = 0 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_ringbuf_insert(mut buf: *mut rc_ringbuf_t, mut val: libc::c_double) -> libc::c_int {
    let mut new_index: libc::c_int = 0;
    if buf.is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_ringbuf_insert, received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if ((*buf).initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_ringbuf_insert, ringbuf uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    new_index = (*buf).index + 1 as libc::c_int;
    if new_index >= (*buf).size {
        new_index = 0 as libc::c_int;
    }
    *((*buf).d).offset(new_index as isize) = val;
    (*buf).index = new_index;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_ringbuf_get_value(mut buf: *mut rc_ringbuf_t, mut pos: libc::c_int) -> libc::c_double {
    let mut return_index: libc::c_int = 0;
    if buf.is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_ringbuf_get_value, received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -1.0f32 as libc::c_double;
    }
    if (pos < 0 as libc::c_int || pos > (*buf).size - 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_ringbuf_get_value, position out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        return -1.0f32 as libc::c_double;
    }
    if ((*buf).initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_ringbuf_get_value, ringbuf uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -1.0f32 as libc::c_double;
    }
    return_index = (*buf).index - pos;
    if return_index < 0 as libc::c_int {
        return_index += (*buf).size;
    }
    return *((*buf).d).offset(return_index as isize);
}
#[no_mangle]
pub unsafe extern "C" fn rc_ringbuf_std_dev(mut buf: rc_ringbuf_t) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut mean: libc::c_double = 0.;
    let mut mean_sqr: libc::c_double = 0.;
    let mut diff: libc::c_double = 0.;
    if (buf.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_ringbuf_std_dev, ringbuf not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -1.0f32 as libc::c_double;
    }
    if buf.size == 1 as libc::c_int {
        return 0.0f32 as libc::c_double;
    }
    mean = 0.0f32 as libc::c_double;
    i = 0 as libc::c_int;
    while i < buf.size {
        mean += *(buf.d).offset(i as isize);
        i += 1;
    }
    mean = mean / buf.size as libc::c_double;
    mean_sqr = 0.0f32 as libc::c_double;
    i = 0 as libc::c_int;
    while i < buf.size {
        diff = *(buf.d).offset(i as isize) - mean;
        mean_sqr += diff * diff;
        i += 1;
    }
    return sqrt(mean_sqr / (buf.size - 1 as libc::c_int) as libc::c_double);
}
