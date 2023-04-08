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
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn rc_get_random_double() -> libc::c_double;
    fn __vectorized_mult_accumulate(a: *mut libc::c_double, b: *mut libc::c_double, n: libc::c_int) -> libc::c_double;
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
pub struct rc_vector_t {
    pub len: libc::c_int,
    pub d: *mut libc::c_double,
    pub initialized: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_alloc(mut v: *mut rc_vector_t, mut length: libc::c_int) -> libc::c_int {
    if (length < 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_alloc, length must be >=1\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if v.is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_alloc, received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*v).initialized != 0 && (*v).len == length {
        return 0 as libc::c_int;
    }
    rc_vector_free(v);
    (*v).d = malloc((length as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong))
        as *mut libc::c_double;
    if ((*v).d).is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_alloc, not enough memory\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*v).len = length;
    (*v).initialized = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_free(mut v: *mut rc_vector_t) -> libc::c_int {
    let mut new: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if v.is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR rc_vector_free, received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*v).initialized != 0 {
        free((*v).d as *mut libc::c_void);
    }
    *v = new;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_empty() -> rc_vector_t {
    let mut out: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_zeros(mut v: *mut rc_vector_t, mut length: libc::c_int) -> libc::c_int {
    if (length < 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_zeros, length must be >=1\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if v.is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_zeros, received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_vector_free(v);
    (*v).d = calloc(
        length as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    if ((*v).d).is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_zeros, not enough memory\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*v).len = length;
    (*v).initialized = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_ones(mut v: *mut rc_vector_t, mut length: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (rc_vector_alloc(v, length) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_ones, failed to allocate vector\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < length {
        *((*v).d).offset(i as isize) = 1.0f32 as libc::c_double;
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_random(mut v: *mut rc_vector_t, mut length: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (rc_vector_alloc(v, length) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR rc_vector_random, failed to allocate vector\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < length {
        *((*v).d).offset(i as isize) = rc_get_random_double();
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_fibonnaci(mut v: *mut rc_vector_t, mut length: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (rc_vector_alloc(v, length) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR rc_vector_fibonnaci, failed to allocate vector\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    *((*v).d).offset(0 as libc::c_int as isize) = 1.0f32 as libc::c_double;
    if length > 1 as libc::c_int {
        *((*v).d).offset(1 as libc::c_int as isize) = 1.0f32 as libc::c_double;
    }
    i = 2 as libc::c_int;
    while i < length {
        *((*v).d).offset(i as isize) =
            *((*v).d).offset((i - 1 as libc::c_int) as isize) + *((*v).d).offset((i - 2 as libc::c_int) as isize);
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_from_array(
    mut v: *mut rc_vector_t,
    mut ptr: *mut libc::c_double,
    mut length: libc::c_int,
) -> libc::c_int {
    if ptr.is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_from_array, received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_vector_alloc(v, length) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_from_array, failed to allocate vector\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    memcpy(
        (*v).d as *mut libc::c_void,
        ptr as *const libc::c_void,
        (length as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_duplicate(mut a: rc_vector_t, mut b: *mut rc_vector_t) -> libc::c_int {
    if (a.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_duplicate_vector, a not initialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_vector_alloc(b, a.len) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_duplicate_vector, failed to allocate vector\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    memcpy(
        (*b).d as *mut libc::c_void,
        a.d as *const libc::c_void,
        (a.len as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_print(mut v: rc_vector_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (v.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_print, vector not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < v.len {
        printf(
            b"%7.4f  \0" as *const u8 as *const libc::c_char,
            *(v.d).offset(i as isize),
        );
        i += 1;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_print_sci(mut v: rc_vector_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (v.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_print_sci, vector not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < v.len {
        printf(
            b"%11.4e  \0" as *const u8 as *const libc::c_char,
            *(v.d).offset(i as isize),
        );
        i += 1;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_zero_out(mut v: *mut rc_vector_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if ((*v).initialized != 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_zero_out,vector not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < (*v).len {
        *((*v).d).offset(i as isize) = 0.0f64;
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_times_scalar(mut v: *mut rc_vector_t, mut s: libc::c_double) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if ((*v).initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_times_scalar, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < (*v).len {
        *((*v).d).offset(i as isize) *= s;
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_norm(mut v: rc_vector_t, mut p: libc::c_double) -> libc::c_double {
    let mut norm: libc::c_double = 0.0f32 as libc::c_double;
    let mut i: libc::c_int = 0;
    if (v.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_norm, vector not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int) as libc::c_double;
    }
    if (p <= 0.0f64) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_norm, p must be a positive real value\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int) as libc::c_double;
    }
    if p < 1.001f64 && p > 0.999f64 {
        i = 0 as libc::c_int;
        while i < v.len {
            norm += fabs(*(v.d).offset(i as isize));
            i += 1;
        }
        return norm;
    }
    if p < 2.001f64 && p > 1.999f64 {
        i = 0 as libc::c_int;
        while i < v.len {
            norm += *(v.d).offset(i as isize) * *(v.d).offset(i as isize);
            i += 1;
        }
        return sqrt(norm);
    }
    i = 0 as libc::c_int;
    while i < v.len {
        norm += pow(fabs(*(v.d).offset(i as isize)), p);
        i += 1;
    }
    return pow(norm, 1.0f64 / p);
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_max(mut v: rc_vector_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut index: libc::c_int = 0 as libc::c_int;
    let mut tmp: libc::c_double = -1.7976931348623157e+308f64;
    if (v.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_max, vector not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < v.len {
        if *(v.d).offset(i as isize) > tmp {
            index = i;
            tmp = *(v.d).offset(i as isize);
        }
        i += 1;
    }
    return index;
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_min(mut v: rc_vector_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut index: libc::c_int = 0 as libc::c_int;
    let mut tmp: libc::c_double = 1.7976931348623157e+308f64;
    if (v.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_min, vector not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < v.len {
        if *(v.d).offset(i as isize) < tmp {
            index = i;
            tmp = *(v.d).offset(i as isize);
        }
        i += 1;
    }
    return index;
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_std_dev(mut v: rc_vector_t) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut mean: libc::c_double = 0.;
    let mut mean_sqr: libc::c_double = 0.;
    let mut diff: libc::c_double = 0.;
    if (v.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_std_dev, vector not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -1.0f32 as libc::c_double;
    }
    if v.len == 1 as libc::c_int {
        return 0.0f32 as libc::c_double;
    }
    mean = 0.0f32 as libc::c_double;
    i = 0 as libc::c_int;
    while i < v.len {
        mean += *(v.d).offset(i as isize);
        i += 1;
    }
    mean = mean / v.len as libc::c_double;
    mean_sqr = 0.0f32 as libc::c_double;
    i = 0 as libc::c_int;
    while i < v.len {
        diff = *(v.d).offset(i as isize) - mean;
        mean_sqr += diff * diff;
        i += 1;
    }
    return sqrt(mean_sqr / (v.len - 1 as libc::c_int) as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_mean(mut v: rc_vector_t) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut sum: libc::c_double = 0.0f32 as libc::c_double;
    if (v.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_mean, vector not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -1.0f32 as libc::c_double;
    }
    i = 0 as libc::c_int;
    while i < v.len {
        sum += *(v.d).offset(i as isize);
        i += 1;
    }
    return sum / v.len as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_projection(
    mut v: rc_vector_t,
    mut e: rc_vector_t,
    mut p: *mut rc_vector_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut factor: libc::c_double = 0.;
    if (v.initialized == 0 || e.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_projection, received uninitialized vector\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (v.len != e.len) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_projection, vectors not of same length\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_vector_alloc(p, v.len) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_projection, failed to allocate p\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    factor = rc_vector_dot_product(v, e) / rc_vector_dot_product(e, e);
    i = 0 as libc::c_int;
    while i < v.len {
        *((*p).d).offset(i as isize) = factor * *(e.d).offset(i as isize);
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_dot_product(mut v1: rc_vector_t, mut v2: rc_vector_t) -> libc::c_double {
    if (v1.initialized == 0 || v2.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_dot_product, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -1.0f32 as libc::c_double;
    }
    if (v1.len != v2.len) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_dot_product, dimension mismatch\n\0" as *const u8 as *const libc::c_char,
        );
        return -1.0f32 as libc::c_double;
    }
    return __vectorized_mult_accumulate(v1.d, v2.d, v1.len);
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_cross_product(
    mut v1: rc_vector_t,
    mut v2: rc_vector_t,
    mut p: *mut rc_vector_t,
) -> libc::c_int {
    if (v1.initialized == 0 || v2.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_cross_product, vector not initialized yet.\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (v1.len != 3 as libc::c_int || v2.len != 3 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_cross_product, vector must have length 3\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_vector_alloc(p, 3 as libc::c_int) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_cross_product, failed to allocate p\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    *((*p).d).offset(0 as libc::c_int as isize) = *(v1.d).offset(1 as libc::c_int as isize)
        * *(v2.d).offset(2 as libc::c_int as isize)
        - *(v1.d).offset(2 as libc::c_int as isize) * *(v2.d).offset(1 as libc::c_int as isize);
    *((*p).d).offset(1 as libc::c_int as isize) = *(v1.d).offset(2 as libc::c_int as isize)
        * *(v2.d).offset(0 as libc::c_int as isize)
        - *(v1.d).offset(0 as libc::c_int as isize) * *(v2.d).offset(2 as libc::c_int as isize);
    *((*p).d).offset(2 as libc::c_int as isize) = *(v1.d).offset(0 as libc::c_int as isize)
        * *(v2.d).offset(1 as libc::c_int as isize)
        - *(v1.d).offset(1 as libc::c_int as isize) * *(v2.d).offset(0 as libc::c_int as isize);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_sum(
    mut v1: rc_vector_t,
    mut v2: rc_vector_t,
    mut s: *mut rc_vector_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (v1.initialized == 0 || v2.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_sum, received uninitialized vector\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (v1.len != v2.len) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_sum, vectors not of same length\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_vector_alloc(s, v1.len) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_sum, failed to allocate s\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < v1.len {
        *((*s).d).offset(i as isize) = *(v1.d).offset(i as isize) + *(v2.d).offset(i as isize);
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_sum_inplace(mut v1: *mut rc_vector_t, mut v2: rc_vector_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if ((*v1).initialized == 0 || v2.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_sum_inplace, received uninitialized vector\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if ((*v1).len != v2.len) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_sum_inplace, vectors not of same length\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < (*v1).len {
        *((*v1).d).offset(i as isize) += *(v2.d).offset(i as isize);
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_vector_subtract(
    mut v1: rc_vector_t,
    mut v2: rc_vector_t,
    mut s: *mut rc_vector_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (v1.initialized == 0 || v2.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_substract, received uninitialized vector\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (v1.len != v2.len) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_substract, vectors not of same length\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_vector_alloc(s, v1.len) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_vector_substract, failed to allocate s\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < v1.len {
        *((*s).d).offset(i as isize) = *(v1.d).offset(i as isize) - *(v2.d).offset(i as isize);
        i += 1;
    }
    return 0 as libc::c_int;
}
