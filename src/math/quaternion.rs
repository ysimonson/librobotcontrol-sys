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
    fn asin(_: libc::c_double) -> libc::c_double;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    static mut zero_tolerance: libc::c_double;
    fn rc_vector_alloc(v: *mut rc_vector_t, length: libc::c_int) -> libc::c_int;
    fn rc_vector_free(v: *mut rc_vector_t) -> libc::c_int;
    fn rc_vector_norm(v: rc_vector_t, p: libc::c_double) -> libc::c_double;
    fn rc_matrix_alloc(A: *mut rc_matrix_t, rows: libc::c_int, cols: libc::c_int) -> libc::c_int;
    fn rc_matrix_free(A: *mut rc_matrix_t) -> libc::c_int;
    fn rc_matrix_times_col_vec(A: rc_matrix_t, v: rc_vector_t, c: *mut rc_vector_t) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rc_matrix_t {
    pub rows: libc::c_int,
    pub cols: libc::c_int,
    pub d: *mut *mut libc::c_double,
    pub initialized: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn rc_quaternion_norm(mut q: rc_vector_t) -> libc::c_double {
    if (q.len != 4 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_norm, expected vector of length 4\n\0" as *const u8 as *const libc::c_char,
        );
        return -1.0f64;
    }
    return rc_vector_norm(q, 2 as libc::c_int as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn rc_quaternion_norm_array(mut q: *mut libc::c_double) -> libc::c_double {
    let mut sum: libc::c_double = 0.0f64;
    let mut i: libc::c_int = 0;
    if q.is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_norm_array, received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -1.0f64;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        sum += *q.offset(i as isize) * *q.offset(i as isize);
        i += 1;
    }
    return sqrt(sum);
}
#[no_mangle]
pub unsafe extern "C" fn rc_normalize_quaternion(mut q: *mut rc_vector_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_double = 0.;
    if ((*q).len != 4 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_normalize_quaternion, expected vector of length 4\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    len = rc_vector_norm(*q, 2 as libc::c_int as libc::c_double);
    if (len <= 0.0f64) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_normalize_quaternion, unable to calculate norm\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        *((*q).d).offset(i as isize) /= len;
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_normalize_quaternion_array(mut q: *mut libc::c_double) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.0f64;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        sum += *q.offset(i as isize) * *q.offset(i as isize);
        i += 1;
    }
    len = sqrtf(sum as libc::c_float) as libc::c_double;
    if (fabs(len) < zero_tolerance) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in quaternion has 0 length\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        *q.offset(i as isize) = *q.offset(i as isize) / len;
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_quaternion_to_tb(mut q: rc_vector_t, mut tb: *mut rc_vector_t) -> libc::c_int {
    if (q.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_to_tb, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (q.len != 4 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_to_tb, expected vector of length 4\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_vector_alloc(tb, 3 as libc::c_int) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_to_tb, failed to alloc array\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_quaternion_to_tb_array(q.d, (*tb).d);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_quaternion_to_tb_array(
    mut q: *mut libc::c_double,
    mut tb: *mut libc::c_double,
) -> libc::c_int {
    if (q.is_null() || tb.is_null()) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_quaternion_to_tb_array, received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    *tb.offset(1 as libc::c_int as isize) = asin(
        2.0f64
            * (*q.offset(0 as libc::c_int as isize) * *q.offset(2 as libc::c_int as isize)
                - *q.offset(1 as libc::c_int as isize) * *q.offset(3 as libc::c_int as isize)),
    );
    *tb.offset(0 as libc::c_int as isize) = atan2(
        2.0f64
            * (*q.offset(2 as libc::c_int as isize) * *q.offset(3 as libc::c_int as isize)
                + *q.offset(0 as libc::c_int as isize) * *q.offset(1 as libc::c_int as isize)),
        1.0f64
            - 2.0f64
                * (*q.offset(1 as libc::c_int as isize) * *q.offset(1 as libc::c_int as isize)
                    + *q.offset(2 as libc::c_int as isize) * *q.offset(2 as libc::c_int as isize)),
    );
    *tb.offset(2 as libc::c_int as isize) = atan2(
        2.0f64
            * (*q.offset(1 as libc::c_int as isize) * *q.offset(2 as libc::c_int as isize)
                + *q.offset(0 as libc::c_int as isize) * *q.offset(3 as libc::c_int as isize)),
        1.0f64
            - 2.0f64
                * (*q.offset(2 as libc::c_int as isize) * *q.offset(2 as libc::c_int as isize)
                    + *q.offset(3 as libc::c_int as isize) * *q.offset(3 as libc::c_int as isize)),
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_quaternion_from_tb(mut tb: rc_vector_t, mut q: *mut rc_vector_t) -> libc::c_int {
    if (tb.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_from_tb, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (tb.len != 3 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_from_tb, expected vector of length 3\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_vector_alloc(q, 4 as libc::c_int) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_from_tb, failed to alloc array\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_quaternion_from_tb_array(tb.d, (*q).d);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_quaternion_from_tb_array(
    mut tb: *mut libc::c_double,
    mut q: *mut libc::c_double,
) -> libc::c_int {
    if (q.is_null() || q.is_null()) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_quaternion_from_tb_array, received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut tbt: [libc::c_double; 3] = [0.; 3];
    tbt[0 as libc::c_int as usize] = *tb.offset(0 as libc::c_int as isize) / 2.0f64;
    tbt[1 as libc::c_int as usize] = *tb.offset(1 as libc::c_int as isize) / 2.0f64;
    tbt[2 as libc::c_int as usize] = *tb.offset(2 as libc::c_int as isize) / 2.0f64;
    let mut cosX2: libc::c_double = cos(tbt[0 as libc::c_int as usize]);
    let mut sinX2: libc::c_double = sin(tbt[0 as libc::c_int as usize]);
    let mut cosY2: libc::c_double = cos(tbt[1 as libc::c_int as usize]);
    let mut sinY2: libc::c_double = sin(tbt[1 as libc::c_int as usize]);
    let mut cosZ2: libc::c_double = cos(tbt[2 as libc::c_int as usize]);
    let mut sinZ2: libc::c_double = sin(tbt[2 as libc::c_int as usize]);
    *q.offset(0 as libc::c_int as isize) = cosX2 * cosY2 * cosZ2 + sinX2 * sinY2 * sinZ2;
    *q.offset(1 as libc::c_int as isize) = sinX2 * cosY2 * cosZ2 - cosX2 * sinY2 * sinZ2;
    *q.offset(2 as libc::c_int as isize) = cosX2 * sinY2 * cosZ2 + sinX2 * cosY2 * sinZ2;
    *q.offset(3 as libc::c_int as isize) = cosX2 * cosY2 * sinZ2 - sinX2 * sinY2 * cosZ2;
    rc_normalize_quaternion_array(q);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_quaternion_conjugate(mut q: rc_vector_t, mut c: *mut rc_vector_t) -> libc::c_int {
    if (q.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_conjugate, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (q.len != 4 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_conjugate, expected vector of length 4\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_vector_alloc(c, 4 as libc::c_int) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_conjugate, failed to alloc array\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    *((*c).d).offset(0 as libc::c_int as isize) = *(q.d).offset(0 as libc::c_int as isize);
    *((*c).d).offset(1 as libc::c_int as isize) = -*(q.d).offset(1 as libc::c_int as isize);
    *((*c).d).offset(2 as libc::c_int as isize) = -*(q.d).offset(2 as libc::c_int as isize);
    *((*c).d).offset(3 as libc::c_int as isize) = -*(q.d).offset(3 as libc::c_int as isize);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_quaternion_conjugate_inplace(mut q: *mut rc_vector_t) -> libc::c_int {
    if ((*q).initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_conjugate, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if ((*q).len != 4 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_conjugate, expected vector of length 4\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    *((*q).d).offset(1 as libc::c_int as isize) = -*((*q).d).offset(1 as libc::c_int as isize);
    *((*q).d).offset(2 as libc::c_int as isize) = -*((*q).d).offset(2 as libc::c_int as isize);
    *((*q).d).offset(3 as libc::c_int as isize) = -*((*q).d).offset(3 as libc::c_int as isize);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_quaternion_conjugate_array(
    mut q: *mut libc::c_double,
    mut c: *mut libc::c_double,
) -> libc::c_int {
    if (q.is_null() || c.is_null()) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_quaternion_conjugate_array, received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    *c.offset(0 as libc::c_int as isize) = *q.offset(0 as libc::c_int as isize);
    *c.offset(1 as libc::c_int as isize) = -*q.offset(1 as libc::c_int as isize);
    *c.offset(2 as libc::c_int as isize) = -*q.offset(2 as libc::c_int as isize);
    *c.offset(3 as libc::c_int as isize) = -*q.offset(3 as libc::c_int as isize);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_quaternion_conjugate_array_inplace(mut q: *mut libc::c_double) -> libc::c_int {
    if q.is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_quaternion_conjugate_array_inplace, received NULL pointer\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    *q.offset(1 as libc::c_int as isize) = -*q.offset(1 as libc::c_int as isize);
    *q.offset(2 as libc::c_int as isize) = -*q.offset(2 as libc::c_int as isize);
    *q.offset(3 as libc::c_int as isize) = -*q.offset(3 as libc::c_int as isize);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_quaternion_imaginary_part(mut q: rc_vector_t, mut img: *mut rc_vector_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (q.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_imaginary_part, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (q.len != 4 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_imaginary_part, expected vector of length 4\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_vector_alloc(img, 3 as libc::c_int) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_imaginary_part, failed to alloc array\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        *((*img).d).offset(i as isize) = *(q.d).offset((i + 1 as libc::c_int) as isize);
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_quaternion_multiply(
    mut a: rc_vector_t,
    mut b: rc_vector_t,
    mut c: *mut rc_vector_t,
) -> libc::c_int {
    let mut tmp: rc_matrix_t = {
        let mut init = rc_matrix_t {
            rows: 0 as libc::c_int,
            cols: 0 as libc::c_int,
            d: 0 as *mut *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if (a.initialized == 0 || b.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_multiply, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (a.len != 4 as libc::c_int || b.len != 4 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_multiply, expected vector of length 4\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_matrix_alloc(&mut tmp, 4 as libc::c_int, 4 as libc::c_int) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_multiply, failed to alloc matrix\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    *(*(tmp.d).offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize) =
        *(a.d).offset(0 as libc::c_int as isize);
    *(*(tmp.d).offset(0 as libc::c_int as isize)).offset(1 as libc::c_int as isize) =
        -*(a.d).offset(1 as libc::c_int as isize);
    *(*(tmp.d).offset(0 as libc::c_int as isize)).offset(2 as libc::c_int as isize) =
        -*(a.d).offset(2 as libc::c_int as isize);
    *(*(tmp.d).offset(0 as libc::c_int as isize)).offset(3 as libc::c_int as isize) =
        -*(a.d).offset(3 as libc::c_int as isize);
    *(*(tmp.d).offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize) =
        *(a.d).offset(1 as libc::c_int as isize);
    *(*(tmp.d).offset(1 as libc::c_int as isize)).offset(1 as libc::c_int as isize) =
        *(a.d).offset(0 as libc::c_int as isize);
    *(*(tmp.d).offset(1 as libc::c_int as isize)).offset(2 as libc::c_int as isize) =
        -*(a.d).offset(3 as libc::c_int as isize);
    *(*(tmp.d).offset(1 as libc::c_int as isize)).offset(3 as libc::c_int as isize) =
        *(a.d).offset(2 as libc::c_int as isize);
    *(*(tmp.d).offset(2 as libc::c_int as isize)).offset(0 as libc::c_int as isize) =
        *(a.d).offset(2 as libc::c_int as isize);
    *(*(tmp.d).offset(2 as libc::c_int as isize)).offset(1 as libc::c_int as isize) =
        *(a.d).offset(3 as libc::c_int as isize);
    *(*(tmp.d).offset(2 as libc::c_int as isize)).offset(2 as libc::c_int as isize) =
        *(a.d).offset(0 as libc::c_int as isize);
    *(*(tmp.d).offset(2 as libc::c_int as isize)).offset(3 as libc::c_int as isize) =
        -*(a.d).offset(1 as libc::c_int as isize);
    *(*(tmp.d).offset(3 as libc::c_int as isize)).offset(0 as libc::c_int as isize) =
        *(a.d).offset(3 as libc::c_int as isize);
    *(*(tmp.d).offset(3 as libc::c_int as isize)).offset(1 as libc::c_int as isize) =
        -*(a.d).offset(2 as libc::c_int as isize);
    *(*(tmp.d).offset(3 as libc::c_int as isize)).offset(2 as libc::c_int as isize) =
        *(a.d).offset(1 as libc::c_int as isize);
    *(*(tmp.d).offset(3 as libc::c_int as isize)).offset(3 as libc::c_int as isize) =
        *(a.d).offset(0 as libc::c_int as isize);
    if (rc_matrix_times_col_vec(tmp, b, c) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_multiply, failed to multiply\n\0" as *const u8 as *const libc::c_char,
        );
        rc_matrix_free(&mut tmp);
        return -(1 as libc::c_int);
    }
    rc_matrix_free(&mut tmp);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_quaternion_multiply_array(
    mut a: *mut libc::c_double,
    mut b: *mut libc::c_double,
    mut c: *mut libc::c_double,
) -> libc::c_int {
    if (a.is_null() || b.is_null() || c.is_null()) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_quaternion_multiply_array, received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tmp: [[libc::c_double; 4]; 4] = [[0.; 4]; 4];
    tmp[0 as libc::c_int as usize][0 as libc::c_int as usize] = *a.offset(0 as libc::c_int as isize);
    tmp[0 as libc::c_int as usize][1 as libc::c_int as usize] = -*a.offset(1 as libc::c_int as isize);
    tmp[0 as libc::c_int as usize][2 as libc::c_int as usize] = -*a.offset(2 as libc::c_int as isize);
    tmp[0 as libc::c_int as usize][3 as libc::c_int as usize] = -*a.offset(3 as libc::c_int as isize);
    tmp[1 as libc::c_int as usize][0 as libc::c_int as usize] = *a.offset(1 as libc::c_int as isize);
    tmp[1 as libc::c_int as usize][1 as libc::c_int as usize] = *a.offset(0 as libc::c_int as isize);
    tmp[1 as libc::c_int as usize][2 as libc::c_int as usize] = -*a.offset(3 as libc::c_int as isize);
    tmp[1 as libc::c_int as usize][3 as libc::c_int as usize] = *a.offset(2 as libc::c_int as isize);
    tmp[2 as libc::c_int as usize][0 as libc::c_int as usize] = *a.offset(2 as libc::c_int as isize);
    tmp[2 as libc::c_int as usize][1 as libc::c_int as usize] = *a.offset(3 as libc::c_int as isize);
    tmp[2 as libc::c_int as usize][2 as libc::c_int as usize] = *a.offset(0 as libc::c_int as isize);
    tmp[2 as libc::c_int as usize][3 as libc::c_int as usize] = -*a.offset(1 as libc::c_int as isize);
    tmp[3 as libc::c_int as usize][0 as libc::c_int as usize] = *a.offset(3 as libc::c_int as isize);
    tmp[3 as libc::c_int as usize][1 as libc::c_int as usize] = -*a.offset(2 as libc::c_int as isize);
    tmp[3 as libc::c_int as usize][2 as libc::c_int as usize] = *a.offset(1 as libc::c_int as isize);
    tmp[3 as libc::c_int as usize][3 as libc::c_int as usize] = *a.offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        *c.offset(i as isize) = 0.0f64;
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            *c.offset(i as isize) += tmp[i as usize][j as usize] * *b.offset(j as isize);
            j += 1;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_quaternion_rotate(mut p: *mut rc_vector_t, mut q: rc_vector_t) -> libc::c_int {
    let mut conj: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut tmp: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if (q.initialized == 0 || (*p).initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_rotate_inplace, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (q.len != 4 as libc::c_int || (*p).len != 4 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_rotate_inplace, expected vector of length 4\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_quaternion_conjugate(q, &mut conj) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_rotate_inplace, failed to conjugate\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_quaternion_multiply(*p, conj, &mut tmp) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_rotate_inplace, failed to multiply\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut conj);
        return -(1 as libc::c_int);
    }
    if (rc_quaternion_multiply(q, tmp, p) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_rotate_inplace, failed to multiply\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut conj);
        rc_vector_free(&mut tmp);
        return -(1 as libc::c_int);
    }
    rc_vector_free(&mut conj);
    rc_vector_free(&mut tmp);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_quaternion_rotate_array(
    mut p: *mut libc::c_double,
    mut q: *mut libc::c_double,
) -> libc::c_int {
    let mut conj: [libc::c_double; 4] = [0.; 4];
    let mut tmp: [libc::c_double; 4] = [0.; 4];
    if (p.is_null() || q.is_null()) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_quaternion_rotate_array, received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    conj[0 as libc::c_int as usize] = *q.offset(0 as libc::c_int as isize);
    conj[1 as libc::c_int as usize] = -*q.offset(1 as libc::c_int as isize);
    conj[2 as libc::c_int as usize] = -*q.offset(2 as libc::c_int as isize);
    conj[3 as libc::c_int as usize] = -*q.offset(3 as libc::c_int as isize);
    rc_quaternion_multiply_array(p, conj.as_mut_ptr(), tmp.as_mut_ptr());
    rc_quaternion_multiply_array(q, tmp.as_mut_ptr(), p);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_quaternion_rotate_vector(mut v: *mut rc_vector_t, mut q: rc_vector_t) -> libc::c_int {
    let mut vq: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if (q.initialized == 0 || (*v).initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_rotate_vector, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (q.len != 4 as libc::c_int || (*v).len != 3 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_rotate_vector, incorrect length\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_vector_alloc(&mut vq, 4 as libc::c_int) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_rotate_vector, failed to alloc vector\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    *(vq.d).offset(0 as libc::c_int as isize) = 0.0f64;
    *(vq.d).offset(1 as libc::c_int as isize) = *((*v).d).offset(0 as libc::c_int as isize);
    *(vq.d).offset(2 as libc::c_int as isize) = *((*v).d).offset(1 as libc::c_int as isize);
    *(vq.d).offset(3 as libc::c_int as isize) = *((*v).d).offset(2 as libc::c_int as isize);
    if (rc_quaternion_rotate(&mut vq, q) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_rotate_vector, failed to rotate\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut vq);
        return -(1 as libc::c_int);
    }
    *((*v).d).offset(0 as libc::c_int as isize) = *(vq.d).offset(1 as libc::c_int as isize);
    *((*v).d).offset(1 as libc::c_int as isize) = *(vq.d).offset(2 as libc::c_int as isize);
    *((*v).d).offset(2 as libc::c_int as isize) = *(vq.d).offset(3 as libc::c_int as isize);
    rc_vector_free(&mut vq);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_quaternion_rotate_vector_array(
    mut v: *mut libc::c_double,
    mut q: *mut libc::c_double,
) -> libc::c_int {
    let mut vq: [libc::c_double; 4] = [0.; 4];
    if (v.is_null() || q.is_null()) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_quaternion_rotate_vector_array, received NULL pointer\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    vq[0 as libc::c_int as usize] = 0.0f64;
    vq[1 as libc::c_int as usize] = *v.offset(0 as libc::c_int as isize);
    vq[2 as libc::c_int as usize] = *v.offset(1 as libc::c_int as isize);
    vq[3 as libc::c_int as usize] = *v.offset(2 as libc::c_int as isize);
    rc_quaternion_rotate_array(vq.as_mut_ptr(), q);
    *v.offset(0 as libc::c_int as isize) = vq[1 as libc::c_int as usize];
    *v.offset(1 as libc::c_int as isize) = vq[2 as libc::c_int as usize];
    *v.offset(2 as libc::c_int as isize) = vq[3 as libc::c_int as usize];
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_quaternion_to_rotation_matrix(mut q: rc_vector_t, mut m: *mut rc_matrix_t) -> libc::c_int {
    let mut q0s: libc::c_double = 0.;
    let mut q1s: libc::c_double = 0.;
    let mut q2s: libc::c_double = 0.;
    let mut q3s: libc::c_double = 0.;
    if (q.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_to_rotation_matrix, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (q.len != 4 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_to_rotation_matrix, expected vector of length 4\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_matrix_alloc(m, 3 as libc::c_int, 3 as libc::c_int) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_quaternion_to_rotation_matrix, failed to alloc matrix\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    q0s = *(q.d).offset(0 as libc::c_int as isize) * *(q.d).offset(0 as libc::c_int as isize);
    q1s = *(q.d).offset(1 as libc::c_int as isize) * *(q.d).offset(1 as libc::c_int as isize);
    q2s = *(q.d).offset(2 as libc::c_int as isize) * *(q.d).offset(2 as libc::c_int as isize);
    q3s = *(q.d).offset(3 as libc::c_int as isize) * *(q.d).offset(3 as libc::c_int as isize);
    *(*((*m).d).offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize) = q0s + q1s - q2s - q3s;
    *(*((*m).d).offset(1 as libc::c_int as isize)).offset(1 as libc::c_int as isize) = q0s - q1s + q2s - q3s;
    *(*((*m).d).offset(2 as libc::c_int as isize)).offset(2 as libc::c_int as isize) = q0s - q1s - q2s + q3s;
    *(*((*m).d).offset(0 as libc::c_int as isize)).offset(1 as libc::c_int as isize) = 2.0f64
        * (*(q.d).offset(1 as libc::c_int as isize) * *(q.d).offset(2 as libc::c_int as isize)
            - *(q.d).offset(0 as libc::c_int as isize) * *(q.d).offset(3 as libc::c_int as isize));
    *(*((*m).d).offset(0 as libc::c_int as isize)).offset(2 as libc::c_int as isize) = 2.0f64
        * (*(q.d).offset(1 as libc::c_int as isize) * *(q.d).offset(3 as libc::c_int as isize)
            + *(q.d).offset(0 as libc::c_int as isize) * *(q.d).offset(2 as libc::c_int as isize));
    *(*((*m).d).offset(1 as libc::c_int as isize)).offset(2 as libc::c_int as isize) = 2.0f64
        * (*(q.d).offset(2 as libc::c_int as isize) * *(q.d).offset(3 as libc::c_int as isize)
            - *(q.d).offset(0 as libc::c_int as isize) * *(q.d).offset(1 as libc::c_int as isize));
    *(*((*m).d).offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize) = 2.0f64
        * (*(q.d).offset(1 as libc::c_int as isize) * *(q.d).offset(2 as libc::c_int as isize)
            + *(q.d).offset(0 as libc::c_int as isize) * *(q.d).offset(3 as libc::c_int as isize));
    *(*((*m).d).offset(2 as libc::c_int as isize)).offset(0 as libc::c_int as isize) = 2.0f64
        * (*(q.d).offset(1 as libc::c_int as isize) * *(q.d).offset(3 as libc::c_int as isize)
            - *(q.d).offset(0 as libc::c_int as isize) * *(q.d).offset(2 as libc::c_int as isize));
    *(*((*m).d).offset(2 as libc::c_int as isize)).offset(1 as libc::c_int as isize) = 2.0f64
        * (*(q.d).offset(2 as libc::c_int as isize) * *(q.d).offset(3 as libc::c_int as isize)
            + *(q.d).offset(0 as libc::c_int as isize) * *(q.d).offset(1 as libc::c_int as isize));
    return 0 as libc::c_int;
}
