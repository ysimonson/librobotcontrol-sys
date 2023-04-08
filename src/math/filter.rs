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
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn rc_ringbuf_alloc(buf: *mut rc_ringbuf_t, size: libc::c_int) -> libc::c_int;
    fn rc_ringbuf_free(buf: *mut rc_ringbuf_t) -> libc::c_int;
    fn rc_vector_free(v: *mut rc_vector_t) -> libc::c_int;
    fn rc_vector_zeros(v: *mut rc_vector_t, length: libc::c_int) -> libc::c_int;
    fn rc_vector_from_array(v: *mut rc_vector_t, ptr: *mut libc::c_double, length: libc::c_int) -> libc::c_int;
    fn rc_vector_duplicate(a: rc_vector_t, b: *mut rc_vector_t) -> libc::c_int;
    fn rc_ringbuf_reset(buf: *mut rc_ringbuf_t) -> libc::c_int;
    fn rc_ringbuf_insert(buf: *mut rc_ringbuf_t, val: libc::c_double) -> libc::c_int;
    fn rc_ringbuf_get_value(buf: *mut rc_ringbuf_t, position: libc::c_int) -> libc::c_double;
    static mut zero_tolerance: libc::c_double;
    fn rc_vector_alloc(v: *mut rc_vector_t, length: libc::c_int) -> libc::c_int;
    fn rc_poly_conv(a: rc_vector_t, b: rc_vector_t, c: *mut rc_vector_t) -> libc::c_int;
    fn rc_poly_power(a: rc_vector_t, n: libc::c_int, b: *mut rc_vector_t) -> libc::c_int;
    fn rc_poly_butter(N: libc::c_int, wc: libc::c_double, b: *mut rc_vector_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rc_vector_t {
    pub len: libc::c_int,
    pub d: *mut libc::c_double,
    pub initialized: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rc_ringbuf_t {
    pub d: *mut libc::c_double,
    pub size: libc::c_int,
    pub index: libc::c_int,
    pub initialized: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rc_filter_t {
    pub order: libc::c_int,
    pub dt: libc::c_double,
    pub gain: libc::c_double,
    pub num: rc_vector_t,
    pub den: rc_vector_t,
    pub sat_en: libc::c_int,
    pub sat_min: libc::c_double,
    pub sat_max: libc::c_double,
    pub sat_flag: libc::c_int,
    pub ss_en: libc::c_int,
    pub ss_steps: libc::c_double,
    pub in_buf: rc_ringbuf_t,
    pub out_buf: rc_ringbuf_t,
    pub newest_input: libc::c_double,
    pub newest_output: libc::c_double,
    pub step: uint64_t,
    pub initialized: libc::c_int,
}
unsafe extern "C" fn __print_poly_z(mut v: rc_vector_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    static mut super_0: [*mut libc::c_char; 10] = [
        b"\xE2\x81\xB0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xC2\xB9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xC2\xB2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xC2\xB3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xE2\x81\xB4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xE2\x81\xB5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xE2\x81\xB6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xE2\x81\xB7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xE2\x81\xB8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xE2\x81\xB9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    if (v.len > 10 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_print, filter order must be <=10\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < v.len - 2 as libc::c_int {
        printf(
            b"%7.4fz%s + \0" as *const u8 as *const libc::c_char,
            *(v.d).offset(i as isize),
            super_0[(v.len - i - 1 as libc::c_int) as usize],
        );
        i += 1;
    }
    if v.len >= 2 as libc::c_int {
        printf(
            b"%7.4fz  + \0" as *const u8 as *const libc::c_char,
            *(v.d).offset((v.len - 2 as libc::c_int) as isize),
        );
    }
    printf(
        b"%7.4f\n\0" as *const u8 as *const libc::c_char,
        *(v.d).offset((v.len - 1 as libc::c_int) as isize),
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_empty() -> rc_filter_t {
    let mut f: rc_filter_t = {
        let mut init = rc_filter_t {
            order: 0 as libc::c_int,
            dt: 0.0f64,
            gain: 1.0f64,
            num: {
                let mut init = rc_vector_t {
                    len: 0 as libc::c_int,
                    d: 0 as *mut libc::c_double,
                    initialized: 0 as libc::c_int,
                };
                init
            },
            den: {
                let mut init = rc_vector_t {
                    len: 0 as libc::c_int,
                    d: 0 as *mut libc::c_double,
                    initialized: 0 as libc::c_int,
                };
                init
            },
            sat_en: 0 as libc::c_int,
            sat_min: 0.0f64,
            sat_max: 0.0f64,
            sat_flag: 0 as libc::c_int,
            ss_en: 0 as libc::c_int,
            ss_steps: 0 as libc::c_int as libc::c_double,
            in_buf: {
                let mut init = rc_ringbuf_t {
                    d: 0 as *mut libc::c_double,
                    size: 0 as libc::c_int,
                    index: 0 as libc::c_int,
                    initialized: 0 as libc::c_int,
                };
                init
            },
            out_buf: {
                let mut init = rc_ringbuf_t {
                    d: 0 as *mut libc::c_double,
                    size: 0 as libc::c_int,
                    index: 0 as libc::c_int,
                    initialized: 0 as libc::c_int,
                };
                init
            },
            newest_input: 0.0f64,
            newest_output: 0.0f64,
            step: 0 as libc::c_int as uint64_t,
            initialized: 0 as libc::c_int,
        };
        init
    };
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_alloc(
    mut f: *mut rc_filter_t,
    mut num: rc_vector_t,
    mut den: rc_vector_t,
    mut dt: libc::c_double,
) -> libc::c_int {
    if (dt <= 0.0f64) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_alloc, dt must be >0\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (num.initialized == 0 || den.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_alloc, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (num.len > den.len) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_alloc, improper transfer function\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (fabs(*(den.d).offset(0 as libc::c_int as isize)) < zero_tolerance) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_alloc, first coefficient in denominator is 0\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_filter_free(f);
    if (rc_vector_duplicate(num, &mut (*f).num) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_alloc, failed to duplicate numerator\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_vector_duplicate(den, &mut (*f).den) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_alloc, failed to duplicate denominator\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut (*f).num);
        return -(1 as libc::c_int);
    }
    let mut buflen: libc::c_int = den.len;
    if buflen < 2 as libc::c_int {
        buflen = 2 as libc::c_int;
    }
    if (rc_ringbuf_alloc(&mut (*f).in_buf, buflen) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_alloc, failed to allocate ring buffer\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut (*f).num);
        rc_vector_free(&mut (*f).den);
        return -(1 as libc::c_int);
    }
    if (rc_ringbuf_alloc(&mut (*f).out_buf, buflen) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_alloc, failed to allocate ring buffer\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut (*f).num);
        rc_vector_free(&mut (*f).den);
        rc_ringbuf_free(&mut (*f).in_buf);
        return -(1 as libc::c_int);
    }
    (*f).dt = dt;
    (*f).order = den.len - 1 as libc::c_int;
    (*f).initialized = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_alloc_from_arrays(
    mut f: *mut rc_filter_t,
    mut dt: libc::c_double,
    mut num: *mut libc::c_double,
    mut numlen: libc::c_int,
    mut den: *mut libc::c_double,
    mut denlen: libc::c_int,
) -> libc::c_int {
    if (numlen < 1 as libc::c_int || denlen < 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_alloc_from_arrays, numlen & denlen must be >=1\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (numlen > denlen) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_alloc_from_arrays, improper transfer function\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (num.is_null() || den.is_null() || f.is_null()) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_alloc_from_arrays, received null pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (dt < 0.0f64) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_alloc_from_arrays, dt must be >0\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (fabs(*den.offset(0 as libc::c_int as isize)) < zero_tolerance) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_alloc_from_arrays, first coefficient in denominator is 0\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_filter_free(f);
    if (rc_vector_from_array(&mut (*f).num, num, numlen) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_alloc_from_arrays, failed to alloc vector\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_vector_from_array(&mut (*f).den, den, denlen) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_alloc_from_arrays, failed to alloc vector\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut (*f).num);
        return -(1 as libc::c_int);
    }
    if (rc_ringbuf_alloc(&mut (*f).in_buf, denlen) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_alloc, failed to allocate ring buffer\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut (*f).num);
        rc_vector_free(&mut (*f).den);
        return -(1 as libc::c_int);
    }
    if (rc_ringbuf_alloc(&mut (*f).out_buf, denlen) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_alloc, failed to allocate ring buffer\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut (*f).num);
        rc_vector_free(&mut (*f).den);
        rc_ringbuf_free(&mut (*f).in_buf);
        return -(1 as libc::c_int);
    }
    (*f).dt = dt;
    (*f).order = denlen - 1 as libc::c_int;
    (*f).initialized = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_duplicate(mut f: *mut rc_filter_t, mut old: rc_filter_t) -> libc::c_int {
    if (old.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_duplicate, old filter not initialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_filter_alloc(f, old.num, old.den, old.dt) != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_duplicate, failed to alloc memory\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*f).gain = old.gain;
    (*f).sat_en = old.sat_en;
    (*f).sat_min = old.sat_min;
    (*f).sat_max = old.sat_max;
    (*f).ss_en = old.ss_en;
    (*f).ss_steps = old.ss_steps;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_free(mut f: *mut rc_filter_t) -> libc::c_int {
    let mut new: rc_filter_t = {
        let mut init = rc_filter_t {
            order: 0 as libc::c_int,
            dt: 0.0f64,
            gain: 1.0f64,
            num: {
                let mut init = rc_vector_t {
                    len: 0 as libc::c_int,
                    d: 0 as *mut libc::c_double,
                    initialized: 0 as libc::c_int,
                };
                init
            },
            den: {
                let mut init = rc_vector_t {
                    len: 0 as libc::c_int,
                    d: 0 as *mut libc::c_double,
                    initialized: 0 as libc::c_int,
                };
                init
            },
            sat_en: 0 as libc::c_int,
            sat_min: 0.0f64,
            sat_max: 0.0f64,
            sat_flag: 0 as libc::c_int,
            ss_en: 0 as libc::c_int,
            ss_steps: 0 as libc::c_int as libc::c_double,
            in_buf: {
                let mut init = rc_ringbuf_t {
                    d: 0 as *mut libc::c_double,
                    size: 0 as libc::c_int,
                    index: 0 as libc::c_int,
                    initialized: 0 as libc::c_int,
                };
                init
            },
            out_buf: {
                let mut init = rc_ringbuf_t {
                    d: 0 as *mut libc::c_double,
                    size: 0 as libc::c_int,
                    index: 0 as libc::c_int,
                    initialized: 0 as libc::c_int,
                };
                init
            },
            newest_input: 0.0f64,
            newest_output: 0.0f64,
            step: 0 as libc::c_int as uint64_t,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if f.is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_free, received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_ringbuf_free(&mut (*f).in_buf);
    rc_ringbuf_free(&mut (*f).out_buf);
    rc_vector_free(&mut (*f).num);
    rc_vector_free(&mut (*f).den);
    *f = new;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_march(mut f: *mut rc_filter_t, mut new_input: libc::c_double) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut rel_deg: libc::c_int = 0;
    let mut tmp1: libc::c_double = 0.0f64;
    let mut tmp2: libc::c_double = 0.0f64;
    let mut new_out: libc::c_double = 0.;
    if ((*f).initialized == 0) as libc::c_int as libc::c_long != 0 {
        printf(b"ERROR in rc_filter_march, filter uninitialized\n\0" as *const u8 as *const libc::c_char);
        return -1.0f64;
    }
    rc_ringbuf_insert(&mut (*f).in_buf, new_input);
    (*f).newest_input = new_input;
    rel_deg = (*f).den.len - (*f).num.len;
    i = 0 as libc::c_int;
    while i < (*f).num.len {
        tmp1 += *((*f).num.d).offset(i as isize) * rc_ringbuf_get_value(&mut (*f).in_buf, i + rel_deg);
        i += 1;
    }
    if fabs((*f).gain - 1.0f64) > zero_tolerance {
        tmp1 = tmp1 * (*f).gain;
    }
    i = 0 as libc::c_int;
    while i < (*f).order {
        tmp2 -= *((*f).den.d).offset((i + 1 as libc::c_int) as isize) * rc_ringbuf_get_value(&mut (*f).out_buf, i);
        i += 1;
    }
    new_out = tmp2 + tmp1;
    if fabs(*((*f).den.d).offset(0 as libc::c_int as isize) - 1.0f64) > zero_tolerance {
        new_out /= *((*f).den.d).offset(0 as libc::c_int as isize);
    }
    if (*f).ss_en != 0 && ((*f).step as libc::c_double) < (*f).ss_steps {
        let mut a: libc::c_double = (*f).sat_max * ((*f).step as libc::c_double / (*f).ss_steps);
        let mut b: libc::c_double = (*f).sat_min * ((*f).step as libc::c_double / (*f).ss_steps);
        if new_out > a {
            new_out = a;
        }
        if new_out < b {
            new_out = b;
        }
    }
    if (*f).sat_en != 0 {
        if new_out > (*f).sat_max {
            new_out = (*f).sat_max;
            (*f).sat_flag = 1 as libc::c_int;
        } else if new_out < (*f).sat_min {
            new_out = (*f).sat_min;
            (*f).sat_flag = 1 as libc::c_int;
        } else {
            (*f).sat_flag = 0 as libc::c_int;
        }
    }
    (*f).newest_output = new_out;
    rc_ringbuf_insert(&mut (*f).out_buf, new_out);
    (*f).step = ((*f).step).wrapping_add(1);
    return new_out;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_reset(mut f: *mut rc_filter_t) -> libc::c_int {
    if ((*f).initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_reset, filter uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_ringbuf_reset(&mut (*f).in_buf);
    rc_ringbuf_reset(&mut (*f).out_buf);
    (*f).newest_input = 0.0f64;
    (*f).newest_output = 0.0f64;
    (*f).sat_flag = 0 as libc::c_int;
    (*f).step = 0 as libc::c_int as uint64_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_print(mut f: rc_filter_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (f.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_print, filter not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (f.order > 9 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_print, filter order must be <=10\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    printf(b"order: %d\n\0" as *const u8 as *const libc::c_char, f.order);
    printf(b"timestep dt: %0.4f\n\0" as *const u8 as *const libc::c_char, f.dt);
    __print_poly_z(f.num);
    printf(b"--------\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < f.order {
        printf(b"------------\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    __print_poly_z(f.den);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_enable_saturation(
    mut f: *mut rc_filter_t,
    mut min: libc::c_double,
    mut max: libc::c_double,
) -> libc::c_int {
    if ((*f).initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_enable_saturation, filter uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (min > max) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERORR in rc_filter_enable_saturation, max must be >= min\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*f).sat_en = 1 as libc::c_int;
    (*f).sat_min = min;
    (*f).sat_max = max;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_get_saturation_flag(mut f: *mut rc_filter_t) -> libc::c_int {
    if ((*f).initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_get_saturation_flag, filter uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return (*f).sat_flag;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_enable_soft_start(
    mut f: *mut rc_filter_t,
    mut seconds: libc::c_double,
) -> libc::c_int {
    if ((*f).initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_enable_soft_start, filter uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (seconds <= 0.0f64) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_enable_soft_start, seconds must be >=0\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if ((*f).sat_en == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_enable_soft_start, saturation must be enabled first\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*f).ss_en = 1 as libc::c_int;
    (*f).ss_steps = seconds / (*f).dt;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_previous_input(mut f: *mut rc_filter_t, mut steps: libc::c_int) -> libc::c_double {
    if ((*f).initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_previous_input, filter uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -1.0f64;
    }
    return rc_ringbuf_get_value(&mut (*f).in_buf, steps);
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_previous_output(mut f: *mut rc_filter_t, mut steps: libc::c_int) -> libc::c_double {
    if ((*f).initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_previous_output, filter uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -1.0f64;
    }
    return rc_ringbuf_get_value(&mut (*f).out_buf, steps);
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_prefill_inputs(mut f: *mut rc_filter_t, mut in_0: libc::c_double) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if ((*f).initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_prefill_inputs, filter uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i <= (*f).order {
        rc_ringbuf_insert(&mut (*f).in_buf, in_0);
        i += 1;
    }
    (*f).newest_input = in_0;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_prefill_outputs(mut f: *mut rc_filter_t, mut out: libc::c_double) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if ((*f).initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_prefill_outputs, filter uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i <= (*f).order {
        rc_ringbuf_insert(&mut (*f).out_buf, out);
        i += 1;
    }
    (*f).newest_output = out;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_multiply(
    mut f1: rc_filter_t,
    mut f2: rc_filter_t,
    mut f3: *mut rc_filter_t,
) -> libc::c_int {
    let mut newnum: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut newden: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if (f1.initialized == 0 || f2.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_multiply, filter uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (fabs(f1.dt - f2.dt) > zero_tolerance) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_multiply, timestep dt must match\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_poly_conv(f1.num, f2.num, &mut newnum) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_multiply, failed to polyconv\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_poly_conv(f1.den, f2.den, &mut newden) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_multiply, failed to polyconv\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut newnum);
        return -(1 as libc::c_int);
    }
    if (rc_filter_alloc(f3, newnum, newden, f1.dt) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_multiply, failed to alloc filter\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*f3).gain = f1.gain * f2.gain;
    rc_vector_free(&mut newnum);
    rc_vector_free(&mut newden);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_multiply_three(
    mut f1: rc_filter_t,
    mut f2: rc_filter_t,
    mut f3: rc_filter_t,
    mut out: *mut rc_filter_t,
) -> libc::c_int {
    let mut newnum: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut newden: rc_vector_t = {
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
    if (f1.initialized == 0 || f2.initialized == 0 || f3.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_multiply_three, filter uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (fabs(f1.dt - f2.dt) > zero_tolerance || fabs(f2.dt - f3.dt) > zero_tolerance) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"ERROR in rc_filter_multiply_three, timestep dt must match\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_poly_conv(f1.num, f2.num, &mut tmp) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_multiply_three, failed to polyconv\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_poly_conv(f3.num, tmp, &mut newnum) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_multiply_three, failed to polyconv\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut tmp);
        return -(1 as libc::c_int);
    }
    if (rc_poly_conv(f1.den, f2.den, &mut tmp) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_multiply_three, failed to polyconv\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut newnum);
        rc_vector_free(&mut tmp);
        return -(1 as libc::c_int);
    }
    if (rc_poly_conv(tmp, f3.den, &mut newden) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_multiply_three, failed to polyconv\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut newnum);
        rc_vector_free(&mut tmp);
        return -(1 as libc::c_int);
    }
    if (rc_filter_alloc(out, newnum, newden, f1.dt) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_multiply_three, failed to alloc filter\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut tmp);
        rc_vector_free(&mut newnum);
        rc_vector_free(&mut newden);
        return -(1 as libc::c_int);
    }
    (*out).gain = f1.gain * f2.gain * f3.gain;
    rc_vector_free(&mut tmp);
    rc_vector_free(&mut newnum);
    rc_vector_free(&mut newden);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_c2d_tustin(
    mut f: *mut rc_filter_t,
    mut dt: libc::c_double,
    mut num: rc_vector_t,
    mut den: rc_vector_t,
    mut w: libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut a: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut A0: libc::c_double = 0.;
    let mut numZ: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut denZ: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut p1: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut p2: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut temp: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut v1: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut v2: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if (num.initialized == 0 || den.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_c2d_tustin, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (dt <= 0.0f64) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_c2d_tustin, dt must be positive\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (w > 3.14159265358979323846f64 / dt) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_c2d_tustin, w larger than nyquist frequency\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    a = 2.0f64 * (1.0f64 - cos(w * dt)) / (w * dt * sin(w * dt));
    c = 2.0f64 / (a * dt);
    m = num.len - 1 as libc::c_int;
    n = den.len - 1 as libc::c_int;
    rc_vector_zeros(&mut numZ, n + 1 as libc::c_int);
    rc_vector_zeros(&mut denZ, n + 1 as libc::c_int);
    rc_vector_alloc(&mut p1, 2 as libc::c_int);
    *(p1.d).offset(0 as libc::c_int as isize) = 1.0f64;
    *(p1.d).offset(1 as libc::c_int as isize) = -1.0f64;
    rc_vector_alloc(&mut p2, 2 as libc::c_int);
    *(p2.d).offset(0 as libc::c_int as isize) = 1.0f64;
    *(p2.d).offset(1 as libc::c_int as isize) = 1.0f64;
    i = 0 as libc::c_int;
    while i <= m {
        rc_poly_power(p1, m - i, &mut v1);
        rc_poly_power(p2, n - m + i, &mut v2);
        rc_poly_conv(v1, v2, &mut temp);
        j = 0 as libc::c_int;
        while j < n + 1 as libc::c_int {
            *(numZ.d).offset(j as isize) +=
                *(num.d).offset(i as isize) * pow(c, (m - i) as libc::c_double) * *(temp.d).offset(j as isize);
            j += 1;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i <= n {
        rc_poly_power(p1, n - i, &mut v1);
        rc_poly_power(p2, i, &mut v2);
        rc_poly_conv(v1, v2, &mut temp);
        j = 0 as libc::c_int;
        while j < n + 1 as libc::c_int {
            *(denZ.d).offset(j as isize) +=
                *(den.d).offset(i as isize) * pow(c, (n - i) as libc::c_double) * *(temp.d).offset(j as isize);
            j += 1;
        }
        i += 1;
    }
    A0 = *(denZ.d).offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < n + 1 as libc::c_int {
        *(numZ.d).offset(i as isize) = *(numZ.d).offset(i as isize) / A0;
        *(denZ.d).offset(i as isize) = *(denZ.d).offset(i as isize) / A0;
        i += 1;
    }
    rc_vector_free(&mut p1);
    rc_vector_free(&mut p2);
    rc_vector_free(&mut temp);
    rc_vector_free(&mut v1);
    rc_vector_free(&mut v2);
    if (rc_filter_alloc(f, numZ, denZ, dt) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_c2d_tustin, failed to alloc filter\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut numZ);
        rc_vector_free(&mut denZ);
        return -(1 as libc::c_int);
    }
    rc_vector_free(&mut numZ);
    rc_vector_free(&mut denZ);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_normalize(mut f: *mut rc_filter_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut val: libc::c_double = 0.;
    if ((*f).initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_normalize, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    val = *((*f).den.d).offset(0 as libc::c_int as isize);
    if (fabs(val) < zero_tolerance) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_normalize, leading coefficient is 0\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if fabs(val - 1.0f64) < zero_tolerance {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*f).num.len {
        *((*f).num.d).offset(i as isize) /= val;
        i += 1;
    }
    i = 1 as libc::c_int;
    while i < (*f).den.len {
        *((*f).den.d).offset(i as isize) /= val;
        i += 1;
    }
    *((*f).den.d).offset(0 as libc::c_int as isize) = 1.0f64;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_first_order_lowpass(
    mut f: *mut rc_filter_t,
    mut dt: libc::c_double,
    mut time_constant: libc::c_double,
) -> libc::c_int {
    let mut c: libc::c_double = 0.;
    let mut num: [libc::c_double; 2] = [0.; 2];
    let mut den: [libc::c_double; 2] = [0.; 2];
    if (time_constant <= 0.0f64) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_first_order_lowpass, time constant must be >0\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (dt <= 0.0f64) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_first_order_lowpass, dt must be >0\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    c = dt / time_constant;
    num[0 as libc::c_int as usize] = c;
    num[1 as libc::c_int as usize] = 0.0f64;
    den[0 as libc::c_int as usize] = 1.0f64;
    den[1 as libc::c_int as usize] = c - 1.0f64;
    if (rc_filter_alloc_from_arrays(
        f,
        dt,
        num.as_mut_ptr(),
        2 as libc::c_int,
        den.as_mut_ptr(),
        2 as libc::c_int,
    ) != 0) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"ERROR in rc_filter_first_order_lowpass, failed to alloc filter\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_first_order_highpass(
    mut f: *mut rc_filter_t,
    mut dt: libc::c_double,
    mut time_constant: libc::c_double,
) -> libc::c_int {
    let mut c: libc::c_double = 0.;
    let mut num: [libc::c_double; 2] = [0.; 2];
    let mut den: [libc::c_double; 2] = [0.; 2];
    if (time_constant <= 0.0f64) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_first_order_highpass, time constant must be >0\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (dt <= 0.0f64) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_first_order_highpass, dt must be >0\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    c = dt / time_constant;
    num[0 as libc::c_int as usize] = 1.0f64 - c;
    num[1 as libc::c_int as usize] = c - 1.0f64;
    den[0 as libc::c_int as usize] = 1.0f64;
    den[1 as libc::c_int as usize] = c - 1.0f64;
    if (rc_filter_alloc_from_arrays(
        f,
        dt,
        num.as_mut_ptr(),
        2 as libc::c_int,
        den.as_mut_ptr(),
        2 as libc::c_int,
    ) != 0) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"ERROR in rc_filter_first_order_highpass, failed to alloc filter\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_butterworth_lowpass(
    mut f: *mut rc_filter_t,
    mut order: libc::c_int,
    mut dt: libc::c_double,
    mut wc: libc::c_double,
) -> libc::c_int {
    let mut num: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut den: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if (order < 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_butterworth_lowpass, order must be >=1\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_poly_butter(order, wc, &mut den) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_butterworth_lowpass, failed to find butterworth polynomial\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_vector_alloc(&mut num, 1 as libc::c_int);
    *(num.d).offset(0 as libc::c_int as isize) = 1.0f64;
    if (rc_filter_c2d_tustin(f, dt, num, den, wc) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_butterworth_lowpass, failed to c2d_tustin\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut num);
        rc_vector_free(&mut den);
        return -(1 as libc::c_int);
    }
    rc_vector_free(&mut num);
    rc_vector_free(&mut den);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_butterworth_highpass(
    mut f: *mut rc_filter_t,
    mut order: libc::c_int,
    mut dt: libc::c_double,
    mut wc: libc::c_double,
) -> libc::c_int {
    let mut num: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut den: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if (order < 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_butterworth_highpass, order must be >=1\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_poly_butter(order, wc, &mut den) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_butterworth_highpass, failed to find butterwoth polynomial\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_vector_zeros(&mut num, order + 1 as libc::c_int);
    *(num.d).offset(0 as libc::c_int as isize) = 1.0f64;
    if (rc_filter_c2d_tustin(f, dt, num, den, wc) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_butterworth_highpass, failed to c2d_tustin\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut num);
        rc_vector_free(&mut den);
        return -(1 as libc::c_int);
    }
    rc_vector_free(&mut num);
    rc_vector_free(&mut den);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_moving_average(
    mut f: *mut rc_filter_t,
    mut samples: libc::c_int,
    mut dt: libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut num: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut den: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if (samples < 2 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_moving_average, samples must be >=2\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (dt <= 0.0f64) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_moving_average, dt must be >0\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_vector_alloc(&mut num, samples);
    rc_vector_alloc(&mut den, samples);
    i = 0 as libc::c_int;
    while i < samples {
        *(num.d).offset(i as isize) = 1.0f64 / samples as libc::c_double;
        *(den.d).offset(i as isize) = 0.0f64;
        i += 1;
    }
    *(den.d).offset(0 as libc::c_int as isize) = 1.0f64;
    if (rc_filter_alloc(f, num, den, dt) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_moving_average, failed to alloc filter\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut num);
        rc_vector_free(&mut den);
        return -(1 as libc::c_int);
    }
    (*f).dt = dt;
    rc_vector_free(&mut num);
    rc_vector_free(&mut den);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_integrator(mut f: *mut rc_filter_t, mut dt: libc::c_double) -> libc::c_int {
    let mut num: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut den: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if (dt <= 0.0f64) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_integrator, dt must be >0\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_vector_alloc(&mut num, 1 as libc::c_int);
    rc_vector_alloc(&mut den, 2 as libc::c_int);
    *(num.d).offset(0 as libc::c_int as isize) = dt;
    *(den.d).offset(0 as libc::c_int as isize) = 1.0f64;
    *(den.d).offset(1 as libc::c_int as isize) = -1.0f64;
    if (rc_filter_alloc(f, num, den, dt) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_integrator, failed to alloc filter\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut num);
        rc_vector_free(&mut den);
        return -(1 as libc::c_int);
    }
    rc_vector_free(&mut num);
    rc_vector_free(&mut den);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_double_integrator(mut f: *mut rc_filter_t, mut dt: libc::c_double) -> libc::c_int {
    let mut num: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut den: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if (dt <= 0.0f64) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_double_integrator, dt must be >0\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_vector_alloc(&mut num, 1 as libc::c_int);
    rc_vector_alloc(&mut den, 3 as libc::c_int);
    *(num.d).offset(0 as libc::c_int as isize) = dt * dt;
    *(den.d).offset(0 as libc::c_int as isize) = 1.0f64;
    *(den.d).offset(1 as libc::c_int as isize) = -2.0f64;
    *(den.d).offset(2 as libc::c_int as isize) = 1.0f64;
    if (rc_filter_alloc(f, num, den, dt) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_double_integrator, failed to alloc filter\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut num);
        rc_vector_free(&mut den);
        return -(1 as libc::c_int);
    }
    rc_vector_free(&mut num);
    rc_vector_free(&mut den);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_pid(
    mut f: *mut rc_filter_t,
    mut kp: libc::c_double,
    mut ki: libc::c_double,
    mut kd: libc::c_double,
    mut Tf: libc::c_double,
    mut dt: libc::c_double,
) -> libc::c_int {
    let mut num: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut den: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if (dt < 0.0f64) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_pid, dt must be >0\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (Tf <= dt / 2 as libc::c_int as libc::c_double) as libc::c_int as libc::c_long != 0 {
        printf(b"ERROR in rc_filter_pid, Tf must be > dt/2 for stability\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if fabs(ki) < zero_tolerance && fabs(kd) > zero_tolerance {
        rc_vector_alloc(&mut num, 2 as libc::c_int);
        rc_vector_alloc(&mut den, 2 as libc::c_int);
        *(num.d).offset(0 as libc::c_int as isize) = (kp * Tf + kd) / Tf;
        *(num.d).offset(1 as libc::c_int as isize) = (kp * (dt - Tf) - kd) / Tf;
        *(den.d).offset(0 as libc::c_int as isize) = 1.0f64;
        *(den.d).offset(1 as libc::c_int as isize) = -(Tf - dt) / Tf;
    } else if fabs(ki) > zero_tolerance && fabs(kd) < zero_tolerance {
        rc_vector_alloc(&mut num, 2 as libc::c_int);
        rc_vector_alloc(&mut den, 2 as libc::c_int);
        *(num.d).offset(0 as libc::c_int as isize) = kp;
        *(num.d).offset(1 as libc::c_int as isize) = ki * dt - kp;
        *(den.d).offset(0 as libc::c_int as isize) = 1.0f64;
        *(den.d).offset(1 as libc::c_int as isize) = -1.0f64;
    } else if fabs(ki) < zero_tolerance && fabs(kd) < zero_tolerance {
        rc_vector_alloc(&mut num, 1 as libc::c_int);
        rc_vector_alloc(&mut den, 1 as libc::c_int);
        *(num.d).offset(0 as libc::c_int as isize) = kp;
        *(den.d).offset(0 as libc::c_int as isize) = 1.0f64;
    } else {
        rc_vector_alloc(&mut num, 3 as libc::c_int);
        rc_vector_alloc(&mut den, 3 as libc::c_int);
        *(num.d).offset(0 as libc::c_int as isize) = (kp * Tf + kd) / Tf;
        *(num.d).offset(1 as libc::c_int as isize) = (ki * dt * Tf + kp * (dt - Tf) - kp * Tf - 2.0f64 * kd) / Tf;
        *(num.d).offset(2 as libc::c_int as isize) = ((ki * dt - kp) * (dt - Tf) + kd) / Tf;
        *(den.d).offset(0 as libc::c_int as isize) = 1.0f64;
        *(den.d).offset(1 as libc::c_int as isize) = (dt - 2.0f64 * Tf) / Tf;
        *(den.d).offset(2 as libc::c_int as isize) = (Tf - dt) / Tf;
    }
    if (rc_filter_alloc(f, num, den, dt) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_pid, failed to alloc filter\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut num);
        rc_vector_free(&mut den);
        return -(1 as libc::c_int);
    }
    rc_vector_free(&mut num);
    rc_vector_free(&mut den);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_filter_third_order_complement(
    mut lp: *mut rc_filter_t,
    mut hp: *mut rc_filter_t,
    mut freq: libc::c_double,
    mut damp: libc::c_double,
    mut dt: libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut den: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut numlp: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut numhp: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if (freq <= 0.0f64) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_third_order_complement, freq must be >0\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (damp <= 0.0f64) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_third_order_complement, damp must be >0\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (dt <= 0.0f64) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_filter_third_order_complement, dt must be >0\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    a = freq + 2.0f64 * damp * freq;
    b = (1.0f64 + 2.0f64 * damp) * freq * freq;
    c = freq * freq * freq;
    t = 2.0f64 / dt;
    rc_vector_alloc(&mut den, 4 as libc::c_int);
    *(den.d).offset(0 as libc::c_int as isize) = t * t * t + a * t * t + b * t + c;
    *(den.d).offset(1 as libc::c_int as isize) = -3.0f64 * t * t * t - a * t * t + b * t + 3.0f64 * c;
    *(den.d).offset(2 as libc::c_int as isize) = 3.0f64 * t * t * t - a * t * t - b * t + 3.0f64 * c;
    *(den.d).offset(3 as libc::c_int as isize) = -t * t * t + a * t * t - b * t + c;
    rc_vector_alloc(&mut numlp, 4 as libc::c_int);
    *(numlp.d).offset(0 as libc::c_int as isize) = b * t + c;
    *(numlp.d).offset(1 as libc::c_int as isize) = b * t + 3.0f64 * c;
    *(numlp.d).offset(2 as libc::c_int as isize) = -b * t + 3.0f64 * c;
    *(numlp.d).offset(3 as libc::c_int as isize) = -b * t + c;
    if rc_filter_alloc(lp, numlp, den, dt) != 0 {
        return -(1 as libc::c_int);
    }
    if rc_filter_normalize(lp) != 0 {
        return -(1 as libc::c_int);
    }
    rc_vector_alloc(&mut numhp, 4 as libc::c_int);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        *(numhp.d).offset(i as isize) = *((*lp).den.d).offset(i as isize) - *((*lp).num.d).offset(i as isize);
        i += 1;
    }
    rc_filter_alloc(hp, numhp, (*lp).den, dt);
    rc_vector_free(&mut den);
    rc_vector_free(&mut numlp);
    rc_vector_free(&mut numhp);
    return 0 as libc::c_int;
}
