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
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
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
pub struct rc_i2c_state_t {
    pub devAddr: uint8_t,
    pub fd: libc::c_int,
    pub initialized: libc::c_int,
    pub lock: libc::c_int,
}
static mut i2c: [rc_i2c_state_t; 6] = [rc_i2c_state_t {
    devAddr: 0,
    fd: 0,
    initialized: 0,
    lock: 0,
}; 6];
unsafe extern "C" fn __check_bus_range(mut bus: libc::c_int) -> libc::c_int {
    if (bus < 0 as libc::c_int || bus > 5 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: i2c bus must be between 0 & %d\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_i2c_init(mut bus: libc::c_int, mut devAddr: uint8_t) -> libc::c_int {
    if (__check_bus_range(bus) != 0) as libc::c_int as libc::c_long != 0 {
        return -(1 as libc::c_int);
    }
    if i2c[bus as usize].initialized != 0 {
        return rc_i2c_set_device_address(bus, devAddr);
    }
    i2c[bus as usize].lock = 1 as libc::c_int;
    i2c[bus as usize].initialized = 0 as libc::c_int;
    let mut str: [libc::c_char; 16] = [0; 16];
    sprintf(
        str.as_mut_ptr(),
        b"/dev/i2c-%d\0" as *const u8 as *const libc::c_char,
        bus,
    );
    i2c[bus as usize].fd = open(str.as_mut_ptr(), 0o2 as libc::c_int);
    if i2c[bus as usize].fd == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"ERROR: in rc_i2c_init, failed to open /dev/i2c\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (ioctl(
        i2c[bus as usize].fd,
        0x703 as libc::c_int as libc::c_ulong,
        devAddr as libc::c_int,
    ) < 0 as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"ERROR: in rc_i2c_init, ioctl slave address change failed\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i2c[bus as usize].devAddr = devAddr;
    i2c[bus as usize].lock = 0 as libc::c_int;
    i2c[bus as usize].initialized = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_i2c_close(mut bus: libc::c_int) -> libc::c_int {
    if (__check_bus_range(bus) != 0) as libc::c_int as libc::c_long != 0 {
        return -(1 as libc::c_int);
    }
    close(i2c[bus as usize].fd);
    i2c[bus as usize].devAddr = 0 as libc::c_int as uint8_t;
    i2c[bus as usize].initialized = 0 as libc::c_int;
    i2c[bus as usize].lock = 0 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_i2c_set_device_address(mut bus: libc::c_int, mut devAddr: uint8_t) -> libc::c_int {
    if (__check_bus_range(bus) != 0) as libc::c_int as libc::c_long != 0 {
        return -(1 as libc::c_int);
    }
    if (i2c[bus as usize].initialized == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_i2c_set_device_address, bus not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if i2c[bus as usize].devAddr as libc::c_int == devAddr as libc::c_int {
        return 0 as libc::c_int;
    }
    if (ioctl(
        i2c[bus as usize].fd,
        0x703 as libc::c_int as libc::c_ulong,
        devAddr as libc::c_int,
    ) < 0 as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"ERROR: in rc_i2c_set_device_address, ioctl slave address change failed\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i2c[bus as usize].devAddr = devAddr;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_i2c_read_byte(
    mut bus: libc::c_int,
    mut regAddr: uint8_t,
    mut data: *mut uint8_t,
) -> libc::c_int {
    return rc_i2c_read_bytes(bus, regAddr, 1 as libc::c_int as size_t, data);
}
#[no_mangle]
pub unsafe extern "C" fn rc_i2c_read_bytes(
    mut bus: libc::c_int,
    mut regAddr: uint8_t,
    mut count: size_t,
    mut data: *mut uint8_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut old_lock: libc::c_int = 0;
    if (__check_bus_range(bus) != 0) as libc::c_int as libc::c_long != 0 {
        return -(1 as libc::c_int);
    }
    if (i2c[bus as usize].initialized == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_i2c_read_bytes, bus not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    old_lock = i2c[bus as usize].lock;
    i2c[bus as usize].lock = 1 as libc::c_int;
    ret = write(
        i2c[bus as usize].fd,
        &mut regAddr as *mut uint8_t as *const libc::c_void,
        1 as libc::c_int as size_t,
    ) as libc::c_int;
    if (ret != 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_i2c_read_bytes, failed to write to bus\n\0" as *const u8 as *const libc::c_char,
        );
        i2c[bus as usize].lock = old_lock;
        return -(1 as libc::c_int);
    }
    ret = read(i2c[bus as usize].fd, data as *mut libc::c_void, count) as libc::c_int;
    if (ret as size_t != count) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_i2c_read_bytes, received %d bytes from device, expected %d\n\0" as *const u8
                as *const libc::c_char,
            ret,
            count as libc::c_int,
        );
        i2c[bus as usize].lock = old_lock;
        return -(1 as libc::c_int);
    }
    i2c[bus as usize].lock = old_lock;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn rc_i2c_read_word(
    mut bus: libc::c_int,
    mut regAddr: uint8_t,
    mut data: *mut uint16_t,
) -> libc::c_int {
    return rc_i2c_read_words(bus, regAddr, 1 as libc::c_int as size_t, data);
}
#[no_mangle]
pub unsafe extern "C" fn rc_i2c_read_words(
    mut bus: libc::c_int,
    mut regAddr: uint8_t,
    mut count: size_t,
    mut data: *mut uint16_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut old_lock: libc::c_int = 0;
    let mut i: size_t = 0;
    let vla = count.wrapping_mul(2 as libc::c_int as libc::c_ulong) as usize;
    let mut buf: Vec<libc::c_char> = ::std::vec::from_elem(0, vla);
    if (__check_bus_range(bus) != 0) as libc::c_int as libc::c_long != 0 {
        return -(1 as libc::c_int);
    }
    if (i2c[bus as usize].initialized == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_i2c_read_words, bus not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    old_lock = i2c[bus as usize].lock;
    i2c[bus as usize].lock = 1 as libc::c_int;
    ret = write(
        i2c[bus as usize].fd,
        &mut regAddr as *mut uint8_t as *const libc::c_void,
        1 as libc::c_int as size_t,
    ) as libc::c_int;
    if (ret != 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_i2c_read_words, failed to write to bus\n\0" as *const u8 as *const libc::c_char,
        );
        i2c[bus as usize].lock = old_lock;
        return -(1 as libc::c_int);
    }
    ret = read(
        i2c[bus as usize].fd,
        buf.as_mut_ptr() as *mut libc::c_void,
        count.wrapping_mul(2 as libc::c_int as libc::c_ulong),
    ) as libc::c_int;
    if ret != count.wrapping_mul(2 as libc::c_int as libc::c_ulong) as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: in rc_i2c_read_words, received %d bytes, expected %zu\n\0" as *const u8 as *const libc::c_char,
            ret,
            count.wrapping_mul(2 as libc::c_int as libc::c_ulong),
        );
        i2c[bus as usize].lock = old_lock;
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int as size_t;
    while i < count {
        *data.offset(i as isize) = ((*buf
            .as_mut_ptr()
            .offset(i.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize)
            as uint16_t as libc::c_int)
            << 8 as libc::c_int
            | *buf.as_mut_ptr().offset(
                i.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) as libc::c_int) as uint16_t;
        i = i.wrapping_add(1);
    }
    i2c[bus as usize].lock = old_lock;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_i2c_write_bytes(
    mut bus: libc::c_int,
    mut regAddr: uint8_t,
    mut count: size_t,
    mut data: *mut uint8_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut old_lock: libc::c_int = 0;
    let mut i: size_t = 0;
    let vla = count.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize;
    let mut writeData: Vec<uint8_t> = ::std::vec::from_elem(0, vla);
    if (__check_bus_range(bus) != 0) as libc::c_int as libc::c_long != 0 {
        return -(1 as libc::c_int);
    }
    if (i2c[bus as usize].initialized == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_i2c_write_bytes, bus not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    old_lock = i2c[bus as usize].lock;
    i2c[bus as usize].lock = 1 as libc::c_int;
    *writeData.as_mut_ptr().offset(0 as libc::c_int as isize) = regAddr;
    i = 0 as libc::c_int as size_t;
    while i < count {
        *writeData
            .as_mut_ptr()
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) = *data.offset(i as isize);
        i = i.wrapping_add(1);
    }
    ret = write(
        i2c[bus as usize].fd,
        writeData.as_mut_ptr() as *const libc::c_void,
        count.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as libc::c_int;
    if (ret != count.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        fprintf(
            stderr,
            b"ERROR in rc_i2c_write_bytes, bus wrote %d bytes, expected %zu\n\0" as *const u8 as *const libc::c_char,
            ret,
            count.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        i2c[bus as usize].lock = old_lock;
        return -(1 as libc::c_int);
    }
    i2c[bus as usize].lock = old_lock;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_i2c_write_byte(
    mut bus: libc::c_int,
    mut regAddr: uint8_t,
    mut data: uint8_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut old_lock: libc::c_int = 0;
    let mut writeData: [uint8_t; 2] = [0; 2];
    if (__check_bus_range(bus) != 0) as libc::c_int as libc::c_long != 0 {
        return -(1 as libc::c_int);
    }
    if (i2c[bus as usize].initialized == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_i2c_write_byte, bus not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    old_lock = i2c[bus as usize].lock;
    i2c[bus as usize].lock = 1 as libc::c_int;
    writeData[0 as libc::c_int as usize] = regAddr;
    writeData[1 as libc::c_int as usize] = data;
    ret = write(
        i2c[bus as usize].fd,
        writeData.as_mut_ptr() as *const libc::c_void,
        2 as libc::c_int as size_t,
    ) as libc::c_int;
    if (ret != 2 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_i2c_write_byte, system write returned %d, expected 2\n\0" as *const u8
                as *const libc::c_char,
            ret,
        );
        i2c[bus as usize].lock = old_lock;
        return -(1 as libc::c_int);
    }
    i2c[bus as usize].lock = old_lock;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_i2c_write_words(
    mut bus: libc::c_int,
    mut regAddr: uint8_t,
    mut count: size_t,
    mut data: *mut uint16_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut old_lock: libc::c_int = 0;
    let mut i: size_t = 0;
    let vla = count
        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as usize;
    let mut writeData: Vec<uint8_t> = ::std::vec::from_elem(0, vla);
    if (__check_bus_range(bus) != 0) as libc::c_int as libc::c_long != 0 {
        return -(1 as libc::c_int);
    }
    if (i2c[bus as usize].initialized == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_i2c_write_words, bus not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    old_lock = i2c[bus as usize].lock;
    i2c[bus as usize].lock = 1 as libc::c_int;
    *writeData.as_mut_ptr().offset(0 as libc::c_int as isize) = regAddr;
    i = 0 as libc::c_int as size_t;
    while i < count {
        *writeData.as_mut_ptr().offset(
            i.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        ) = (*data.offset(i as isize) as libc::c_int >> 8 as libc::c_int) as uint8_t;
        *writeData.as_mut_ptr().offset(
            i.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
        ) = (*data.offset(i as isize) as libc::c_int & 0xff as libc::c_int) as uint8_t;
        i = i.wrapping_add(1);
    }
    ret = write(
        i2c[bus as usize].fd,
        writeData.as_mut_ptr() as *const libc::c_void,
        count
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as libc::c_int;
    if (ret != count.wrapping_mul(2 as libc::c_int as libc::c_ulong) as libc::c_int + 1 as libc::c_int) as libc::c_int
        as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"ERROR: in rc_i2c_write_words, system write returned %d, expected %zu\n\0" as *const u8
                as *const libc::c_char,
            ret,
            count
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        i2c[bus as usize].lock = old_lock;
        return -(1 as libc::c_int);
    }
    i2c[bus as usize].lock = old_lock;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_i2c_write_word(
    mut bus: libc::c_int,
    mut regAddr: uint8_t,
    mut data: uint16_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut old_lock: libc::c_int = 0;
    let mut writeData: [uint8_t; 3] = [0; 3];
    if (__check_bus_range(bus) != 0) as libc::c_int as libc::c_long != 0 {
        return -(1 as libc::c_int);
    }
    if (i2c[bus as usize].initialized == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_i2c_write_words, bus not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    old_lock = i2c[bus as usize].lock;
    i2c[bus as usize].lock = 1 as libc::c_int;
    writeData[0 as libc::c_int as usize] = regAddr;
    writeData[1 as libc::c_int as usize] = (data as libc::c_int >> 8 as libc::c_int) as uint8_t;
    writeData[2 as libc::c_int as usize] = (data as libc::c_int & 0xff as libc::c_int) as uint8_t;
    ret = write(
        i2c[bus as usize].fd,
        writeData.as_mut_ptr() as *const libc::c_void,
        3 as libc::c_int as size_t,
    ) as libc::c_int;
    if (ret != 3 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_i2c_write_word, system write returned %d, expected 3\n\0" as *const u8
                as *const libc::c_char,
            ret,
        );
        i2c[bus as usize].lock = old_lock;
        return -(1 as libc::c_int);
    }
    i2c[bus as usize].lock = old_lock;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_i2c_send_byte(mut bus: libc::c_int, mut data: uint8_t) -> libc::c_int {
    return rc_i2c_send_bytes(bus, 1 as libc::c_int as size_t, &mut data);
}
#[no_mangle]
pub unsafe extern "C" fn rc_i2c_send_bytes(
    mut bus: libc::c_int,
    mut count: size_t,
    mut data: *mut uint8_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if (__check_bus_range(bus) != 0) as libc::c_int as libc::c_long != 0 {
        return -(1 as libc::c_int);
    }
    if (i2c[bus as usize].initialized == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_i2c_send_bytes, bus not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut old_lock: libc::c_int = i2c[bus as usize].lock;
    i2c[bus as usize].lock = 1 as libc::c_int;
    ret = write(i2c[bus as usize].fd, data as *const libc::c_void, count) as libc::c_int;
    if ret != count as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: in rc_i2c_send_bytes, system write returned %d, expected %zu\n\0" as *const u8
                as *const libc::c_char,
            ret,
            count,
        );
        i2c[bus as usize].lock = old_lock;
        return -(1 as libc::c_int);
    }
    i2c[bus as usize].lock = old_lock;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_i2c_lock_bus(mut bus: libc::c_int) -> libc::c_int {
    if (__check_bus_range(bus) != 0) as libc::c_int as libc::c_long != 0 {
        return -(1 as libc::c_int);
    }
    let mut ret: libc::c_int = i2c[bus as usize].lock;
    i2c[bus as usize].lock = 1 as libc::c_int;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn rc_i2c_unlock_bus(mut bus: libc::c_int) -> libc::c_int {
    if (__check_bus_range(bus) != 0) as libc::c_int as libc::c_long != 0 {
        return -(1 as libc::c_int);
    }
    let mut ret: libc::c_int = i2c[bus as usize].lock;
    i2c[bus as usize].lock = 0 as libc::c_int;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn rc_i2c_get_lock(mut bus: libc::c_int) -> libc::c_int {
    if (__check_bus_range(bus) != 0) as libc::c_int as libc::c_long != 0 {
        return -(1 as libc::c_int);
    }
    return i2c[bus as usize].lock;
}
#[no_mangle]
pub unsafe extern "C" fn rc_i2c_get_fd(mut bus: libc::c_int) -> libc::c_int {
    if (__check_bus_range(bus) != 0) as libc::c_int as libc::c_long != 0 {
        return -(1 as libc::c_int);
    }
    if (i2c[bus as usize].initialized == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_i2c_get_fd, bus not initialized yet\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return i2c[bus as usize].fd;
}
#[no_mangle]
pub unsafe extern "C" fn rc_i2c_read_data(
    mut bus: libc::c_int,
    mut regAddr: uint8_t,
    mut length: size_t,
    mut data: *mut uint8_t,
) -> libc::c_int {
    return rc_i2c_read_bytes(bus, regAddr, length, data);
}
