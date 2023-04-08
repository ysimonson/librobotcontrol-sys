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
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn nanosleep(__requested_time: *const timespec, __remaining: *mut timespec) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint64_t = __uint64_t;
#[no_mangle]
pub unsafe extern "C" fn rc_nanosleep(mut ns: uint64_t) {
    let mut req: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut rem: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    req.tv_sec = ns.wrapping_div(1000000000 as libc::c_int as libc::c_ulong) as __time_t;
    req.tv_nsec = ns.wrapping_rem(1000000000 as libc::c_int as libc::c_ulong) as __syscall_slong_t;
    *__errno_location() = 0 as libc::c_int;
    while nanosleep(&mut req, &mut rem) != 0 && *__errno_location() == 4 as libc::c_int {
        req.tv_sec = rem.tv_sec;
        req.tv_nsec = rem.tv_nsec;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rc_usleep(mut us: libc::c_uint) {
    rc_nanosleep(us.wrapping_mul(1000 as libc::c_int as libc::c_uint) as uint64_t);
}
#[no_mangle]
pub unsafe extern "C" fn rc_nanos_since_epoch() -> uint64_t {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(0 as libc::c_int, &mut ts);
    return (ts.tv_sec as uint64_t)
        .wrapping_mul(1000000000 as libc::c_int as libc::c_ulong)
        .wrapping_add(ts.tv_nsec as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn rc_nanos_since_boot() -> uint64_t {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(1 as libc::c_int, &mut ts);
    return (ts.tv_sec as uint64_t)
        .wrapping_mul(1000000000 as libc::c_int as libc::c_ulong)
        .wrapping_add(ts.tv_nsec as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn rc_nanos_thread_time() -> uint64_t {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(3 as libc::c_int, &mut ts);
    return (ts.tv_sec as uint64_t)
        .wrapping_mul(1000000000 as libc::c_int as libc::c_ulong)
        .wrapping_add(ts.tv_nsec as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn rc_timespec_to_micros(mut ts: timespec) -> uint64_t {
    return (ts.tv_sec as uint64_t)
        .wrapping_mul(1000000000 as libc::c_int as libc::c_ulong)
        .wrapping_add(ts.tv_nsec as libc::c_ulong)
        .wrapping_div(1000 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn rc_timespec_to_millis(mut ts: timespec) -> uint64_t {
    return (ts.tv_sec as uint64_t)
        .wrapping_mul(1000000000 as libc::c_int as libc::c_ulong)
        .wrapping_add(ts.tv_nsec as libc::c_ulong)
        .wrapping_div(1000000 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn rc_timeval_to_micros(mut tv: timeval) -> uint64_t {
    return (tv.tv_sec as uint64_t)
        .wrapping_mul(1000000 as libc::c_int as libc::c_ulong)
        .wrapping_add(tv.tv_usec as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn rc_timeval_to_millis(mut tv: timeval) -> uint64_t {
    return (tv.tv_sec as uint64_t)
        .wrapping_mul(1000000 as libc::c_int as libc::c_ulong)
        .wrapping_add(tv.tv_usec as libc::c_ulong)
        .wrapping_div(1000 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn rc_timespec_diff(mut A: timespec, mut B: timespec) -> timespec {
    let mut temp: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    if B.tv_sec > A.tv_sec {
        temp = A;
        A = B;
        B = temp;
    } else if B.tv_sec == A.tv_sec && B.tv_nsec > A.tv_nsec {
        temp = A;
        A = B;
        B = temp;
    }
    if A.tv_nsec - B.tv_nsec < 0 as libc::c_int as libc::c_long {
        temp.tv_sec = A.tv_sec - B.tv_sec - 1 as libc::c_int as libc::c_long;
        temp.tv_nsec = 1000000000 as libc::c_long + A.tv_nsec - B.tv_nsec;
    } else {
        temp.tv_sec = A.tv_sec - B.tv_sec;
        temp.tv_nsec = A.tv_nsec - B.tv_nsec;
    }
    return temp;
}
#[no_mangle]
pub unsafe extern "C" fn rc_timespec_add(mut start: *mut timespec, mut seconds: libc::c_double) {
    let mut s: libc::c_int = seconds as libc::c_int;
    (*start).tv_sec += s as libc::c_long;
    (*start).tv_nsec = ((*start).tv_nsec as libc::c_double
        + (seconds - s as libc::c_double) * 1000000000 as libc::c_int as libc::c_double)
        as __syscall_slong_t;
    if (*start).tv_nsec >= 1000000000 as libc::c_int as libc::c_long {
        (*start).tv_nsec -= 1000000000 as libc::c_int as libc::c_long;
        (*start).tv_sec += 1 as libc::c_int as libc::c_long;
    } else if (*start).tv_nsec < 0 as libc::c_int as libc::c_long {
        (*start).tv_nsec += 1000000000 as libc::c_int as libc::c_long;
        (*start).tv_sec -= 1 as libc::c_int as libc::c_long;
    }
}
