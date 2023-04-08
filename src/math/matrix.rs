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
    fn perror(__s: *const libc::c_char);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn rc_get_random_double() -> libc::c_double;
    fn rc_vector_alloc(v: *mut rc_vector_t, length: libc::c_int) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rc_matrix_t {
    pub rows: libc::c_int,
    pub cols: libc::c_int,
    pub d: *mut *mut libc::c_double,
    pub initialized: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_empty() -> rc_matrix_t {
    let mut out: rc_matrix_t = {
        let mut init = rc_matrix_t {
            rows: 0 as libc::c_int,
            cols: 0 as libc::c_int,
            d: 0 as *mut *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_alloc(
    mut A: *mut rc_matrix_t,
    mut rows: libc::c_int,
    mut cols: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (rows < 1 as libc::c_int || cols < 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_alloc, rows and cols must be >=1\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if A.is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_alloc, received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*A).initialized == 1 as libc::c_int && rows == (*A).rows && cols == (*A).cols {
        return 0 as libc::c_int;
    }
    rc_matrix_free(A);
    (*A).d =
        malloc((rows as libc::c_ulong).wrapping_mul(::core::mem::size_of::<*mut libc::c_double>() as libc::c_ulong))
            as *mut *mut libc::c_double;
    if ((*A).d).is_null() as libc::c_int as libc::c_long != 0 {
        perror(b"ERROR in rc_matrix_alloc\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"tried allocating a %dx%d matrix\n\0" as *const u8 as *const libc::c_char,
            rows,
            cols,
        );
        return -(1 as libc::c_int);
    }
    let mut ptr: *mut libc::c_void = malloc(
        ((rows * cols) as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    if ptr.is_null() as libc::c_int as libc::c_long != 0 {
        perror(b"ERROR in rc_matrix_alloc\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"tried allocating a %dx%d matrix\n\0" as *const u8 as *const libc::c_char,
            rows,
            cols,
        );
        free((*A).d as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < rows {
        let ref mut fresh0 = *((*A).d).offset(i as isize);
        *fresh0 = (ptr as *mut libc::c_char).offset(
            ((i * cols) as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
                as isize,
        ) as *mut libc::c_double;
        i += 1;
    }
    (*A).rows = rows;
    (*A).cols = cols;
    (*A).initialized = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_free(mut A: *mut rc_matrix_t) -> libc::c_int {
    let mut new: rc_matrix_t = {
        let mut init = rc_matrix_t {
            rows: 0 as libc::c_int,
            cols: 0 as libc::c_int,
            d: 0 as *mut *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if A.is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_free, received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if !((*A).d).is_null() && (*A).initialized == 1 as libc::c_int {
        free(*((*A).d).offset(0 as libc::c_int as isize) as *mut libc::c_void);
    }
    free((*A).d as *mut libc::c_void);
    *A = new;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_zeros(
    mut A: *mut rc_matrix_t,
    mut rows: libc::c_int,
    mut cols: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (rows < 1 as libc::c_int || cols < 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_create_matrix_zeros, rows and cols must be >=1\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if A.is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_create_matrix_zeros, received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_matrix_free(A);
    (*A).d =
        malloc((rows as libc::c_ulong).wrapping_mul(::core::mem::size_of::<*mut libc::c_double>() as libc::c_ulong))
            as *mut *mut libc::c_double;
    if ((*A).d).is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_create_matrix_zeros, not enough memory\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut ptr: *mut libc::c_void = calloc(
        (rows * cols) as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    );
    if ptr.is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_create_matrix_zeros, not enough memory\n\0" as *const u8 as *const libc::c_char,
        );
        free((*A).d as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < rows {
        let ref mut fresh1 = *((*A).d).offset(i as isize);
        *fresh1 = (ptr as *mut libc::c_char).offset(
            ((i * cols) as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
                as isize,
        ) as *mut libc::c_double;
        i += 1;
    }
    (*A).rows = rows;
    (*A).cols = cols;
    (*A).initialized = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_identity(mut A: *mut rc_matrix_t, mut dim: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (rc_matrix_zeros(A, dim, dim) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_identity, failed to allocate matrix\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < dim {
        *(*((*A).d).offset(i as isize)).offset(i as isize) = 1.0f64;
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_random(
    mut A: *mut rc_matrix_t,
    mut rows: libc::c_int,
    mut cols: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (rc_matrix_alloc(A, rows, cols) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_random, failed to allocate matrix\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < (*A).rows * (*A).cols {
        *(*((*A).d).offset(0 as libc::c_int as isize)).offset(i as isize) = rc_get_random_double();
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_diagonal(mut A: *mut rc_matrix_t, mut v: rc_vector_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (v.initialized != 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_diagonal, vector not initialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_matrix_zeros(A, v.len, v.len) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_diagonal, failed to allocate matrix\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < v.len {
        *(*((*A).d).offset(i as isize)).offset(i as isize) = *(v.d).offset(i as isize);
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_duplicate(mut A: rc_matrix_t, mut B: *mut rc_matrix_t) -> libc::c_int {
    if (A.initialized != 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_duplicate not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_matrix_alloc(B, A.rows, A.cols) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_duplicate, failed to allocate memory\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    memcpy(
        *((*B).d).offset(0 as libc::c_int as isize) as *mut libc::c_void,
        *(A.d).offset(0 as libc::c_int as isize) as *const libc::c_void,
        ((A.rows * A.cols) as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_print(mut A: rc_matrix_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (A.initialized != 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_print, matrix not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < A.rows {
        j = 0 as libc::c_int;
        while j < A.cols {
            printf(
                b"%7.4f  \0" as *const u8 as *const libc::c_char,
                *(*(A.d).offset(i as isize)).offset(j as isize),
            );
            j += 1;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_print_sci(mut A: rc_matrix_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (A.initialized != 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_print_sci, matrix not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < A.rows {
        j = 0 as libc::c_int;
        while j < A.cols {
            printf(
                b"%11.4e  \0" as *const u8 as *const libc::c_char,
                *(*(A.d).offset(i as isize)).offset(j as isize),
            );
            j += 1;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_zero_out(mut A: *mut rc_matrix_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if ((*A).initialized != 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_zero_out, matrix not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < (*A).rows {
        j = 0 as libc::c_int;
        while j < (*A).cols {
            *(*((*A).d).offset(i as isize)).offset(j as isize) = 0.0f64;
            j += 1;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_times_scalar(mut A: *mut rc_matrix_t, mut s: libc::c_double) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if ((*A).initialized != 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_times_scalar. matrix uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < (*A).rows * (*A).cols {
        *(*((*A).d).offset(0 as libc::c_int as isize)).offset(i as isize) *= s;
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_multiply(
    mut A: rc_matrix_t,
    mut B: rc_matrix_t,
    mut C: *mut rc_matrix_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tmp: *mut libc::c_double = 0 as *mut libc::c_double;
    if (A.initialized != 1 as libc::c_int || B.initialized != 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_multiply, matrix not initialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (A.cols != B.rows) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_multiply, dimension mismatch\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_matrix_alloc(C, A.rows, B.cols) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_multiply, can't allocate memory for C\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut fresh2 = ::std::vec::from_elem(
        0,
        (B.rows as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong) as usize,
    );
    tmp = fresh2.as_mut_ptr() as *mut libc::c_double;
    if tmp.is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_multiply, alloca failed, stack overflow\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < B.cols {
        j = 0 as libc::c_int;
        while j < B.rows {
            *tmp.offset(j as isize) = *(*(B.d).offset(j as isize)).offset(i as isize);
            j += 1;
        }
        j = 0 as libc::c_int;
        while j < A.rows {
            *(*((*C).d).offset(j as isize)).offset(i as isize) =
                __vectorized_mult_accumulate(*(A.d).offset(j as isize), tmp, B.rows);
            j += 1;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_left_multiply_inplace(mut A: rc_matrix_t, mut B: *mut rc_matrix_t) -> libc::c_int {
    let mut tmp: rc_matrix_t = {
        let mut init = rc_matrix_t {
            rows: 0 as libc::c_int,
            cols: 0 as libc::c_int,
            d: 0 as *mut *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if (A.initialized != 1 as libc::c_int || (*B).initialized != 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_left_multiply_inplace, matrix not initialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (A.cols != (*B).rows) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_left_multiply_inplace, dimension mismatch\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_matrix_multiply(A, *B, &mut tmp) != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_left_multiply_inplace, failed to multiply\n\0" as *const u8 as *const libc::c_char,
        );
        rc_matrix_free(&mut tmp);
        return -(1 as libc::c_int);
    }
    rc_matrix_free(B);
    *B = tmp;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_right_multiply_inplace(mut A: *mut rc_matrix_t, mut B: rc_matrix_t) -> libc::c_int {
    let mut tmp: rc_matrix_t = {
        let mut init = rc_matrix_t {
            rows: 0 as libc::c_int,
            cols: 0 as libc::c_int,
            d: 0 as *mut *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if ((*A).initialized != 1 as libc::c_int || B.initialized != 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_right_multiply_inplace, matrix not initialized\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if ((*A).cols != B.rows) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_right_multiply_inplace, dimension mismatch\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_matrix_multiply(*A, B, &mut tmp) != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_right_multiply_inplace, failed to multiply\n\0" as *const u8 as *const libc::c_char,
        );
        rc_matrix_free(&mut tmp);
        return -(1 as libc::c_int);
    }
    rc_matrix_free(A);
    *A = tmp;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_add(mut A: rc_matrix_t, mut B: rc_matrix_t, mut C: *mut rc_matrix_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (A.initialized != 1 as libc::c_int || B.initialized != 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_add, matrix not initialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (A.rows != B.rows || A.cols != B.cols) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_add, dimension mismatch\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_matrix_alloc(C, A.rows, A.cols) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_add, can't allocate memory for C\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < A.rows * A.cols {
        *(*((*C).d).offset(0 as libc::c_int as isize)).offset(i as isize) = *(*(A.d).offset(0 as libc::c_int as isize))
            .offset(i as isize)
            + *(*(B.d).offset(0 as libc::c_int as isize)).offset(i as isize);
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_add_inplace(mut A: *mut rc_matrix_t, mut B: rc_matrix_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if ((*A).initialized != 1 as libc::c_int || B.initialized != 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_add_inplace, matrix not initialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if ((*A).rows != B.rows || (*A).cols != B.cols) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_add_inplace, dimension mismatch\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < (*A).rows * (*A).cols {
        *(*((*A).d).offset(0 as libc::c_int as isize)).offset(i as isize) +=
            *(*(B.d).offset(0 as libc::c_int as isize)).offset(i as isize);
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_subtract_inplace(mut A: *mut rc_matrix_t, mut B: rc_matrix_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if ((*A).initialized != 1 as libc::c_int || B.initialized != 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_subtract_inplace, matrix not initialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if ((*A).rows != B.rows || (*A).cols != B.cols) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_subtract_inplace, dimension mismatch\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < (*A).rows * (*A).cols {
        *(*((*A).d).offset(0 as libc::c_int as isize)).offset(i as isize) -=
            *(*(B.d).offset(0 as libc::c_int as isize)).offset(i as isize);
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_transpose(mut A: rc_matrix_t, mut T: *mut rc_matrix_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (A.initialized != 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_transpose, received uninitialized matrix\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_matrix_alloc(T, A.cols, A.rows) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_transpose, can't allocate memory for T\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < A.rows {
        j = 0 as libc::c_int;
        while j < A.cols {
            *(*((*T).d).offset(j as isize)).offset(i as isize) = *(*(A.d).offset(i as isize)).offset(j as isize);
            j += 1;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_transpose_inplace(mut A: *mut rc_matrix_t) -> libc::c_int {
    let mut tmp: rc_matrix_t = {
        let mut init = rc_matrix_t {
            rows: 0 as libc::c_int,
            cols: 0 as libc::c_int,
            d: 0 as *mut *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if A.is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_transpose_matrix_inplace, received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if ((*A).initialized == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_transpose_matrix_inplace, matrix uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*A).rows == 1 as libc::c_int && (*A).cols == 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (rc_matrix_transpose(*A, &mut tmp) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_transpose_matrix_inplace, can't transpose\n\0" as *const u8 as *const libc::c_char,
        );
        rc_matrix_free(&mut tmp);
        return -(1 as libc::c_int);
    }
    rc_matrix_free(A);
    *A = tmp;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_times_col_vec(
    mut A: rc_matrix_t,
    mut v: rc_vector_t,
    mut c: *mut rc_vector_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (A.initialized != 1 as libc::c_int || v.initialized != 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_times_col_vec, matrix or vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (A.cols != v.len) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_times_col_vec, dimension mismatch\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_vector_alloc(c, A.rows) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_times_col_vec, failed to allocate c\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < A.rows {
        *((*c).d).offset(i as isize) = __vectorized_mult_accumulate(*(A.d).offset(i as isize), v.d, v.len);
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_row_vec_times_matrix(
    mut v: rc_vector_t,
    mut A: rc_matrix_t,
    mut c: *mut rc_vector_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tmp: *mut libc::c_double = 0 as *mut libc::c_double;
    if (A.initialized != 1 as libc::c_int || v.initialized != 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_row_vec_times_matrix, matrix or vector uninitialized\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (A.rows != v.len) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_row_vec_times_matrix, dimension mismatch\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut fresh3 = ::std::vec::from_elem(
        0,
        (A.rows as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong) as usize,
    );
    tmp = fresh3.as_mut_ptr() as *mut libc::c_double;
    if tmp.is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_row_vec_times_matrix, alloca failed, stack overflow\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_vector_alloc(c, A.cols) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_row_vec_times_matrix, failed to allocate c\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < A.cols {
        j = 0 as libc::c_int;
        while j < A.rows {
            *tmp.offset(j as isize) = *(*(A.d).offset(j as isize)).offset(i as isize);
            j += 1;
        }
        *((*c).d).offset(i as isize) = __vectorized_mult_accumulate(v.d, tmp, v.len);
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_outer_product(
    mut v1: rc_vector_t,
    mut v2: rc_vector_t,
    mut A: *mut rc_matrix_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (v1.initialized != 1 as libc::c_int || v2.initialized != 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_outer_product, vector uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_matrix_alloc(A, v1.len, v2.len) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_outer_product, failed to allocate A\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < v1.len {
        j = 0 as libc::c_int;
        while j < v2.len {
            *(*((*A).d).offset(i as isize)).offset(j as isize) =
                *(v1.d).offset(i as isize) * *(v2.d).offset(j as isize);
            j += 1;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_determinant(mut A: rc_matrix_t) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ratio: libc::c_double = 0.;
    let mut det: libc::c_double = 0.;
    let mut tmp: rc_matrix_t = {
        let mut init = rc_matrix_t {
            rows: 0 as libc::c_int,
            cols: 0 as libc::c_int,
            d: 0 as *mut *mut libc::c_double,
            initialized: 0 as libc::c_int,
        };
        init
    };
    if (A.initialized != 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_determinant, received uninitialized matrix\n\0" as *const u8 as *const libc::c_char,
        );
        return -1.0f64;
    }
    if (A.rows != A.cols) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_determinant, expected square matrix\n\0" as *const u8 as *const libc::c_char,
        );
        return -1.0f64;
    }
    if A.rows == 1 as libc::c_int {
        return *(*(A.d).offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize);
    }
    if A.rows == 2 as libc::c_int {
        return *(*(A.d).offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            * *(*(A.d).offset(1 as libc::c_int as isize)).offset(1 as libc::c_int as isize)
            - *(*(A.d).offset(0 as libc::c_int as isize)).offset(1 as libc::c_int as isize)
                * *(*(A.d).offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize);
    }
    if (rc_matrix_duplicate(A, &mut tmp) != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_determinant, failed to allocate duplicate\n\0" as *const u8 as *const libc::c_char,
        );
        return -1.0f64;
    }
    i = 0 as libc::c_int;
    while i < A.rows - 1 as libc::c_int {
        j = i + 1 as libc::c_int;
        while j < A.rows {
            ratio =
                *(*(tmp.d).offset(j as isize)).offset(i as isize) / *(*(tmp.d).offset(i as isize)).offset(i as isize);
            k = 0 as libc::c_int;
            while k < A.rows {
                *(*(tmp.d).offset(j as isize)).offset(k as isize) -=
                    ratio * *(*(tmp.d).offset(i as isize)).offset(k as isize);
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    det = 1.0f64;
    i = 0 as libc::c_int;
    while i < A.rows {
        det *= *(*(tmp.d).offset(i as isize)).offset(i as isize);
        i += 1;
    }
    rc_matrix_free(&mut tmp);
    return det;
}
#[no_mangle]
pub unsafe extern "C" fn rc_matrix_symmetrize(mut P: *mut rc_matrix_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut val: libc::c_double = 0.;
    if P.is_null() {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_symmetrize, matrix pointer is NULL\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*P).initialized != 1 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_symmetrize, matrix uninitialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*P).rows != (*P).cols {
        fprintf(
            stderr,
            b"ERROR in rc_matrix_symmetrize, matrix must be square\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < (*P).rows - 1 as libc::c_int {
        j = i + 1 as libc::c_int;
        while j < (*P).cols {
            val = (*(*((*P).d).offset(i as isize)).offset(j as isize)
                + *(*((*P).d).offset(j as isize)).offset(i as isize))
                / 2.0f64;
            *(*((*P).d).offset(i as isize)).offset(j as isize) = val;
            *(*((*P).d).offset(j as isize)).offset(i as isize) = val;
            j += 1;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
