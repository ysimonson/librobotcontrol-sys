#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[no_mangle]
pub unsafe extern "C" fn __vectorized_mult_accumulate(
    mut a: *mut libc::c_double,
    mut b: *mut libc::c_double,
    mut n: libc::c_int,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut sum: libc::c_double = 0.0f64;
    i = 0 as libc::c_int;
    while i < n {
        sum += *a.offset(i as isize) * *b.offset(i as isize);
        i += 1;
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C" fn __vectorized_square_accumulate(
    mut a: *mut libc::c_double,
    mut n: libc::c_int,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut sum: libc::c_double = 0.0f64;
    i = 0 as libc::c_int;
    while i < n {
        sum += *a.offset(i as isize) * *a.offset(i as isize);
        i += 1;
    }
    return sum;
}
