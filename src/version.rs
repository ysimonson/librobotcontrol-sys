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
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_version() -> libc::c_uint {
    return ((1 as libc::c_int) << 16 as libc::c_int | (0 as libc::c_int) << 8 as libc::c_int | 5 as libc::c_int)
        as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn rc_version_string() -> *const libc::c_char {
    return b"1.0.5\0" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn rc_version_print() {
    printf(b"1.0.5\0" as *const u8 as *const libc::c_char);
}
