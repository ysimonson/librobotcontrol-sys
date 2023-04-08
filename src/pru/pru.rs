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
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn rc_usleep(us: libc::c_uint);
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
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
pub type uint32_t = __uint32_t;
static mut shared_mem_32bit_ptr: *mut libc::c_uint = 0 as *const libc::c_uint as *mut libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn rc_pru_start(mut ch: libc::c_int, mut fw_name: *const libc::c_char) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut buf: [libc::c_char; 64] = [0; 64];
    if ch != 0 as libc::c_int && ch != 1 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_pru_start, PRU channel must be 0 or 1\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if fw_name.is_null() {
        fprintf(
            stderr,
            b"ERROR in rc_pru_start, received NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    memset(
        buf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
    );
    snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        b"/lib/firmware/%s\0" as *const u8 as *const libc::c_char,
        fw_name,
    );
    if access(buf.as_mut_ptr(), 0 as libc::c_int) != 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_pru_start, requested firmware %s doesn't exist in /lib/firmware\n\0" as *const u8
                as *const libc::c_char,
            fw_name,
        );
        return -(1 as libc::c_int);
    }
    if rc_pru_stop(ch) != 0 {
        return -(1 as libc::c_int);
    }
    if ch == 0 as libc::c_int {
        fd = open(
            b"/dev/remoteproc/pruss-core0/firmware\0" as *const u8 as *const libc::c_char,
            0o1 as libc::c_int,
        );
    } else {
        fd = open(
            b"/dev/remoteproc/pruss-core1/firmware\0" as *const u8 as *const libc::c_char,
            0o1 as libc::c_int,
        );
    }
    if fd == -(1 as libc::c_int) {
        perror(b"ERROR in rc_pru_start opening remoteproc driver\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"need to be root to use the pru\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    ret = write(fd, fw_name as *const libc::c_void, strlen(fw_name)) as libc::c_int;
    close(fd);
    if ret == -(1 as libc::c_int) {
        perror(b"ERROR in rc_pru_start setting firmware name\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if ch == 0 as libc::c_int {
        fd = open(
            b"/dev/remoteproc/pruss-core0/state\0" as *const u8 as *const libc::c_char,
            0o1 as libc::c_int,
        );
    } else {
        fd = open(
            b"/dev/remoteproc/pruss-core1/state\0" as *const u8 as *const libc::c_char,
            0o1 as libc::c_int,
        );
    }
    if fd == -(1 as libc::c_int) {
        perror(b"ERROR in rc_pru_start opening remoteproc driver\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"PRU probably not enabled in device tree\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    ret = write(
        fd,
        b"start\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        5 as libc::c_int as size_t,
    ) as libc::c_int;
    close(fd);
    if ret == -(1 as libc::c_int) {
        perror(b"ERROR in rc_pru_start starting remoteproc\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    rc_usleep(250000 as libc::c_int as libc::c_uint);
    if ch == 0 as libc::c_int {
        fd = open(
            b"/dev/remoteproc/pruss-core0/state\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    } else {
        fd = open(
            b"/dev/remoteproc/pruss-core1/state\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    if fd == -(1 as libc::c_int) {
        perror(b"ERROR in rc_pru_start opening remoteproc driver\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"PRU probably not enabled in device tree\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    memset(
        buf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
    );
    ret = read(
        fd,
        buf.as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
    ) as libc::c_int;
    close(fd);
    if ret == -(1 as libc::c_int) {
        perror(b"ERROR in rc_pru_start reading state\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if strcmp(buf.as_mut_ptr(), b"running\n\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_pru_init, pru%d failed to start\n\0" as *const u8 as *const libc::c_char,
            ch,
        );
        fprintf(
            stderr,
            b"expected state to become 'running', instead is: %s\n\0" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_pru_shared_mem_ptr() -> *mut uint32_t {
    let mut fd: libc::c_int = 0;
    let mut map: *mut libc::c_uint = 0 as *mut libc::c_uint;
    if !shared_mem_32bit_ptr.is_null() {
        return shared_mem_32bit_ptr;
    }
    fd = open(
        b"/dev/mem\0" as *const u8 as *const libc::c_char,
        0o2 as libc::c_int | 0o4010000 as libc::c_int,
    );
    if fd == -(1 as libc::c_int) {
        perror(b"ERROR: in rc_pru_shared_mem_ptr could not open /dev/mem\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"Need to be root to access PRU shared memory\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut uint32_t;
    }
    map = mmap(
        0 as *mut libc::c_void,
        0x80000 as libc::c_int as size_t,
        0x1 as libc::c_int | 0x2 as libc::c_int,
        0x1 as libc::c_int,
        fd,
        0x4a300000 as libc::c_int as __off_t,
    ) as *mut libc::c_uint;
    if map == -(1 as libc::c_int) as *mut libc::c_void as *mut libc::c_uint {
        perror(b"ERROR in rc_pru_shared_mem_ptr failed to map memory\0" as *const u8 as *const libc::c_char);
        close(fd);
        return 0 as *mut uint32_t;
    }
    close(fd);
    shared_mem_32bit_ptr = map.offset((0x10000 as libc::c_int / 4 as libc::c_int) as isize);
    return shared_mem_32bit_ptr;
}
#[no_mangle]
pub unsafe extern "C" fn rc_pru_stop(mut ch: libc::c_int) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut buf: [libc::c_char; 64] = [0; 64];
    if ch != 0 as libc::c_int && ch != 1 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_pru_stop, PRU channel must be 0 or 1\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if ch == 0 as libc::c_int {
        fd = open(
            b"/dev/remoteproc/pruss-core0/state\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    } else {
        fd = open(
            b"/dev/remoteproc/pruss-core1/state\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    if fd == -(1 as libc::c_int) {
        perror(b"ERROR in rc_pru_stop opening remoteproc driver\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"PRU probably not enabled in device tree\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    memset(
        buf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
    );
    ret = read(
        fd,
        buf.as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
    ) as libc::c_int;
    close(fd);
    if ret == -(1 as libc::c_int) {
        perror(b"ERROR in rc_pru_stop reading state before stopping\0" as *const u8 as *const libc::c_char);
        close(fd);
        return -(1 as libc::c_int);
    }
    if strcmp(buf.as_mut_ptr(), b"offline\n\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return 0 as libc::c_int;
    } else {
        if strcmp(buf.as_mut_ptr(), b"running\n\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            if ch == 0 as libc::c_int {
                fd = open(
                    b"/dev/remoteproc/pruss-core0/state\0" as *const u8 as *const libc::c_char,
                    0o1 as libc::c_int,
                );
            } else {
                fd = open(
                    b"/dev/remoteproc/pruss-core1/state\0" as *const u8 as *const libc::c_char,
                    0o1 as libc::c_int,
                );
            }
            ret = write(
                fd,
                b"stop\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                4 as libc::c_int as size_t,
            ) as libc::c_int;
            close(fd);
            if ret == -(1 as libc::c_int) {
                perror(b"ERROR in rc_pru_stop while writing to remoteproc state\0" as *const u8 as *const libc::c_char);
                return -(1 as libc::c_int);
            }
        } else {
            fprintf(
                stderr,
                b"ERROR in rc_pru_stop remoteproc state should be 'offline' or 'running', read:%s\n\0" as *const u8
                    as *const libc::c_char,
                buf.as_mut_ptr(),
            );
            return -(1 as libc::c_int);
        }
    }
    if ch == 0 as libc::c_int {
        fd = open(
            b"/dev/remoteproc/pruss-core0/state\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    } else {
        fd = open(
            b"/dev/remoteproc/pruss-core1/state\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    memset(
        buf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
    );
    ret = read(
        fd,
        buf.as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
    ) as libc::c_int;
    close(fd);
    if ret == -(1 as libc::c_int) {
        perror(b"ERROR in rc_pru_stop reading state after stopping\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if strcmp(buf.as_mut_ptr(), b"offline\n\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_pru_stop, remoteproc state should now be 'offline', read:%s\n\0" as *const u8
                as *const libc::c_char,
            buf.as_mut_ptr(),
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
