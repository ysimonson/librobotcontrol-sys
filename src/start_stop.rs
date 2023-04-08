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
    pub type __dirstream;
    static mut stderr: *mut FILE;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn sigaction(__sig: libc::c_int, __act: *const sigaction, __oact: *mut sigaction) -> libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn getpgid(__pid: __pid_t) -> __pid_t;
    fn __errno_location() -> *mut libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn rc_usleep(us: libc::c_uint);
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
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
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type C2RustUnnamed_9 = libc::c_uint;
pub const SEGV_MTESERR: C2RustUnnamed_9 = 9;
pub const SEGV_MTEAERR: C2RustUnnamed_9 = 8;
pub const SEGV_ADIPERR: C2RustUnnamed_9 = 7;
pub const SEGV_ADIDERR: C2RustUnnamed_9 = 6;
pub const SEGV_ACCADI: C2RustUnnamed_9 = 5;
pub const SEGV_PKUERR: C2RustUnnamed_9 = 4;
pub const SEGV_BNDERR: C2RustUnnamed_9 = 3;
pub const SEGV_ACCERR: C2RustUnnamed_9 = 2;
pub const SEGV_MAPERR: C2RustUnnamed_9 = 1;
pub type __sighandler_t = Option<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_10,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> ()>,
}
pub type DIR = __dirstream;
pub type rc_state_t = libc::c_uint;
pub const EXITING: rc_state_t = 3;
pub const PAUSED: rc_state_t = 2;
pub const RUNNING: rc_state_t = 1;
pub const UNINITIALIZED: rc_state_t = 0;
static mut rc_state: rc_state_t = UNINITIALIZED;
#[no_mangle]
pub unsafe extern "C" fn rc_get_state() -> rc_state_t {
    return rc_state;
}
#[no_mangle]
pub unsafe extern "C" fn rc_set_state(mut new_state: rc_state_t) {
    rc_state = new_state;
}
#[no_mangle]
pub unsafe extern "C" fn rc_print_state() -> libc::c_int {
    match rc_state as libc::c_uint {
        0 => {
            printf(b"UNINITIALIZED\0" as *const u8 as *const libc::c_char);
        }
        2 => {
            printf(b"PAUSED\0" as *const u8 as *const libc::c_char);
        }
        1 => {
            printf(b"RUNNING\0" as *const u8 as *const libc::c_char);
        }
        3 => {
            printf(b"EXITING\0" as *const u8 as *const libc::c_char);
        }
        _ => {
            fprintf(stderr, b"ERROR: invalid state\n\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_make_pid_file() -> libc::c_int {
    let mut fd: *mut FILE = 0 as *mut FILE;
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut current_pid: pid_t = 0;
    if access(
        b"/run/shm/robotcontrol.pid\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"ERROR: in rc_make_pid_file, file already exists, a new one was not written\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"You have either called this function twice, or you need to \n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"call rc_kill_existing_process BEFORE rc_make_pid_file\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    *__errno_location() = 0 as libc::c_int;
    dir = opendir(b"/run/shm/\0" as *const u8 as *const libc::c_char);
    if !dir.is_null() {
        closedir(dir);
    } else {
        perror(b"ERROR in rc_make_pid_file trying to open /run/shm/\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    fd = fopen(
        b"/run/shm/robotcontrol.pid\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if fd.is_null() {
        perror(b"ERROR in rc_make_pid_file\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    current_pid = getpid();
    fprintf(fd, b"%d\0" as *const u8 as *const libc::c_char, current_pid);
    fflush(fd);
    fclose(fd);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_kill_existing_process(mut timeout_s: libc::c_float) -> libc::c_int {
    let mut fd: *mut FILE = 0 as *mut FILE;
    let mut old_pid: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut num_checks: libc::c_int = 0;
    if timeout_s < 0.1f32 {
        fprintf(
            stderr,
            b"ERROR in rc_kill_existing_process, timeout_s must be >= 0.1f\n\0" as *const u8 as *const libc::c_char,
        );
        return -(4 as libc::c_int);
    }
    if access(
        b"/run/shm/robotcontrol.pid\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) != 0
    {
        return 0 as libc::c_int;
    }
    if access(
        b"/run/shm/robotcontrol.pid\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
    ) != 0
    {
        fprintf(
            stderr,
            b"ERROR, in rc_kill_existing_process, don't have write access \n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"to PID file. Existing process is probably running as root.\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"Try running 'sudo rc_kill'\n\0" as *const u8 as *const libc::c_char,
        );
        return -(3 as libc::c_int);
    }
    fd = fopen(
        b"/run/shm/robotcontrol.pid\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if fd.is_null() {
        fprintf(
            stderr,
            b"WARNING, in rc_kill_existing_process, PID file exists but is not\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"readable. Attempting to delete it.\n\0" as *const u8 as *const libc::c_char,
        );
        remove(b"/run/shm/robotcontrol.pid\0" as *const u8 as *const libc::c_char);
        return -(2 as libc::c_int);
    }
    ret = fscanf(
        fd,
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut old_pid as *mut libc::c_int,
    );
    fclose(fd);
    if ret != 1 as libc::c_int {
        fprintf(
            stderr,
            b"WARNING, in rc_kill_existing_process, PID file exists but contains\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"invalid contents. Attempting to delete it.\n\0" as *const u8 as *const libc::c_char,
        );
        remove(b"/run/shm/robotcontrol.pid\0" as *const u8 as *const libc::c_char);
        return -(2 as libc::c_int);
    }
    if old_pid == 0 as libc::c_int {
        fprintf(
            stderr,
            b"WARNING, in rc_kill_existing_process, PID file exists but contains\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"invalid contents. Attempting to delete it.\n\0" as *const u8 as *const libc::c_char,
        );
        remove(b"/run/shm/robotcontrol.pid\0" as *const u8 as *const libc::c_char);
        return -(2 as libc::c_int);
    }
    if old_pid == getpid() {
        return 0 as libc::c_int;
    }
    if getpgid(old_pid) < 0 as libc::c_int {
        remove(b"/run/shm/robotcontrol.pid\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    }
    if kill(old_pid, 2 as libc::c_int) == -(1 as libc::c_int) {
        if *__errno_location() == 1 as libc::c_int {
            fprintf(
                stderr,
                b"ERROR in rc_kill_existing_process, insufficient permissions to stop\n\0" as *const u8
                    as *const libc::c_char,
            );
            fprintf(
                stderr,
                b"en existing process which is probably running as root.\n\0" as *const u8 as *const libc::c_char,
            );
            fprintf(
                stderr,
                b"Try running 'sudo rc_kill' to stop it.\n\n\0" as *const u8 as *const libc::c_char,
            );
            return -(3 as libc::c_int);
        }
        remove(b"/run/shm/robotcontrol.pid\0" as *const u8 as *const libc::c_char);
        return -(2 as libc::c_int);
    }
    num_checks = (timeout_s / 0.1f32) as libc::c_int;
    i = 0 as libc::c_int;
    while i <= num_checks {
        if getpgid(old_pid) == -(1 as libc::c_int) {
            remove(b"/run/shm/robotcontrol.pid\0" as *const u8 as *const libc::c_char);
            return 1 as libc::c_int;
        } else {
            rc_usleep(100000 as libc::c_int as libc::c_uint);
        }
        i += 1;
    }
    kill(old_pid, 9 as libc::c_int);
    i = 0 as libc::c_int;
    while i <= num_checks {
        if getpgid(old_pid) == -(1 as libc::c_int) {
            remove(b"/run/shm/robotcontrol.pid\0" as *const u8 as *const libc::c_char);
            return 1 as libc::c_int;
        } else {
            rc_usleep(100000 as libc::c_int as libc::c_uint);
        }
        i += 1;
    }
    remove(b"/run/shm/robotcontrol.pid\0" as *const u8 as *const libc::c_char);
    fprintf(
        stderr,
        b"WARNING in rc_kill_existing_process, process failed to\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"close cleanly and had to be killed.\n\0" as *const u8 as *const libc::c_char,
    );
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn rc_remove_pid_file() -> libc::c_int {
    if access(
        b"/run/shm/robotcontrol.pid\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {
        return remove(b"/run/shm/robotcontrol.pid\0" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_enable_signal_handler() -> libc::c_int {
    let mut action: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_10 { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    action.__sigaction_handler.sa_sigaction = None;
    action.__sigaction_handler.sa_handler = Some(shutdown_signal_handler as unsafe extern "C" fn(libc::c_int) -> ());
    if sigaction(2 as libc::c_int, &mut action, 0 as *mut sigaction) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: failed to set sigaction\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if sigaction(15 as libc::c_int, &mut action, 0 as *mut sigaction) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: failed to set sigaction\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if sigaction(1 as libc::c_int, &mut action, 0 as *mut sigaction) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: failed to set sigaction\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    action.__sigaction_handler.sa_handler = None;
    action.__sigaction_handler.sa_sigaction =
        Some(segfault_handler as unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> ());
    action.sa_flags = (4 as libc::c_int as libc::c_uint | 0x80000000 as libc::c_uint) as libc::c_int;
    if sigaction(11 as libc::c_int, &mut action, 0 as *mut sigaction) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: failed to set sigaction\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_disable_signal_handler() -> libc::c_int {
    let mut action: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_10 { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    action.__sigaction_handler.sa_handler = None;
    if sigaction(2 as libc::c_int, &mut action, 0 as *mut sigaction) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: failed to set sigaction\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if sigaction(15 as libc::c_int, &mut action, 0 as *mut sigaction) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: failed to set sigaction\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if sigaction(1 as libc::c_int, &mut action, 0 as *mut sigaction) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: failed to set sigaction\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if sigaction(11 as libc::c_int, &mut action, 0 as *mut sigaction) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: failed to set sigaction\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn segfault_handler(
    mut _signum: libc::c_int,
    mut info: *mut siginfo_t,
    mut _context: *mut libc::c_void,
) {
    fprintf(
        stderr,
        b"ERROR: Segmentation Fault\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"Fault address: %p\n\0" as *const u8 as *const libc::c_char,
        (*info)._sifields._sigfault.si_addr,
    );
    match (*info).si_code {
        1 => {
            fprintf(stderr, b"Address not mapped.\n\0" as *const u8 as *const libc::c_char);
        }
        2 => {
            fprintf(
                stderr,
                b"Access to this address is not allowed.\n\0" as *const u8 as *const libc::c_char,
            );
        }
        _ => {
            fprintf(stderr, b"Unknown reason.\n\0" as *const u8 as *const libc::c_char);
        }
    }
    rc_set_state(EXITING);
}
unsafe extern "C" fn shutdown_signal_handler(mut signo: libc::c_int) {
    match signo {
        2 => {
            rc_set_state(EXITING);
            printf(b"\nreceived SIGINT Ctrl-C\n\0" as *const u8 as *const libc::c_char);
        }
        15 => {
            rc_set_state(EXITING);
            printf(b"\nreceived SIGTERM\n\0" as *const u8 as *const libc::c_char);
        }
        1 | _ => {}
    };
}
