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
    fn rc_vector_empty() -> rc_vector_t;
    fn rc_vector_alloc(v: *mut rc_vector_t, length: libc::c_int) -> libc::c_int;
    fn rc_vector_free(v: *mut rc_vector_t) -> libc::c_int;
    fn rc_vector_zeros(v: *mut rc_vector_t, length: libc::c_int) -> libc::c_int;
    fn rc_vector_ones(v: *mut rc_vector_t, length: libc::c_int) -> libc::c_int;
    fn rc_vector_duplicate(a: rc_vector_t, b: *mut rc_vector_t) -> libc::c_int;
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
pub unsafe extern "C" fn rc_poly_print(mut v: rc_vector_t) -> libc::c_int {
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
    if (v.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_print, vector not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (v.len > 10 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_print, vector length must be <=10\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < v.len - 2 as libc::c_int {
        printf(
            b"%7.4fx%s + \0" as *const u8 as *const libc::c_char,
            *(v.d).offset(i as isize),
            super_0[(v.len - i - 1 as libc::c_int) as usize],
        );
        i += 1;
    }
    if v.len >= 2 as libc::c_int {
        printf(
            b"%7.4fx  + \0" as *const u8 as *const libc::c_char,
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
pub unsafe extern "C" fn rc_poly_conv(mut a: rc_vector_t, mut b: rc_vector_t, mut c: *mut rc_vector_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (a.initialized == 0 || b.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_conv, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_vector_zeros(c, a.len + b.len - 1 as libc::c_int) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_conv, failed to alloc vector\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < a.len {
        j = 0 as libc::c_int;
        while j < b.len {
            *((*c).d).offset((i + j) as isize) += *(a.d).offset(i as isize) * *(b.d).offset(j as isize);
            j += 1;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_poly_power(mut a: rc_vector_t, mut n: libc::c_int, mut b: *mut rc_vector_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tmp: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if (a.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_power, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (n < 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_power, negative exponents not allowed\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if n == 0 as libc::c_int {
        if (rc_vector_alloc(b, 1 as libc::c_int) != 0) as libc::c_int as libc::c_long != 0 {
            fprintf(
                stderr,
                b"ERROR in rc_poly_power, failed to alloc vector\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        *((*b).d).offset(0 as libc::c_int as isize) = 1.0f32 as libc::c_double;
        return 0 as libc::c_int;
    }
    if (rc_vector_duplicate(a, b) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_power, failed to duplicate vector\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if n == 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    i = 2 as libc::c_int;
    while i <= n {
        if (rc_poly_conv(a, *b, &mut tmp) != 0) as libc::c_int as libc::c_long != 0 {
            fprintf(
                stderr,
                b"ERROR in rc_poly_power, failed to poly_conv\n\0" as *const u8 as *const libc::c_char,
            );
            rc_vector_free(&mut tmp);
            rc_vector_free(b);
            return -(1 as libc::c_int);
        }
        rc_vector_free(b);
        *b = tmp;
        tmp = rc_vector_empty();
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_poly_add(mut a: rc_vector_t, mut b: rc_vector_t, mut c: *mut rc_vector_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut longest: rc_vector_t = rc_vector_t {
        len: 0,
        d: 0 as *mut libc::c_double,
        initialized: 0,
    };
    if (a.initialized == 0 || b.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_add, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if a.len > b.len {
        longest = a;
    } else {
        longest = b;
        b = a;
    }
    if (rc_vector_duplicate(longest, c) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_add, failed to duplicate vector\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    diff = (*c).len - b.len;
    i = diff;
    while i < (*c).len {
        *((*c).d).offset(i as isize) += *(b.d).offset((i - diff) as isize);
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_poly_add_inplace(mut a: *mut rc_vector_t, mut b: rc_vector_t) -> libc::c_int {
    let mut tmp: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if ((*a).initialized == 0 || b.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_add_in_place, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_poly_add(*a, b, &mut tmp) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_add_in_place, add failed\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_vector_free(a);
    *a = tmp;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_poly_subtract(
    mut a: rc_vector_t,
    mut b: rc_vector_t,
    mut c: *mut rc_vector_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut longest: rc_vector_t = rc_vector_t {
        len: 0,
        d: 0 as *mut libc::c_double,
        initialized: 0,
    };
    if (a.initialized == 0 || b.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_subtract, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if a.len > b.len {
        longest = a;
    } else {
        longest = b;
        b = a;
    }
    if (rc_vector_duplicate(longest, c) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_subtract, failed to duplicate vector\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    diff = (*c).len - b.len;
    i = diff;
    while i < (*c).len {
        *((*c).d).offset(i as isize) -= *(b.d).offset((i - diff) as isize);
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_poly_subtract_inplace(mut a: *mut rc_vector_t, mut b: rc_vector_t) -> libc::c_int {
    let mut tmp: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if ((*a).initialized == 0 || b.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_subtract_in_place, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_poly_subtract(*a, b, &mut tmp) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_subtract_in_place, subtract failed\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_vector_free(a);
    *a = tmp;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_poly_differentiate(
    mut a: rc_vector_t,
    mut d: libc::c_int,
    mut b: *mut rc_vector_t,
) -> libc::c_int {
    let mut tmp: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut tmp_r: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut i: libc::c_int = 0;
    let mut new_order: libc::c_int = 0;
    if (a.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_differentiate, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (d < 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_differentiate, d must be >=0\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if d >= a.len {
        return rc_vector_zeros(b, 1 as libc::c_int);
    }
    if d == 0 as libc::c_int {
        return rc_vector_duplicate(a, b);
    }
    new_order = a.len - 1 as libc::c_int;
    if (rc_vector_alloc(&mut tmp, new_order) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_differentiate, failed to alloc vector\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < new_order {
        *(tmp.d).offset(i as isize) = *(a.d).offset(i as isize) * (new_order - i) as libc::c_double;
        i += 1;
    }
    if d == 1 as libc::c_int {
        rc_vector_free(b);
        *b = tmp;
        return 0 as libc::c_int;
    }
    if (rc_poly_differentiate(tmp, d - 1 as libc::c_int, &mut tmp_r) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_differentiate, failed to differentiate recursively\n\0" as *const u8
                as *const libc::c_char,
        );
        rc_vector_free(&mut tmp);
        return -(1 as libc::c_int);
    }
    rc_vector_free(&mut tmp);
    rc_vector_free(b);
    *b = tmp_r;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_poly_divide(
    mut n: rc_vector_t,
    mut d: rc_vector_t,
    mut div: *mut rc_vector_t,
    mut rem: *mut rc_vector_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut tmp: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if (n.initialized == 0 || d.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_divide, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    diff = n.len - d.len;
    if (diff < 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_divide, order of num must be >= to den\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_vector_zeros(div, diff + 1 as libc::c_int) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_divide, failed to alloc vector\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_vector_duplicate(n, &mut tmp) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_divide, failed to duplicate vector\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(div);
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i <= diff {
        *((*div).d).offset(i as isize) = *(tmp.d).offset(i as isize) / *(d.d).offset(0 as libc::c_int as isize);
        j = 0 as libc::c_int;
        while j < d.len {
            *(tmp.d).offset((j + i) as isize) -= *((*div).d).offset(i as isize) * *(d.d).offset(j as isize);
            j += 1;
        }
        i += 1;
    }
    if (rc_vector_alloc(rem, d.len - 1 as libc::c_int) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_divide, failed alloc rem vector\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut tmp);
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < d.len - 1 as libc::c_int {
        *((*rem).d).offset(i as isize) = *(tmp.d).offset((i + diff + 1 as libc::c_int) as isize);
        i += 1;
    }
    rc_vector_free(&mut tmp);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_poly_butter(
    mut N: libc::c_int,
    mut wc: libc::c_double,
    mut b: *mut rc_vector_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut P2: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut P3: rc_vector_t = {
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
    if (N < 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_butter, order must be >1\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_vector_ones(b, 1 as libc::c_int) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_butter, failed to alloc vector\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_vector_alloc(&mut P2, 2 as libc::c_int) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_butter, failed to alloc vector\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(b);
        return -(1 as libc::c_int);
    }
    if (rc_vector_alloc(&mut P3, 3 as libc::c_int) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_poly_butter, failed to alloc vector\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(b);
        rc_vector_free(&mut P2);
        return -(1 as libc::c_int);
    }
    if N % 2 as libc::c_int == 0 as libc::c_int {
        *(P3.d).offset(0 as libc::c_int as isize) = 1.0f64 / (wc * wc);
        *(P3.d).offset(2 as libc::c_int as isize) = 1.0f64;
        i = 1 as libc::c_int;
        loop {
            if !(i <= N / 2 as libc::c_int) {
                current_block = 14401909646449704462;
                break;
            }
            *(P3.d).offset(1 as libc::c_int as isize) = -2.0f64
                * cos(
                    (2 as libc::c_int * i + (N - 1 as libc::c_int)) as libc::c_double * 3.14159265358979323846f64
                        / (2.0f64 * N as libc::c_double),
                )
                / wc;
            rc_vector_duplicate(*b, &mut tmp);
            if (rc_poly_conv(tmp, P3, b) != 0) as libc::c_int as libc::c_long != 0 {
                fprintf(
                    stderr,
                    b"ERROR in rc_poly_butter, failed to polyconv\n\0" as *const u8 as *const libc::c_char,
                );
                ret = -(1 as libc::c_int);
                current_block = 12940182072025652210;
                break;
            } else {
                i += 1;
            }
        }
    } else {
        current_block = 14401909646449704462;
    }
    match current_block {
        14401909646449704462 => {
            if N % 2 as libc::c_int == 1 as libc::c_int {
                *(P2.d).offset(0 as libc::c_int as isize) = 1.0f64 / wc;
                *(P2.d).offset(1 as libc::c_int as isize) = 1.0f64;
                rc_vector_duplicate(*b, &mut tmp);
                if (rc_poly_conv(tmp, P2, b) != 0) as libc::c_int as libc::c_long != 0 {
                    fprintf(
                        stderr,
                        b"ERROR in rc_poly_butter, failed to polyconv\n\0" as *const u8 as *const libc::c_char,
                    );
                    ret = -(1 as libc::c_int);
                } else {
                    *(P3.d).offset(0 as libc::c_int as isize) = 1.0f64 / (wc * wc);
                    *(P3.d).offset(2 as libc::c_int as isize) = 1.0f64;
                    i = 1 as libc::c_int;
                    while i <= (N - 1 as libc::c_int) / 2 as libc::c_int {
                        *(P3.d).offset(1 as libc::c_int as isize) = -2.0f64
                            * cos((2 as libc::c_int * i + (N - 1 as libc::c_int)) as libc::c_double
                                * 3.14159265358979323846f64
                                / (2.0f64 * N as libc::c_double))
                            / wc;
                        rc_vector_duplicate(*b, &mut tmp);
                        if (rc_poly_conv(tmp, P3, b) != 0) as libc::c_int as libc::c_long != 0 {
                            fprintf(
                                stderr,
                                b"ERROR in rc_poly_butter, failed to polyconv\n\0" as *const u8 as *const libc::c_char,
                            );
                            ret = -(1 as libc::c_int);
                            break;
                        } else {
                            i += 1;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    rc_vector_free(&mut tmp);
    rc_vector_free(&mut P2);
    rc_vector_free(&mut P3);
    return ret;
}
