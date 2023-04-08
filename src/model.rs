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
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE) -> *mut libc::c_char;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn system(__command: *const libc::c_char) -> libc::c_int;
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
pub type rc_model_category_t = libc::c_uint;
pub const CATEGORY_PC: rc_model_category_t = 3;
pub const CATEGORY_RPI: rc_model_category_t = 2;
pub const CATEGORY_BEAGLEBONE: rc_model_category_t = 1;
pub const CATEGORY_UNKNOWN: rc_model_category_t = 0;
static mut current_model: rc_model_t = MODEL_UNKNOWN;
static mut current_category: rc_model_category_t = CATEGORY_UNKNOWN;
static mut has_checked: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn __check_model() {
    let mut c: [libc::c_char; 128] = [0; 128];
    let mut ret: libc::c_int = 0;
    let mut fd: *mut FILE = 0 as *mut FILE;
    current_model = MODEL_UNKNOWN;
    current_category = CATEGORY_UNKNOWN;
    has_checked = 1 as libc::c_int;
    ret = system(b"uname -m | grep -q x86\0" as *const u8 as *const libc::c_char);
    if ret == 0 as libc::c_int {
        current_model = MODEL_PC;
        current_category = CATEGORY_PC;
        return;
    }
    fd = fopen(
        b"/proc/device-tree/model\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if fd.is_null() {
        return;
    }
    memset(
        c.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        128 as libc::c_int as libc::c_ulong,
    );
    if (fgets(c.as_mut_ptr(), 128 as libc::c_int, fd)).is_null() {
        perror(b"ERROR in rc_model reading /proc/device-tree/model\0" as *const u8 as *const libc::c_char);
        fclose(fd);
        return;
    }
    fclose(fd);
    if !(strstr(c.as_mut_ptr(), b"BeagleBone\0" as *const u8 as *const libc::c_char)).is_null()
        || !(strstr(c.as_mut_ptr(), b"PocketBeagle\0" as *const u8 as *const libc::c_char)).is_null()
    {
        current_category = CATEGORY_BEAGLEBONE;
        if strcmp(
            c.as_mut_ptr(),
            b"TI AM335x BeagleBone Black\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            current_model = MODEL_BB_BLACK;
            return;
        }
        if strcmp(
            c.as_mut_ptr(),
            b"TI AM335x BeagleBone Black RoboticsCape\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            current_model = MODEL_BB_BLACK_RC;
            return;
        }
        if strcmp(
            c.as_mut_ptr(),
            b"TI AM335x BeagleBone Black Wireless\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            current_model = MODEL_BB_BLACK_W;
            return;
        }
        if strcmp(
            c.as_mut_ptr(),
            b"TI AM335x BeagleBone Black Wireless RoboticsCape\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            current_model = MODEL_BB_BLACK_W_RC;
            return;
        }
        if strcmp(
            c.as_mut_ptr(),
            b"TI AM335x BeagleBone Green\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            current_model = MODEL_BB_GREEN;
            return;
        }
        if strcmp(
            c.as_mut_ptr(),
            b"TI AM335x BeagleBone Green Wireless\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            current_model = MODEL_BB_GREEN_W;
            return;
        }
        if strcmp(
            c.as_mut_ptr(),
            b"TI AM335x BeagleBone Blue\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            current_model = MODEL_BB_BLUE;
            return;
        }
        if strcmp(
            c.as_mut_ptr(),
            b"TI AM335x PocketBeagle\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            current_model = MODEL_BB_POCKET;
            return;
        }
        return;
    }
    if !(strstr(c.as_mut_ptr(), b"Raspberry Pi\0" as *const u8 as *const libc::c_char)).is_null() {
        current_category = CATEGORY_RPI;
        if !(strstr(
            c.as_mut_ptr(),
            b"Raspberry Pi Model B+\0" as *const u8 as *const libc::c_char,
        ))
        .is_null()
        {
            current_model = MODEL_RPI_B_PLUS;
            return;
        }
        if !(strstr(
            c.as_mut_ptr(),
            b"Raspberry Pi Model B\0" as *const u8 as *const libc::c_char,
        ))
        .is_null()
        {
            current_model = MODEL_RPI_B;
            return;
        }
        if !(strstr(
            c.as_mut_ptr(),
            b"Raspberry Pi 2 Model B\0" as *const u8 as *const libc::c_char,
        ))
        .is_null()
        {
            current_model = MODEL_RPI2_B;
            return;
        }
        if !(strstr(
            c.as_mut_ptr(),
            b"Raspberry Pi 3 Model B+\0" as *const u8 as *const libc::c_char,
        ))
        .is_null()
        {
            current_model = MODEL_RPI3_B_PLUS;
            return;
        }
        if !(strstr(
            c.as_mut_ptr(),
            b"Raspberry Pi 3 Model\0" as *const u8 as *const libc::c_char,
        ))
        .is_null()
        {
            current_model = MODEL_RPI3_B;
            return;
        }
        if !(strstr(
            c.as_mut_ptr(),
            b"Raspberry Pi Zero W\0" as *const u8 as *const libc::c_char,
        ))
        .is_null()
        {
            current_model = MODEL_RPI0_W;
            return;
        }
        if !(strstr(
            c.as_mut_ptr(),
            b"Raspberry Pi Zero\0" as *const u8 as *const libc::c_char,
        ))
        .is_null()
        {
            current_model = MODEL_RPI0;
            return;
        }
        if !(strstr(
            c.as_mut_ptr(),
            b"Raspberry Pi Computer Module 3\0" as *const u8 as *const libc::c_char,
        ))
        .is_null()
        {
            current_model = MODEL_RPI_CM3;
            return;
        }
        if !(strstr(
            c.as_mut_ptr(),
            b"Raspberry Pi Compute Module\0" as *const u8 as *const libc::c_char,
        ))
        .is_null()
        {
            current_model = MODEL_RPI_CM;
            return;
        }
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rc_model() -> rc_model_t {
    if has_checked != 0 {
        return current_model;
    }
    __check_model();
    return current_model;
}
#[no_mangle]
pub unsafe extern "C" fn rc_model_category() -> rc_model_category_t {
    if has_checked != 0 {
        return current_category;
    }
    __check_model();
    has_checked = 1 as libc::c_int;
    return current_model as rc_model_category_t;
}
#[no_mangle]
pub unsafe extern "C" fn rc_model_print() {
    let mut model: rc_model_t = rc_model();
    match model as libc::c_uint {
        0 => {
            printf(b"MODEL_UNKNOWN\0" as *const u8 as *const libc::c_char);
        }
        1 => {
            printf(b"MODEL_BB_BLACK\0" as *const u8 as *const libc::c_char);
        }
        2 => {
            printf(b"MODEL_BB_BLACK_RC\0" as *const u8 as *const libc::c_char);
        }
        3 => {
            printf(b"MODEL_BB_BLACK_W\0" as *const u8 as *const libc::c_char);
        }
        4 => {
            printf(b"MODEL_BB_BLACK_W_RC\0" as *const u8 as *const libc::c_char);
        }
        5 => {
            printf(b"MODEL_BB_GREEN\0" as *const u8 as *const libc::c_char);
        }
        6 => {
            printf(b"MODEL_BB_GREEN_W\0" as *const u8 as *const libc::c_char);
        }
        7 => {
            printf(b"MODEL_BB_BLUE\0" as *const u8 as *const libc::c_char);
        }
        8 => {
            printf(b"MODEL_BB_POCKET\0" as *const u8 as *const libc::c_char);
        }
        9 => {
            printf(b"MODEL_RPI_B\0" as *const u8 as *const libc::c_char);
        }
        10 => {
            printf(b"MODEL_RPI_B_PLUS\0" as *const u8 as *const libc::c_char);
        }
        11 => {
            printf(b"MODEL_RPI2_B\0" as *const u8 as *const libc::c_char);
        }
        12 => {
            printf(b"MODEL_RPI3_B\0" as *const u8 as *const libc::c_char);
        }
        13 => {
            printf(b"MODEL_RPI3_B_PLUS\0" as *const u8 as *const libc::c_char);
        }
        14 => {
            printf(b"MODEL_RPI0\0" as *const u8 as *const libc::c_char);
        }
        15 => {
            printf(b"MODEL_RPI0_W\0" as *const u8 as *const libc::c_char);
        }
        16 => {
            printf(b"MODEL_RPI_CM\0" as *const u8 as *const libc::c_char);
        }
        17 => {
            printf(b"MODEL_RPI_CM3\0" as *const u8 as *const libc::c_char);
        }
        18 => {
            printf(b"MODEL_PC\0" as *const u8 as *const libc::c_char);
        }
        _ => {
            fprintf(
                stderr,
                b"ERROR: in rc_model_print, invalid model detected\n\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn rc_model_category_print() {
    let mut category: rc_model_category_t = rc_model_category();
    match category as libc::c_uint {
        0 => {
            printf(b"CATEGORY_UNKNOWN\0" as *const u8 as *const libc::c_char);
        }
        1 => {
            printf(b"CATEGORY_BEAGLEBONE\0" as *const u8 as *const libc::c_char);
        }
        2 => {
            printf(b"CATEGORY_RPI\0" as *const u8 as *const libc::c_char);
        }
        3 => {
            printf(b"CATEGORY_PC\0" as *const u8 as *const libc::c_char);
        }
        _ => {
            fprintf(
                stderr,
                b"ERROR: in rc_model_category_print, invalid category detected\n\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
