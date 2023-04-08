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
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn __errno_location() -> *mut libc::c_int;
    fn glob(
        __pattern: *const libc::c_char,
        __flags: libc::c_int,
        __errfunc: Option<unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int>,
        __pglob: *mut glob_t,
    ) -> libc::c_int;
    fn globfree(__pglob: *mut glob_t);
    fn rc_usleep(us: libc::c_uint);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
pub type __size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut libc::c_char,
    pub gl_offs: __size_t,
    pub gl_flags: libc::c_int,
    pub gl_closedir: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub gl_readdir: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    pub gl_opendir: Option<unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void>,
    pub gl_lstat: Option<unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> libc::c_int>,
    pub gl_stat: Option<unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> libc::c_int>,
}
static mut dutyA_fd: [libc::c_int; 3] = [0; 3];
static mut dutyB_fd: [libc::c_int; 3] = [0; 3];
static mut period_ns: [libc::c_uint; 3] = [0; 3];
static mut init_flag: [libc::c_int; 3] = [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int];
static mut mode: libc::c_int = 0;
static mut ssindex: [libc::c_int; 3] = [0; 3];
unsafe extern "C" fn __export_channels(mut ss: libc::c_int) -> libc::c_int {
    let mut export_fd: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut len: libc::c_int = 0;
    let mut globbuf: glob_t = glob_t {
        gl_pathc: 0,
        gl_pathv: 0 as *mut *mut libc::c_char,
        gl_offs: 0,
        gl_flags: 0,
        gl_closedir: None,
        gl_readdir: None,
        gl_opendir: None,
        gl_lstat: None,
        gl_stat: None,
    };
    len = snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b"/sys/devices/platform/ocp/4830%d000.epwmss/4830%d200.pwm/pwm/pwmchip*/export\0" as *const u8
            as *const libc::c_char,
        ss * 2 as libc::c_int,
        ss * 2 as libc::c_int,
    );
    glob(buf.as_mut_ptr(), 0 as libc::c_int, None, &mut globbuf);
    if globbuf.gl_pathc == 0 as libc::c_int as libc::c_ulong {
        perror(b"ERROR in rc_pwm_init, can't find pwm export file\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"Probably not running on BeagleBone or device tree not configured\n\0" as *const u8 as *const libc::c_char,
        );
        globfree(&mut globbuf);
        return -(1 as libc::c_int);
    }
    if globbuf.gl_pathc > 1 as libc::c_int as libc::c_ulong {
        perror(b"ERROR in rc_pwm_init, too many pwmchipx diectories found\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"please report this issue on robotcontrol github\n\0" as *const u8 as *const libc::c_char,
        );
        globfree(&mut globbuf);
        return -(1 as libc::c_int);
    }
    export_fd = open(
        *(globbuf.gl_pathv).offset(0 as libc::c_int as isize),
        0o1 as libc::c_int,
    );
    if (export_fd < 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        perror(b"ERROR in rc_pwm_init, can't open pwm export file for writing\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"tried opening: %s\n\0" as *const u8 as *const libc::c_char,
            *(globbuf.gl_pathv).offset(0 as libc::c_int as isize),
        );
        globfree(&mut globbuf);
        return -(1 as libc::c_int);
    }
    ssindex[ss as usize] =
        atoi((*(globbuf.gl_pathv).offset(0 as libc::c_int as isize)).offset(66 as libc::c_int as isize));
    globfree(&mut globbuf);
    len = write(
        export_fd,
        b"0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    ) as libc::c_int;
    if (len < 0 as libc::c_int && *__errno_location() != 16 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        perror(b"ERROR: in rc_pwm_init, failed to write 0 to export file\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    len = write(
        export_fd,
        b"1\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    ) as libc::c_int;
    if (len < 0 as libc::c_int && *__errno_location() != 16 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        perror(b"ERROR: in rc_pwm_init, failed to write 1 to export file\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    close(export_fd);
    len = snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b"/sys/class/pwm/pwmchip%d/pwm0/enable\0" as *const u8 as *const libc::c_char,
        ss * 2 as libc::c_int,
    );
    if access(buf.as_mut_ptr(), 0 as libc::c_int) == 0 as libc::c_int {
        mode = 0 as libc::c_int;
    } else {
        len = snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwm-%d:0/enable\0" as *const u8 as *const libc::c_char,
            ssindex[ss as usize],
        );
        if access(buf.as_mut_ptr(), 0 as libc::c_int) == 0 as libc::c_int {
            mode = 1 as libc::c_int;
        } else {
            fprintf(
                stderr,
                b"ERROR in rc_pwm_init, export failed for subsystem %d channel %d\n\0" as *const u8
                    as *const libc::c_char,
                ss,
                0 as libc::c_int,
            );
            fprintf(
                stderr,
                b"tried accessing %s\n\0" as *const u8 as *const libc::c_char,
                buf.as_mut_ptr(),
            );
            return -(1 as libc::c_int);
        }
    }
    if mode == 0 as libc::c_int {
        len = snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwmchip%d/pwm1/enable\0" as *const u8 as *const libc::c_char,
            ss * 2 as libc::c_int,
        );
    } else {
        len = snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwm-%d:1/enable\0" as *const u8 as *const libc::c_char,
            ssindex[ss as usize],
        );
    }
    if access(buf.as_mut_ptr(), 0 as libc::c_int) != 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_pwm_init, export failed for subsystem %d channel %d\n\0" as *const u8 as *const libc::c_char,
            ss,
            0 as libc::c_int,
        );
        fprintf(
            stderr,
            b"tried accessing %s\n\0" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __unexport_channels(mut ss: libc::c_int) -> libc::c_int {
    let mut unexport_fd: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut len: libc::c_int = 0;
    let mut globbuf: glob_t = glob_t {
        gl_pathc: 0,
        gl_pathv: 0 as *mut *mut libc::c_char,
        gl_offs: 0,
        gl_flags: 0,
        gl_closedir: None,
        gl_readdir: None,
        gl_opendir: None,
        gl_lstat: None,
        gl_stat: None,
    };
    len = snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b"/sys/devices/platform/ocp/4830%d000.epwmss/4830%d200.pwm/pwm/pwmchip*/unexport\0" as *const u8
            as *const libc::c_char,
        ss * 2 as libc::c_int,
        ss * 2 as libc::c_int,
    );
    glob(buf.as_mut_ptr(), 0 as libc::c_int, None, &mut globbuf);
    if globbuf.gl_pathc == 0 as libc::c_int as libc::c_ulong {
        perror(b"ERROR in rc_pwm_init, can't find pwm unexport file\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"Probably not running on BeagleBone or device tree not configured\n\0" as *const u8 as *const libc::c_char,
        );
        globfree(&mut globbuf);
        return -(1 as libc::c_int);
    }
    if globbuf.gl_pathc > 1 as libc::c_int as libc::c_ulong {
        perror(b"ERROR in rc_pwm_init, too many pwmchipx diectories found\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"please report this issue on robotcontrol github\n\0" as *const u8 as *const libc::c_char,
        );
        globfree(&mut globbuf);
        return -(1 as libc::c_int);
    }
    unexport_fd = open(
        *(globbuf.gl_pathv).offset(0 as libc::c_int as isize),
        0o1 as libc::c_int,
    );
    if (unexport_fd < 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        perror(b"ERROR in rc_pwm_init, can't open pwm unexport file for writing\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"tried opening: %s\n\0" as *const u8 as *const libc::c_char,
            *(globbuf.gl_pathv).offset(0 as libc::c_int as isize),
        );
        globfree(&mut globbuf);
        return -(1 as libc::c_int);
    }
    len = write(
        unexport_fd,
        b"0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    ) as libc::c_int;
    if (len < 0 as libc::c_int && *__errno_location() != 16 as libc::c_int && *__errno_location() != 19 as libc::c_int)
        as libc::c_int as libc::c_long
        != 0
    {
        perror(b"ERROR: in rc_pwm_init, failed to write 0 to unexport file\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    len = write(
        unexport_fd,
        b"1\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    ) as libc::c_int;
    if (len < 0 as libc::c_int && *__errno_location() != 16 as libc::c_int && *__errno_location() != 19 as libc::c_int)
        as libc::c_int as libc::c_long
        != 0
    {
        perror(b"ERROR: in rc_pwm_init, failed to write 1 to unexport file\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    close(unexport_fd);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_pwm_init(mut ss: libc::c_int, mut frequency: libc::c_int) -> libc::c_int {
    let mut periodA_fd: libc::c_int = 0;
    let mut periodB_fd: libc::c_int = 0;
    let mut enableA_fd: libc::c_int = 0;
    let mut enableB_fd: libc::c_int = 0;
    let mut polarityA_fd: libc::c_int = 0;
    let mut polarityB_fd: libc::c_int = 0;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut len: libc::c_int = 0;
    if ss < 0 as libc::c_int || ss > 2 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_pwm_init, PWM subsystem must be 0 1 or 2\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if frequency < 1 as libc::c_int || frequency > 1000000000 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_pwm_init, frequency must be between %dHz and %dHz\n\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            1000000000 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if __unexport_channels(ss) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if __export_channels(ss) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if mode == 0 as libc::c_int {
        len = snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwmchip%d/pwm0/duty_cycle\0" as *const u8 as *const libc::c_char,
            ss * 2 as libc::c_int,
        );
    } else {
        len = snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwm-%d:0/duty_cycle\0" as *const u8 as *const libc::c_char,
            ssindex[ss as usize],
        );
    }
    dutyA_fd[ss as usize] = open(buf.as_mut_ptr(), 0o1 as libc::c_int);
    if (dutyA_fd[ss as usize] == -(1 as libc::c_int)) as libc::c_int as libc::c_long != 0 {
        rc_usleep(600000 as libc::c_int as libc::c_uint);
        dutyA_fd[ss as usize] = open(buf.as_mut_ptr(), 0o1 as libc::c_int);
        if (dutyA_fd[ss as usize] == -(1 as libc::c_int)) as libc::c_int as libc::c_long != 0 {
            perror(
                b"ERROR in rc_pwm_init, failed to open duty_cycle channel A FD\0" as *const u8 as *const libc::c_char,
            );
            fprintf(
                stderr,
                b"tried accessing: %s\n\0" as *const u8 as *const libc::c_char,
                buf.as_mut_ptr(),
            );
            return -(1 as libc::c_int);
        }
    }
    if mode == 0 as libc::c_int {
        len = snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwmchip%d/pwm1/duty_cycle\0" as *const u8 as *const libc::c_char,
            ss * 2 as libc::c_int,
        );
    } else {
        len = snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwm-%d:1/duty_cycle\0" as *const u8 as *const libc::c_char,
            ssindex[ss as usize],
        );
    }
    dutyB_fd[ss as usize] = open(buf.as_mut_ptr(), 0o1 as libc::c_int);
    if (dutyB_fd[ss as usize] == -(1 as libc::c_int)) as libc::c_int as libc::c_long != 0 {
        perror(b"ERROR in rc_pwm_init, failed to open duty_cycle channel B FD\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"tried accessing: %s\n\0" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
        );
        return -(1 as libc::c_int);
    }
    if mode == 0 as libc::c_int {
        len = snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwmchip%d/pwm0/enable\0" as *const u8 as *const libc::c_char,
            ss * 2 as libc::c_int,
        );
    } else {
        len = snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwm-%d:0/enable\0" as *const u8 as *const libc::c_char,
            ssindex[ss as usize],
        );
    }
    enableA_fd = open(buf.as_mut_ptr(), 0o1 as libc::c_int);
    if (enableA_fd == -(1 as libc::c_int)) as libc::c_int as libc::c_long != 0 {
        perror(b"ERROR in rc_pwm_init, failed to open pwm A enable fd\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"tried accessing: %s\n\0" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
        );
        return -(1 as libc::c_int);
    }
    if mode == 0 as libc::c_int {
        len = snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwmchip%d/pwm1/enable\0" as *const u8 as *const libc::c_char,
            ss * 2 as libc::c_int,
        );
    } else {
        len = snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwm-%d:1/enable\0" as *const u8 as *const libc::c_char,
            ssindex[ss as usize],
        );
    }
    enableB_fd = open(buf.as_mut_ptr(), 0o1 as libc::c_int);
    if (enableB_fd == -(1 as libc::c_int)) as libc::c_int as libc::c_long != 0 {
        perror(b"ERROR in rc_pwm_init, failed to open pwm B enable fd\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"tried accessing: %s\n\0" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
        );
        return -(1 as libc::c_int);
    }
    if mode == 0 as libc::c_int {
        len = snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwmchip%d/pwm0/period\0" as *const u8 as *const libc::c_char,
            ss * 2 as libc::c_int,
        );
    } else {
        len = snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwm-%d:0/period\0" as *const u8 as *const libc::c_char,
            ssindex[ss as usize],
        );
    }
    periodA_fd = open(buf.as_mut_ptr(), 0o1 as libc::c_int);
    if (periodA_fd == -(1 as libc::c_int)) as libc::c_int as libc::c_long != 0 {
        perror(b"ERROR in rc_pwm_init, failed to open pwm A period fd\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"tried accessing: %s\n\0" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
        );
        return -(1 as libc::c_int);
    }
    if mode == 0 as libc::c_int {
        len = snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwmchip%d/pwm1/period\0" as *const u8 as *const libc::c_char,
            ss * 2 as libc::c_int,
        );
    } else {
        len = snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwm-%d:1/period\0" as *const u8 as *const libc::c_char,
            ssindex[ss as usize],
        );
    }
    periodB_fd = open(buf.as_mut_ptr(), 0o1 as libc::c_int);
    if (periodB_fd == -(1 as libc::c_int)) as libc::c_int as libc::c_long != 0 {
        perror(b"ERROR in rc_pwm_init, failed to open pwm B period fd\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"tried accessing: %s\n\0" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
        );
        return -(1 as libc::c_int);
    }
    if mode == 0 as libc::c_int {
        len = snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwmchip%d/pwm0/polarity\0" as *const u8 as *const libc::c_char,
            ss * 2 as libc::c_int,
        );
    } else {
        len = snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwm-%d:0/polarity\0" as *const u8 as *const libc::c_char,
            ssindex[ss as usize],
        );
    }
    polarityA_fd = open(buf.as_mut_ptr(), 0o1 as libc::c_int);
    if (polarityA_fd == -(1 as libc::c_int)) as libc::c_int as libc::c_long != 0 {
        perror(b"ERROR in rc_pwm_init, failed to open pwm A polarity fd\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"tried accessing: %s\n\0" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
        );
        return -(1 as libc::c_int);
    }
    if mode == 0 as libc::c_int {
        len = snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwmchip%d/pwm1/polarity\0" as *const u8 as *const libc::c_char,
            ss * 2 as libc::c_int,
        );
    } else {
        len = snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwm-%d:1/polarity\0" as *const u8 as *const libc::c_char,
            ssindex[ss as usize],
        );
    }
    polarityB_fd = open(buf.as_mut_ptr(), 0o1 as libc::c_int);
    if (polarityB_fd == -(1 as libc::c_int)) as libc::c_int as libc::c_long != 0 {
        perror(b"ERROR in rc_pwm_init, failed to open pwm B polarity fd\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"tried accessing: %s\n\0" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
        );
        return -(1 as libc::c_int);
    }
    period_ns[ss as usize] = (1000000000 as libc::c_int / frequency) as libc::c_uint;
    len = snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b"%d\0" as *const u8 as *const libc::c_char,
        period_ns[ss as usize],
    );
    if (write(periodA_fd, buf.as_mut_ptr() as *const libc::c_void, len as size_t)
        == -(1 as libc::c_int) as libc::c_long) as libc::c_int as libc::c_long
        != 0
    {
        perror(b"ERROR in rc_pwm_init, failed to write to channel A period fd\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if (write(periodB_fd, buf.as_mut_ptr() as *const libc::c_void, len as size_t)
        == -(1 as libc::c_int) as libc::c_long) as libc::c_int as libc::c_long
        != 0
    {
        perror(b"ERROR in rc_pwm_init, failed to write to channel B period fd\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if (write(
        polarityA_fd,
        b"normal\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        7 as libc::c_int as size_t,
    ) == -(1 as libc::c_int) as libc::c_long) as libc::c_int as libc::c_long
        != 0
    {
        perror(b"ERROR in rc_pwm_init, failed to write to channel A polarity fd\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if (write(
        polarityB_fd,
        b"normal\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        7 as libc::c_int as size_t,
    ) == -(1 as libc::c_int) as libc::c_long) as libc::c_int as libc::c_long
        != 0
    {
        perror(b"ERROR in rc_pwm_init, failed to write to channel B polarity fd\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if (write(
        dutyA_fd[ss as usize],
        b"0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    ) == -(1 as libc::c_int) as libc::c_long) as libc::c_int as libc::c_long
        != 0
    {
        perror(b"ERROR in rc_pwm_init, failed to write to channel A duty fd\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if (write(
        dutyB_fd[ss as usize],
        b"0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    ) == -(1 as libc::c_int) as libc::c_long) as libc::c_int as libc::c_long
        != 0
    {
        perror(b"ERROR in rc_pwm_init, failed to write to channel B duty fd\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if (write(
        enableA_fd,
        b"1\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    ) == -(1 as libc::c_int) as libc::c_long) as libc::c_int as libc::c_long
        != 0
    {
        perror(b"ERROR in rc_pwm_init, failed to write to channel A enable fd\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if (write(
        enableB_fd,
        b"1\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    ) == -(1 as libc::c_int) as libc::c_long) as libc::c_int as libc::c_long
        != 0
    {
        perror(b"ERROR in rc_pwm_init, failed to write to channel B enable fd\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    close(enableA_fd);
    close(enableB_fd);
    close(periodA_fd);
    close(periodB_fd);
    close(polarityA_fd);
    close(polarityB_fd);
    init_flag[ss as usize] = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_pwm_cleanup(mut ss: libc::c_int) -> libc::c_int {
    let mut enableA_fd: libc::c_int = 0;
    let mut enableB_fd: libc::c_int = 0;
    let mut buf: [libc::c_char; 128] = [0; 128];
    if (ss < 0 as libc::c_int || ss > 2 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_pwm_close, subsystem must be between 0 and 2\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if init_flag[ss as usize] == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if mode == 0 as libc::c_int {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwmchip%d/pwm0/enable\0" as *const u8 as *const libc::c_char,
            ss * 2 as libc::c_int,
        );
    } else {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwm-%d:0/enable\0" as *const u8 as *const libc::c_char,
            ssindex[ss as usize],
        );
    }
    enableA_fd = open(buf.as_mut_ptr(), 0o1 as libc::c_int);
    if (enableA_fd == -(1 as libc::c_int)) as libc::c_int as libc::c_long != 0 {
        perror(b"ERROR in rc_pwm_cleanup, failed to open pwm A enable fd\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if mode == 0 as libc::c_int {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwmchip%d/pwm1/enable\0" as *const u8 as *const libc::c_char,
            ss * 2 as libc::c_int,
        );
    } else {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/sys/class/pwm/pwm-%d:1/enable\0" as *const u8 as *const libc::c_char,
            ssindex[ss as usize],
        );
    }
    enableB_fd = open(buf.as_mut_ptr(), 0o1 as libc::c_int);
    if (enableB_fd == -(1 as libc::c_int)) as libc::c_int as libc::c_long != 0 {
        perror(b"ERROR in rc_pwm_cleanup, failed to open pwm B enable fd\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if (write(
        dutyA_fd[ss as usize],
        b"0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    ) == -(1 as libc::c_int) as libc::c_long) as libc::c_int as libc::c_long
        != 0
    {
        perror(b"ERROR in rc_pwm_init, failed to write to channel A duty fd\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if (write(
        dutyB_fd[ss as usize],
        b"0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    ) == -(1 as libc::c_int) as libc::c_long) as libc::c_int as libc::c_long
        != 0
    {
        perror(b"ERROR in rc_pwm_init, failed to write to channel B duty fd\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if (write(
        enableA_fd,
        b"0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    ) == -(1 as libc::c_int) as libc::c_long) as libc::c_int as libc::c_long
        != 0
    {
        perror(b"ERROR in rc_pwm_init, failed to write to channel A enable fd\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if (write(
        enableB_fd,
        b"0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    ) == -(1 as libc::c_int) as libc::c_long) as libc::c_int as libc::c_long
        != 0
    {
        perror(b"ERROR in rc_pwm_init, failed to write to channel A enable fd\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    close(enableA_fd);
    close(enableB_fd);
    close(dutyA_fd[ss as usize]);
    close(dutyB_fd[ss as usize]);
    __unexport_channels(ss);
    init_flag[ss as usize] = 0 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_pwm_set_duty(
    mut ss: libc::c_int,
    mut ch: libc::c_char,
    mut duty: libc::c_double,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut duty_ns: libc::c_int = 0;
    let mut buf: [libc::c_char; 128] = [0; 128];
    if (ss < 0 as libc::c_int || ss > 2 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_pwm_set_duty, PWM subsystem must be between 0 and 2\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (init_flag[ss as usize] == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_pwm_set_duty, subsystem %d not initialized yet\n\0" as *const u8 as *const libc::c_char,
            ss,
        );
        return -(1 as libc::c_int);
    }
    if (duty > 1.0f64 || duty < 0.0f64) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_pwm_set_duty, duty must be between 0.0f & 1.0f\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    duty_ns = (duty * period_ns[ss as usize] as libc::c_double) as libc::c_int;
    len = snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b"%d\0" as *const u8 as *const libc::c_char,
        duty_ns,
    );
    match ch as libc::c_int {
        65 => {
            ret = write(
                dutyA_fd[ss as usize],
                buf.as_mut_ptr() as *const libc::c_void,
                len as size_t,
            ) as libc::c_int;
        }
        66 => {
            ret = write(
                dutyB_fd[ss as usize],
                buf.as_mut_ptr() as *const libc::c_void,
                len as size_t,
            ) as libc::c_int;
        }
        _ => {
            fprintf(
                stderr,
                b"ERROR in rc_pwm_set_duty_ns, pwm channel must be 'A' or 'B'\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    if (ret == -(1 as libc::c_int)) as libc::c_int as libc::c_long != 0 {
        perror(b"ERROR in rc_pwm_set_duty_ns, failed to write to duty_cycle fd\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_pwm_set_duty_ns(
    mut ss: libc::c_int,
    mut ch: libc::c_char,
    mut duty_ns: libc::c_uint,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut buf: [libc::c_char; 128] = [0; 128];
    if (ss < 0 as libc::c_int || ss > 2 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_pwm_set_duty_ns, PWM subsystem must be between 0 and 2\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (init_flag[ss as usize] == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_pwm_set_duty_ns, subsystem %d not initialized yet\n\0" as *const u8 as *const libc::c_char,
            ss,
        );
        return -(1 as libc::c_int);
    }
    if (duty_ns > period_ns[ss as usize]) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_pwm_set_duty_ns, duty must be between 0 & %d for current frequency\n\0" as *const u8
                as *const libc::c_char,
            period_ns[ss as usize],
        );
        return -(1 as libc::c_int);
    }
    len = snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b"%d\0" as *const u8 as *const libc::c_char,
        duty_ns,
    );
    match ch as libc::c_int {
        65 => {
            ret = write(
                dutyA_fd[ss as usize],
                buf.as_mut_ptr() as *const libc::c_void,
                len as size_t,
            ) as libc::c_int;
        }
        66 => {
            ret = write(
                dutyB_fd[ss as usize],
                buf.as_mut_ptr() as *const libc::c_void,
                len as size_t,
            ) as libc::c_int;
        }
        _ => {
            fprintf(
                stderr,
                b"ERROR in rc_pwm_set_duty_ns, pwm channel must be 'A' or 'B'\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    if (ret == -(1 as libc::c_int)) as libc::c_int as libc::c_long != 0 {
        perror(b"ERROR in rc_pwm_set_duty_ns, failed to write to duty_cycle fd\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
