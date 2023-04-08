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
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn rc_model() -> rc_model_t;
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
pub type rc_model_t = libc::c_uint;
pub const MODEL_PC: rc_model_t = 18;
pub const MODEL_RPI_CM3: rc_model_t = 17;
pub const MODEL_RPI_CM: rc_model_t = 16;
pub const MODEL_RPI0_W: rc_model_t = 15;
pub const MODEL_RPI0: rc_model_t = 14;
pub const MODEL_RPI3_B_PLUS: rc_model_t = 13;
pub const MODEL_RPI3_B: rc_model_t = 12;
pub const MODEL_RPI2_B: rc_model_t = 11;
pub const MODEL_RPI_B_PLUS: rc_model_t = 10;
pub const MODEL_RPI_B: rc_model_t = 9;
pub const MODEL_BB_POCKET: rc_model_t = 8;
pub const MODEL_BB_BLUE: rc_model_t = 7;
pub const MODEL_BB_GREEN_W: rc_model_t = 6;
pub const MODEL_BB_GREEN: rc_model_t = 5;
pub const MODEL_BB_BLACK_W_RC: rc_model_t = 4;
pub const MODEL_BB_BLACK_W: rc_model_t = 3;
pub const MODEL_BB_BLACK_RC: rc_model_t = 2;
pub const MODEL_BB_BLACK: rc_model_t = 1;
pub const MODEL_UNKNOWN: rc_model_t = 0;
static mut fd: [libc::c_int; 3] = [0; 3];
static mut init_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn rc_encoder_eqep_init() -> libc::c_int {
    let mut temp_fd: libc::c_int = 0;
    if init_flag != 0 {
        return 0 as libc::c_int;
    }
    temp_fd = open(
        b"/sys/devices/platform/ocp/48300000.epwmss/48300180.eqep/enabled\0" as *const u8 as *const libc::c_char,
        0o1 as libc::c_int,
    );
    if temp_fd < 0 as libc::c_int {
        perror(b"ERROR in rc_encoder_eqep_init, failed to open device driver\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"Perhaps kernel or device tree is too old\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if write(
        temp_fd,
        b"1\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    ) == -(1 as libc::c_int) as libc::c_long
    {
        perror(b"ERROR in rc_encoder_eqep_init, failed to enable device driver\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    close(temp_fd);
    temp_fd = open(
        b"/sys/devices/platform/ocp/48300000.epwmss/48300180.eqep/position\0" as *const u8 as *const libc::c_char,
        0o2 as libc::c_int,
    );
    if temp_fd < 0 as libc::c_int {
        perror(b"ERROR in rc_encoder_eqep_init, failed to open device driver\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"Perhaps kernel or device tree is too old\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if write(
        temp_fd,
        b"0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    ) == -(1 as libc::c_int) as libc::c_long
    {
        perror(b"ERROR in rc_encoder_eqep_init, failed to zero out position\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    fd[0 as libc::c_int as usize] = temp_fd;
    if rc_model() as libc::c_uint != MODEL_BB_POCKET as libc::c_int as libc::c_uint {
        temp_fd = open(
            b"/sys/devices/platform/ocp/48302000.epwmss/48302180.eqep/enabled\0" as *const u8 as *const libc::c_char,
            0o1 as libc::c_int,
        );
        if temp_fd < 0 as libc::c_int {
            perror(
                b"ERROR in rc_encoder_eqep_init, failed to open device driver\0" as *const u8 as *const libc::c_char,
            );
            fprintf(
                stderr,
                b"Perhaps kernel or device tree is too old\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if write(
            temp_fd,
            b"1\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as size_t,
        ) == -(1 as libc::c_int) as libc::c_long
        {
            perror(
                b"ERROR in rc_encoder_eqep_init, failed to enable device driver\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        close(temp_fd);
        temp_fd = open(
            b"/sys/devices/platform/ocp/48302000.epwmss/48302180.eqep/position\0" as *const u8 as *const libc::c_char,
            0o2 as libc::c_int,
        );
        if temp_fd < 0 as libc::c_int {
            perror(
                b"ERROR in rc_encoder_eqep_init, failed to open device driver\0" as *const u8 as *const libc::c_char,
            );
            fprintf(
                stderr,
                b"Perhaps kernel or device tree is too old\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if write(
            temp_fd,
            b"0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as size_t,
        ) == -(1 as libc::c_int) as libc::c_long
        {
            perror(b"ERROR in rc_encoder_eqep_init, failed to zero out position\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        }
    }
    fd[1 as libc::c_int as usize] = temp_fd;
    temp_fd = open(
        b"/sys/devices/platform/ocp/48304000.epwmss/48304180.eqep/enabled\0" as *const u8 as *const libc::c_char,
        0o1 as libc::c_int,
    );
    if temp_fd < 0 as libc::c_int {
        perror(b"ERROR in rc_encoder_eqep_init, failed to open device driver\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"Perhaps kernel or device tree is too old\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if write(
        temp_fd,
        b"1\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    ) == -(1 as libc::c_int) as libc::c_long
    {
        perror(b"ERROR in rc_encoder_eqep_init, failed to enable device driver\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    close(temp_fd);
    temp_fd = open(
        b"/sys/devices/platform/ocp/48304000.epwmss/48304180.eqep/position\0" as *const u8 as *const libc::c_char,
        0o2 as libc::c_int,
    );
    if temp_fd < 0 as libc::c_int {
        perror(b"ERROR in rc_encoder_eqep_init, failed to open device driver\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"Perhaps kernel or device tree is too old\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if write(
        temp_fd,
        b"0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    ) == -(1 as libc::c_int) as libc::c_long
    {
        perror(b"ERROR in rc_encoder_eqep_init, failed to zero out position\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    fd[2 as libc::c_int as usize] = temp_fd;
    init_flag = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_encoder_eqep_cleanup() -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        close(fd[i as usize]);
        i += 1;
    }
    init_flag = 0 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_encoder_eqep_read(mut ch: libc::c_int) -> libc::c_int {
    let mut buf: [libc::c_char; 12] = [0; 12];
    if (init_flag == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_encoder_eqep_read, please initialize with rc_encoder_eqep_init() first\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (ch == 4 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_encoder_eqep_read, channel 4 is read by the PRU, use rc_encoder_pru_read instead\n\0"
                as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (ch < 1 as libc::c_int || ch > 3 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_encoder_eqep_read, encoder channel must be between 1 & 3 inclusive\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (lseek(
        fd[(ch - 1 as libc::c_int) as usize],
        0 as libc::c_int as __off_t,
        0 as libc::c_int,
    ) < 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long
        != 0
    {
        perror(
            b"ERROR: in rc_encoder_eqep_read, failed to seek to beginning of fd\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (read(
        fd[(ch - 1 as libc::c_int) as usize],
        buf.as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    ) == -(1 as libc::c_int) as libc::c_long) as libc::c_int as libc::c_long
        != 0
    {
        perror(b"ERROR in rc_encoder_eqep_read, can't read position fd\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return atoi(buf.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn rc_encoder_eqep_write(mut ch: libc::c_int, mut pos: libc::c_int) -> libc::c_int {
    let mut buf: [libc::c_char; 12] = [0; 12];
    if (init_flag == 0) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_encoder_eqep_write, please initialize with rc_encoder_eqep_init() first\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (ch == 4 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_encoder_eqep_write, channel 4 is read by the PRU, use rc_encoder_pru_write instead\n\0"
                as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (ch < 1 as libc::c_int || ch > 3 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_encoder_eqep_write, encoder channel must be between 1 & 3 inclusive\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (lseek(
        fd[(ch - 1 as libc::c_int) as usize],
        0 as libc::c_int as __off_t,
        0 as libc::c_int,
    ) < 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long
        != 0
    {
        perror(
            b"ERROR: in rc_encoder_eqep_write, failed to seek to beginning of fd\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    sprintf(buf.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, pos);
    if (write(
        fd[(ch - 1 as libc::c_int) as usize],
        buf.as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    ) == -(1 as libc::c_int) as libc::c_long) as libc::c_int as libc::c_long
        != 0
    {
        perror(b"ERROR in rc_encoder_eqep_write, can't write position fd\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
