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
    fn perror(__s: *const libc::c_char);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn rc_gpio_get_value(chip: libc::c_int, pin: libc::c_int) -> libc::c_int;
    fn rc_gpio_init_event(
        chip: libc::c_int,
        pin: libc::c_int,
        handle_flags: libc::c_int,
        event_flags: libc::c_int,
    ) -> libc::c_int;
    fn rc_gpio_poll(
        chip: libc::c_int,
        pin: libc::c_int,
        timeout_ms: libc::c_int,
        event_time_ns: *mut uint64_t,
    ) -> libc::c_int;
    fn rc_usleep(us: libc::c_uint);
    fn pthread_mutex_init(__mutex: *mut pthread_mutex_t, __mutexattr: *const pthread_mutexattr_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn rc_pthread_timed_join(
        thread: pthread_t,
        retval: *mut *mut libc::c_void,
        timeout_sec: libc::c_float,
    ) -> libc::c_int;
    fn rc_pthread_create(
        thread: *mut pthread_t,
        func: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        arg: *mut libc::c_void,
        policy: libc::c_int,
        priority: libc::c_int,
    ) -> libc::c_int;
    fn pthread_cond_wait(__cond: *mut pthread_cond_t, __mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_init(__cond: *mut pthread_cond_t, __cond_attr: *const pthread_condattr_t) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: libc::c_ulonglong,
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct btn_cfg_t {
    pub press_cb: Option<unsafe extern "C" fn() -> ()>,
    pub release_cb: Option<unsafe extern "C" fn() -> ()>,
    pub poll_thread: pthread_t,
    pub started: libc::c_char,
    pub pol: libc::c_char,
    pub press_mutex: pthread_mutex_t,
    pub press_condition: pthread_cond_t,
    pub release_mutex: pthread_mutex_t,
    pub release_condition: pthread_cond_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread_cfg_t {
    pub chip: libc::c_int,
    pub pin: libc::c_int,
    pub pol: libc::c_int,
    pub debounce: libc::c_int,
}
static mut cfg: [[*mut btn_cfg_t; 64]; 6] = [[0 as *const btn_cfg_t as *mut btn_cfg_t; 64]; 6];
static mut shutdown_flag: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn poll_thread_func(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut val: libc::c_int = 0;
    let mut event: libc::c_int = 0;
    let mut chip: libc::c_int = 0;
    let mut pin: libc::c_int = 0;
    let mut pol: libc::c_int = 0;
    let mut debounce: libc::c_int = 0;
    let mut press_expected_event: libc::c_int = 0;
    let mut release_expected_event: libc::c_int = 0;
    let mut press_thread: pthread_t = 0;
    let mut release_thread: pthread_t = 0;
    let mut thread_cfg: thread_cfg_t = *(arg as *mut thread_cfg_t);
    chip = thread_cfg.chip;
    pin = thread_cfg.pin;
    pol = thread_cfg.pol;
    debounce = thread_cfg.debounce;
    (*cfg[chip as usize][pin as usize]).started = 1 as libc::c_int as libc::c_char;
    if pol == 1 as libc::c_int {
        press_expected_event = 2 as libc::c_int;
        release_expected_event = 1 as libc::c_int;
    } else {
        press_expected_event = 1 as libc::c_int;
        release_expected_event = 2 as libc::c_int;
    }
    while shutdown_flag == 0 {
        event = rc_gpio_poll(chip, pin, 500 as libc::c_int, 0 as *mut uint64_t);
        if event == -(1 as libc::c_int) {
            fprintf(
                stderr,
                b"ERROR in rc_button handler thread\n\0" as *const u8 as *const libc::c_char,
            );
            return 0 as *mut libc::c_void;
        }
        if event == 0 as libc::c_int {
            continue;
        }
        if debounce != 0 {
            rc_usleep(debounce as libc::c_uint);
            val = rc_gpio_get_value(chip, pin);
            if val == -(1 as libc::c_int) {
                fprintf(
                    stderr,
                    b"ERROR in rc_button handler thread\n\0" as *const u8 as *const libc::c_char,
                );
                return 0 as *mut libc::c_void;
            }
            if event == 2 as libc::c_int && val != 0 as libc::c_int {
                continue;
            }
            if event == 1 as libc::c_int && val != 1 as libc::c_int {
                continue;
            }
        }
        if event == press_expected_event {
            if ((*cfg[chip as usize][pin as usize]).press_cb).is_some() {
                rc_pthread_create(
                    &mut press_thread,
                    ::core::mem::transmute::<
                        Option<unsafe extern "C" fn() -> ()>,
                        Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
                    >((*cfg[chip as usize][pin as usize]).press_cb),
                    0 as *mut libc::c_void,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
            }
            pthread_mutex_lock(
                &mut (**(*cfg.as_mut_ptr().offset(chip as isize))
                    .as_mut_ptr()
                    .offset(pin as isize))
                .press_mutex,
            );
            pthread_cond_broadcast(
                &mut (**(*cfg.as_mut_ptr().offset(chip as isize))
                    .as_mut_ptr()
                    .offset(pin as isize))
                .press_condition,
            );
            pthread_mutex_unlock(
                &mut (**(*cfg.as_mut_ptr().offset(chip as isize))
                    .as_mut_ptr()
                    .offset(pin as isize))
                .press_mutex,
            );
        } else if event == release_expected_event {
            if ((*cfg[chip as usize][pin as usize]).release_cb).is_some() {
                rc_pthread_create(
                    &mut release_thread,
                    ::core::mem::transmute::<
                        Option<unsafe extern "C" fn() -> ()>,
                        Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
                    >((*cfg[chip as usize][pin as usize]).release_cb),
                    0 as *mut libc::c_void,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
            }
            pthread_mutex_lock(
                &mut (**(*cfg.as_mut_ptr().offset(chip as isize))
                    .as_mut_ptr()
                    .offset(pin as isize))
                .release_mutex,
            );
            pthread_cond_broadcast(
                &mut (**(*cfg.as_mut_ptr().offset(chip as isize))
                    .as_mut_ptr()
                    .offset(pin as isize))
                .release_condition,
            );
            pthread_mutex_unlock(
                &mut (**(*cfg.as_mut_ptr().offset(chip as isize))
                    .as_mut_ptr()
                    .offset(pin as isize))
                .release_mutex,
            );
        }
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn rc_button_init(
    mut chip: libc::c_int,
    mut pin: libc::c_int,
    mut polarity: libc::c_char,
    mut debounce_us: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ptr: *mut btn_cfg_t = 0 as *mut btn_cfg_t;
    let mut thread_cfg: thread_cfg_t = thread_cfg_t {
        chip: 0,
        pin: 0,
        pol: 0,
        debounce: 0,
    };
    if chip < 0 as libc::c_int || chip >= 6 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_button_init, chip out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if pin < 0 as libc::c_int || pin >= 64 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_button_init, pin out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if polarity as libc::c_int != 0 as libc::c_int && polarity as libc::c_int != 1 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_button_init\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"polarity must be RC_BTN_POLARITY_NORM_LOW or RC_BTN_POLARITY_NORM_HIGH\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if debounce_us < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_button_init, debounce_us must be >=0\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if !ptr.is_null() {
        fprintf(
            stderr,
            b"ERROR in rc_button_init, button already initialized\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_gpio_init_event(
        chip,
        pin,
        ((1 as libc::c_ulong) << 0 as libc::c_int) as libc::c_int,
        ((1 as libc::c_ulong) << 0 as libc::c_int | (1 as libc::c_ulong) << 1 as libc::c_int) as libc::c_int,
    ) == -(1 as libc::c_int)
    {
        fprintf(
            stderr,
            b"ERROR: in rc_button_init, failed to setup GPIO pin\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    ptr = malloc(::core::mem::size_of::<btn_cfg_t>() as libc::c_ulong) as *mut btn_cfg_t;
    if ptr.is_null() {
        perror(b"ERROR in rc_button_init\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    (*ptr).press_cb = None;
    (*ptr).release_cb = None;
    (*ptr).poll_thread = 0 as libc::c_int as pthread_t;
    (*ptr).started = 0 as libc::c_int as libc::c_char;
    (*ptr).pol = polarity;
    pthread_mutex_init(&mut (*ptr).press_mutex, 0 as *const pthread_mutexattr_t);
    pthread_cond_init(&mut (*ptr).press_condition, 0 as *const pthread_condattr_t);
    pthread_mutex_init(&mut (*ptr).release_mutex, 0 as *const pthread_mutexattr_t);
    pthread_cond_init(&mut (*ptr).release_condition, 0 as *const pthread_condattr_t);
    thread_cfg.chip = chip;
    thread_cfg.pin = pin;
    thread_cfg.pol = polarity as libc::c_int;
    thread_cfg.debounce = debounce_us;
    shutdown_flag = 0 as libc::c_int;
    cfg[chip as usize][pin as usize] = ptr;
    if rc_pthread_create(
        &mut (*ptr).poll_thread,
        Some(poll_thread_func as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        &mut thread_cfg as *mut thread_cfg_t as *mut libc::c_void,
        0 as libc::c_int,
        0 as libc::c_int,
    ) != 0
    {
        fprintf(
            stderr,
            b"ERROR in rc_button_init, failed to start press handler thread\n\0" as *const u8 as *const libc::c_char,
        );
        cfg[chip as usize][pin as usize] = 0 as *mut btn_cfg_t;
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while (*ptr).started as libc::c_int == 0 as libc::c_int {
        i += 1;
        if i >= 100 as libc::c_int {
            fprintf(
                stderr,
                b"ERROR in rc_button_init, timeout waiting for thread to start\n\0" as *const u8 as *const libc::c_char,
            );
            cfg[chip as usize][pin as usize] = 0 as *mut btn_cfg_t;
            return -(1 as libc::c_int);
        }
        rc_usleep(1000 as libc::c_int as libc::c_uint);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_button_cleanup() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    shutdown_flag = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 64 as libc::c_int {
            if !(cfg[i as usize][j as usize]).is_null() {
                ret = rc_pthread_timed_join(
                    (*cfg[i as usize][j as usize]).poll_thread,
                    0 as *mut *mut libc::c_void,
                    3.0f64 as libc::c_float,
                );
                if ret == -(1 as libc::c_int) {
                    fprintf(
                        stderr,
                        b"WARNING in rc_button_cleanup, problem joining button handler thread for pin %d\n\0"
                            as *const u8 as *const libc::c_char,
                        i,
                    );
                } else if ret == 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"WARNING in rc_button_cleanup, thread exit timeout for pin %d\n\0" as *const u8
                            as *const libc::c_char,
                        i,
                    );
                    fprintf(
                        stderr,
                        b"most likely cause is your button press callback function is stuck and didn't return\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                free(cfg[i as usize][j as usize] as *mut libc::c_void);
                cfg[i as usize][j as usize] = 0 as *mut btn_cfg_t;
            }
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rc_button_set_callbacks(
    mut chip: libc::c_int,
    mut pin: libc::c_int,
    mut press_func: Option<unsafe extern "C" fn() -> ()>,
    mut release_func: Option<unsafe extern "C" fn() -> ()>,
) -> libc::c_int {
    if chip < 0 as libc::c_int || chip >= 6 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_button_set_callbacks, chip out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if pin < 0 as libc::c_int || pin >= 64 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_button_set_callbacks, pin out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (cfg[chip as usize][pin as usize]).is_null() {
        fprintf(
            stderr,
            b"ERROR in rc_button_set_callbacks, pin not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*cfg[chip as usize][pin as usize]).press_cb = press_func;
    (*cfg[chip as usize][pin as usize]).release_cb = release_func;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_button_get_state(mut chip: libc::c_int, mut pin: libc::c_int) -> libc::c_int {
    let mut val: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if chip < 0 as libc::c_int || chip >= 6 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_button_get_state, chip out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if pin < 0 as libc::c_int || pin >= 64 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_button_get_state, pin out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (cfg[chip as usize][pin as usize]).is_null() {
        fprintf(
            stderr,
            b"ERROR in rc_button_get_state, pin not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    val = rc_gpio_get_value(chip, pin);
    if val == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"ERROR in rc_button_get_state\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*cfg[chip as usize][pin as usize]).pol as libc::c_int == 1 as libc::c_int {
        if val != 0 {
            ret = 0 as libc::c_int;
        } else {
            ret = 1 as libc::c_int;
        }
    } else if val != 0 {
        ret = 1 as libc::c_int;
    } else {
        ret = 0 as libc::c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn rc_button_wait_for_event(
    mut chip: libc::c_int,
    mut pin: libc::c_int,
    mut press_or_release: libc::c_int,
) -> libc::c_int {
    if chip < 0 as libc::c_int || chip >= 6 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_button_wait_for_event, chip out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if pin < 0 as libc::c_int || pin >= 64 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_button_wait_for_event, pin out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (cfg[chip as usize][pin as usize]).is_null() {
        fprintf(
            stderr,
            b"ERROR in rc_button_wait_for_event, pin not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if press_or_release == 1 as libc::c_int {
        pthread_mutex_lock(
            &mut (**(*cfg.as_mut_ptr().offset(chip as isize))
                .as_mut_ptr()
                .offset(pin as isize))
            .press_mutex,
        );
        pthread_cond_wait(
            &mut (**(*cfg.as_mut_ptr().offset(chip as isize))
                .as_mut_ptr()
                .offset(pin as isize))
            .press_condition,
            &mut (**(*cfg.as_mut_ptr().offset(chip as isize))
                .as_mut_ptr()
                .offset(pin as isize))
            .press_mutex,
        );
        pthread_mutex_unlock(
            &mut (**(*cfg.as_mut_ptr().offset(chip as isize))
                .as_mut_ptr()
                .offset(pin as isize))
            .press_mutex,
        );
    } else if press_or_release == 0 as libc::c_int {
        pthread_mutex_lock(
            &mut (**(*cfg.as_mut_ptr().offset(chip as isize))
                .as_mut_ptr()
                .offset(pin as isize))
            .release_mutex,
        );
        pthread_cond_wait(
            &mut (**(*cfg.as_mut_ptr().offset(chip as isize))
                .as_mut_ptr()
                .offset(pin as isize))
            .release_condition,
            &mut (**(*cfg.as_mut_ptr().offset(chip as isize))
                .as_mut_ptr()
                .offset(pin as isize))
            .release_mutex,
        );
        pthread_mutex_unlock(
            &mut (**(*cfg.as_mut_ptr().offset(chip as isize))
                .as_mut_ptr()
                .offset(pin as isize))
            .release_mutex,
        );
    } else {
        fprintf(
            stderr,
            b"ERROR in rc_button_wait_for_event, argument should be RC_BTN_STATE_PRESSED or RC_BTN_STATE_RELEASED\n\0"
                as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
