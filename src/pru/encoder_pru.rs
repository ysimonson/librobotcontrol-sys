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
    fn rc_pru_start(ch: libc::c_int, fw_name: *const libc::c_char) -> libc::c_int;
    fn rc_pru_shared_mem_ptr() -> *mut uint32_t;
    fn rc_pru_stop(ch: libc::c_int) -> libc::c_int;
    fn rc_usleep(us: libc::c_uint);
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
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
pub type uint32_t = __uint32_t;
static mut shared_mem_32bit_ptr: *mut libc::c_uint = 0 as *const libc::c_uint as *mut libc::c_uint;
static mut init_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn rc_encoder_pru_init() -> libc::c_int {
    let mut i: libc::c_int = 0;
    shared_mem_32bit_ptr = rc_pru_shared_mem_ptr();
    if shared_mem_32bit_ptr.is_null() {
        fprintf(
            stderr,
            b"ERROR in rc_encoder_pru_init, failed to map shared memory pointer\n\0" as *const u8
                as *const libc::c_char,
        );
        init_flag = 0 as libc::c_int;
        return -(1 as libc::c_int);
    }
    ::core::ptr::write_volatile(
        shared_mem_32bit_ptr.offset(16 as libc::c_int as isize),
        42 as libc::c_int as libc::c_uint,
    );
    if rc_pru_start(
        0 as libc::c_int,
        b"am335x-pru0-rc-encoder-fw\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        fprintf(
            stderr,
            b"ERROR in rc_encoder_pru_init, failed to start PRU%d\n\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < 40 as libc::c_int {
        if *shared_mem_32bit_ptr.offset(16 as libc::c_int as isize) == 0 as libc::c_int as libc::c_uint {
            init_flag = 1 as libc::c_int;
            return 0 as libc::c_int;
        }
        rc_usleep(100000 as libc::c_int as libc::c_uint);
        i += 1;
    }
    fprintf(
        stderr,
        b"ERROR in rc_encoder_pru_init, %s failed to load\n\0" as *const u8 as *const libc::c_char,
        b"am335x-pru0-rc-encoder-fw\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"attempting to stop PRU%d\n\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    rc_pru_stop(0 as libc::c_int);
    init_flag = 0 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn rc_encoder_pru_cleanup() {
    if !shared_mem_32bit_ptr.is_null() {
        ::core::ptr::write_volatile(
            shared_mem_32bit_ptr.offset(16 as libc::c_int as isize),
            0 as libc::c_int as libc::c_uint,
        );
    }
    rc_pru_stop(0 as libc::c_int);
    shared_mem_32bit_ptr = 0 as *mut libc::c_uint;
    init_flag = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_encoder_pru_read() -> libc::c_int {
    if shared_mem_32bit_ptr.is_null() || init_flag == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_encoder_pru_read, call rc_encoder_pru_init first\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return *shared_mem_32bit_ptr.offset(16 as libc::c_int as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_encoder_pru_write(mut pos: libc::c_int) -> libc::c_int {
    if shared_mem_32bit_ptr.is_null() || init_flag == 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_encoder_pru_write, call rc_encoder_pru_init first\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    ::core::ptr::write_volatile(
        shared_mem_32bit_ptr.offset(16 as libc::c_int as isize),
        pos as libc::c_uint,
    );
    return 0 as libc::c_int;
}
