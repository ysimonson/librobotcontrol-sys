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
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn rc_vector_alloc(v: *mut rc_vector_t, length: libc::c_int) -> libc::c_int;
    fn rc_vector_free(v: *mut rc_vector_t) -> libc::c_int;
    fn rc_vector_ones(v: *mut rc_vector_t, length: libc::c_int) -> libc::c_int;
    fn rc_vector_duplicate(a: rc_vector_t, b: *mut rc_vector_t) -> libc::c_int;
    fn rc_matrix_alloc(A: *mut rc_matrix_t, rows: libc::c_int, cols: libc::c_int) -> libc::c_int;
    fn rc_matrix_free(A: *mut rc_matrix_t) -> libc::c_int;
    fn rc_matrix_zeros(A: *mut rc_matrix_t, rows: libc::c_int, cols: libc::c_int) -> libc::c_int;
    fn rc_matrix_identity(A: *mut rc_matrix_t, dim: libc::c_int) -> libc::c_int;
    fn rc_matrix_duplicate(A: rc_matrix_t, B: *mut rc_matrix_t) -> libc::c_int;
    fn rc_matrix_multiply(A: rc_matrix_t, B: rc_matrix_t, C: *mut rc_matrix_t) -> libc::c_int;
    fn rc_matrix_row_vec_times_matrix(v: rc_vector_t, A: rc_matrix_t, c: *mut rc_vector_t) -> libc::c_int;
    fn rc_matrix_determinant(A: rc_matrix_t) -> libc::c_double;
    fn __vectorized_mult_accumulate(a: *mut libc::c_double, b: *mut libc::c_double, n: libc::c_int) -> libc::c_double;
    fn __vectorized_square_accumulate(a: *mut libc::c_double, n: libc::c_int) -> libc::c_double;
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
pub static mut zero_tolerance: libc::c_double = 1e-8f64;
unsafe extern "C" fn __householder_reflection(
    mut step: libc::c_int,
    mut Q: *mut rc_matrix_t,
    mut R: *mut rc_matrix_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut norm: libc::c_double = 0.;
    let mut tau: libc::c_double = 0.;
    let mut taui: libc::c_double = 0.;
    let mut dot: libc::c_double = 0.;
    let mut n: libc::c_int = (*R).rows - step;
    let mut Rrows: libc::c_int = (*R).rows;
    let mut p: libc::c_int = (*R).rows - n;
    let vla = n as usize;
    let mut x: Vec<libc::c_double> = ::std::vec::from_elem(0., vla);
    let vla_0 = n as usize;
    let vla_1 = n as usize;
    let mut H: Vec<libc::c_double> = ::std::vec::from_elem(0., vla_0 * vla_1);
    let vla_2 = ((*R).cols - p) as usize;
    let vla_3 = ((*R).rows - p) as usize;
    let mut tmp: Vec<libc::c_double> = ::std::vec::from_elem(0., vla_2 * vla_3);
    let vla_4 = (*Q).rows as usize;
    let vla_5 = n as usize;
    let mut tmp2: Vec<libc::c_double> = ::std::vec::from_elem(0., vla_4 * vla_5);
    let vla_6 = n as usize;
    let mut col: Vec<libc::c_double> = ::std::vec::from_elem(0., vla_6);
    let mut q: libc::c_int = (*Q).cols - n;
    j = step;
    while j < Rrows {
        *x.as_mut_ptr().offset((j - step) as isize) = *(*((*R).d).offset(j as isize)).offset(step as isize);
        j += 1;
    }
    norm = 0.0f64;
    i = 0 as libc::c_int;
    while i < n {
        norm += *x.as_mut_ptr().offset(i as isize) * *x.as_mut_ptr().offset(i as isize);
        i += 1;
    }
    norm = sqrt(norm);
    if *x.as_mut_ptr().offset(0 as libc::c_int as isize) >= 0.0f64 {
        *x.as_mut_ptr().offset(0 as libc::c_int as isize) = *x.as_mut_ptr().offset(0 as libc::c_int as isize) + norm;
        norm = -norm;
    } else {
        *x.as_mut_ptr().offset(0 as libc::c_int as isize) = *x.as_mut_ptr().offset(0 as libc::c_int as isize) - norm;
    }
    dot = __vectorized_square_accumulate(x.as_mut_ptr(), n);
    tau = -2.0f64 / dot;
    i = 0 as libc::c_int;
    while i < n {
        taui = tau * *x.as_mut_ptr().offset(i as isize);
        *H.as_mut_ptr().offset(i as isize * vla_1 as isize).offset(i as isize) =
            1.0f64 + taui * *x.as_mut_ptr().offset(i as isize);
        j = i + 1 as libc::c_int;
        while j < n {
            *H.as_mut_ptr().offset(i as isize * vla_1 as isize).offset(j as isize) =
                taui * *x.as_mut_ptr().offset(j as isize);
            j += 1;
        }
        i += 1;
    }
    i = 1 as libc::c_int;
    while i < n {
        j = 0 as libc::c_int;
        while j < i {
            *H.as_mut_ptr().offset(i as isize * vla_1 as isize).offset(j as isize) =
                *H.as_mut_ptr().offset(j as isize * vla_1 as isize).offset(i as isize);
            j += 1;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*R).rows - p {
        j = 0 as libc::c_int;
        while j < (*R).cols - p {
            *tmp.as_mut_ptr().offset(j as isize * vla_3 as isize).offset(i as isize) =
                *(*((*R).d).offset((i + p) as isize)).offset((j + p) as isize);
            j += 1;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*R).rows - p {
        if i == 0 as libc::c_int {
            *(*((*R).d).offset((i + p) as isize)).offset(p as isize) = norm;
        } else {
            *(*((*R).d).offset((i + p) as isize)).offset(p as isize) = 0.0f64;
        }
        j = 1 as libc::c_int;
        while j < (*R).cols - p {
            *(*((*R).d).offset((i + p) as isize)).offset((j + p) as isize) = __vectorized_mult_accumulate(
                H.as_mut_ptr().offset(i as isize * vla_1 as isize),
                tmp.as_mut_ptr().offset(j as isize * vla_3 as isize),
                n,
            );
            j += 1;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*Q).rows {
        j = 0 as libc::c_int;
        while j < n {
            *tmp2.as_mut_ptr().offset(i as isize * vla_5 as isize).offset(j as isize) =
                *(*((*Q).d).offset(i as isize)).offset((j + q) as isize);
            j += 1;
        }
        i += 1;
    }
    j = 0 as libc::c_int;
    while j < (*Q).cols - q {
        k = 0 as libc::c_int;
        while k < n {
            *col.as_mut_ptr().offset(k as isize) =
                *H.as_mut_ptr().offset(k as isize * vla_1 as isize).offset(j as isize);
            k += 1;
        }
        i = 0 as libc::c_int;
        while i < (*Q).rows {
            *(*((*Q).d).offset(i as isize)).offset((j + q) as isize) = __vectorized_mult_accumulate(
                tmp2.as_mut_ptr().offset(i as isize * vla_5 as isize),
                col.as_mut_ptr(),
                n,
            );
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_algebra_qr_decomp(
    mut A: rc_matrix_t,
    mut Q: *mut rc_matrix_t,
    mut R: *mut rc_matrix_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut steps: libc::c_int = 0;
    if (A.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_algebra_qr_decomp, matrix not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_matrix_duplicate(A, R) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_algebra_qr_decomp, failed to duplicate A\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_matrix_identity(Q, A.rows);
    if A.rows == A.cols {
        steps = A.cols - 1 as libc::c_int;
    } else if A.rows > A.cols {
        steps = A.cols;
    } else {
        steps = A.rows - 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < steps {
        if __householder_reflection(i, Q, R) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_algebra_lup_decomp(
    mut A: rc_matrix_t,
    mut L: *mut rc_matrix_t,
    mut U: *mut rc_matrix_t,
    mut P: *mut rc_matrix_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut tmpint: libc::c_int = 0;
    let mut s1: libc::c_double = 0.;
    let mut s2: libc::c_double = 0.;
    let mut ptmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rowtmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut Adup: rc_matrix_t = {
        let mut init = rc_matrix_t {
            rows: 0 as libc::c_int,
            cols: 0 as libc::c_int,
            d: 0 as *mut *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if (A.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_algebra_lup_decomp, matrix not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (A.cols != A.rows) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_algebra_lup_decomp, matrix is not square\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    m = A.cols;
    if (rc_matrix_duplicate(A, &mut Adup) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_algebra_lup_decomp, failed to duplicate A\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_matrix_identity(L, m) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_algebra_lup_decomp, failed to allocate identity matrix\n\0" as *const u8
                as *const libc::c_char,
        );
        rc_matrix_free(&mut Adup);
        return -(1 as libc::c_int);
    }
    if (rc_matrix_alloc(U, m, m) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_algebra_lup_decomp, failed to allocate U\n\0" as *const u8 as *const libc::c_char,
        );
        rc_matrix_free(&mut Adup);
        rc_matrix_free(L);
        return -(1 as libc::c_int);
    }
    if (rc_matrix_zeros(P, m, m) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_algebra_lup_decomp, failed to allocate matrix of zeros\n\0" as *const u8
                as *const libc::c_char,
        );
        rc_matrix_free(&mut Adup);
        rc_matrix_free(L);
        rc_matrix_free(U);
        return -(1 as libc::c_int);
    }
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (m as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong) as usize,
    );
    ptmp = fresh0.as_mut_ptr() as *mut libc::c_int;
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (m as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong) as usize,
    );
    rowtmp = fresh1.as_mut_ptr() as *mut libc::c_void;
    if (ptmp.is_null() || rowtmp.is_null()) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_algebra_lup_decomp, alloca failed, stack overflow\n\0" as *const u8 as *const libc::c_char,
        );
        rc_matrix_free(&mut Adup);
        rc_matrix_free(L);
        rc_matrix_free(U);
        rc_matrix_free(P);
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < m {
        *ptmp.offset(i as isize) = i;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < m - 1 as libc::c_int {
        index = i;
        j = i;
        while j < m {
            if fabs(*(*(A.d).offset(j as isize)).offset(i as isize))
                >= fabs(*(*(A.d).offset(index as isize)).offset(i as isize))
            {
                index = j;
            }
            j += 1;
        }
        if index != i {
            tmpint = *ptmp.offset(index as isize);
            *ptmp.offset(index as isize) = *ptmp.offset(i as isize);
            *ptmp.offset(i as isize) = tmpint;
            memcpy(
                rowtmp,
                *(Adup.d).offset(index as isize) as *const libc::c_void,
                (m as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
            );
            memcpy(
                *(Adup.d).offset(index as isize) as *mut libc::c_void,
                *(Adup.d).offset(i as isize) as *const libc::c_void,
                (m as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
            );
            memcpy(
                *(Adup.d).offset(i as isize) as *mut libc::c_void,
                rowtmp,
                (m as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
            );
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < m {
        *(*((*P).d).offset(i as isize)).offset(*ptmp.offset(i as isize) as isize) = 1.0f64;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < m {
        j = 0 as libc::c_int;
        while j < m {
            s1 = 0.0f64;
            s2 = 0.0f64;
            k = 0 as libc::c_int;
            while k < i {
                s1 += *(*((*U).d).offset(k as isize)).offset(j as isize)
                    * *(*((*L).d).offset(i as isize)).offset(k as isize);
                k += 1;
            }
            k = 0 as libc::c_int;
            while k < j {
                s2 += *(*((*U).d).offset(k as isize)).offset(j as isize)
                    * *(*((*L).d).offset(i as isize)).offset(k as isize);
                k += 1;
            }
            if j >= i {
                *(*((*U).d).offset(i as isize)).offset(j as isize) =
                    *(*(Adup.d).offset(i as isize)).offset(j as isize) - s1;
            }
            if i >= j {
                *(*((*L).d).offset(i as isize)).offset(j as isize) =
                    (*(*(Adup.d).offset(i as isize)).offset(j as isize) - s2)
                        / *(*((*U).d).offset(j as isize)).offset(j as isize);
            }
            j += 1;
        }
        i += 1;
    }
    rc_matrix_free(&mut Adup);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_algebra_invert_matrix(mut A: rc_matrix_t, mut Ainv: *mut rc_matrix_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut L: rc_matrix_t = {
        let mut init = rc_matrix_t {
            rows: 0 as libc::c_int,
            cols: 0 as libc::c_int,
            d: 0 as *mut *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut U: rc_matrix_t = {
        let mut init = rc_matrix_t {
            rows: 0 as libc::c_int,
            cols: 0 as libc::c_int,
            d: 0 as *mut *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut P: rc_matrix_t = {
        let mut init = rc_matrix_t {
            rows: 0 as libc::c_int,
            cols: 0 as libc::c_int,
            d: 0 as *mut *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut D: rc_matrix_t = {
        let mut init = rc_matrix_t {
            rows: 0 as libc::c_int,
            cols: 0 as libc::c_int,
            d: 0 as *mut *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut tmp: rc_matrix_t = {
        let mut init = rc_matrix_t {
            rows: 0 as libc::c_int,
            cols: 0 as libc::c_int,
            d: 0 as *mut *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if (A.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_inverse, matrix uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (A.cols != A.rows) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_inverse, nonsquare matrix\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if fabs(rc_matrix_determinant(A)) < zero_tolerance {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_inverse, matrix is singular\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_matrix_identity(&mut D, A.cols) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_inverse, failed to alloc identity\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_matrix_alloc(&mut tmp, A.rows, A.rows) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_inverse, failed to alloc matrix\n\0" as *const u8 as *const libc::c_char,
        );
        rc_matrix_free(&mut D);
        return -(1 as libc::c_int);
    }
    if (rc_algebra_lup_decomp(A, &mut L, &mut U, &mut P) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_inverse, failed to LUP decomp\n\0" as *const u8 as *const libc::c_char,
        );
        rc_matrix_free(&mut D);
        rc_matrix_free(&mut tmp);
        return -(1 as libc::c_int);
    }
    j = 0 as libc::c_int;
    while j < A.cols {
        i = 0 as libc::c_int;
        while i < A.cols {
            k = 0 as libc::c_int;
            while k < i {
                *(*(D.d).offset(i as isize)).offset(j as isize) -=
                    *(*(L.d).offset(i as isize)).offset(k as isize) * *(*(D.d).offset(k as isize)).offset(j as isize);
                k += 1;
            }
            i += 1;
        }
        i = A.cols - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            *(*(tmp.d).offset(i as isize)).offset(j as isize) = *(*(D.d).offset(i as isize)).offset(j as isize);
            k = i + 1 as libc::c_int;
            while k < A.cols {
                *(*(tmp.d).offset(i as isize)).offset(j as isize) -=
                    *(*(U.d).offset(i as isize)).offset(k as isize) * *(*(tmp.d).offset(k as isize)).offset(j as isize);
                k += 1;
            }
            *(*(tmp.d).offset(i as isize)).offset(j as isize) =
                *(*(tmp.d).offset(i as isize)).offset(j as isize) / *(*(U.d).offset(i as isize)).offset(i as isize);
            i -= 1;
        }
        j += 1;
    }
    rc_matrix_free(&mut L);
    rc_matrix_free(&mut U);
    rc_matrix_free(&mut D);
    i = 0 as libc::c_int;
    if (rc_matrix_multiply(tmp, P, Ainv) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_inverse, failed to multiply matrix\n\0" as *const u8 as *const libc::c_char,
        );
        i = -(1 as libc::c_int);
    }
    rc_matrix_free(&mut tmp);
    rc_matrix_free(&mut P);
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn rc_algebra_invert_matrix_inplace(mut A: *mut rc_matrix_t) -> libc::c_int {
    let mut Atmp: rc_matrix_t = {
        let mut init = rc_matrix_t {
            rows: 0 as libc::c_int,
            cols: 0 as libc::c_int,
            d: 0 as *mut *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if (rc_algebra_invert_matrix(*A, &mut Atmp) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_algebra_invert_matrix_inplace, failed to invert\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_matrix_free(A);
    *A = Atmp;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_algebra_lin_system_solve(
    mut A: rc_matrix_t,
    mut b: rc_vector_t,
    mut x: *mut rc_vector_t,
) -> libc::c_int {
    let mut fMaxElem: libc::c_double = 0.;
    let mut fAcc: libc::c_double = 0.;
    let mut nDim: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut Atemp: rc_matrix_t = {
        let mut init = rc_matrix_t {
            rows: 0 as libc::c_int,
            cols: 0 as libc::c_int,
            d: 0 as *mut *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut btemp: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if A.initialized == 0 || b.initialized == 0 {
        fprintf(
            stderr,
            b"ERROR in rc_algebra_lin_system_solve, matrix or vector uninitialized\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if A.cols != b.len {
        fprintf(
            stderr,
            b"ERROR in rc_algebra_lin_system_solve, dimension mismatch\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    nDim = A.cols;
    if (rc_vector_alloc(x, nDim) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_algebra_lin_system_solve, failed to alloc vector\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_matrix_duplicate(A, &mut Atemp) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_algebra_lin_system_solve, failed to duplicate matrix\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(x);
        return -(1 as libc::c_int);
    }
    if (rc_vector_duplicate(b, &mut btemp) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_algebra_lin_system_solve, failed to duplicate vector\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(x);
        rc_matrix_free(&mut Atemp);
        return -(1 as libc::c_int);
    }
    k = 0 as libc::c_int;
    while k < nDim - 1 as libc::c_int {
        fMaxElem = fabs(*(*(Atemp.d).offset(k as isize)).offset(k as isize));
        m = k;
        i = k + 1 as libc::c_int;
        while i < nDim {
            if fMaxElem < fabs(*(*(Atemp.d).offset(i as isize)).offset(k as isize)) {
                fMaxElem = *(*(Atemp.d).offset(i as isize)).offset(k as isize);
                m = i;
            }
            i += 1;
        }
        if m != k {
            i = k;
            while i < nDim {
                fAcc = *(*(Atemp.d).offset(k as isize)).offset(i as isize);
                *(*(Atemp.d).offset(k as isize)).offset(i as isize) =
                    *(*(Atemp.d).offset(m as isize)).offset(i as isize);
                *(*(Atemp.d).offset(m as isize)).offset(i as isize) = fAcc;
                i += 1;
            }
            fAcc = *(btemp.d).offset(k as isize);
            *(btemp.d).offset(k as isize) = *(btemp.d).offset(m as isize);
            *(btemp.d).offset(m as isize) = fAcc;
        }
        if (fabs(*(*(Atemp.d).offset(k as isize)).offset(k as isize)) < zero_tolerance) as libc::c_int as libc::c_long
            != 0
        {
            fprintf(
                stderr,
                b"ERROR in rc_algebra_lin_system_solve, matrix not full rank\n\0" as *const u8 as *const libc::c_char,
            );
            rc_matrix_free(&mut Atemp);
            rc_vector_free(&mut btemp);
            rc_vector_free(x);
            return -(1 as libc::c_int);
        }
        j = k + 1 as libc::c_int;
        while j < nDim {
            fAcc = -*(*(Atemp.d).offset(j as isize)).offset(k as isize)
                / *(*(Atemp.d).offset(k as isize)).offset(k as isize);
            i = k;
            while i < nDim {
                *(*(Atemp.d).offset(j as isize)).offset(i as isize) = *(*(Atemp.d).offset(j as isize))
                    .offset(i as isize)
                    + fAcc * *(*(Atemp.d).offset(k as isize)).offset(i as isize);
                i += 1;
            }
            *(btemp.d).offset(j as isize) = *(btemp.d).offset(j as isize) + fAcc * *(btemp.d).offset(k as isize);
            j += 1;
        }
        k += 1;
    }
    k = nDim - 1 as libc::c_int;
    while k >= 0 as libc::c_int {
        *((*x).d).offset(k as isize) = *(btemp.d).offset(k as isize);
        i = k + 1 as libc::c_int;
        while i < nDim {
            *((*x).d).offset(k as isize) -=
                *(*(Atemp.d).offset(k as isize)).offset(i as isize) * *((*x).d).offset(i as isize);
            i += 1;
        }
        *((*x).d).offset(k as isize) =
            *((*x).d).offset(k as isize) / *(*(Atemp.d).offset(k as isize)).offset(k as isize);
        k -= 1;
    }
    rc_matrix_free(&mut Atemp);
    rc_vector_free(&mut btemp);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_algebra_set_zero_tolerance(mut tol: libc::c_double) {
    zero_tolerance = tol;
}
#[no_mangle]
pub unsafe extern "C" fn rc_algebra_lin_system_solve_qr(
    mut A: rc_matrix_t,
    mut b: rc_vector_t,
    mut x: *mut rc_vector_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut temp: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut Q: rc_matrix_t = {
        let mut init = rc_matrix_t {
            rows: 0 as libc::c_int,
            cols: 0 as libc::c_int,
            d: 0 as *mut *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut R: rc_matrix_t = {
        let mut init = rc_matrix_t {
            rows: 0 as libc::c_int,
            cols: 0 as libc::c_int,
            d: 0 as *mut *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if (A.initialized == 0 || b.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_algebra_lin_system_solve_qr, matrix or vector uninitialized\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_algebra_qr_decomp(A, &mut Q, &mut R) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_algebra_lin_system_solve_qr, failed to perform QR decomp\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_matrix_row_vec_times_matrix(b, Q, &mut temp) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_algebra_lin_system_solve_qr, failed to multiply vec by matrix\n\0" as *const u8
                as *const libc::c_char,
        );
        rc_matrix_free(&mut Q);
        rc_matrix_free(&mut R);
        return -(1 as libc::c_int);
    }
    if (rc_vector_alloc(x, R.cols) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_algebra_lin_system_solve_qr, failed to alloc vector\n\0" as *const u8 as *const libc::c_char,
        );
        rc_matrix_free(&mut Q);
        rc_matrix_free(&mut R);
        rc_vector_free(&mut temp);
        return -(1 as libc::c_int);
    }
    k = R.cols - 1 as libc::c_int;
    while k >= 0 as libc::c_int {
        *((*x).d).offset(k as isize) = *(temp.d).offset(k as isize);
        i = k + 1 as libc::c_int;
        while i < R.cols {
            *((*x).d).offset(k as isize) -=
                *(*(R.d).offset(k as isize)).offset(i as isize) * *((*x).d).offset(i as isize);
            i += 1;
        }
        *((*x).d).offset(k as isize) = *((*x).d).offset(k as isize) / *(*(R.d).offset(k as isize)).offset(k as isize);
        k -= 1;
    }
    rc_matrix_free(&mut Q);
    rc_matrix_free(&mut R);
    rc_vector_free(&mut temp);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_algebra_fit_ellipsoid(
    mut pts: rc_matrix_t,
    mut ctr: *mut rc_vector_t,
    mut lens: *mut rc_vector_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut A: rc_matrix_t = {
        let mut init = rc_matrix_t {
            rows: 0 as libc::c_int,
            cols: 0 as libc::c_int,
            d: 0 as *mut *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut b: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    let mut f: rc_vector_t = {
        let mut init = rc_vector_t {
            len: 0 as libc::c_int,
            d: 0 as *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if (pts.initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_fit_ellipsoid, matrix not initialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (pts.cols != 3 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_fit_ellipsoid, matrix pts must have 3 columns\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    p = pts.rows;
    if p < 6 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_fit_ellipsoid, matrix pts must have at least 6 rows\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_vector_ones(&mut b, p) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_fit_ellipsoid, failed to alloc vector\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_matrix_alloc(&mut A, p, 6 as libc::c_int) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_fit_ellipsoid, failed to alloc matrix\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut b);
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < p {
        *(*(A.d).offset(i as isize)).offset(0 as libc::c_int as isize) = *(*(pts.d).offset(i as isize))
            .offset(0 as libc::c_int as isize)
            * *(*(pts.d).offset(i as isize)).offset(0 as libc::c_int as isize);
        *(*(A.d).offset(i as isize)).offset(1 as libc::c_int as isize) =
            *(*(pts.d).offset(i as isize)).offset(0 as libc::c_int as isize);
        *(*(A.d).offset(i as isize)).offset(2 as libc::c_int as isize) = *(*(pts.d).offset(i as isize))
            .offset(1 as libc::c_int as isize)
            * *(*(pts.d).offset(i as isize)).offset(1 as libc::c_int as isize);
        *(*(A.d).offset(i as isize)).offset(3 as libc::c_int as isize) =
            *(*(pts.d).offset(i as isize)).offset(1 as libc::c_int as isize);
        *(*(A.d).offset(i as isize)).offset(4 as libc::c_int as isize) = *(*(pts.d).offset(i as isize))
            .offset(2 as libc::c_int as isize)
            * *(*(pts.d).offset(i as isize)).offset(2 as libc::c_int as isize);
        *(*(A.d).offset(i as isize)).offset(5 as libc::c_int as isize) =
            *(*(pts.d).offset(i as isize)).offset(2 as libc::c_int as isize);
        i += 1;
    }
    if (rc_algebra_lin_system_solve_qr(A, b, &mut f) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_fit_ellipsoid, failed to solve QR\n\0" as *const u8 as *const libc::c_char,
        );
        rc_matrix_free(&mut A);
        rc_vector_free(&mut b);
        rc_vector_free(&mut f);
        return -(1 as libc::c_int);
    }
    rc_matrix_free(&mut A);
    rc_vector_free(&mut b);
    if (rc_vector_alloc(ctr, 3 as libc::c_int) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_fit_ellipsoid, failed to allocate ctr\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut f);
        return -(1 as libc::c_int);
    }
    *((*ctr).d).offset(0 as libc::c_int as isize) =
        -*(f.d).offset(1 as libc::c_int as isize) / (2.0f64 * *(f.d).offset(0 as libc::c_int as isize));
    *((*ctr).d).offset(1 as libc::c_int as isize) =
        -*(f.d).offset(3 as libc::c_int as isize) / (2.0f64 * *(f.d).offset(2 as libc::c_int as isize));
    *((*ctr).d).offset(2 as libc::c_int as isize) =
        -*(f.d).offset(5 as libc::c_int as isize) / (2.0f64 * *(f.d).offset(4 as libc::c_int as isize));
    if (rc_vector_alloc(&mut b, 3 as libc::c_int) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_fit_ellipsoid, failed to alloc vector\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_matrix_alloc(&mut A, 3 as libc::c_int, 3 as libc::c_int) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_fit_ellipsoid, failed to alloc matrix\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut b);
        return -(1 as libc::c_int);
    }
    *(*(A.d).offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize) = *(f.d)
        .offset(0 as libc::c_int as isize)
        * *((*ctr).d).offset(0 as libc::c_int as isize)
        * *((*ctr).d).offset(0 as libc::c_int as isize)
        + 1.0f64;
    *(*(A.d).offset(0 as libc::c_int as isize)).offset(1 as libc::c_int as isize) = *(f.d)
        .offset(0 as libc::c_int as isize)
        * *((*ctr).d).offset(1 as libc::c_int as isize)
        * *((*ctr).d).offset(1 as libc::c_int as isize);
    *(*(A.d).offset(0 as libc::c_int as isize)).offset(2 as libc::c_int as isize) = *(f.d)
        .offset(0 as libc::c_int as isize)
        * *((*ctr).d).offset(2 as libc::c_int as isize)
        * *((*ctr).d).offset(2 as libc::c_int as isize);
    *(*(A.d).offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize) = *(f.d)
        .offset(2 as libc::c_int as isize)
        * *((*ctr).d).offset(0 as libc::c_int as isize)
        * *((*ctr).d).offset(0 as libc::c_int as isize);
    *(*(A.d).offset(1 as libc::c_int as isize)).offset(1 as libc::c_int as isize) = *(f.d)
        .offset(2 as libc::c_int as isize)
        * *((*ctr).d).offset(1 as libc::c_int as isize)
        * *((*ctr).d).offset(1 as libc::c_int as isize)
        + 1.0f64;
    *(*(A.d).offset(1 as libc::c_int as isize)).offset(2 as libc::c_int as isize) = *(f.d)
        .offset(2 as libc::c_int as isize)
        * *((*ctr).d).offset(2 as libc::c_int as isize)
        * *((*ctr).d).offset(2 as libc::c_int as isize);
    *(*(A.d).offset(2 as libc::c_int as isize)).offset(0 as libc::c_int as isize) = *(f.d)
        .offset(4 as libc::c_int as isize)
        * *((*ctr).d).offset(0 as libc::c_int as isize)
        * *((*ctr).d).offset(0 as libc::c_int as isize);
    *(*(A.d).offset(2 as libc::c_int as isize)).offset(1 as libc::c_int as isize) = *(f.d)
        .offset(4 as libc::c_int as isize)
        * *((*ctr).d).offset(1 as libc::c_int as isize)
        * *((*ctr).d).offset(1 as libc::c_int as isize);
    *(*(A.d).offset(2 as libc::c_int as isize)).offset(2 as libc::c_int as isize) = *(f.d)
        .offset(4 as libc::c_int as isize)
        * *((*ctr).d).offset(2 as libc::c_int as isize)
        * *((*ctr).d).offset(2 as libc::c_int as isize)
        + 1.0f64;
    *(b.d).offset(0 as libc::c_int as isize) = *(f.d).offset(0 as libc::c_int as isize);
    *(b.d).offset(1 as libc::c_int as isize) = *(f.d).offset(2 as libc::c_int as isize);
    *(b.d).offset(2 as libc::c_int as isize) = *(f.d).offset(4 as libc::c_int as isize);
    if (rc_algebra_lin_system_solve(A, b, lens) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_fit_ellipsoid, failed to solve linear system\n\0" as *const u8 as *const libc::c_char,
        );
        rc_matrix_free(&mut A);
        rc_vector_free(&mut b);
        rc_vector_free(&mut f);
        return -(1 as libc::c_int);
    }
    *((*lens).d).offset(0 as libc::c_int as isize) = 1.0f64 / sqrt(*((*lens).d).offset(0 as libc::c_int as isize));
    *((*lens).d).offset(1 as libc::c_int as isize) = 1.0f64 / sqrt(*((*lens).d).offset(1 as libc::c_int as isize));
    *((*lens).d).offset(2 as libc::c_int as isize) = 1.0f64 / sqrt(*((*lens).d).offset(2 as libc::c_int as isize));
    rc_matrix_free(&mut A);
    rc_vector_free(&mut b);
    rc_vector_free(&mut f);
    return 0 as libc::c_int;
}
