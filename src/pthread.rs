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
    fn __errno_location() -> *mut libc::c_int;
    fn getpriority(__which: __priority_which_t, __who: id_t) -> libc::c_int;
    fn setpriority(__which: __priority_which_t, __who: id_t, __prio: libc::c_int) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn sched_get_priority_max(__algorithm: libc::c_int) -> libc::c_int;
    fn sched_get_priority_min(__algorithm: libc::c_int) -> libc::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_timedjoin_np(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
        __abstime: *const timespec,
    ) -> libc::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_attr_init(__attr: *mut pthread_attr_t) -> libc::c_int;
    fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> libc::c_int;
    fn pthread_attr_setschedparam(__attr: *mut pthread_attr_t, __param: *const sched_param) -> libc::c_int;
    fn pthread_attr_setschedpolicy(__attr: *mut pthread_attr_t, __policy: libc::c_int) -> libc::c_int;
    fn pthread_attr_setinheritsched(__attr: *mut pthread_attr_t, __inherit: libc::c_int) -> libc::c_int;
    fn pthread_setschedparam(
        __target_thread: pthread_t,
        __policy: libc::c_int,
        __param: *const sched_param,
    ) -> libc::c_int;
    fn pthread_getschedparam(
        __target_thread: pthread_t,
        __policy: *mut libc::c_int,
        __param: *mut sched_param,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __id_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
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
pub type __priority_which = libc::c_uint;
pub const PRIO_USER: __priority_which = 2;
pub const PRIO_PGRP: __priority_which = 1;
pub const PRIO_PROCESS: __priority_which = 0;
pub type id_t = __id_t;
pub type __priority_which_t = __priority_which;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: libc::c_int,
}
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_EXPLICIT_SCHED: C2RustUnnamed = 1;
pub const PTHREAD_INHERIT_SCHED: C2RustUnnamed = 0;
#[no_mangle]
pub unsafe extern "C" fn rc_pthread_create(
    mut thread: *mut pthread_t,
    mut func: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    mut arg: *mut libc::c_void,
    mut policy: libc::c_int,
    mut priority: libc::c_int,
) -> libc::c_int {
    let mut pthread_attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    let mut pthread_param: sched_param = sched_param { sched_priority: 0 };
    if policy != 1 as libc::c_int && policy != 2 as libc::c_int && policy != 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_pthread_create: policy must be SCHED_FIFO, SCHED_RR, or SCHED_OTHER\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if thread.is_null() || func.is_none() {
        fprintf(
            stderr,
            b"ERROR in rc_pthread_create: received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    pthread_attr_init(&mut pthread_attr);
    if priority != 0 as libc::c_int || policy != 0 as libc::c_int {
        *__errno_location() = pthread_attr_setinheritsched(&mut pthread_attr, PTHREAD_EXPLICIT_SCHED as libc::c_int);
        if *__errno_location() != 0 {
            perror(b"ERROR: pthread_attr_setinheritsched: \0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        }
    }
    let max_pri: libc::c_int = sched_get_priority_max(policy);
    let min_pri: libc::c_int = sched_get_priority_min(policy);
    if priority > max_pri || priority < min_pri {
        fprintf(
            stderr,
            b"ERROR in rc_pthread_create, priority must be between %d & %d\n\0" as *const u8 as *const libc::c_char,
            min_pri,
            max_pri,
        );
        return -(1 as libc::c_int);
    }
    *__errno_location() = pthread_attr_setschedpolicy(&mut pthread_attr, policy);
    if *__errno_location() != 0 {
        perror(b"ERROR: pthread_attr_setschedpolicy\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    pthread_param.sched_priority = priority;
    *__errno_location() = pthread_attr_setschedparam(&mut pthread_attr, &mut pthread_param);
    if *__errno_location() != 0 {
        perror(b"ERROR: pthread_attr_setschedparam\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    *__errno_location() = pthread_create(thread, &mut pthread_attr, func, arg);
    if *__errno_location() == 1 as libc::c_int {
        fprintf(
            stderr,
            b"WARNING: in rc_pthread_create, insufficient privileges to set scheduling policy\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"starting thread with inherited scheduling policy instead\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"to silence this warning, call with policy=SCHED_OTHER & priority=0\n\0" as *const u8
                as *const libc::c_char,
        );
        policy = 0 as libc::c_int;
        priority = 0 as libc::c_int;
        *__errno_location() = pthread_create(thread, 0 as *const pthread_attr_t, func, arg);
        if *__errno_location() != 0 as libc::c_int {
            perror(b"ERROR: in rc_pthread_create \0" as *const u8 as *const libc::c_char);
            pthread_attr_destroy(&mut pthread_attr);
            return -(1 as libc::c_int);
        }
    } else if *__errno_location() != 0 {
        perror(b"ERROR: in rc_pthread_create: \0" as *const u8 as *const libc::c_char);
        pthread_attr_destroy(&mut pthread_attr);
        return -(1 as libc::c_int);
    }
    let mut policy_new: libc::c_int = 0;
    let mut params_new: sched_param = sched_param { sched_priority: 0 };
    *__errno_location() = pthread_getschedparam(*thread, &mut policy_new, &mut params_new);
    if *__errno_location() != 0 {
        perror(b"ERROR: pthread_getschedparam\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if policy_new != policy {
        fprintf(
            stderr,
            b"WARNING in rc_pthread_create, policy actually got set to %d, expected %d\n\0" as *const u8
                as *const libc::c_char,
            policy_new,
            policy,
        );
    }
    if params_new.sched_priority != priority {
        fprintf(
            stderr,
            b"WARNING in rc_pthread_create, priority actually got set to %d, expected %d\n\0" as *const u8
                as *const libc::c_char,
            params_new.sched_priority,
            priority,
        );
    }
    if pthread_attr_destroy(&mut pthread_attr) != 0 {
        fprintf(
            stderr,
            b"WARNING, failed to destroy pthread_attr\n\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_pthread_timed_join(
    mut thread: pthread_t,
    mut retval: *mut *mut libc::c_void,
    mut timeout_sec: libc::c_float,
) -> libc::c_int {
    let mut thread_timeout: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(0 as libc::c_int, &mut thread_timeout);
    let mut timeout_ns: uint64_t = (timeout_sec * 1000000000 as libc::c_int as libc::c_float) as uint64_t;
    thread_timeout.tv_sec = (thread_timeout.tv_sec as libc::c_ulong)
        .wrapping_add(timeout_ns.wrapping_div(1000000000 as libc::c_int as libc::c_ulong))
        as __time_t as __time_t;
    thread_timeout.tv_nsec = (thread_timeout.tv_nsec as libc::c_ulong)
        .wrapping_add(timeout_ns.wrapping_rem(1000000000 as libc::c_int as libc::c_ulong))
        as __syscall_slong_t as __syscall_slong_t;
    if thread_timeout.tv_nsec > 1000000000 as libc::c_int as libc::c_long {
        thread_timeout.tv_sec += 1 as libc::c_int as libc::c_long;
        thread_timeout.tv_nsec -= 1000000000 as libc::c_int as libc::c_long;
    }
    *__errno_location() = pthread_timedjoin_np(thread, retval, &mut thread_timeout);
    if *__errno_location() == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if *__errno_location() == 110 as libc::c_int {
        return 1 as libc::c_int;
    }
    perror(b"ERROR: in rc_pthread_timed_join: \0" as *const u8 as *const libc::c_char);
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn rc_pthread_print_properties(mut thread: pthread_t) -> libc::c_int {
    let mut policy: libc::c_int = 0;
    let mut params: sched_param = sched_param { sched_priority: 0 };
    if pthread_getschedparam(thread, &mut policy, &mut params) != 0 {
        perror(b"ERROR: pthread_getschedparam\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    printf(
        b"policy=%s, priority=%d\n\0" as *const u8 as *const libc::c_char,
        if policy == 1 as libc::c_int {
            b"SCHED_FIFO\0" as *const u8 as *const libc::c_char
        } else if policy == 2 as libc::c_int {
            b"SCHED_RR\0" as *const u8 as *const libc::c_char
        } else if policy == 0 as libc::c_int {
            b"SCHED_OTHER\0" as *const u8 as *const libc::c_char
        } else {
            b"???\0" as *const u8 as *const libc::c_char
        },
        params.sched_priority,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_pthread_set_properties_self(
    mut policy: libc::c_int,
    mut priority: libc::c_int,
) -> libc::c_int {
    let mut params: sched_param = sched_param { sched_priority: 0 };
    params.sched_priority = priority;
    *__errno_location() = pthread_setschedparam(pthread_self(), policy, &mut params);
    if *__errno_location() != 0 {
        perror(b"ERROR in rc_pthread_set_properties_self: \0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_pthread_get_process_niceness() -> libc::c_int {
    let mut which: libc::c_int = PRIO_PROCESS as libc::c_int;
    let mut pid: id_t = 0;
    let mut ret: libc::c_int = 0;
    pid = getpid() as id_t;
    *__errno_location() = 0 as libc::c_int;
    ret = getpriority(which as __priority_which_t, pid);
    if *__errno_location() != 0 {
        perror(b"ERROR in rc_pthread_get_process_niceness: \0" as *const u8 as *const libc::c_char);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn rc_pthread_set_process_niceness(mut niceness: libc::c_int) -> libc::c_int {
    let mut which: libc::c_int = PRIO_PROCESS as libc::c_int;
    let mut pid: id_t = 0;
    let mut ret: libc::c_int = 0;
    pid = getpid() as id_t;
    *__errno_location() = 0 as libc::c_int;
    ret = setpriority(which as __priority_which_t, pid, niceness);
    if *__errno_location() != 0 {
        perror(b"ERROR in rc_pthread_set_process_niceness: \0" as *const u8 as *const libc::c_char);
    }
    return ret;
}
