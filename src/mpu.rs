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
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getchar() -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn abs(_: libc::c_int) -> libc::c_int;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(__cond: *mut pthread_cond_t, __mutex: *mut pthread_mutex_t) -> libc::c_int;
    static mut zero_tolerance: libc::c_double;
    fn rc_vector_empty() -> rc_vector_t;
    fn rc_vector_alloc(v: *mut rc_vector_t, length: libc::c_int) -> libc::c_int;
    fn rc_vector_free(v: *mut rc_vector_t) -> libc::c_int;
    fn rc_vector_std_dev(v: rc_vector_t) -> libc::c_double;
    fn rc_matrix_empty() -> rc_matrix_t;
    fn rc_matrix_alloc(A: *mut rc_matrix_t, rows: libc::c_int, cols: libc::c_int) -> libc::c_int;
    fn rc_matrix_free(A: *mut rc_matrix_t) -> libc::c_int;
    fn rc_quaternion_to_tb_array(q: *mut libc::c_double, tb: *mut libc::c_double) -> libc::c_int;
    fn rc_quaternion_from_tb_array(tb: *mut libc::c_double, q: *mut libc::c_double) -> libc::c_int;
    fn rc_quaternion_rotate_vector_array(v: *mut libc::c_double, q: *mut libc::c_double) -> libc::c_int;
    fn rc_filter_march(f: *mut rc_filter_t, new_input: libc::c_double) -> libc::c_double;
    fn rc_filter_prefill_outputs(f: *mut rc_filter_t, out: libc::c_double) -> libc::c_int;
    fn rc_filter_prefill_inputs(f: *mut rc_filter_t, in_0: libc::c_double) -> libc::c_int;
    fn rc_filter_first_order_highpass(f: *mut rc_filter_t, dt: libc::c_double, tc: libc::c_double) -> libc::c_int;
    fn rc_filter_first_order_lowpass(f: *mut rc_filter_t, dt: libc::c_double, tc: libc::c_double) -> libc::c_int;
    fn rc_algebra_fit_ellipsoid(
        points: rc_matrix_t,
        center: *mut rc_vector_t,
        lengths: *mut rc_vector_t,
    ) -> libc::c_int;
    fn rc_usleep(us: libc::c_uint);
    fn rc_nanos_since_epoch() -> uint64_t;
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
    fn rc_gpio_cleanup(chip: libc::c_int, pin: libc::c_int);
    fn rc_i2c_init(bus: libc::c_int, devAddr: uint8_t) -> libc::c_int;
    fn rc_i2c_set_device_address(bus: libc::c_int, devAddr: uint8_t) -> libc::c_int;
    fn rc_i2c_read_byte(bus: libc::c_int, regAddr: uint8_t, data: *mut uint8_t) -> libc::c_int;
    fn rc_i2c_read_bytes(bus: libc::c_int, regAddr: uint8_t, count: size_t, data: *mut uint8_t) -> libc::c_int;
    fn rc_i2c_read_word(bus: libc::c_int, regAddr: uint8_t, data: *mut uint16_t) -> libc::c_int;
    fn rc_i2c_write_byte(bus: libc::c_int, regAddr: uint8_t, data: uint8_t) -> libc::c_int;
    fn rc_i2c_write_bytes(bus: libc::c_int, regAddr: uint8_t, count: size_t, data: *mut uint8_t) -> libc::c_int;
    fn rc_i2c_lock_bus(bus: libc::c_int) -> libc::c_int;
    fn rc_i2c_unlock_bus(bus: libc::c_int) -> libc::c_int;
    fn rc_i2c_get_lock(bus: libc::c_int) -> libc::c_int;
    fn rc_pthread_create(
        thread: *mut pthread_t,
        func: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        arg: *mut libc::c_void,
        policy: libc::c_int,
        priority: libc::c_int,
    ) -> libc::c_int;
    fn rc_pthread_timed_join(
        thread: pthread_t,
        retval: *mut *mut libc::c_void,
        timeout_sec: libc::c_float,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
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
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_0 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_0 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_0 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_0 = 0;
pub type rc_mpu_accel_fsr_t = libc::c_uint;
pub const ACCEL_FSR_16G: rc_mpu_accel_fsr_t = 3;
pub const ACCEL_FSR_8G: rc_mpu_accel_fsr_t = 2;
pub const ACCEL_FSR_4G: rc_mpu_accel_fsr_t = 1;
pub const ACCEL_FSR_2G: rc_mpu_accel_fsr_t = 0;
pub type rc_mpu_gyro_fsr_t = libc::c_uint;
pub const GYRO_FSR_2000DPS: rc_mpu_gyro_fsr_t = 3;
pub const GYRO_FSR_1000DPS: rc_mpu_gyro_fsr_t = 2;
pub const GYRO_FSR_500DPS: rc_mpu_gyro_fsr_t = 1;
pub const GYRO_FSR_250DPS: rc_mpu_gyro_fsr_t = 0;
pub type rc_mpu_accel_dlpf_t = libc::c_uint;
pub const ACCEL_DLPF_5: rc_mpu_accel_dlpf_t = 7;
pub const ACCEL_DLPF_10: rc_mpu_accel_dlpf_t = 6;
pub const ACCEL_DLPF_20: rc_mpu_accel_dlpf_t = 5;
pub const ACCEL_DLPF_41: rc_mpu_accel_dlpf_t = 4;
pub const ACCEL_DLPF_92: rc_mpu_accel_dlpf_t = 3;
pub const ACCEL_DLPF_184: rc_mpu_accel_dlpf_t = 2;
pub const ACCEL_DLPF_460: rc_mpu_accel_dlpf_t = 1;
pub const ACCEL_DLPF_OFF: rc_mpu_accel_dlpf_t = 0;
pub type rc_mpu_gyro_dlpf_t = libc::c_uint;
pub const GYRO_DLPF_5: rc_mpu_gyro_dlpf_t = 7;
pub const GYRO_DLPF_10: rc_mpu_gyro_dlpf_t = 6;
pub const GYRO_DLPF_20: rc_mpu_gyro_dlpf_t = 5;
pub const GYRO_DLPF_41: rc_mpu_gyro_dlpf_t = 4;
pub const GYRO_DLPF_92: rc_mpu_gyro_dlpf_t = 3;
pub const GYRO_DLPF_184: rc_mpu_gyro_dlpf_t = 2;
pub const GYRO_DLPF_250: rc_mpu_gyro_dlpf_t = 1;
pub const GYRO_DLPF_OFF: rc_mpu_gyro_dlpf_t = 0;
pub type rc_mpu_orientation_t = libc::c_uint;
pub const ORIENTATION_X_BACK: rc_mpu_orientation_t = 161;
pub const ORIENTATION_X_FORWARD: rc_mpu_orientation_t = 133;
pub const ORIENTATION_Y_DOWN: rc_mpu_orientation_t = 336;
pub const ORIENTATION_Y_UP: rc_mpu_orientation_t = 112;
pub const ORIENTATION_X_DOWN: rc_mpu_orientation_t = 266;
pub const ORIENTATION_X_UP: rc_mpu_orientation_t = 14;
pub const ORIENTATION_Z_DOWN: rc_mpu_orientation_t = 396;
pub const ORIENTATION_Z_UP: rc_mpu_orientation_t = 136;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rc_mpu_config_t {
    pub gpio_interrupt_pin_chip: libc::c_int,
    pub gpio_interrupt_pin: libc::c_int,
    pub i2c_bus: libc::c_int,
    pub i2c_addr: uint8_t,
    pub show_warnings: libc::c_int,
    pub accel_fsr: rc_mpu_accel_fsr_t,
    pub gyro_fsr: rc_mpu_gyro_fsr_t,
    pub accel_dlpf: rc_mpu_accel_dlpf_t,
    pub gyro_dlpf: rc_mpu_gyro_dlpf_t,
    pub enable_magnetometer: libc::c_int,
    pub dmp_sample_rate: libc::c_int,
    pub dmp_fetch_accel_gyro: libc::c_int,
    pub dmp_auto_calibrate_gyro: libc::c_int,
    pub orient: rc_mpu_orientation_t,
    pub compass_time_constant: libc::c_double,
    pub dmp_interrupt_sched_policy: libc::c_int,
    pub dmp_interrupt_priority: libc::c_int,
    pub read_mag_after_callback: libc::c_int,
    pub mag_sample_rate_div: libc::c_int,
    pub tap_threshold: libc::c_int,
}
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct rc_mpu_data_t {
    pub accel: [libc::c_double; 3],
    pub gyro: [libc::c_double; 3],
    pub mag: [libc::c_double; 3],
    pub temp: libc::c_double,
    pub raw_gyro: [int16_t; 3],
    pub raw_accel: [int16_t; 3],
    pub accel_to_ms2: libc::c_double,
    pub gyro_to_degs: libc::c_double,
    pub dmp_quat: [libc::c_double; 4],
    pub dmp_TaitBryan: [libc::c_double; 3],
    pub tap_detected: libc::c_int,
    pub last_tap_direction: libc::c_int,
    pub last_tap_count: libc::c_int,
    pub fused_quat: [libc::c_double; 4],
    pub fused_TaitBryan: [libc::c_double; 3],
    pub compass_heading: libc::c_double,
    pub compass_heading_raw: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rc_filter_t {
    pub order: libc::c_int,
    pub dt: libc::c_double,
    pub gain: libc::c_double,
    pub num: rc_vector_t,
    pub den: rc_vector_t,
    pub sat_en: libc::c_int,
    pub sat_min: libc::c_double,
    pub sat_max: libc::c_double,
    pub sat_flag: libc::c_int,
    pub ss_en: libc::c_int,
    pub ss_steps: libc::c_double,
    pub in_buf: rc_ringbuf_t,
    pub out_buf: rc_ringbuf_t,
    pub newest_input: libc::c_double,
    pub newest_output: libc::c_double,
    pub step: uint64_t,
    pub initialized: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rc_ringbuf_t {
    pub d: *mut libc::c_double,
    pub size: libc::c_int,
    pub index: libc::c_int,
    pub initialized: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rc_vector_t {
    pub len: libc::c_int,
    pub d: *mut libc::c_double,
    pub initialized: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rc_matrix_t {
    pub rows: libc::c_int,
    pub cols: libc::c_int,
    pub d: *mut *mut libc::c_double,
    pub initialized: libc::c_int,
}
static mut dmp_start_addr: uint16_t = 0x400 as libc::c_int as uint16_t;
static mut dmp_firmware: [uint8_t; 3062] = [
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x70 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x65 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x54 as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xef as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xb as libc::c_int as uint8_t,
    0x12 as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0x30 as libc::c_int as uint8_t,
    0xc3 as libc::c_int as uint8_t,
    0xe as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0xe9 as libc::c_int as uint8_t,
    0x14 as libc::c_int as uint8_t,
    0xd5 as libc::c_int as uint8_t,
    0x40 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x13 as libc::c_int as uint8_t,
    0x71 as libc::c_int as uint8_t,
    0xf as libc::c_int as uint8_t,
    0x8e as libc::c_int as uint8_t,
    0x38 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0x30 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0x25 as libc::c_int as uint8_t,
    0x8e as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0x30 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xf as libc::c_int as uint8_t,
    0xfe as libc::c_int as uint8_t,
    0xa9 as libc::c_int as uint8_t,
    0xd6 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1a as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0x79 as libc::c_int as uint8_t,
    0xa1 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x3c as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x38 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0x6f as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x3e as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0x30 as libc::c_int as uint8_t,
    0x40 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0xca as libc::c_int as uint8_t,
    0xe3 as libc::c_int as uint8_t,
    0x9 as libc::c_int as uint8_t,
    0x3e as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x40 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x60 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0x18 as libc::c_int as uint8_t,
    0x6e as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x6 as libc::c_int as uint8_t,
    0x92 as libc::c_int as uint8_t,
    0xa as libc::c_int as uint8_t,
    0x16 as libc::c_int as uint8_t,
    0xc0 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x56 as libc::c_int as uint8_t,
    0xfd as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0xd3 as libc::c_int as uint8_t,
    0x77 as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xe1 as libc::c_int as uint8_t,
    0xc4 as libc::c_int as uint8_t,
    0x96 as libc::c_int as uint8_t,
    0xe0 as libc::c_int as uint8_t,
    0xc5 as libc::c_int as uint8_t,
    0xbe as libc::c_int as uint8_t,
    0xaa as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xb as libc::c_int as uint8_t,
    0x2b as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x16 as libc::c_int as uint8_t,
    0x57 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0x59 as libc::c_int as uint8_t,
    0x40 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1d as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x6c as libc::c_int as uint8_t,
    0x1d as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x3f as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xeb as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x3e as libc::c_int as uint8_t,
    0xb3 as libc::c_int as uint8_t,
    0xb6 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xd as libc::c_int as uint8_t,
    0x22 as libc::c_int as uint8_t,
    0x78 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x2f as libc::c_int as uint8_t,
    0x3c as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x19 as libc::c_int as uint8_t,
    0x42 as libc::c_int as uint8_t,
    0xb5 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x39 as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xb3 as libc::c_int as uint8_t,
    0x65 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xe as libc::c_int as uint8_t,
    0x9f as libc::c_int as uint8_t,
    0xc9 as libc::c_int as uint8_t,
    0x1d as libc::c_int as uint8_t,
    0xcf as libc::c_int as uint8_t,
    0x4c as libc::c_int as uint8_t,
    0x34 as libc::c_int as uint8_t,
    0x30 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x50 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x3b as libc::c_int as uint8_t,
    0xb6 as libc::c_int as uint8_t,
    0x7a as libc::c_int as uint8_t,
    0xe8 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x64 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xc8 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0x92 as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x22 as libc::c_int as uint8_t,
    0x5e as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xd as libc::c_int as uint8_t,
    0x22 as libc::c_int as uint8_t,
    0x9f as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x32 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0x46 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x63 as libc::c_int as uint8_t,
    0xd4 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0xd6 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0xcc as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0xcc as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0x72 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x40 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x6 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x7 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x64 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x64 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x40 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x32 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0x65 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0xf as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0x9b as libc::c_int as uint8_t,
    0xfc as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x40 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x6 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xb2 as libc::c_int as uint8_t,
    0x6a as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0xfb as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x68 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xfc as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x7c as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x65 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x64 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0xe8 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x64 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x25 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x16 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x2f as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0x45 as libc::c_int as uint8_t,
    0x81 as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0x72 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x44 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xc6 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x47 as libc::c_int as uint8_t,
    0x78 as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x6 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x14 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x25 as libc::c_int as uint8_t,
    0x4d as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x2f as libc::c_int as uint8_t,
    0x70 as libc::c_int as uint8_t,
    0x6d as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
    0xae as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1b as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x64 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1b as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xe as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xe as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xa as libc::c_int as uint8_t,
    0xc7 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x32 as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0x9c as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xb as libc::c_int as uint8_t,
    0x2b as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x64 as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xe5 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x26 as libc::c_int as uint8_t,
    0xd3 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x6 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x96 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x3c as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0xa as libc::c_int as uint8_t,
    0x4e as libc::c_int as uint8_t,
    0x68 as libc::c_int as uint8_t,
    0xcd as libc::c_int as uint8_t,
    0xcf as libc::c_int as uint8_t,
    0x77 as libc::c_int as uint8_t,
    0x9 as libc::c_int as uint8_t,
    0x50 as libc::c_int as uint8_t,
    0x16 as libc::c_int as uint8_t,
    0x67 as libc::c_int as uint8_t,
    0x59 as libc::c_int as uint8_t,
    0xc6 as libc::c_int as uint8_t,
    0x19 as libc::c_int as uint8_t,
    0xce as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x17 as libc::c_int as uint8_t,
    0xd7 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xc7 as libc::c_int as uint8_t,
    0x93 as libc::c_int as uint8_t,
    0x8f as libc::c_int as uint8_t,
    0x9d as libc::c_int as uint8_t,
    0x1e as libc::c_int as uint8_t,
    0x1b as libc::c_int as uint8_t,
    0x1c as libc::c_int as uint8_t,
    0x19 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0x18 as libc::c_int as uint8_t,
    0x85 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x40 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x40 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x67 as libc::c_int as uint8_t,
    0x7d as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0x7e as libc::c_int as uint8_t,
    0x72 as libc::c_int as uint8_t,
    0x90 as libc::c_int as uint8_t,
    0x2e as libc::c_int as uint8_t,
    0x55 as libc::c_int as uint8_t,
    0x4c as libc::c_int as uint8_t,
    0xf6 as libc::c_int as uint8_t,
    0xe6 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xdc as libc::c_int as uint8_t,
    0xb4 as libc::c_int as uint8_t,
    0xb8 as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xb9 as libc::c_int as uint8_t,
    0xab as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0xb3 as libc::c_int as uint8_t,
    0xb7 as libc::c_int as uint8_t,
    0xbb as libc::c_int as uint8_t,
    0x8e as libc::c_int as uint8_t,
    0x9e as libc::c_int as uint8_t,
    0xae as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x32 as libc::c_int as uint8_t,
    0xf5 as libc::c_int as uint8_t,
    0x1b as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xb4 as libc::c_int as uint8_t,
    0xb8 as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xa9 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xaa as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xaa as libc::c_int as uint8_t,
    0xc5 as libc::c_int as uint8_t,
    0xcd as libc::c_int as uint8_t,
    0xc7 as libc::c_int as uint8_t,
    0xa9 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0xc9 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xa9 as libc::c_int as uint8_t,
    0x89 as libc::c_int as uint8_t,
    0x26 as libc::c_int as uint8_t,
    0x46 as libc::c_int as uint8_t,
    0x66 as libc::c_int as uint8_t,
    0xb2 as libc::c_int as uint8_t,
    0x89 as libc::c_int as uint8_t,
    0x99 as libc::c_int as uint8_t,
    0xa9 as libc::c_int as uint8_t,
    0x2d as libc::c_int as uint8_t,
    0x55 as libc::c_int as uint8_t,
    0x7d as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0x8a as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x96 as libc::c_int as uint8_t,
    0x36 as libc::c_int as uint8_t,
    0x56 as libc::c_int as uint8_t,
    0x76 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xb4 as libc::c_int as uint8_t,
    0xb2 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0xc0 as libc::c_int as uint8_t,
    0xb8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0x11 as libc::c_int as uint8_t,
    0xb2 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xf0 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0x44 as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0x64 as libc::c_int as uint8_t,
    0x18 as libc::c_int as uint8_t,
    0xb2 as libc::c_int as uint8_t,
    0xb9 as libc::c_int as uint8_t,
    0xb4 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0x29 as libc::c_int as uint8_t,
    0x55 as libc::c_int as uint8_t,
    0x7d as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xb5 as libc::c_int as uint8_t,
    0xb1 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0x93 as libc::c_int as uint8_t,
    0xf0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x50 as libc::c_int as uint8_t,
    0xf5 as libc::c_int as uint8_t,
    0xb2 as libc::c_int as uint8_t,
    0xb6 as libc::c_int as uint8_t,
    0xaa as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0x93 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x54 as libc::c_int as uint8_t,
    0x7c as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xb9 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0x93 as libc::c_int as uint8_t,
    0x61 as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdb as libc::c_int as uint8_t,
    0x81 as libc::c_int as uint8_t,
    0x9a as libc::c_int as uint8_t,
    0xb9 as libc::c_int as uint8_t,
    0xae as libc::c_int as uint8_t,
    0xf5 as libc::c_int as uint8_t,
    0x60 as libc::c_int as uint8_t,
    0x68 as libc::c_int as uint8_t,
    0x70 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0xb9 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0x92 as libc::c_int as uint8_t,
    0xdb as libc::c_int as uint8_t,
    0x31 as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0x85 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xc1 as libc::c_int as uint8_t,
    0xbb as libc::c_int as uint8_t,
    0xad as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0xc2 as libc::c_int as uint8_t,
    0xc5 as libc::c_int as uint8_t,
    0xc7 as libc::c_int as uint8_t,
    0xb8 as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xb8 as libc::c_int as uint8_t,
    0xaa as libc::c_int as uint8_t,
    0xb3 as libc::c_int as uint8_t,
    0x8d as libc::c_int as uint8_t,
    0xb4 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0xd as libc::c_int as uint8_t,
    0x35 as libc::c_int as uint8_t,
    0x5d as libc::c_int as uint8_t,
    0xb2 as libc::c_int as uint8_t,
    0xb6 as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xaf as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x96 as libc::c_int as uint8_t,
    0x19 as libc::c_int as uint8_t,
    0x8f as libc::c_int as uint8_t,
    0x9f as libc::c_int as uint8_t,
    0xa7 as libc::c_int as uint8_t,
    0xe as libc::c_int as uint8_t,
    0x16 as libc::c_int as uint8_t,
    0x1e as libc::c_int as uint8_t,
    0xb4 as libc::c_int as uint8_t,
    0x9a as libc::c_int as uint8_t,
    0xb8 as libc::c_int as uint8_t,
    0xaa as libc::c_int as uint8_t,
    0x87 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x54 as libc::c_int as uint8_t,
    0x7c as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0x8a as libc::c_int as uint8_t,
    0xb6 as libc::c_int as uint8_t,
    0x91 as libc::c_int as uint8_t,
    0x32 as libc::c_int as uint8_t,
    0x56 as libc::c_int as uint8_t,
    0x76 as libc::c_int as uint8_t,
    0xb2 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x94 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xc8 as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0xcd as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xb8 as libc::c_int as uint8_t,
    0xb4 as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x99 as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x2d as libc::c_int as uint8_t,
    0x55 as libc::c_int as uint8_t,
    0x7d as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xe as libc::c_int as uint8_t,
    0x16 as libc::c_int as uint8_t,
    0x1e as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x54 as libc::c_int as uint8_t,
    0x7c as libc::c_int as uint8_t,
    0x92 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xf0 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x50 as libc::c_int as uint8_t,
    0x78 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0xc4 as libc::c_int as uint8_t,
    0xcd as libc::c_int as uint8_t,
    0xfc as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xd as libc::c_int as uint8_t,
    0xdb as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xfc as libc::c_int as uint8_t,
    0x2d as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xa6 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xa6 as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xb2 as libc::c_int as uint8_t,
    0xb6 as libc::c_int as uint8_t,
    0x86 as libc::c_int as uint8_t,
    0x96 as libc::c_int as uint8_t,
    0xa6 as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xc8 as libc::c_int as uint8_t,
    0x41 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xa6 as libc::c_int as uint8_t,
    0xc8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0xb4 as libc::c_int as uint8_t,
    0xb8 as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x92 as libc::c_int as uint8_t,
    0xf5 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x54 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x35 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0x18 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xc7 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0xc5 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0x69 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0xc1 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x14 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0x4e as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0x11 as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0xf5 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0x92 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x9 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0x1c as libc::c_int as uint8_t,
    0x1c as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xc0 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x29 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0xd as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0xc2 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0x1a as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xc7 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0xc7 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0x69 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0xc3 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x14 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0x2e as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0x11 as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0xf5 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0x92 as libc::c_int as uint8_t,
    0x50 as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x9 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0x1c as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xc0 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x49 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0xd as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0xc4 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xad as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0xcc as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x9 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0x92 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xf5 as libc::c_int as uint8_t,
    0x7c as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x3a as libc::c_int as uint8_t,
    0xcf as libc::c_int as uint8_t,
    0x94 as libc::c_int as uint8_t,
    0x4a as libc::c_int as uint8_t,
    0x6e as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0xdb as libc::c_int as uint8_t,
    0x69 as libc::c_int as uint8_t,
    0x31 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xad as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x87 as libc::c_int as uint8_t,
    0x95 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0x21 as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xa5 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0x17 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xae as libc::c_int as uint8_t,
    0x8e as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xc0 as libc::c_int as uint8_t,
    0xc3 as libc::c_int as uint8_t,
    0xae as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0xc6 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0xc3 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x85 as libc::c_int as uint8_t,
    0x95 as libc::c_int as uint8_t,
    0xc8 as libc::c_int as uint8_t,
    0xa5 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xc0 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0xe as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x8e as libc::c_int as uint8_t,
    0x9e as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xc6 as libc::c_int as uint8_t,
    0x3e as libc::c_int as uint8_t,
    0x56 as libc::c_int as uint8_t,
    0xf5 as libc::c_int as uint8_t,
    0x54 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x72 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x15 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0x45 as libc::c_int as uint8_t,
    0x85 as libc::c_int as uint8_t,
    0x6e as libc::c_int as uint8_t,
    0xf5 as libc::c_int as uint8_t,
    0x8e as libc::c_int as uint8_t,
    0x9e as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x42 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0x5a as libc::c_int as uint8_t,
    0x8e as libc::c_int as uint8_t,
    0x9e as libc::c_int as uint8_t,
    0x6 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x69 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x1c as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0x1e as libc::c_int as uint8_t,
    0x11 as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xf5 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x1e as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0x36 as libc::c_int as uint8_t,
    0x25 as libc::c_int as uint8_t,
    0xdb as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0x85 as libc::c_int as uint8_t,
    0xa5 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xc1 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0x85 as libc::c_int as uint8_t,
    0xa5 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x85 as libc::c_int as uint8_t,
    0x95 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0x9 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xa5 as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0x92 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xf5 as libc::c_int as uint8_t,
    0x78 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x1a as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x9f as libc::c_int as uint8_t,
    0x26 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0x21 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0x1d as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x87 as libc::c_int as uint8_t,
    0x9f as libc::c_int as uint8_t,
    0x39 as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0xaf as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xfb as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0xb as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0x87 as libc::c_int as uint8_t,
    0x9f as libc::c_int as uint8_t,
    0x39 as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0xaf as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0x1d as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0xfc as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x69 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xaf as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0x8f as libc::c_int as uint8_t,
    0x9f as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xcc as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0xdb as libc::c_int as uint8_t,
    0x45 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xaf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x8f as libc::c_int as uint8_t,
    0x9f as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xca as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x9 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xaf as libc::c_int as uint8_t,
    0x8f as libc::c_int as uint8_t,
    0xcb as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xad as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0x8d as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xa5 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xa6 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0x12 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x95 as libc::c_int as uint8_t,
    0xd as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xa6 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xa5 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xc1 as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xa6 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xb2 as libc::c_int as uint8_t,
    0xb6 as libc::c_int as uint8_t,
    0x86 as libc::c_int as uint8_t,
    0x96 as libc::c_int as uint8_t,
    0xa6 as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xca as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0x49 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xa6 as libc::c_int as uint8_t,
    0xcb as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0xb4 as libc::c_int as uint8_t,
    0xb8 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xad as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xc0 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x8f as libc::c_int as uint8_t,
    0xcb as libc::c_int as uint8_t,
    0xc3 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xb2 as libc::c_int as uint8_t,
    0xb6 as libc::c_int as uint8_t,
    0x86 as libc::c_int as uint8_t,
    0x96 as libc::c_int as uint8_t,
    0xc8 as libc::c_int as uint8_t,
    0xc1 as libc::c_int as uint8_t,
    0xcb as libc::c_int as uint8_t,
    0xc3 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0xb4 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x21 as libc::c_int as uint8_t,
    0xdb as libc::c_int as uint8_t,
    0x71 as libc::c_int as uint8_t,
    0x8d as libc::c_int as uint8_t,
    0x9d as libc::c_int as uint8_t,
    0x71 as libc::c_int as uint8_t,
    0x85 as libc::c_int as uint8_t,
    0x95 as libc::c_int as uint8_t,
    0x21 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xad as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x85 as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0x8d as libc::c_int as uint8_t,
    0x29 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0x85 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xc2 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x8d as libc::c_int as uint8_t,
    0x94 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0x11 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x87 as libc::c_int as uint8_t,
    0x21 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0xa as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xc8 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0x11 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xc8 as libc::c_int as uint8_t,
    0xbb as libc::c_int as uint8_t,
    0xaf as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xb8 as libc::c_int as uint8_t,
    0xf6 as libc::c_int as uint8_t,
    0xb5 as libc::c_int as uint8_t,
    0xb9 as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0x8a as libc::c_int as uint8_t,
    0x95 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0x3c as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x5c as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x7c as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0xa5 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xb1 as libc::c_int as uint8_t,
    0x85 as libc::c_int as uint8_t,
    0x30 as libc::c_int as uint8_t,
    0xf7 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0x30 as libc::c_int as uint8_t,
    0xad as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xb4 as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x99 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0x2d as libc::c_int as uint8_t,
    0x55 as libc::c_int as uint8_t,
    0x7d as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xb5 as libc::c_int as uint8_t,
    0x91 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xf6 as libc::c_int as uint8_t,
    0x29 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xfb as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xfc as libc::c_int as uint8_t,
    0x29 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0x51 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xfc as libc::c_int as uint8_t,
    0x51 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x79 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xfb as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xfc as libc::c_int as uint8_t,
    0x79 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa1 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa1 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xad as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0x93 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x54 as libc::c_int as uint8_t,
    0x7c as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xf6 as libc::c_int as uint8_t,
    0x9d as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0xdb as libc::c_int as uint8_t,
    0x2d as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x50 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xdb as libc::c_int as uint8_t,
    0x55 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x78 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xdb as libc::c_int as uint8_t,
    0x7d as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x9c as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0xf5 as libc::c_int as uint8_t,
    0x30 as libc::c_int as uint8_t,
    0xdb as libc::c_int as uint8_t,
    0x38 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x48 as libc::c_int as uint8_t,
    0xdb as libc::c_int as uint8_t,
    0x58 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x68 as libc::c_int as uint8_t,
    0xdb as libc::c_int as uint8_t,
    0x70 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x90 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x54 as libc::c_int as uint8_t,
    0x7c as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0x5c as libc::c_int as uint8_t,
    0x38 as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xae as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0x87 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xc1 as libc::c_int as uint8_t,
    0xc1 as libc::c_int as uint8_t,
    0xb1 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xc6 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0x36 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0x36 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0x36 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0x36 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0x36 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf7 as libc::c_int as uint8_t,
    0x8d as libc::c_int as uint8_t,
    0x9d as libc::c_int as uint8_t,
    0xad as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0x18 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xae as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf7 as libc::c_int as uint8_t,
    0xad as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0x30 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xae as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0xa7 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xc3 as libc::c_int as uint8_t,
    0xc5 as libc::c_int as uint8_t,
    0xc7 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x9b as libc::c_int as uint8_t,
    0xa7 as libc::c_int as uint8_t,
    0x7a as libc::c_int as uint8_t,
    0xad as libc::c_int as uint8_t,
    0xf7 as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x94 as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0xa7 as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xae as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0x19 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0xc6 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0x94 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0xf6 as libc::c_int as uint8_t,
    0x32 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0x93 as libc::c_int as uint8_t,
    0xdb as libc::c_int as uint8_t,
    0x9 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xaa as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xae as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xa7 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x9b as libc::c_int as uint8_t,
    0x7a as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x94 as libc::c_int as uint8_t,
    0xae as libc::c_int as uint8_t,
    0x19 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xaa as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x81 as libc::c_int as uint8_t,
    0xc0 as libc::c_int as uint8_t,
    0xc3 as libc::c_int as uint8_t,
    0xc5 as libc::c_int as uint8_t,
    0xc7 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0x92 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0xf6 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0xad as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0x50 as libc::c_int as uint8_t,
    0xad as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0x78 as libc::c_int as uint8_t,
    0xad as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0xa1 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xc3 as libc::c_int as uint8_t,
    0xc5 as libc::c_int as uint8_t,
    0xc7 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa1 as libc::c_int as uint8_t,
    0x81 as libc::c_int as uint8_t,
    0x94 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0x18 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0x89 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xc3 as libc::c_int as uint8_t,
    0xc5 as libc::c_int as uint8_t,
    0xc7 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xb8 as libc::c_int as uint8_t,
    0xb4 as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0x86 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x31 as libc::c_int as uint8_t,
    0x9b as libc::c_int as uint8_t,
    0x6 as libc::c_int as uint8_t,
    0x99 as libc::c_int as uint8_t,
    0x7 as libc::c_int as uint8_t,
    0xab as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x9b as libc::c_int as uint8_t,
    0xf0 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x14 as libc::c_int as uint8_t,
    0x40 as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0xb4 as libc::c_int as uint8_t,
    0xb8 as libc::c_int as uint8_t,
    0xf0 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x8a as libc::c_int as uint8_t,
    0x9a as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x50 as libc::c_int as uint8_t,
    0x78 as libc::c_int as uint8_t,
    0xb7 as libc::c_int as uint8_t,
    0x9b as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x29 as libc::c_int as uint8_t,
    0x51 as libc::c_int as uint8_t,
    0x79 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x70 as libc::c_int as uint8_t,
    0x59 as libc::c_int as uint8_t,
    0x44 as libc::c_int as uint8_t,
    0x69 as libc::c_int as uint8_t,
    0x38 as libc::c_int as uint8_t,
    0x64 as libc::c_int as uint8_t,
    0x48 as libc::c_int as uint8_t,
    0x31 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xbb as libc::c_int as uint8_t,
    0xab as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x54 as libc::c_int as uint8_t,
    0x7c as libc::c_int as uint8_t,
    0xf0 as libc::c_int as uint8_t,
    0xb3 as libc::c_int as uint8_t,
    0x8b as libc::c_int as uint8_t,
    0xb8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x50 as libc::c_int as uint8_t,
    0x78 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0xb4 as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0x26 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x59 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0xbb as libc::c_int as uint8_t,
    0xab as libc::c_int as uint8_t,
    0xb3 as libc::c_int as uint8_t,
    0x8b as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x26 as libc::c_int as uint8_t,
    0x46 as libc::c_int as uint8_t,
    0x66 as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0xb8 as libc::c_int as uint8_t,
    0xf0 as libc::c_int as uint8_t,
    0x8a as libc::c_int as uint8_t,
    0x9c as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x29 as libc::c_int as uint8_t,
    0x51 as libc::c_int as uint8_t,
    0x79 as libc::c_int as uint8_t,
    0x8b as libc::c_int as uint8_t,
    0x29 as libc::c_int as uint8_t,
    0x51 as libc::c_int as uint8_t,
    0x79 as libc::c_int as uint8_t,
    0x8a as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x70 as libc::c_int as uint8_t,
    0x59 as libc::c_int as uint8_t,
    0x8b as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x58 as libc::c_int as uint8_t,
    0x71 as libc::c_int as uint8_t,
    0x8a as libc::c_int as uint8_t,
    0x44 as libc::c_int as uint8_t,
    0x69 as libc::c_int as uint8_t,
    0x38 as libc::c_int as uint8_t,
    0x8b as libc::c_int as uint8_t,
    0x39 as libc::c_int as uint8_t,
    0x40 as libc::c_int as uint8_t,
    0x68 as libc::c_int as uint8_t,
    0x8a as libc::c_int as uint8_t,
    0x64 as libc::c_int as uint8_t,
    0x48 as libc::c_int as uint8_t,
    0x31 as libc::c_int as uint8_t,
    0x8b as libc::c_int as uint8_t,
    0x30 as libc::c_int as uint8_t,
    0x49 as libc::c_int as uint8_t,
    0x60 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x54 as libc::c_int as uint8_t,
    0x7c as libc::c_int as uint8_t,
    0xf0 as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x50 as libc::c_int as uint8_t,
    0x78 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0x26 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x59 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x26 as libc::c_int as uint8_t,
    0x46 as libc::c_int as uint8_t,
    0x66 as libc::c_int as uint8_t,
    0xf0 as libc::c_int as uint8_t,
    0x89 as libc::c_int as uint8_t,
    0x9c as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x29 as libc::c_int as uint8_t,
    0x51 as libc::c_int as uint8_t,
    0x79 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x70 as libc::c_int as uint8_t,
    0x59 as libc::c_int as uint8_t,
    0x44 as libc::c_int as uint8_t,
    0x69 as libc::c_int as uint8_t,
    0x38 as libc::c_int as uint8_t,
    0x64 as libc::c_int as uint8_t,
    0x48 as libc::c_int as uint8_t,
    0x31 as libc::c_int as uint8_t,
    0xa9 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x9 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x59 as libc::c_int as uint8_t,
    0x70 as libc::c_int as uint8_t,
    0xab as libc::c_int as uint8_t,
    0x11 as libc::c_int as uint8_t,
    0x38 as libc::c_int as uint8_t,
    0x40 as libc::c_int as uint8_t,
    0x69 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x19 as libc::c_int as uint8_t,
    0x31 as libc::c_int as uint8_t,
    0x48 as libc::c_int as uint8_t,
    0x60 as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x3c as libc::c_int as uint8_t,
    0x41 as libc::c_int as uint8_t,
    0x5c as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x7c as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x87 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0x19 as libc::c_int as uint8_t,
    0x86 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x6e as libc::c_int as uint8_t,
    0x76 as libc::c_int as uint8_t,
    0x7e as libc::c_int as uint8_t,
    0xa9 as libc::c_int as uint8_t,
    0x99 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x2d as libc::c_int as uint8_t,
    0x55 as libc::c_int as uint8_t,
    0x7d as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xb1 as libc::c_int as uint8_t,
    0xb5 as libc::c_int as uint8_t,
    0xb9 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xae as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xaa as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xab as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0x87 as libc::c_int as uint8_t,
    0xc4 as libc::c_int as uint8_t,
    0xaa as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xbb as libc::c_int as uint8_t,
    0xaf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xb9 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xb1 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0x8e as libc::c_int as uint8_t,
    0x60 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xc8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x93 as libc::c_int as uint8_t,
    0x85 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x4a as libc::c_int as uint8_t,
    0xb1 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0xb5 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0x9a as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0xb7 as libc::c_int as uint8_t,
    0x9f as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xae as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0x8a as libc::c_int as uint8_t,
    0xc2 as libc::c_int as uint8_t,
    0xb2 as libc::c_int as uint8_t,
    0xb6 as libc::c_int as uint8_t,
    0x8e as libc::c_int as uint8_t,
    0x9e as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xfb as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0x1d as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xad as libc::c_int as uint8_t,
    0x61 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xae as libc::c_int as uint8_t,
    0xfb as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xad as libc::c_int as uint8_t,
    0x19 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xae as libc::c_int as uint8_t,
    0xfb as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0x16 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xad as libc::c_int as uint8_t,
    0x8d as libc::c_int as uint8_t,
    0x61 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xf5 as libc::c_int as uint8_t,
    0x9c as libc::c_int as uint8_t,
    0x9c as libc::c_int as uint8_t,
    0x8d as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0x2b as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xb6 as libc::c_int as uint8_t,
    0xae as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0xb as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xae as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xad as libc::c_int as uint8_t,
    0x51 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xae as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xb9 as libc::c_int as uint8_t,
    0xb1 as libc::c_int as uint8_t,
    0xb6 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0x9c as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0xb9 as libc::c_int as uint8_t,
    0xb1 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0x9a as libc::c_int as uint8_t,
    0xb5 as libc::c_int as uint8_t,
    0xaa as libc::c_int as uint8_t,
    0xc0 as libc::c_int as uint8_t,
    0xfd as libc::c_int as uint8_t,
    0x30 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0xb7 as libc::c_int as uint8_t,
    0x9f as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0xb5 as libc::c_int as uint8_t,
    0x8b as libc::c_int as uint8_t,
    0x93 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0xab as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xab as libc::c_int as uint8_t,
    0xc0 as libc::c_int as uint8_t,
    0xc3 as libc::c_int as uint8_t,
    0xb2 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0xc1 as libc::c_int as uint8_t,
    0xc3 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xb1 as libc::c_int as uint8_t,
    0xb9 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0x8b as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0x91 as libc::c_int as uint8_t,
    0xb6 as libc::c_int as uint8_t,
    0x9 as libc::c_int as uint8_t,
    0xb4 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xab as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0x87 as libc::c_int as uint8_t,
    0x9c as libc::c_int as uint8_t,
    0xb9 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xdd as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xb3 as libc::c_int as uint8_t,
    0x8b as libc::c_int as uint8_t,
    0x8b as libc::c_int as uint8_t,
    0x8b as libc::c_int as uint8_t,
    0x8b as libc::c_int as uint8_t,
    0x8b as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0x87 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xb2 as libc::c_int as uint8_t,
    0x8b as libc::c_int as uint8_t,
    0xb6 as libc::c_int as uint8_t,
    0x9b as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0x87 as libc::c_int as uint8_t,
    0xb5 as libc::c_int as uint8_t,
    0x9a as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0x9b as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xdc as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0xb9 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xbb as libc::c_int as uint8_t,
    0xb3 as libc::c_int as uint8_t,
    0xb7 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xaa as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x9a as libc::c_int as uint8_t,
    0xaa as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0xb4 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0xa7 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0xb7 as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0x87 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x66 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0xf0 as libc::c_int as uint8_t,
    0x79 as libc::c_int as uint8_t,
    0x51 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x90 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x87 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0xa7 as libc::c_int as uint8_t,
    0x81 as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0x62 as libc::c_int as uint8_t,
    0x93 as libc::c_int as uint8_t,
    0xf0 as libc::c_int as uint8_t,
    0x71 as libc::c_int as uint8_t,
    0x71 as libc::c_int as uint8_t,
    0x60 as libc::c_int as uint8_t,
    0x85 as libc::c_int as uint8_t,
    0x94 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x29 as libc::c_int as uint8_t,
    0x51 as libc::c_int as uint8_t,
    0x79 as libc::c_int as uint8_t,
    0x90 as libc::c_int as uint8_t,
    0xa5 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x4c as libc::c_int as uint8_t,
    0x6c as libc::c_int as uint8_t,
    0x87 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0x95 as libc::c_int as uint8_t,
    0x18 as libc::c_int as uint8_t,
    0x85 as libc::c_int as uint8_t,
    0x78 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0x90 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x4c as libc::c_int as uint8_t,
    0x6c as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x6c as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x92 as libc::c_int as uint8_t,
    0x19 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0x26 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x4d as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0x48 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x96 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x39 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0x3c as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x95 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x39 as libc::c_int as uint8_t,
    0xa6 as libc::c_int as uint8_t,
    0x86 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0x87 as libc::c_int as uint8_t,
    0xa7 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x89 as libc::c_int as uint8_t,
    0x95 as libc::c_int as uint8_t,
    0x19 as libc::c_int as uint8_t,
    0xa9 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0x38 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x89 as libc::c_int as uint8_t,
    0x39 as libc::c_int as uint8_t,
    0xa9 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0x3c as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x2e as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x39 as libc::c_int as uint8_t,
    0x90 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x95 as libc::c_int as uint8_t,
    0x31 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x9 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x95 as libc::c_int as uint8_t,
    0x39 as libc::c_int as uint8_t,
    0xa9 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0x26 as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x90 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xd as libc::c_int as uint8_t,
    0x89 as libc::c_int as uint8_t,
    0x99 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0x21 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0x2e as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x89 as libc::c_int as uint8_t,
    0x99 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x31 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0x2e as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x86 as libc::c_int as uint8_t,
    0x96 as libc::c_int as uint8_t,
    0x31 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0x2e as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x87 as libc::c_int as uint8_t,
    0x31 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0x2e as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0x92 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0x41 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0x2e as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0x19 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0x2e as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xc0 as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x22 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xa6 as libc::c_int as uint8_t,
    0x2e as libc::c_int as uint8_t,
    0xa7 as libc::c_int as uint8_t,
    0x2e as libc::c_int as uint8_t,
    0xa9 as libc::c_int as uint8_t,
    0x22 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x29 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0x2a as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xa9 as libc::c_int as uint8_t,
    0x2e as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0x92 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0x31 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0xa6 as libc::c_int as uint8_t,
    0x96 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x9c as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0x30 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x9c as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x92 as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0x4c as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xca as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0x35 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x96 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0xa6 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
];
static mut read_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut read_condition: pthread_cond_t = pthread_cond_t {
    __data: {
        let mut init = __pthread_cond_s {
            __wseq: __atomic_wide_counter {
                __value64: 0 as libc::c_int as libc::c_ulonglong,
            },
            __g1_start: __atomic_wide_counter {
                __value64: 0 as libc::c_int as libc::c_ulonglong,
            },
            __g_refs: [0 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint],
            __g_size: [0 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint],
            __g1_orig_size: 0 as libc::c_int as libc::c_uint,
            __wrefs: 0 as libc::c_int as libc::c_uint,
            __g_signals: [0 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint],
        };
        init
    },
};
static mut tap_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut tap_condition: pthread_cond_t = pthread_cond_t {
    __data: {
        let mut init = __pthread_cond_s {
            __wseq: __atomic_wide_counter {
                __value64: 0 as libc::c_int as libc::c_ulonglong,
            },
            __g1_start: __atomic_wide_counter {
                __value64: 0 as libc::c_int as libc::c_ulonglong,
            },
            __g_refs: [0 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint],
            __g_size: [0 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint],
            __g1_orig_size: 0 as libc::c_int as libc::c_uint,
            __wrefs: 0 as libc::c_int as libc::c_uint,
            __g_signals: [0 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint],
        };
        init
    },
};
static mut config: rc_mpu_config_t = rc_mpu_config_t {
    gpio_interrupt_pin_chip: 0,
    gpio_interrupt_pin: 0,
    i2c_bus: 0,
    i2c_addr: 0,
    show_warnings: 0,
    accel_fsr: ACCEL_FSR_2G,
    gyro_fsr: GYRO_FSR_250DPS,
    accel_dlpf: ACCEL_DLPF_OFF,
    gyro_dlpf: GYRO_DLPF_OFF,
    enable_magnetometer: 0,
    dmp_sample_rate: 0,
    dmp_fetch_accel_gyro: 0,
    dmp_auto_calibrate_gyro: 0,
    orient: 0 as rc_mpu_orientation_t,
    compass_time_constant: 0.,
    dmp_interrupt_sched_policy: 0,
    dmp_interrupt_priority: 0,
    read_mag_after_callback: 0,
    mag_sample_rate_div: 0,
    tap_threshold: 0,
};
static mut bypass_en: libc::c_int = 0;
static mut dmp_en: libc::c_int = 0 as libc::c_int;
static mut packet_len: libc::c_int = 0;
static mut imu_interrupt_thread: pthread_t = 0;
static mut thread_running_flag: libc::c_int = 0;
static mut dmp_callback_func: Option<unsafe extern "C" fn() -> ()> = None;
static mut tap_callback_func: Option<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()> = None;
static mut mag_factory_adjust: [libc::c_double; 3] = [0.; 3];
static mut mag_offsets: [libc::c_double; 3] = [0.; 3];
static mut mag_scales: [libc::c_double; 3] = [0.; 3];
static mut accel_lengths: [libc::c_double; 3] = [0.; 3];
static mut last_read_successful: libc::c_int = 0;
static mut last_interrupt_timestamp_nanos: uint64_t = 0;
static mut last_tap_timestamp_nanos: uint64_t = 0;
static mut data_ptr: *mut rc_mpu_data_t = 0 as *const rc_mpu_data_t as *mut rc_mpu_data_t;
static mut imu_shutdown_flag: libc::c_int = 0 as libc::c_int;
static mut low_pass: rc_filter_t = rc_filter_t {
    order: 0,
    dt: 0.,
    gain: 0.,
    num: rc_vector_t {
        len: 0,
        d: 0 as *mut libc::c_double,
        initialized: 0,
    },
    den: rc_vector_t {
        len: 0,
        d: 0 as *mut libc::c_double,
        initialized: 0,
    },
    sat_en: 0,
    sat_min: 0.,
    sat_max: 0.,
    sat_flag: 0,
    ss_en: 0,
    ss_steps: 0.,
    in_buf: rc_ringbuf_t {
        d: 0 as *const libc::c_double as *mut libc::c_double,
        size: 0,
        index: 0,
        initialized: 0,
    },
    out_buf: rc_ringbuf_t {
        d: 0 as *const libc::c_double as *mut libc::c_double,
        size: 0,
        index: 0,
        initialized: 0,
    },
    newest_input: 0.,
    newest_output: 0.,
    step: 0,
    initialized: 0,
};
static mut high_pass: rc_filter_t = rc_filter_t {
    order: 0,
    dt: 0.,
    gain: 0.,
    num: rc_vector_t {
        len: 0,
        d: 0 as *mut libc::c_double,
        initialized: 0,
    },
    den: rc_vector_t {
        len: 0,
        d: 0 as *mut libc::c_double,
        initialized: 0,
    },
    sat_en: 0,
    sat_min: 0.,
    sat_max: 0.,
    sat_flag: 0,
    ss_en: 0,
    ss_steps: 0.,
    in_buf: rc_ringbuf_t {
        d: 0 as *const libc::c_double as *mut libc::c_double,
        size: 0,
        index: 0,
        initialized: 0,
    },
    out_buf: rc_ringbuf_t {
        d: 0 as *const libc::c_double as *mut libc::c_double,
        size: 0,
        index: 0,
        initialized: 0,
    },
    newest_input: 0.,
    newest_output: 0.,
    step: 0,
    initialized: 0,
};
static mut was_last_steady: libc::c_int = 0 as libc::c_int;
static mut startMagYaw: libc::c_double = 0.0f64;
#[no_mangle]
pub unsafe extern "C" fn rc_mpu_default_config() -> rc_mpu_config_t {
    let mut conf: rc_mpu_config_t = rc_mpu_config_t {
        gpio_interrupt_pin_chip: 0,
        gpio_interrupt_pin: 0,
        i2c_bus: 0,
        i2c_addr: 0,
        show_warnings: 0,
        accel_fsr: ACCEL_FSR_2G,
        gyro_fsr: GYRO_FSR_250DPS,
        accel_dlpf: ACCEL_DLPF_OFF,
        gyro_dlpf: GYRO_DLPF_OFF,
        enable_magnetometer: 0,
        dmp_sample_rate: 0,
        dmp_fetch_accel_gyro: 0,
        dmp_auto_calibrate_gyro: 0,
        orient: 0 as rc_mpu_orientation_t,
        compass_time_constant: 0.,
        dmp_interrupt_sched_policy: 0,
        dmp_interrupt_priority: 0,
        read_mag_after_callback: 0,
        mag_sample_rate_div: 0,
        tap_threshold: 0,
    };
    conf.gpio_interrupt_pin_chip = 3 as libc::c_int;
    conf.gpio_interrupt_pin = 21 as libc::c_int;
    conf.i2c_bus = 2 as libc::c_int;
    conf.i2c_addr = 0x68 as libc::c_int as uint8_t;
    conf.show_warnings = 0 as libc::c_int;
    conf.accel_fsr = ACCEL_FSR_8G;
    conf.gyro_fsr = GYRO_FSR_2000DPS;
    conf.accel_dlpf = ACCEL_DLPF_184;
    conf.gyro_dlpf = GYRO_DLPF_184;
    conf.enable_magnetometer = 0 as libc::c_int;
    conf.dmp_sample_rate = 100 as libc::c_int;
    conf.dmp_fetch_accel_gyro = 0 as libc::c_int;
    conf.dmp_auto_calibrate_gyro = 0 as libc::c_int;
    conf.orient = ORIENTATION_Z_UP;
    conf.compass_time_constant = 20.0f64;
    conf.dmp_interrupt_sched_policy = 0 as libc::c_int;
    conf.dmp_interrupt_priority = 0 as libc::c_int;
    conf.read_mag_after_callback = 1 as libc::c_int;
    conf.mag_sample_rate_div = 4 as libc::c_int;
    conf.tap_threshold = 210 as libc::c_int;
    return conf;
}
#[no_mangle]
pub unsafe extern "C" fn rc_mpu_set_config_to_default(mut conf: *mut rc_mpu_config_t) -> libc::c_int {
    *conf = rc_mpu_default_config();
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_mpu_initialize(mut data: *mut rc_mpu_data_t, mut conf: rc_mpu_config_t) -> libc::c_int {
    config = conf;
    if rc_i2c_get_lock(config.i2c_bus) != 0 {
        printf(b"i2c bus claimed by another process\n\0" as *const u8 as *const libc::c_char);
        printf(b"Continuing with rc_mpu_initialize() anyway.\n\0" as *const u8 as *const libc::c_char);
    }
    if rc_i2c_init(config.i2c_bus, config.i2c_addr) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"failed to initialize i2c bus\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_i2c_lock_bus(config.i2c_bus);
    if __reset_mpu() < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: failed to reset_mpu9250\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if __check_who_am_i() != 0 {
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if __load_gyro_calibration() < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: failed to load gyro calibration offsets\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if __load_accel_calibration() < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: failed to load accel calibration offsets\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if rc_i2c_write_byte(
        config.i2c_bus,
        0x19 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    ) != 0
    {
        fprintf(stderr, b"I2C bus write error\n\0" as *const u8 as *const libc::c_char);
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if __set_gyro_fsr(conf.gyro_fsr, data) != 0 {
        fprintf(
            stderr,
            b"failed to set gyro fsr\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if __set_accel_fsr(conf.accel_fsr, data) != 0 {
        fprintf(
            stderr,
            b"failed to set accel fsr\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if __set_gyro_dlpf(conf.gyro_dlpf) != 0 {
        fprintf(
            stderr,
            b"failed to set gyro dlpf\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if __set_accel_dlpf(conf.accel_dlpf) != 0 {
        fprintf(
            stderr,
            b"failed to set accel_dlpf\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if conf.enable_magnetometer != 0 {
        if __init_magnetometer(0 as libc::c_int) != 0 {
            fprintf(
                stderr,
                b"failed to initialize magnetometer\n\0" as *const u8 as *const libc::c_char,
            );
            rc_i2c_unlock_bus(config.i2c_bus);
            return -(1 as libc::c_int);
        }
    } else {
        __power_off_magnetometer();
    }
    rc_i2c_unlock_bus(config.i2c_bus);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_mpu_read_accel(mut data: *mut rc_mpu_data_t) -> libc::c_int {
    let mut raw: [uint8_t; 6] = [0; 6];
    rc_i2c_set_device_address(config.i2c_bus, config.i2c_addr);
    if rc_i2c_read_bytes(
        config.i2c_bus,
        0x3b as libc::c_int as uint8_t,
        6 as libc::c_int as size_t,
        &mut *raw.as_mut_ptr().offset(0 as libc::c_int as isize),
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    (*data).raw_accel[0 as libc::c_int as usize] = ((raw[0 as libc::c_int as usize] as uint16_t as libc::c_int)
        << 8 as libc::c_int
        | raw[1 as libc::c_int as usize] as libc::c_int) as int16_t;
    (*data).raw_accel[1 as libc::c_int as usize] = ((raw[2 as libc::c_int as usize] as uint16_t as libc::c_int)
        << 8 as libc::c_int
        | raw[3 as libc::c_int as usize] as libc::c_int) as int16_t;
    (*data).raw_accel[2 as libc::c_int as usize] = ((raw[4 as libc::c_int as usize] as uint16_t as libc::c_int)
        << 8 as libc::c_int
        | raw[5 as libc::c_int as usize] as libc::c_int) as int16_t;
    (*data).accel[0 as libc::c_int as usize] =
        (*data).raw_accel[0 as libc::c_int as usize] as libc::c_int as libc::c_double * (*data).accel_to_ms2
            / accel_lengths[0 as libc::c_int as usize];
    (*data).accel[1 as libc::c_int as usize] =
        (*data).raw_accel[1 as libc::c_int as usize] as libc::c_int as libc::c_double * (*data).accel_to_ms2
            / accel_lengths[1 as libc::c_int as usize];
    (*data).accel[2 as libc::c_int as usize] =
        (*data).raw_accel[2 as libc::c_int as usize] as libc::c_int as libc::c_double * (*data).accel_to_ms2
            / accel_lengths[2 as libc::c_int as usize];
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_mpu_read_gyro(mut data: *mut rc_mpu_data_t) -> libc::c_int {
    let mut raw: [uint8_t; 6] = [0; 6];
    rc_i2c_set_device_address(config.i2c_bus, config.i2c_addr);
    if rc_i2c_read_bytes(
        config.i2c_bus,
        0x43 as libc::c_int as uint8_t,
        6 as libc::c_int as size_t,
        &mut *raw.as_mut_ptr().offset(0 as libc::c_int as isize),
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    (*data).raw_gyro[0 as libc::c_int as usize] = ((raw[0 as libc::c_int as usize] as int16_t as libc::c_int)
        << 8 as libc::c_int
        | raw[1 as libc::c_int as usize] as libc::c_int) as int16_t;
    (*data).raw_gyro[1 as libc::c_int as usize] = ((raw[2 as libc::c_int as usize] as int16_t as libc::c_int)
        << 8 as libc::c_int
        | raw[3 as libc::c_int as usize] as libc::c_int) as int16_t;
    (*data).raw_gyro[2 as libc::c_int as usize] = ((raw[4 as libc::c_int as usize] as int16_t as libc::c_int)
        << 8 as libc::c_int
        | raw[5 as libc::c_int as usize] as libc::c_int) as int16_t;
    (*data).gyro[0 as libc::c_int as usize] =
        (*data).raw_gyro[0 as libc::c_int as usize] as libc::c_int as libc::c_double * (*data).gyro_to_degs;
    (*data).gyro[1 as libc::c_int as usize] =
        (*data).raw_gyro[1 as libc::c_int as usize] as libc::c_int as libc::c_double * (*data).gyro_to_degs;
    (*data).gyro[2 as libc::c_int as usize] =
        (*data).raw_gyro[2 as libc::c_int as usize] as libc::c_int as libc::c_double * (*data).gyro_to_degs;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_mpu_read_mag(mut data: *mut rc_mpu_data_t) -> libc::c_int {
    let mut raw: [uint8_t; 7] = [0; 7];
    let mut adc: [int16_t; 3] = [0; 3];
    let mut factory_cal_data: [libc::c_double; 3] = [0.; 3];
    if config.enable_magnetometer == 0 {
        fprintf(
            stderr,
            b"ERROR: can't read magnetometer unless it is enabled in \n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"rc_mpu_config_t struct before calling rc_mpu_initialize\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (rc_i2c_set_device_address(config.i2c_bus, 0xc as libc::c_int as uint8_t) != 0) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"ERROR: in rc_mpu_read_mag, failed to set i2c address\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut st1: uint8_t = 0;
    if (rc_i2c_read_byte(config.i2c_bus, 0x2 as libc::c_int as uint8_t, &mut st1) < 0 as libc::c_int) as libc::c_int
        as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"ERROR reading Magnetometer, i2c_bypass is probably not set\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if st1 as libc::c_int & 0x1 as libc::c_int == 0 {
        if config.show_warnings != 0 {
            printf(b"no new magnetometer data ready, skipping read\n\0" as *const u8 as *const libc::c_char);
        }
        return 0 as libc::c_int;
    }
    if (rc_i2c_read_bytes(
        config.i2c_bus,
        0x3 as libc::c_int as uint8_t,
        7 as libc::c_int as size_t,
        &mut *raw.as_mut_ptr().offset(0 as libc::c_int as isize),
    ) < 0 as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"ERROR: rc_mpu_read_mag failed to read data register\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if raw[6 as libc::c_int as usize] as libc::c_int & (0x1 as libc::c_int) << 3 as libc::c_int != 0 {
        if config.show_warnings != 0 {
            printf(b"WARNING: magnetometer saturated, discarding data\n\0" as *const u8 as *const libc::c_char);
        }
        return -(1 as libc::c_int);
    }
    adc[0 as libc::c_int as usize] = ((raw[1 as libc::c_int as usize] as int16_t as libc::c_int) << 8 as libc::c_int
        | raw[0 as libc::c_int as usize] as libc::c_int) as int16_t;
    adc[1 as libc::c_int as usize] = ((raw[3 as libc::c_int as usize] as int16_t as libc::c_int) << 8 as libc::c_int
        | raw[2 as libc::c_int as usize] as libc::c_int) as int16_t;
    adc[2 as libc::c_int as usize] = ((raw[5 as libc::c_int as usize] as int16_t as libc::c_int) << 8 as libc::c_int
        | raw[4 as libc::c_int as usize] as libc::c_int) as int16_t;
    factory_cal_data[0 as libc::c_int as usize] = adc[1 as libc::c_int as usize] as libc::c_int as libc::c_double
        * mag_factory_adjust[1 as libc::c_int as usize]
        * (4912.0f64 / 32760.0f64);
    factory_cal_data[1 as libc::c_int as usize] = adc[0 as libc::c_int as usize] as libc::c_int as libc::c_double
        * mag_factory_adjust[0 as libc::c_int as usize]
        * (4912.0f64 / 32760.0f64);
    factory_cal_data[2 as libc::c_int as usize] = -(adc[2 as libc::c_int as usize] as libc::c_int) as libc::c_double
        * mag_factory_adjust[2 as libc::c_int as usize]
        * (4912.0f64 / 32760.0f64);
    (*data).mag[0 as libc::c_int as usize] = (factory_cal_data[0 as libc::c_int as usize]
        - mag_offsets[0 as libc::c_int as usize])
        * mag_scales[0 as libc::c_int as usize];
    (*data).mag[1 as libc::c_int as usize] = (factory_cal_data[1 as libc::c_int as usize]
        - mag_offsets[1 as libc::c_int as usize])
        * mag_scales[1 as libc::c_int as usize];
    (*data).mag[2 as libc::c_int as usize] = (factory_cal_data[2 as libc::c_int as usize]
        - mag_offsets[2 as libc::c_int as usize])
        * mag_scales[2 as libc::c_int as usize];
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_mpu_read_temp(mut data: *mut rc_mpu_data_t) -> libc::c_int {
    let mut adc: uint16_t = 0;
    rc_i2c_set_device_address(config.i2c_bus, config.i2c_addr);
    if rc_i2c_read_word(config.i2c_bus, 0x41 as libc::c_int as uint8_t, &mut adc) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"failed to read IMU temperature registers\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*data).temp = 21.0f64 + adc as libc::c_int as libc::c_double / 333.87f64;
    return 0 as libc::c_int;
}
unsafe extern "C" fn __reset_mpu() -> libc::c_int {
    imu_shutdown_flag = 1 as libc::c_int;
    if rc_i2c_set_device_address(config.i2c_bus, config.i2c_addr) == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"ERROR resetting MPU, failed to set i2c device adddress\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_i2c_write_byte(
        config.i2c_bus,
        0x6b as libc::c_int as uint8_t,
        ((0x1 as libc::c_int) << 7 as libc::c_int) as uint8_t,
    ) == -(1 as libc::c_int)
    {
        rc_usleep(10000 as libc::c_int as libc::c_uint);
        if rc_i2c_write_byte(
            config.i2c_bus,
            0x6b as libc::c_int as uint8_t,
            ((0x1 as libc::c_int) << 7 as libc::c_int) as uint8_t,
        ) == -(1 as libc::c_int)
        {
            fprintf(
                stderr,
                b"ERROR resetting MPU, I2C write to reset bit failed\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    rc_usleep(10000 as libc::c_int as libc::c_uint);
    return 0 as libc::c_int;
}
unsafe extern "C" fn __check_who_am_i() -> libc::c_int {
    let mut c: uint8_t = 0;
    if rc_i2c_read_byte(config.i2c_bus, 0x75 as libc::c_int as uint8_t, &mut c) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"i2c_read_byte failed reading who_am_i register\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if c as libc::c_int != 0x68 as libc::c_int
        && c as libc::c_int != 0x69 as libc::c_int
        && c as libc::c_int != 0x70 as libc::c_int
        && c as libc::c_int != 0x71 as libc::c_int
        && c as libc::c_int != 0x73 as libc::c_int
        && c as libc::c_int != 75 as libc::c_int
    {
        fprintf(
            stderr,
            b"invalid who_am_i register: 0x%x\n\0" as *const u8 as *const libc::c_char,
            c as libc::c_int,
        );
        fprintf(
            stderr,
            b"expected 0x68 or 0x69 for mpu6050/9150, 0x70 for mpu6500, 0x71 for mpu9250, 0x75 for mpu9255,\n\0"
                as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __set_accel_fsr(mut fsr: rc_mpu_accel_fsr_t, mut data: *mut rc_mpu_data_t) -> libc::c_int {
    let mut c: uint8_t = 0;
    match fsr as libc::c_uint {
        0 => {
            c = ((0 as libc::c_int) << 3 as libc::c_int) as uint8_t;
            (*data).accel_to_ms2 = 9.80665f64 * 2.0f64 / 32768.0f64;
        }
        1 => {
            c = ((0x1 as libc::c_int) << 3 as libc::c_int) as uint8_t;
            (*data).accel_to_ms2 = 9.80665f64 * 4.0f64 / 32768.0f64;
        }
        2 => {
            c = ((0x2 as libc::c_int) << 3 as libc::c_int) as uint8_t;
            (*data).accel_to_ms2 = 9.80665f64 * 8.0f64 / 32768.0f64;
        }
        3 => {
            c = ((0x3 as libc::c_int) << 3 as libc::c_int) as uint8_t;
            (*data).accel_to_ms2 = 9.80665f64 * 16.0f64 / 32768.0f64;
        }
        _ => {
            fprintf(stderr, b"invalid accel fsr\n\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        }
    }
    return rc_i2c_write_byte(config.i2c_bus, 0x1c as libc::c_int as uint8_t, c);
}
unsafe extern "C" fn __set_gyro_fsr(mut fsr: rc_mpu_gyro_fsr_t, mut data: *mut rc_mpu_data_t) -> libc::c_int {
    let mut c: uint8_t = 0;
    match fsr as libc::c_uint {
        0 => {
            c = ((0 as libc::c_int) << 3 as libc::c_int | 0 as libc::c_int) as uint8_t;
            (*data).gyro_to_degs = 250.0f64 / 32768.0f64;
        }
        1 => {
            c = ((0x1 as libc::c_int) << 3 as libc::c_int | 0 as libc::c_int) as uint8_t;
            (*data).gyro_to_degs = 500.0f64 / 32768.0f64;
        }
        2 => {
            c = ((0x2 as libc::c_int) << 3 as libc::c_int | 0 as libc::c_int) as uint8_t;
            (*data).gyro_to_degs = 1000.0f64 / 32768.0f64;
        }
        3 => {
            c = ((0x3 as libc::c_int) << 3 as libc::c_int | 0 as libc::c_int) as uint8_t;
            (*data).gyro_to_degs = 2000.0f64 / 32768.0f64;
        }
        _ => {
            fprintf(stderr, b"invalid gyro fsr\n\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        }
    }
    return rc_i2c_write_byte(config.i2c_bus, 0x1b as libc::c_int as uint8_t, c);
}
unsafe extern "C" fn __set_accel_dlpf(mut dlpf: rc_mpu_accel_dlpf_t) -> libc::c_int {
    let mut c: uint8_t = ((0 as libc::c_int) << 3 as libc::c_int | 0x40 as libc::c_int) as uint8_t;
    match dlpf as libc::c_uint {
        0 => {
            c = ((0x1 as libc::c_int) << 3 as libc::c_int | 0x40 as libc::c_int) as uint8_t;
        }
        1 => {
            c = (c as libc::c_int | 0 as libc::c_int) as uint8_t;
        }
        2 => {
            c = (c as libc::c_int | 1 as libc::c_int) as uint8_t;
        }
        3 => {
            c = (c as libc::c_int | 2 as libc::c_int) as uint8_t;
        }
        4 => {
            c = (c as libc::c_int | 3 as libc::c_int) as uint8_t;
        }
        5 => {
            c = (c as libc::c_int | 4 as libc::c_int) as uint8_t;
        }
        6 => {
            c = (c as libc::c_int | 5 as libc::c_int) as uint8_t;
        }
        7 => {
            c = (c as libc::c_int | 6 as libc::c_int) as uint8_t;
        }
        _ => {
            fprintf(
                stderr,
                b"invalid config.accel_dlpf\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    return rc_i2c_write_byte(config.i2c_bus, 0x1d as libc::c_int as uint8_t, c);
}
unsafe extern "C" fn __set_gyro_dlpf(mut dlpf: rc_mpu_gyro_dlpf_t) -> libc::c_int {
    let mut c: uint8_t = 0 as libc::c_int as uint8_t;
    match dlpf as libc::c_uint {
        0 => {
            c = (c as libc::c_int | 7 as libc::c_int) as uint8_t;
        }
        1 => {
            c = (c as libc::c_int | 0 as libc::c_int) as uint8_t;
        }
        2 => {
            c = (c as libc::c_int | 1 as libc::c_int) as uint8_t;
        }
        3 => {
            c = (c as libc::c_int | 2 as libc::c_int) as uint8_t;
        }
        4 => {
            c = (c as libc::c_int | 3 as libc::c_int) as uint8_t;
        }
        5 => {
            c = (c as libc::c_int | 4 as libc::c_int) as uint8_t;
        }
        6 => {
            c = (c as libc::c_int | 5 as libc::c_int) as uint8_t;
        }
        7 => {
            c = (c as libc::c_int | 6 as libc::c_int) as uint8_t;
        }
        _ => {
            fprintf(stderr, b"invalid gyro_dlpf\n\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        }
    }
    return rc_i2c_write_byte(config.i2c_bus, 0x1a as libc::c_int as uint8_t, c);
}
unsafe extern "C" fn __init_magnetometer(mut cal_mode: libc::c_int) -> libc::c_int {
    let mut raw: [uint8_t; 3] = [0; 3];
    if __mpu_set_bypass(1 as libc::c_int as libc::c_uchar) != 0 {
        fprintf(
            stderr,
            b"failed to set mpu9250 into bypass i2c mode\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_i2c_set_device_address(config.i2c_bus, 0xc as libc::c_int as uint8_t) != 0 {
        fprintf(
            stderr,
            b"ERROR: in __init_magnetometer, failed to set i2c device address\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_i2c_write_byte(
        config.i2c_bus,
        0xa as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"ERROR: in __init_magnetometer, failed to write to AK8963_CNTL register to power down\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_usleep(1000 as libc::c_int as libc::c_uint);
    if rc_i2c_write_byte(
        config.i2c_bus,
        0xa as libc::c_int as uint8_t,
        0xf as libc::c_int as uint8_t,
    ) != 0
    {
        fprintf(
            stderr,
            b"ERROR: in __init_magnetometer, failed to write to AK8963_CNTL register\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_usleep(1000 as libc::c_int as libc::c_uint);
    if rc_i2c_read_bytes(
        config.i2c_bus,
        0x10 as libc::c_int as uint8_t,
        3 as libc::c_int as size_t,
        &mut *raw.as_mut_ptr().offset(0 as libc::c_int as isize),
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"failed to read magnetometer adjustment register\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_set_device_address(config.i2c_bus, config.i2c_addr);
        return -(1 as libc::c_int);
    }
    mag_factory_adjust[0 as libc::c_int as usize] =
        (raw[0 as libc::c_int as usize] as libc::c_int - 128 as libc::c_int) as libc::c_double / 256.0f64 + 1.0f64;
    mag_factory_adjust[1 as libc::c_int as usize] =
        (raw[1 as libc::c_int as usize] as libc::c_int - 128 as libc::c_int) as libc::c_double / 256.0f64 + 1.0f64;
    mag_factory_adjust[2 as libc::c_int as usize] =
        (raw[2 as libc::c_int as usize] as libc::c_int - 128 as libc::c_int) as libc::c_double / 256.0f64 + 1.0f64;
    if rc_i2c_write_byte(
        config.i2c_bus,
        0xa as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    ) != 0
    {
        fprintf(
            stderr,
            b"ERROR: in __init_magnetometer, failed to write to AK8963_CNTL register to power on\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_usleep(100 as libc::c_int as libc::c_uint);
    let mut c: uint8_t = ((0x1 as libc::c_int) << 4 as libc::c_int | 0x6 as libc::c_int) as uint8_t;
    if rc_i2c_write_byte(config.i2c_bus, 0xa as libc::c_int as uint8_t, c) != 0 {
        fprintf(
            stderr,
            b"ERROR: in __init_magnetometer, failed to write to AK8963_CNTL register to set sampling mode\n\0"
                as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_usleep(100 as libc::c_int as libc::c_uint);
    rc_i2c_set_device_address(config.i2c_bus, config.i2c_addr);
    if cal_mode == 0 {
        __load_mag_calibration();
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __power_off_magnetometer() -> libc::c_int {
    rc_i2c_set_device_address(config.i2c_bus, config.i2c_addr);
    if __mpu_set_bypass(1 as libc::c_int as libc::c_uchar) != 0 {
        fprintf(
            stderr,
            b"failed to set mpu9250 into bypass i2c mode\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_i2c_set_device_address(config.i2c_bus, 0xc as libc::c_int as uint8_t);
    if rc_i2c_write_byte(
        config.i2c_bus,
        0xa as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"failed to write to magnetometer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_i2c_set_device_address(config.i2c_bus, config.i2c_addr);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_mpu_power_off() -> libc::c_int {
    imu_shutdown_flag = 1 as libc::c_int;
    if thread_running_flag != 0 {
        if rc_pthread_timed_join(
            imu_interrupt_thread,
            0 as *mut *mut libc::c_void,
            1.0f64 as libc::c_float,
        ) == 1 as libc::c_int
        {
            fprintf(
                stderr,
                b"WARNING: mpu interrupt thread exit timeout\n\0" as *const u8 as *const libc::c_char,
            );
        }
        pthread_cond_destroy(&mut read_condition);
        pthread_mutex_destroy(&mut read_mutex);
        pthread_cond_destroy(&mut tap_condition);
        pthread_mutex_destroy(&mut tap_mutex);
    }
    if config.enable_magnetometer != 0 {
        __power_off_magnetometer();
    }
    rc_i2c_set_device_address(config.i2c_bus, config.i2c_addr);
    if rc_i2c_write_byte(
        config.i2c_bus,
        0x6b as libc::c_int as uint8_t,
        ((0x1 as libc::c_int) << 7 as libc::c_int) as uint8_t,
    ) != 0
    {
        rc_usleep(1000 as libc::c_int as libc::c_uint);
        if rc_i2c_write_byte(
            config.i2c_bus,
            0x6b as libc::c_int as uint8_t,
            ((0x1 as libc::c_int) << 7 as libc::c_int) as uint8_t,
        ) != 0
        {
            fprintf(
                stderr,
                b"I2C write to MPU9250 Failed\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    if rc_i2c_write_byte(
        config.i2c_bus,
        0x6b as libc::c_int as uint8_t,
        ((0x1 as libc::c_int) << 6 as libc::c_int) as uint8_t,
    ) != 0
    {
        rc_usleep(1000 as libc::c_int as libc::c_uint);
        if rc_i2c_write_byte(
            config.i2c_bus,
            0x6b as libc::c_int as uint8_t,
            ((0x1 as libc::c_int) << 6 as libc::c_int) as uint8_t,
        ) != 0
        {
            fprintf(
                stderr,
                b"I2C write to MPU9250 Failed\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    if dmp_en != 0 {
        rc_gpio_cleanup(config.gpio_interrupt_pin_chip, config.gpio_interrupt_pin);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_mpu_initialize_dmp(mut data: *mut rc_mpu_data_t, mut conf: rc_mpu_config_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tmp: uint8_t = 0;
    if conf.dmp_sample_rate > 200 as libc::c_int || conf.dmp_sample_rate < 4 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR:dmp_sample_rate must be between %d & %d\n\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int,
            200 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if 200 as libc::c_int % conf.dmp_sample_rate != 0 as libc::c_int {
        fprintf(
            stderr,
            b"DMP sample rate must be a divisor of 200\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"acceptable values: 200,100,50,40,25,20,10,8,5,4 (HZ)\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if conf.enable_magnetometer != 0 && conf.compass_time_constant <= 0.1f64 {
        fprintf(
            stderr,
            b"ERROR: compass time constant must be greater than 0.1\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    config = conf;
    data_ptr = data;
    if conf.gyro_dlpf as libc::c_uint == GYRO_DLPF_OFF as libc::c_int as libc::c_uint
        || conf.gyro_dlpf as libc::c_uint == GYRO_DLPF_250 as libc::c_int as libc::c_uint
    {
        fprintf(
            stderr,
            b"WARNING, gyro dlpf bandwidth must be <= 184hz in DMP mode\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"setting to 184hz automatically\n\0" as *const u8 as *const libc::c_char,
        );
        conf.gyro_dlpf = GYRO_DLPF_184;
    }
    if conf.accel_dlpf as libc::c_uint == ACCEL_DLPF_OFF as libc::c_int as libc::c_uint
        || conf.accel_dlpf as libc::c_uint == ACCEL_DLPF_460 as libc::c_int as libc::c_uint
    {
        fprintf(
            stderr,
            b"WARNING, accel dlpf bandwidth must be <= 184hz in DMP mode\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"setting to 184hz automatically\n\0" as *const u8 as *const libc::c_char,
        );
        conf.accel_dlpf = ACCEL_DLPF_184;
    }
    if conf.gyro_fsr as libc::c_uint != GYRO_FSR_2000DPS as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"WARNING, gyro FSR must be GYRO_FSR_2000DPS in DMP mode\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"setting to 2000DPS automatically\n\0" as *const u8 as *const libc::c_char,
        );
        config.gyro_fsr = GYRO_FSR_2000DPS;
    }
    if conf.accel_fsr as libc::c_uint != ACCEL_FSR_8G as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"WARNING, accel FSR must be ACCEL_FSR_8G in DMP mode\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"setting to ACCEL_FSR_8G automatically\n\0" as *const u8 as *const libc::c_char,
        );
        config.accel_fsr = ACCEL_FSR_8G;
    }
    if rc_i2c_init(config.i2c_bus, config.i2c_addr) != 0 {
        fprintf(
            stderr,
            b"rc_mpu_initialize_dmp failed at rc_i2c_init\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_gpio_init_event(
        config.gpio_interrupt_pin_chip,
        config.gpio_interrupt_pin,
        0 as libc::c_int,
        ((1 as libc::c_ulong) << 1 as libc::c_int) as libc::c_int,
    ) == -(1 as libc::c_int)
    {
        fprintf(
            stderr,
            b"ERROR: in rc_mpu_initialize_dmp, failed to initialize GPIO\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"probably insufficient privileges\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_i2c_lock_bus(config.i2c_bus);
    if __reset_mpu() < 0 as libc::c_int {
        fprintf(
            stderr,
            b"failed to __reset_mpu()\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if __check_who_am_i() != 0 {
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    tmp = (0x40 as libc::c_int | 0x8 as libc::c_int) as uint8_t;
    if rc_i2c_write_byte(config.i2c_bus, 0x1d as libc::c_int as uint8_t, tmp) != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_mpu_initialize_dmp, failed to write to ACCEL_CONFIG_2 register\n\0" as *const u8
                as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if __load_gyro_calibration() < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: failed to load gyro calibration offsets\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if __load_accel_calibration() < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: failed to load accel calibration offsets\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if __set_gyro_fsr(config.gyro_fsr, data_ptr) == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"ERROR in rc_mpu_initialize_dmp, failed to set gyro_fsr register\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if __set_accel_fsr(config.accel_fsr, data_ptr) == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"ERROR in rc_mpu_initialize_dmp, failed to set accel_fsr register\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if __set_gyro_dlpf(conf.gyro_dlpf) != 0 {
        fprintf(
            stderr,
            b"failed to set gyro dlpf\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if __set_accel_dlpf(conf.accel_dlpf) != 0 {
        fprintf(
            stderr,
            b"failed to set accel_dlpf\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if __mpu_set_sample_rate(200 as libc::c_int) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: setting IMU sample rate\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if __mpu_set_bypass(1 as libc::c_int as libc::c_uchar) != 0 {
        fprintf(
            stderr,
            b"failed to run __mpu_set_bypass\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if conf.enable_magnetometer != 0 {
        if __init_magnetometer(0 as libc::c_int) != 0 {
            fprintf(
                stderr,
                b"ERROR: failed to initialize_magnetometer\n\0" as *const u8 as *const libc::c_char,
            );
            rc_i2c_unlock_bus(config.i2c_bus);
            return -(1 as libc::c_int);
        }
        if rc_mpu_read_mag(data) == -(1 as libc::c_int) {
            fprintf(
                stderr,
                b"ERROR: failed to initialize_magnetometer\n\0" as *const u8 as *const libc::c_char,
            );
            rc_i2c_unlock_bus(config.i2c_bus);
            return -(1 as libc::c_int);
        }
        let mut x_sum: libc::c_double = 0.0f64;
        let mut y_sum: libc::c_double = 0.0f64;
        let mut mag_vec: [libc::c_double; 3] = [0.; 3];
        i = 0 as libc::c_int;
        while i < 20 as libc::c_int {
            rc_mpu_read_mag(data);
            if __mag_correct_orientation(mag_vec.as_mut_ptr()) != 0 {
                return -(1 as libc::c_int);
            }
            x_sum += mag_vec[0 as libc::c_int as usize];
            y_sum += mag_vec[1 as libc::c_int as usize];
            rc_usleep(10000 as libc::c_int as libc::c_uint);
            i += 1;
        }
        startMagYaw = -atan2(y_sum, x_sum);
    } else {
        __power_off_magnetometer();
    }
    dmp_en = 1 as libc::c_int;
    if __dmp_load_motion_driver_firmware() < 0 as libc::c_int {
        fprintf(
            stderr,
            b"failed to load DMP motion driver\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if __dmp_set_orientation(conf.orient as libc::c_ushort) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: failed to set dmp orientation\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    let mut feature_mask: libc::c_ushort = (0x10 as libc::c_int | 0x1 as libc::c_int) as libc::c_ushort;
    if config.dmp_auto_calibrate_gyro != 0 {
        feature_mask = (feature_mask as libc::c_int | 0x20 as libc::c_int) as libc::c_ushort;
    }
    if config.dmp_fetch_accel_gyro != 0 {
        feature_mask = (feature_mask as libc::c_int
            | (0x40 as libc::c_int | (0x80 as libc::c_int | 0x100 as libc::c_int)))
            as libc::c_ushort;
    }
    if __dmp_enable_feature(feature_mask) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: failed to enable DMP features\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if __dmp_set_fifo_rate(config.dmp_sample_rate as libc::c_ushort) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: failed to set DMP fifo rate\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if __mpu_set_dmp_state(1 as libc::c_int as libc::c_uchar) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: __mpu_set_dmp_state(1) failed\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if __dmp_set_interrupt_mode(0x2 as libc::c_int as libc::c_uchar) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: failed to set DMP interrupt mode to continuous\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    rc_i2c_unlock_bus(config.i2c_bus);
    (*data_ptr).tap_detected = 0 as libc::c_int;
    imu_shutdown_flag = 0 as libc::c_int;
    dmp_callback_func = None;
    tap_callback_func = None;
    if rc_pthread_create(
        &mut imu_interrupt_thread,
        Some(__dmp_interrupt_handler as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
        config.dmp_interrupt_sched_policy,
        config.dmp_interrupt_priority,
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"ERROR failed to start dmp handler thread\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    thread_running_flag = 1 as libc::c_int;
    rc_usleep(1000 as libc::c_int as libc::c_uint);
    return 0 as libc::c_int;
}
unsafe extern "C" fn __mpu_write_mem(
    mut mem_addr: libc::c_ushort,
    mut length: libc::c_ushort,
    mut data: *mut libc::c_uchar,
) -> libc::c_int {
    let mut tmp: [libc::c_uchar; 2] = [0; 2];
    if data.is_null() {
        fprintf(
            stderr,
            b"ERROR: in mpu_write_mem, NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    tmp[0 as libc::c_int as usize] = (mem_addr as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
    tmp[1 as libc::c_int as usize] = (mem_addr as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    if tmp[1 as libc::c_int as usize] as libc::c_int + length as libc::c_int > 256 as libc::c_int {
        fprintf(
            stderr,
            b"mpu_write_mem exceeds bank size\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_i2c_write_bytes(
        config.i2c_bus,
        0x6d as libc::c_int as uint8_t,
        2 as libc::c_int as size_t,
        tmp.as_mut_ptr(),
    ) != 0
    {
        return -(1 as libc::c_int);
    }
    if rc_i2c_write_bytes(config.i2c_bus, 0x6f as libc::c_int as uint8_t, length as size_t, data) != 0 {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __mpu_read_mem(
    mut mem_addr: libc::c_ushort,
    mut length: libc::c_ushort,
    mut data: *mut libc::c_uchar,
) -> libc::c_int {
    let mut tmp: [libc::c_uchar; 2] = [0; 2];
    if data.is_null() {
        fprintf(
            stderr,
            b"ERROR: in mpu_write_mem, NULL pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    tmp[0 as libc::c_int as usize] = (mem_addr as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
    tmp[1 as libc::c_int as usize] = (mem_addr as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    if tmp[1 as libc::c_int as usize] as libc::c_int + length as libc::c_int > 256 as libc::c_int {
        printf(b"mpu_read_mem exceeds bank size\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if rc_i2c_write_bytes(
        config.i2c_bus,
        0x6d as libc::c_int as uint8_t,
        2 as libc::c_int as size_t,
        tmp.as_mut_ptr(),
    ) != 0
    {
        return -(1 as libc::c_int);
    }
    if rc_i2c_read_bytes(config.i2c_bus, 0x6f as libc::c_int as uint8_t, length as size_t, data)
        != length as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __dmp_load_motion_driver_firmware() -> libc::c_int {
    let mut ii: libc::c_ushort = 0;
    let mut this_write: libc::c_ushort = 0;
    let mut cur: [libc::c_uchar; 16] = [0; 16];
    let mut tmp: [libc::c_uchar; 2] = [0; 2];
    rc_i2c_set_device_address(config.i2c_bus, config.i2c_addr);
    ii = 0 as libc::c_int as libc::c_ushort;
    while (ii as libc::c_int) < 3062 as libc::c_int {
        this_write = (if (16 as libc::c_int) < 3062 as libc::c_int - ii as libc::c_int {
            16 as libc::c_int
        } else {
            3062 as libc::c_int - ii as libc::c_int
        }) as libc::c_ushort;
        if __mpu_write_mem(
            ii,
            this_write,
            &*dmp_firmware.as_ptr().offset(ii as isize) as *const uint8_t as *mut uint8_t,
        ) != 0
        {
            fprintf(
                stderr,
                b"dmp firmware write failed\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if __mpu_read_mem(ii, this_write, cur.as_mut_ptr()) != 0 {
            fprintf(
                stderr,
                b"dmp firmware read failed\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if memcmp(
            dmp_firmware.as_ptr().offset(ii as libc::c_int as isize) as *const libc::c_void,
            cur.as_mut_ptr() as *const libc::c_void,
            this_write as libc::c_ulong,
        ) != 0
        {
            fprintf(
                stderr,
                b"dmp firmware write corrupted\n\0" as *const u8 as *const libc::c_char,
            );
            return -(2 as libc::c_int);
        }
        ii = (ii as libc::c_int + this_write as libc::c_int) as libc::c_ushort;
    }
    tmp[0 as libc::c_int as usize] = (dmp_start_addr as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
    tmp[1 as libc::c_int as usize] = (dmp_start_addr as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    if rc_i2c_write_bytes(
        config.i2c_bus,
        0x70 as libc::c_int as uint8_t,
        2 as libc::c_int as size_t,
        tmp.as_mut_ptr(),
    ) != 0
    {
        fprintf(
            stderr,
            b"ERROR writing to MPU6500_PRGM_START register\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __dmp_set_orientation(mut orient: libc::c_ushort) -> libc::c_int {
    let mut gyro_regs: [libc::c_uchar; 3] = [0; 3];
    let mut accel_regs: [libc::c_uchar; 3] = [0; 3];
    let gyro_axes: [libc::c_uchar; 3] = [
        0x4c as libc::c_int as libc::c_uchar,
        0xcd as libc::c_int as libc::c_uchar,
        0x6c as libc::c_int as libc::c_uchar,
    ];
    let accel_axes: [libc::c_uchar; 3] = [
        0xc as libc::c_int as libc::c_uchar,
        0xc9 as libc::c_int as libc::c_uchar,
        0x2c as libc::c_int as libc::c_uchar,
    ];
    let gyro_sign: [libc::c_uchar; 3] = [
        0x36 as libc::c_int as libc::c_uchar,
        0x56 as libc::c_int as libc::c_uchar,
        0x76 as libc::c_int as libc::c_uchar,
    ];
    let accel_sign: [libc::c_uchar; 3] = [
        0x26 as libc::c_int as libc::c_uchar,
        0x46 as libc::c_int as libc::c_uchar,
        0x66 as libc::c_int as libc::c_uchar,
    ];
    gyro_regs[0 as libc::c_int as usize] = gyro_axes[(orient as libc::c_int & 3 as libc::c_int) as usize];
    gyro_regs[1 as libc::c_int as usize] =
        gyro_axes[(orient as libc::c_int >> 3 as libc::c_int & 3 as libc::c_int) as usize];
    gyro_regs[2 as libc::c_int as usize] =
        gyro_axes[(orient as libc::c_int >> 6 as libc::c_int & 3 as libc::c_int) as usize];
    accel_regs[0 as libc::c_int as usize] = accel_axes[(orient as libc::c_int & 3 as libc::c_int) as usize];
    accel_regs[1 as libc::c_int as usize] =
        accel_axes[(orient as libc::c_int >> 3 as libc::c_int & 3 as libc::c_int) as usize];
    accel_regs[2 as libc::c_int as usize] =
        accel_axes[(orient as libc::c_int >> 6 as libc::c_int & 3 as libc::c_int) as usize];
    if __mpu_write_mem(
        1062 as libc::c_int as libc::c_ushort,
        3 as libc::c_int as libc::c_ushort,
        gyro_regs.as_mut_ptr(),
    ) != 0
    {
        fprintf(
            stderr,
            b"ERROR: in dmp_set_orientation, failed to write dmp mem\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if __mpu_write_mem(
        1066 as libc::c_int as libc::c_ushort,
        3 as libc::c_int as libc::c_ushort,
        accel_regs.as_mut_ptr(),
    ) != 0
    {
        fprintf(
            stderr,
            b"ERROR: in dmp_set_orientation, failed to write dmp mem\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    memcpy(
        gyro_regs.as_mut_ptr() as *mut libc::c_void,
        gyro_sign.as_ptr() as *const libc::c_void,
        3 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        accel_regs.as_mut_ptr() as *mut libc::c_void,
        accel_sign.as_ptr() as *const libc::c_void,
        3 as libc::c_int as libc::c_ulong,
    );
    if orient as libc::c_int & 4 as libc::c_int != 0 {
        gyro_regs[0 as libc::c_int as usize] =
            (gyro_regs[0 as libc::c_int as usize] as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
        accel_regs[0 as libc::c_int as usize] =
            (accel_regs[0 as libc::c_int as usize] as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
    }
    if orient as libc::c_int & 0x20 as libc::c_int != 0 {
        gyro_regs[1 as libc::c_int as usize] =
            (gyro_regs[1 as libc::c_int as usize] as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
        accel_regs[1 as libc::c_int as usize] =
            (accel_regs[1 as libc::c_int as usize] as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
    }
    if orient as libc::c_int & 0x100 as libc::c_int != 0 {
        gyro_regs[2 as libc::c_int as usize] =
            (gyro_regs[2 as libc::c_int as usize] as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
        accel_regs[2 as libc::c_int as usize] =
            (accel_regs[2 as libc::c_int as usize] as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
    }
    if __mpu_write_mem(
        1088 as libc::c_int as libc::c_ushort,
        3 as libc::c_int as libc::c_ushort,
        gyro_regs.as_mut_ptr(),
    ) != 0
    {
        fprintf(
            stderr,
            b"ERROR: in dmp_set_orientation, failed to write dmp mem\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if __mpu_write_mem(
        1073 as libc::c_int as libc::c_ushort,
        3 as libc::c_int as libc::c_ushort,
        accel_regs.as_mut_ptr(),
    ) != 0
    {
        fprintf(
            stderr,
            b"ERROR: in dmp_set_orientation, failed to write dmp mem\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __dmp_set_fifo_rate(mut rate: libc::c_ushort) -> libc::c_int {
    let regs_end: [libc::c_uchar; 12] = [
        0xfe as libc::c_int as libc::c_uchar,
        0xf2 as libc::c_int as libc::c_uchar,
        0xab as libc::c_int as libc::c_uchar,
        0xc4 as libc::c_int as libc::c_uchar,
        0xaa as libc::c_int as libc::c_uchar,
        0xf1 as libc::c_int as libc::c_uchar,
        0xdf as libc::c_int as libc::c_uchar,
        0xdf as libc::c_int as libc::c_uchar,
        0xbb as libc::c_int as libc::c_uchar,
        0xaf as libc::c_int as libc::c_uchar,
        0xdf as libc::c_int as libc::c_uchar,
        0xdf as libc::c_int as libc::c_uchar,
    ];
    let mut div: libc::c_ushort = 0;
    let mut tmp: [libc::c_uchar; 8] = [0; 8];
    if rate as libc::c_int > 200 as libc::c_int {
        return -(1 as libc::c_int);
    }
    div = (200 as libc::c_int / rate as libc::c_int - 1 as libc::c_int) as libc::c_ushort;
    tmp[0 as libc::c_int as usize] = (div as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    tmp[1 as libc::c_int as usize] = (div as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    if __mpu_write_mem(
        (22 as libc::c_int + 512 as libc::c_int) as libc::c_ushort,
        2 as libc::c_int as libc::c_ushort,
        tmp.as_mut_ptr(),
    ) != 0
    {
        fprintf(
            stderr,
            b"ERROR: writing dmp sample rate reg\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if __mpu_write_mem(
        2753 as libc::c_int as libc::c_ushort,
        12 as libc::c_int as libc::c_ushort,
        regs_end.as_ptr() as *mut libc::c_uchar,
    ) != 0
    {
        fprintf(
            stderr,
            b"ERROR: writing dmp regs_end\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __mpu_set_bypass(mut bypass_on: uint8_t) -> libc::c_int {
    let mut tmp: uint8_t = 0 as libc::c_int as uint8_t;
    rc_i2c_set_device_address(config.i2c_bus, config.i2c_addr);
    if dmp_en != 0 {
        tmp = (tmp as libc::c_int | (0x1 as libc::c_int) << 6 as libc::c_int) as uint8_t;
    }
    if bypass_on == 0 {
        tmp = (tmp as libc::c_int | (0x1 as libc::c_int) << 5 as libc::c_int) as uint8_t;
    }
    if rc_i2c_write_byte(config.i2c_bus, 0x6a as libc::c_int as uint8_t, tmp) != 0 {
        fprintf(
            stderr,
            b"ERROR in mpu_set_bypass, failed to write USER_CTRL register\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_usleep(3000 as libc::c_int as libc::c_uint);
    tmp = ((0x1 as libc::c_int) << 5 as libc::c_int
        | (0x1 as libc::c_int) << 4 as libc::c_int
        | (0x1 as libc::c_int) << 7 as libc::c_int) as uint8_t;
    if bypass_on != 0 {
        tmp = (tmp as libc::c_int | (0x1 as libc::c_int) << 1 as libc::c_int) as uint8_t;
    }
    if rc_i2c_write_byte(config.i2c_bus, 0x37 as libc::c_int as uint8_t, tmp) != 0 {
        fprintf(
            stderr,
            b"ERROR in mpu_set_bypass, failed to write INT_PIN_CFG register\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if bypass_on != 0 {
        bypass_en = 1 as libc::c_int;
    } else {
        bypass_en = 0 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __dmp_enable_gyro_cal(mut enable: libc::c_uchar) -> libc::c_int {
    if enable != 0 {
        let mut regs: [libc::c_uchar; 9] = [
            0xb8 as libc::c_int as libc::c_uchar,
            0xaa as libc::c_int as libc::c_uchar,
            0xb3 as libc::c_int as libc::c_uchar,
            0x8d as libc::c_int as libc::c_uchar,
            0xb4 as libc::c_int as libc::c_uchar,
            0x98 as libc::c_int as libc::c_uchar,
            0xd as libc::c_int as libc::c_uchar,
            0x35 as libc::c_int as libc::c_uchar,
            0x5d as libc::c_int as libc::c_uchar,
        ];
        return __mpu_write_mem(
            1208 as libc::c_int as libc::c_ushort,
            9 as libc::c_int as libc::c_ushort,
            regs.as_mut_ptr(),
        );
    } else {
        let mut regs_0: [libc::c_uchar; 9] = [
            0xb8 as libc::c_int as libc::c_uchar,
            0xaa as libc::c_int as libc::c_uchar,
            0xaa as libc::c_int as libc::c_uchar,
            0xaa as libc::c_int as libc::c_uchar,
            0xb0 as libc::c_int as libc::c_uchar,
            0x88 as libc::c_int as libc::c_uchar,
            0xc3 as libc::c_int as libc::c_uchar,
            0xc5 as libc::c_int as libc::c_uchar,
            0xc7 as libc::c_int as libc::c_uchar,
        ];
        return __mpu_write_mem(
            1208 as libc::c_int as libc::c_ushort,
            9 as libc::c_int as libc::c_ushort,
            regs_0.as_mut_ptr(),
        );
    };
}
unsafe extern "C" fn __dmp_enable_6x_lp_quat(mut enable: libc::c_uchar) -> libc::c_int {
    let mut regs: [libc::c_uchar; 4] = [0; 4];
    if enable != 0 {
        regs[0 as libc::c_int as usize] = 0x20 as libc::c_int as libc::c_uchar;
        regs[1 as libc::c_int as usize] = 0x28 as libc::c_int as libc::c_uchar;
        regs[2 as libc::c_int as usize] = 0x30 as libc::c_int as libc::c_uchar;
        regs[3 as libc::c_int as usize] = 0x38 as libc::c_int as libc::c_uchar;
    } else {
        memset(
            regs.as_mut_ptr() as *mut libc::c_void,
            0xa3 as libc::c_int,
            4 as libc::c_int as libc::c_ulong,
        );
    }
    __mpu_write_mem(
        2718 as libc::c_int as libc::c_ushort,
        4 as libc::c_int as libc::c_ushort,
        regs.as_mut_ptr(),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn __dmp_enable_lp_quat(mut enable: libc::c_uchar) -> libc::c_int {
    let mut regs: [libc::c_uchar; 4] = [0; 4];
    if enable != 0 {
        regs[0 as libc::c_int as usize] = 0xc0 as libc::c_int as libc::c_uchar;
        regs[1 as libc::c_int as usize] = 0xc2 as libc::c_int as libc::c_uchar;
        regs[2 as libc::c_int as usize] = 0xc4 as libc::c_int as libc::c_uchar;
        regs[3 as libc::c_int as usize] = 0xc6 as libc::c_int as libc::c_uchar;
    } else {
        memset(
            regs.as_mut_ptr() as *mut libc::c_void,
            0x8b as libc::c_int,
            4 as libc::c_int as libc::c_ulong,
        );
    }
    __mpu_write_mem(
        2712 as libc::c_int as libc::c_ushort,
        4 as libc::c_int as libc::c_ushort,
        regs.as_mut_ptr(),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn __mpu_reset_fifo() -> libc::c_int {
    let mut data: uint8_t = 0;
    rc_i2c_set_device_address(config.i2c_bus, config.i2c_addr);
    data = 0 as libc::c_int as uint8_t;
    if rc_i2c_write_byte(config.i2c_bus, 0x38 as libc::c_int as uint8_t, data) != 0 {
        return -(1 as libc::c_int);
    }
    if rc_i2c_write_byte(config.i2c_bus, 0x23 as libc::c_int as uint8_t, data) != 0 {
        return -(1 as libc::c_int);
    }
    if rc_i2c_write_byte(config.i2c_bus, 0x6a as libc::c_int as uint8_t, data) != 0 {
        return -(1 as libc::c_int);
    }
    data = (0x4 as libc::c_int | 0x8 as libc::c_int) as uint8_t;
    if rc_i2c_write_byte(config.i2c_bus, 0x6a as libc::c_int as uint8_t, data) != 0 {
        return -(1 as libc::c_int);
    }
    rc_usleep(50000 as libc::c_int as libc::c_uint);
    data = (0x80 as libc::c_int | 0x40 as libc::c_int) as uint8_t;
    if rc_i2c_write_byte(config.i2c_bus, 0x6a as libc::c_int as uint8_t, data) != 0 {
        return -(1 as libc::c_int);
    }
    data = 0x2 as libc::c_int as uint8_t;
    if rc_i2c_write_byte(config.i2c_bus, 0x38 as libc::c_int as uint8_t, data) != 0 {
        return -(1 as libc::c_int);
    }
    data = 0 as libc::c_int as uint8_t;
    if rc_i2c_write_byte(config.i2c_bus, 0x23 as libc::c_int as uint8_t, data) != 0 {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __dmp_set_interrupt_mode(mut mode: libc::c_uchar) -> libc::c_int {
    let regs_continuous: [libc::c_uchar; 11] = [
        0xd8 as libc::c_int as libc::c_uchar,
        0xb1 as libc::c_int as libc::c_uchar,
        0xb9 as libc::c_int as libc::c_uchar,
        0xf3 as libc::c_int as libc::c_uchar,
        0x8b as libc::c_int as libc::c_uchar,
        0xa3 as libc::c_int as libc::c_uchar,
        0x91 as libc::c_int as libc::c_uchar,
        0xb6 as libc::c_int as libc::c_uchar,
        0x9 as libc::c_int as libc::c_uchar,
        0xb4 as libc::c_int as libc::c_uchar,
        0xd9 as libc::c_int as libc::c_uchar,
    ];
    let regs_gesture: [libc::c_uchar; 11] = [
        0xda as libc::c_int as libc::c_uchar,
        0xb1 as libc::c_int as libc::c_uchar,
        0xb9 as libc::c_int as libc::c_uchar,
        0xf3 as libc::c_int as libc::c_uchar,
        0x8b as libc::c_int as libc::c_uchar,
        0xa3 as libc::c_int as libc::c_uchar,
        0x91 as libc::c_int as libc::c_uchar,
        0xb6 as libc::c_int as libc::c_uchar,
        0xda as libc::c_int as libc::c_uchar,
        0xb4 as libc::c_int as libc::c_uchar,
        0xda as libc::c_int as libc::c_uchar,
    ];
    match mode as libc::c_int {
        2 => {
            return __mpu_write_mem(
                2690 as libc::c_int as libc::c_ushort,
                11 as libc::c_int as libc::c_ushort,
                regs_continuous.as_ptr() as *mut libc::c_uchar,
            );
        }
        1 => {
            return __mpu_write_mem(
                2690 as libc::c_int as libc::c_ushort,
                11 as libc::c_int as libc::c_ushort,
                regs_gesture.as_ptr() as *mut libc::c_uchar,
            );
        }
        _ => return -(1 as libc::c_int),
    };
}
unsafe extern "C" fn __dmp_set_tap_thresh(mut axis: libc::c_uchar, mut thresh: libc::c_ushort) -> libc::c_int {
    let mut tmp: [libc::c_uchar; 4] = [0; 4];
    let mut scaled_thresh: libc::c_double = 0.;
    let mut dmp_thresh: libc::c_ushort = 0;
    let mut dmp_thresh_2: libc::c_ushort = 0;
    if axis as libc::c_int & 0x7 as libc::c_int == 0 || thresh as libc::c_int > 1600 as libc::c_int {
        return -(1 as libc::c_int);
    }
    scaled_thresh = thresh as libc::c_double / 200 as libc::c_int as libc::c_double;
    match config.accel_fsr as libc::c_uint {
        0 => {
            dmp_thresh = (scaled_thresh * 16384 as libc::c_int as libc::c_double) as libc::c_ushort;
            dmp_thresh_2 = (scaled_thresh * 12288 as libc::c_int as libc::c_double) as libc::c_ushort;
        }
        1 => {
            dmp_thresh = (scaled_thresh * 8192 as libc::c_int as libc::c_double) as libc::c_ushort;
            dmp_thresh_2 = (scaled_thresh * 6144 as libc::c_int as libc::c_double) as libc::c_ushort;
        }
        2 => {
            dmp_thresh = (scaled_thresh * 4096 as libc::c_int as libc::c_double) as libc::c_ushort;
            dmp_thresh_2 = (scaled_thresh * 3072 as libc::c_int as libc::c_double) as libc::c_ushort;
        }
        3 => {
            dmp_thresh = (scaled_thresh * 2048 as libc::c_int as libc::c_double) as libc::c_ushort;
            dmp_thresh_2 = (scaled_thresh * 1536 as libc::c_int as libc::c_double) as libc::c_ushort;
        }
        _ => return -(1 as libc::c_int),
    }
    tmp[0 as libc::c_int as usize] = (dmp_thresh as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
    tmp[1 as libc::c_int as usize] = (dmp_thresh as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    tmp[2 as libc::c_int as usize] = (dmp_thresh_2 as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
    tmp[3 as libc::c_int as usize] = (dmp_thresh_2 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    if axis as libc::c_int & 0x1 as libc::c_int != 0 {
        if __mpu_write_mem(
            468 as libc::c_int as libc::c_ushort,
            2 as libc::c_int as libc::c_ushort,
            tmp.as_mut_ptr(),
        ) != 0
        {
            return -(1 as libc::c_int);
        }
        if __mpu_write_mem(
            (256 as libc::c_int + 36 as libc::c_int) as libc::c_ushort,
            2 as libc::c_int as libc::c_ushort,
            tmp.as_mut_ptr().offset(2 as libc::c_int as isize),
        ) != 0
        {
            return -(1 as libc::c_int);
        }
    }
    if axis as libc::c_int & 0x2 as libc::c_int != 0 {
        if __mpu_write_mem(
            472 as libc::c_int as libc::c_ushort,
            2 as libc::c_int as libc::c_ushort,
            tmp.as_mut_ptr(),
        ) != 0
        {
            return -(1 as libc::c_int);
        }
        if __mpu_write_mem(
            (256 as libc::c_int + 40 as libc::c_int) as libc::c_ushort,
            2 as libc::c_int as libc::c_ushort,
            tmp.as_mut_ptr().offset(2 as libc::c_int as isize),
        ) != 0
        {
            return -(1 as libc::c_int);
        }
    }
    if axis as libc::c_int & 0x4 as libc::c_int != 0 {
        if __mpu_write_mem(
            476 as libc::c_int as libc::c_ushort,
            2 as libc::c_int as libc::c_ushort,
            tmp.as_mut_ptr(),
        ) != 0
        {
            return -(1 as libc::c_int);
        }
        if __mpu_write_mem(
            (256 as libc::c_int + 44 as libc::c_int) as libc::c_ushort,
            2 as libc::c_int as libc::c_ushort,
            tmp.as_mut_ptr().offset(2 as libc::c_int as isize),
        ) != 0
        {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __dmp_set_tap_axes(mut axis: libc::c_uchar) -> libc::c_int {
    let mut tmp: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    if axis as libc::c_int & 0x1 as libc::c_int != 0 {
        tmp = (tmp as libc::c_int | 0x30 as libc::c_int) as libc::c_uchar;
    }
    if axis as libc::c_int & 0x2 as libc::c_int != 0 {
        tmp = (tmp as libc::c_int | 0xc as libc::c_int) as libc::c_uchar;
    }
    if axis as libc::c_int & 0x4 as libc::c_int != 0 {
        tmp = (tmp as libc::c_int | 0x3 as libc::c_int) as libc::c_uchar;
    }
    return __mpu_write_mem(
        (256 as libc::c_int + 72 as libc::c_int) as libc::c_ushort,
        1 as libc::c_int as libc::c_ushort,
        &mut tmp,
    );
}
unsafe extern "C" fn __dmp_set_tap_count(mut min_taps: libc::c_uchar) -> libc::c_int {
    let mut tmp: libc::c_uchar = 0;
    if (min_taps as libc::c_int) < 1 as libc::c_int {
        min_taps = 1 as libc::c_int as libc::c_uchar;
    } else if min_taps as libc::c_int > 4 as libc::c_int {
        min_taps = 4 as libc::c_int as libc::c_uchar;
    }
    tmp = (min_taps as libc::c_int - 1 as libc::c_int) as libc::c_uchar;
    return __mpu_write_mem(
        (256 as libc::c_int + 79 as libc::c_int) as libc::c_ushort,
        1 as libc::c_int as libc::c_ushort,
        &mut tmp,
    );
}
unsafe extern "C" fn __dmp_set_tap_time(mut time: libc::c_ushort) -> libc::c_int {
    let mut dmp_time: libc::c_ushort = 0;
    let mut tmp: [libc::c_uchar; 2] = [0; 2];
    dmp_time = (time as libc::c_int / (1000 as libc::c_int / 200 as libc::c_int)) as libc::c_ushort;
    tmp[0 as libc::c_int as usize] = (dmp_time as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
    tmp[1 as libc::c_int as usize] = (dmp_time as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    return __mpu_write_mem(
        478 as libc::c_int as libc::c_ushort,
        2 as libc::c_int as libc::c_ushort,
        tmp.as_mut_ptr(),
    );
}
unsafe extern "C" fn __dmp_set_tap_time_multi(mut time: libc::c_ushort) -> libc::c_int {
    let mut dmp_time: libc::c_ushort = 0;
    let mut tmp: [libc::c_uchar; 2] = [0; 2];
    dmp_time = (time as libc::c_int / (1000 as libc::c_int / 200 as libc::c_int)) as libc::c_ushort;
    tmp[0 as libc::c_int as usize] = (dmp_time as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
    tmp[1 as libc::c_int as usize] = (dmp_time as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    return __mpu_write_mem(
        (256 as libc::c_int + 218 as libc::c_int) as libc::c_ushort,
        2 as libc::c_int as libc::c_ushort,
        tmp.as_mut_ptr(),
    );
}
unsafe extern "C" fn __dmp_set_shake_reject_thresh(mut sf: libc::c_long, mut thresh: libc::c_ushort) -> libc::c_int {
    let mut tmp: [libc::c_uchar; 4] = [0; 4];
    let mut thresh_scaled: libc::c_long = sf / 1000 as libc::c_int as libc::c_long * thresh as libc::c_long;
    tmp[0 as libc::c_int as usize] =
        (thresh_scaled >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_long) as libc::c_uchar;
    tmp[1 as libc::c_int as usize] =
        (thresh_scaled >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_long) as libc::c_uchar;
    tmp[2 as libc::c_int as usize] =
        (thresh_scaled >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_long) as libc::c_uchar;
    tmp[3 as libc::c_int as usize] = (thresh_scaled & 0xff as libc::c_int as libc::c_long) as libc::c_uchar;
    return __mpu_write_mem(
        (256 as libc::c_int + 92 as libc::c_int) as libc::c_ushort,
        4 as libc::c_int as libc::c_ushort,
        tmp.as_mut_ptr(),
    );
}
unsafe extern "C" fn __dmp_set_shake_reject_time(mut time: libc::c_ushort) -> libc::c_int {
    let mut tmp: [libc::c_uchar; 2] = [0; 2];
    time = (time as libc::c_int / (1000 as libc::c_int / 200 as libc::c_int)) as libc::c_ushort;
    tmp[0 as libc::c_int as usize] = (time as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
    tmp[1 as libc::c_int as usize] = (time as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    return __mpu_write_mem(
        (256 as libc::c_int + 90 as libc::c_int) as libc::c_ushort,
        2 as libc::c_int as libc::c_ushort,
        tmp.as_mut_ptr(),
    );
}
unsafe extern "C" fn __dmp_set_shake_reject_timeout(mut time: libc::c_ushort) -> libc::c_int {
    let mut tmp: [libc::c_uchar; 2] = [0; 2];
    time = (time as libc::c_int / (1000 as libc::c_int / 200 as libc::c_int)) as libc::c_ushort;
    tmp[0 as libc::c_int as usize] = (time as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
    tmp[1 as libc::c_int as usize] = (time as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    return __mpu_write_mem(
        (256 as libc::c_int + 88 as libc::c_int) as libc::c_ushort,
        2 as libc::c_int as libc::c_ushort,
        tmp.as_mut_ptr(),
    );
}
unsafe extern "C" fn __dmp_enable_feature(mut mask: libc::c_ushort) -> libc::c_int {
    let mut tmp: [libc::c_uchar; 10] = [0; 10];
    tmp[0 as libc::c_int as usize] =
        (46850825 as libc::c_longlong >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_longlong) as libc::c_uchar;
    tmp[1 as libc::c_int as usize] =
        (46850825 as libc::c_longlong >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_longlong) as libc::c_uchar;
    tmp[2 as libc::c_int as usize] =
        (46850825 as libc::c_longlong >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_longlong) as libc::c_uchar;
    tmp[3 as libc::c_int as usize] =
        (46850825 as libc::c_longlong & 0xff as libc::c_int as libc::c_longlong) as libc::c_uchar;
    if __mpu_write_mem(
        104 as libc::c_int as libc::c_ushort,
        4 as libc::c_int as libc::c_ushort,
        tmp.as_mut_ptr(),
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"ERROR: in dmp_enable_feature, failed to write mpu mem\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    tmp[0 as libc::c_int as usize] = 0xa3 as libc::c_int as libc::c_uchar;
    if mask as libc::c_int & 0x40 as libc::c_int != 0 {
        tmp[1 as libc::c_int as usize] = 0xc0 as libc::c_int as libc::c_uchar;
        tmp[2 as libc::c_int as usize] = 0xc8 as libc::c_int as libc::c_uchar;
        tmp[3 as libc::c_int as usize] = 0xc2 as libc::c_int as libc::c_uchar;
    } else {
        tmp[1 as libc::c_int as usize] = 0xa3 as libc::c_int as libc::c_uchar;
        tmp[2 as libc::c_int as usize] = 0xa3 as libc::c_int as libc::c_uchar;
        tmp[3 as libc::c_int as usize] = 0xa3 as libc::c_int as libc::c_uchar;
    }
    if mask as libc::c_int & (0x80 as libc::c_int | 0x100 as libc::c_int) != 0 {
        tmp[4 as libc::c_int as usize] = 0xc4 as libc::c_int as libc::c_uchar;
        tmp[5 as libc::c_int as usize] = 0xcc as libc::c_int as libc::c_uchar;
        tmp[6 as libc::c_int as usize] = 0xc6 as libc::c_int as libc::c_uchar;
    } else {
        tmp[4 as libc::c_int as usize] = 0xa3 as libc::c_int as libc::c_uchar;
        tmp[5 as libc::c_int as usize] = 0xa3 as libc::c_int as libc::c_uchar;
        tmp[6 as libc::c_int as usize] = 0xa3 as libc::c_int as libc::c_uchar;
    }
    tmp[7 as libc::c_int as usize] = 0xa3 as libc::c_int as libc::c_uchar;
    tmp[8 as libc::c_int as usize] = 0xa3 as libc::c_int as libc::c_uchar;
    tmp[9 as libc::c_int as usize] = 0xa3 as libc::c_int as libc::c_uchar;
    if __mpu_write_mem(
        2727 as libc::c_int as libc::c_ushort,
        10 as libc::c_int as libc::c_ushort,
        tmp.as_mut_ptr(),
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"ERROR: in dmp_enable_feature, failed to write mpu mem\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if mask as libc::c_int & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0 {
        tmp[0 as libc::c_int as usize] = 0x20 as libc::c_int as libc::c_uchar;
    } else {
        tmp[0 as libc::c_int as usize] = 0xd8 as libc::c_int as libc::c_uchar;
    }
    if __mpu_write_mem(
        2742 as libc::c_int as libc::c_ushort,
        1 as libc::c_int as libc::c_ushort,
        tmp.as_mut_ptr(),
    ) != 0
    {
        fprintf(
            stderr,
            b"ERROR: in dmp_enable_feature, failed to write mpu mem\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if mask as libc::c_int & 0x20 as libc::c_int != 0 {
        __dmp_enable_gyro_cal(1 as libc::c_int as libc::c_uchar);
    } else {
        __dmp_enable_gyro_cal(0 as libc::c_int as libc::c_uchar);
    }
    if mask as libc::c_int & (0x80 as libc::c_int | 0x100 as libc::c_int) != 0 {
        if mask as libc::c_int & 0x100 as libc::c_int != 0 {
            tmp[0 as libc::c_int as usize] = 0xb2 as libc::c_int as libc::c_uchar;
            tmp[1 as libc::c_int as usize] = 0x8b as libc::c_int as libc::c_uchar;
            tmp[2 as libc::c_int as usize] = 0xb6 as libc::c_int as libc::c_uchar;
            tmp[3 as libc::c_int as usize] = 0x9b as libc::c_int as libc::c_uchar;
        } else {
            tmp[0 as libc::c_int as usize] = 0xb0 as libc::c_int as libc::c_uchar;
            tmp[1 as libc::c_int as usize] = 0x80 as libc::c_int as libc::c_uchar;
            tmp[2 as libc::c_int as usize] = 0xb4 as libc::c_int as libc::c_uchar;
            tmp[3 as libc::c_int as usize] = 0x90 as libc::c_int as libc::c_uchar;
        }
        __mpu_write_mem(
            2722 as libc::c_int as libc::c_ushort,
            4 as libc::c_int as libc::c_ushort,
            tmp.as_mut_ptr(),
        );
    }
    if mask as libc::c_int & 0x1 as libc::c_int != 0 {
        tmp[0 as libc::c_int as usize] = 0xf8 as libc::c_int as libc::c_uchar;
        __mpu_write_mem(
            2224 as libc::c_int as libc::c_ushort,
            1 as libc::c_int as libc::c_ushort,
            tmp.as_mut_ptr(),
        );
        __dmp_set_tap_thresh(
            0x7 as libc::c_int as libc::c_uchar,
            config.tap_threshold as libc::c_ushort,
        );
        __dmp_set_tap_axes(0x7 as libc::c_int as libc::c_uchar);
        __dmp_set_tap_count(1 as libc::c_int as libc::c_uchar);
        __dmp_set_tap_time(100 as libc::c_int as libc::c_ushort);
        __dmp_set_tap_time_multi(600 as libc::c_int as libc::c_ushort);
        __dmp_set_shake_reject_thresh(
            46850825 as libc::c_longlong as libc::c_long,
            300 as libc::c_int as libc::c_ushort,
        );
        __dmp_set_shake_reject_time(80 as libc::c_int as libc::c_ushort);
        __dmp_set_shake_reject_timeout(100 as libc::c_int as libc::c_ushort);
    } else {
        tmp[0 as libc::c_int as usize] = 0xd8 as libc::c_int as libc::c_uchar;
        __mpu_write_mem(
            2224 as libc::c_int as libc::c_ushort,
            1 as libc::c_int as libc::c_ushort,
            tmp.as_mut_ptr(),
        );
    }
    if mask as libc::c_int & 0x2 as libc::c_int != 0 {
        tmp[0 as libc::c_int as usize] = 0xd9 as libc::c_int as libc::c_uchar;
    } else {
        tmp[0 as libc::c_int as usize] = 0xd8 as libc::c_int as libc::c_uchar;
    }
    __mpu_write_mem(
        1853 as libc::c_int as libc::c_ushort,
        1 as libc::c_int as libc::c_ushort,
        tmp.as_mut_ptr(),
    );
    if mask as libc::c_int & 0x4 as libc::c_int != 0 {
        __dmp_enable_lp_quat(1 as libc::c_int as libc::c_uchar);
    } else {
        __dmp_enable_lp_quat(0 as libc::c_int as libc::c_uchar);
    }
    if mask as libc::c_int & 0x10 as libc::c_int != 0 {
        __dmp_enable_6x_lp_quat(1 as libc::c_int as libc::c_uchar);
    } else {
        __dmp_enable_6x_lp_quat(0 as libc::c_int as libc::c_uchar);
    }
    __mpu_reset_fifo();
    packet_len = 0 as libc::c_int;
    if mask as libc::c_int & 0x40 as libc::c_int != 0 {
        packet_len += 6 as libc::c_int;
    }
    if mask as libc::c_int & (0x80 as libc::c_int | 0x100 as libc::c_int) != 0 {
        packet_len += 6 as libc::c_int;
    }
    if mask as libc::c_int & (0x4 as libc::c_int | 0x10 as libc::c_int) != 0 {
        packet_len += 16 as libc::c_int;
    }
    if mask as libc::c_int & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0 {
        packet_len += 4 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __set_int_enable(mut enable: libc::c_uchar) -> libc::c_int {
    let mut tmp: libc::c_uchar = 0;
    if enable != 0 {
        tmp = 0x2 as libc::c_int as libc::c_uchar;
    } else {
        tmp = 0 as libc::c_int as libc::c_uchar;
    }
    if rc_i2c_write_byte(config.i2c_bus, 0x38 as libc::c_int as uint8_t, tmp) != 0 {
        fprintf(
            stderr,
            b"ERROR: in set_int_enable, failed to write INT_ENABLE register\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_i2c_write_byte(
        config.i2c_bus,
        0x23 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    ) != 0
    {
        fprintf(
            stderr,
            b"ERROR: in set_int_enable, failed to write FIFO_EN register\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __mpu_set_sample_rate(mut rate: libc::c_int) -> libc::c_int {
    if rate > 1000 as libc::c_int || rate < 4 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: sample rate must be between 4 & 1000\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut div: uint8_t = (1000 as libc::c_int / rate - 1 as libc::c_int) as uint8_t;
    if rc_i2c_write_byte(config.i2c_bus, 0x19 as libc::c_int as uint8_t, div) != 0 {
        fprintf(
            stderr,
            b"ERROR: in mpu_set_sample_rate, failed to write SMPLRT_DIV register\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __mpu_set_dmp_state(mut enable: libc::c_uchar) -> libc::c_int {
    if enable != 0 {
        __set_int_enable(0 as libc::c_int as libc::c_uchar);
        __mpu_set_bypass(1 as libc::c_int as libc::c_uchar);
        rc_i2c_write_byte(
            config.i2c_bus,
            0x23 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
        );
        __set_int_enable(1 as libc::c_int as libc::c_uchar);
        __mpu_reset_fifo();
    } else {
        __set_int_enable(0 as libc::c_int as libc::c_uchar);
        rc_i2c_write_byte(
            config.i2c_bus,
            0x23 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
        );
        __mpu_reset_fifo();
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __dmp_interrupt_handler(mut _ptr: *mut libc::c_void) -> *mut libc::c_void {
    let mut ret: libc::c_int = 0;
    let mut mag_div_step: libc::c_int = config.mag_sample_rate_div;
    let mut first_run: libc::c_int = 1 as libc::c_int;
    __mpu_reset_fifo();
    while imu_shutdown_flag == 0 {
        ret = rc_gpio_poll(
            config.gpio_interrupt_pin_chip,
            config.gpio_interrupt_pin,
            300 as libc::c_int,
            &mut last_interrupt_timestamp_nanos,
        );
        if imu_shutdown_flag != 0 {
            break;
        }
        if ret == -(1 as libc::c_int) {
            fprintf(
                stderr,
                b"ERROR in IMU interrupt handler calling poll\n\0" as *const u8 as *const libc::c_char,
            );
        } else if ret == 0 as libc::c_int {
            if config.show_warnings != 0 {
                fprintf(
                    stderr,
                    b"WARNING, gpio poll timeout\n\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            if rc_i2c_get_lock(config.i2c_bus) != 0 {
                fprintf(
                    stderr,
                    b"WARNING: Something has claimed the I2C bus when an\n\0" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    stderr,
                    b"IMU interrupt was received. Reading IMU anyway.\n\0" as *const u8 as *const libc::c_char,
                );
            }
            rc_i2c_lock_bus(config.i2c_bus);
            pthread_mutex_lock(&mut read_mutex);
            pthread_mutex_lock(&mut tap_mutex);
            ret = __read_dmp_fifo(data_ptr);
            rc_i2c_unlock_bus(config.i2c_bus);
            if ret == 0 as libc::c_int {
                last_read_successful = 1 as libc::c_int;
                if (*data_ptr).tap_detected != 0 {
                    last_tap_timestamp_nanos = last_interrupt_timestamp_nanos;
                }
            } else {
                last_read_successful = 0 as libc::c_int;
            }
            if config.enable_magnetometer != 0 && config.read_mag_after_callback == 0 {
                if mag_div_step >= config.mag_sample_rate_div {
                    rc_mpu_read_mag(data_ptr);
                    rc_i2c_set_device_address(config.i2c_bus, config.i2c_addr);
                    mag_div_step = 1 as libc::c_int;
                } else {
                    mag_div_step += 1;
                }
            }
            rc_i2c_unlock_bus(config.i2c_bus);
            if first_run == 1 as libc::c_int {
                first_run = 0 as libc::c_int;
            } else if last_read_successful != 0 {
                if dmp_callback_func.is_some() {
                    ::core::mem::transmute::<_, fn()>(dmp_callback_func.expect("non-null function pointer"))();
                }
                pthread_cond_broadcast(&mut read_condition);
                if (*data_ptr).tap_detected != 0 {
                    if tap_callback_func.is_some() {
                        tap_callback_func.expect("non-null function pointer")(
                            (*data_ptr).last_tap_direction,
                            (*data_ptr).last_tap_count,
                        );
                    }
                    pthread_cond_broadcast(&mut tap_condition);
                }
            }
            pthread_mutex_unlock(&mut read_mutex);
            pthread_mutex_unlock(&mut tap_mutex);
            if config.enable_magnetometer != 0 && config.read_mag_after_callback != 0 {
                if mag_div_step >= config.mag_sample_rate_div {
                    rc_i2c_lock_bus(config.i2c_bus);
                    rc_mpu_read_mag(data_ptr);
                    rc_i2c_unlock_bus(config.i2c_bus);
                    rc_i2c_set_device_address(config.i2c_bus, config.i2c_addr);
                    mag_div_step = 1 as libc::c_int;
                } else {
                    mag_div_step += 1;
                }
            }
        }
    }
    pthread_mutex_lock(&mut read_mutex);
    pthread_cond_broadcast(&mut read_condition);
    pthread_mutex_unlock(&mut read_mutex);
    thread_running_flag = 0 as libc::c_int;
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn rc_mpu_set_dmp_callback(mut func: Option<unsafe extern "C" fn() -> ()>) -> libc::c_int {
    if func.is_none() {
        fprintf(
            stderr,
            b"ERROR: trying to assign NULL pointer to dmp_callback_func\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    dmp_callback_func =
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, Option<unsafe extern "C" fn() -> ()>>(func);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_mpu_set_tap_callback(
    mut func: Option<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()>,
) -> libc::c_int {
    if func.is_none() {
        fprintf(
            stderr,
            b"ERROR: trying to assign NULL pointer to tap_callback_func\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    tap_callback_func = func;
    return 0 as libc::c_int;
}
unsafe extern "C" fn __read_dmp_fifo(mut data: *mut rc_mpu_data_t) -> libc::c_int {
    let mut raw: [libc::c_uchar; 160] = [0; 160];
    let mut quat_q14: [int32_t; 4] = [0; 4];
    let mut quat: [int32_t; 4] = [0; 4];
    let mut quat_mag_sq: int32_t = 0;
    let mut fifo_count: uint16_t = 0;
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    static mut first_run: libc::c_int = 1 as libc::c_int;
    let mut q_tmp: [libc::c_double; 4] = [0.; 4];
    let mut sum: libc::c_double = 0.;
    let mut qlen: libc::c_double = 0.;
    if dmp_en == 0 {
        printf(b"only use mpu_read_fifo in dmp mode\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if packet_len != 32 as libc::c_int && packet_len != 20 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: packet_len is set incorrectly for read_dmp_fifo\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_i2c_set_device_address(config.i2c_bus, config.i2c_addr);
    let mut is_new_dmp_data: libc::c_int = 0 as libc::c_int;
    if rc_i2c_read_word(config.i2c_bus, 0x72 as libc::c_int as uint8_t, &mut fifo_count) < 0 as libc::c_int {
        if config.show_warnings != 0 {
            printf(
                b"fifo_count i2c error: %s\n\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return -(1 as libc::c_int);
    }
    if fifo_count as libc::c_int == 0 as libc::c_int {
        if config.show_warnings != 0 && first_run != 1 as libc::c_int {
            printf(b"WARNING: empty fifo\n\0" as *const u8 as *const libc::c_char);
        }
        return -(1 as libc::c_int);
    } else {
        if fifo_count as libc::c_int == packet_len {
            i = 0 as libc::c_int;
        } else if fifo_count as libc::c_int == 2 as libc::c_int * packet_len {
            if config.show_warnings != 0 && first_run != 1 as libc::c_int {
                printf(b"warning: imu fifo contains two packets\n\0" as *const u8 as *const libc::c_char);
            }
            i = packet_len;
        } else if fifo_count as libc::c_int == 3 as libc::c_int * packet_len {
            if config.show_warnings != 0 && first_run != 1 as libc::c_int {
                printf(b"warning: imu fifo contains three packets\n\0" as *const u8 as *const libc::c_char);
            }
            i = 2 as libc::c_int * packet_len;
        } else if fifo_count as libc::c_int == 4 as libc::c_int * packet_len {
            if config.show_warnings != 0 && first_run != 1 as libc::c_int {
                printf(b"warning: imu fifo contains four packets\n\0" as *const u8 as *const libc::c_char);
            }
            i = 2 as libc::c_int * packet_len;
        } else if fifo_count as libc::c_int == 5 as libc::c_int * packet_len {
            if config.show_warnings != 0 && first_run != 1 as libc::c_int {
                printf(b"warning: imu fifo contains five packets\n\0" as *const u8 as *const libc::c_char);
            }
            i = 2 as libc::c_int * packet_len;
        } else {
            if config.show_warnings != 0 && first_run != 1 as libc::c_int {
                printf(
                    b"warning: %d bytes in FIFO, expected %d\n\0" as *const u8 as *const libc::c_char,
                    fifo_count as libc::c_int,
                    packet_len,
                );
            }
            __mpu_reset_fifo();
            return -(1 as libc::c_int);
        }
    }
    memset(
        raw.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (32 as libc::c_int * 5 as libc::c_int) as libc::c_ulong,
    );
    ret = rc_i2c_read_bytes(
        config.i2c_bus,
        0x74 as libc::c_int as uint8_t,
        fifo_count as size_t,
        &mut *raw.as_mut_ptr().offset(0 as libc::c_int as isize),
    );
    if ret < 0 as libc::c_int {
        ret = rc_i2c_read_bytes(
            config.i2c_bus,
            0x74 as libc::c_int as uint8_t,
            fifo_count as size_t,
            &mut *raw.as_mut_ptr().offset(0 as libc::c_int as isize),
        );
    }
    if ret != fifo_count as libc::c_int {
        if config.show_warnings != 0 {
            fprintf(
                stderr,
                b"ERROR: failed to read fifo buffer register\n\0" as *const u8 as *const libc::c_char,
            );
            printf(
                b"read %d bytes, expected %d\n\0" as *const u8 as *const libc::c_char,
                ret,
                packet_len,
            );
        }
        return -(1 as libc::c_int);
    }
    quat[0 as libc::c_int as usize] = (raw[(i + 0 as libc::c_int) as usize] as int32_t) << 24 as libc::c_int
        | (raw[(i + 1 as libc::c_int) as usize] as int32_t) << 16 as libc::c_int
        | (raw[(i + 2 as libc::c_int) as usize] as int32_t) << 8 as libc::c_int
        | raw[(i + 3 as libc::c_int) as usize] as libc::c_int;
    quat[1 as libc::c_int as usize] = (raw[(i + 4 as libc::c_int) as usize] as int32_t) << 24 as libc::c_int
        | (raw[(i + 5 as libc::c_int) as usize] as int32_t) << 16 as libc::c_int
        | (raw[(i + 6 as libc::c_int) as usize] as int32_t) << 8 as libc::c_int
        | raw[(i + 7 as libc::c_int) as usize] as libc::c_int;
    quat[2 as libc::c_int as usize] = (raw[(i + 8 as libc::c_int) as usize] as int32_t) << 24 as libc::c_int
        | (raw[(i + 9 as libc::c_int) as usize] as int32_t) << 16 as libc::c_int
        | (raw[(i + 10 as libc::c_int) as usize] as int32_t) << 8 as libc::c_int
        | raw[(i + 11 as libc::c_int) as usize] as libc::c_int;
    quat[3 as libc::c_int as usize] = (raw[(i + 12 as libc::c_int) as usize] as int32_t) << 24 as libc::c_int
        | (raw[(i + 13 as libc::c_int) as usize] as int32_t) << 16 as libc::c_int
        | (raw[(i + 14 as libc::c_int) as usize] as int32_t) << 8 as libc::c_int
        | raw[(i + 15 as libc::c_int) as usize] as libc::c_int;
    i += 16 as libc::c_int;
    quat_q14[0 as libc::c_int as usize] = quat[0 as libc::c_int as usize] >> 16 as libc::c_int;
    quat_q14[1 as libc::c_int as usize] = quat[1 as libc::c_int as usize] >> 16 as libc::c_int;
    quat_q14[2 as libc::c_int as usize] = quat[2 as libc::c_int as usize] >> 16 as libc::c_int;
    quat_q14[3 as libc::c_int as usize] = quat[3 as libc::c_int as usize] >> 16 as libc::c_int;
    quat_mag_sq = quat_q14[0 as libc::c_int as usize] * quat_q14[0 as libc::c_int as usize]
        + quat_q14[1 as libc::c_int as usize] * quat_q14[1 as libc::c_int as usize]
        + quat_q14[2 as libc::c_int as usize] * quat_q14[2 as libc::c_int as usize]
        + quat_q14[3 as libc::c_int as usize] * quat_q14[3 as libc::c_int as usize];
    if (quat_mag_sq as libc::c_long)
        < ((1 as libc::c_long) << 28 as libc::c_int) - ((1 as libc::c_long) << 16 as libc::c_int)
        || quat_mag_sq as libc::c_long
            > ((1 as libc::c_long) << 28 as libc::c_int) + ((1 as libc::c_long) << 16 as libc::c_int)
    {
        if config.show_warnings != 0 {
            printf(
                b"warning: Quaternion out of bounds, fifo_count: %d\n\0" as *const u8 as *const libc::c_char,
                fifo_count as libc::c_int,
            );
        }
        __mpu_reset_fifo();
        return -(1 as libc::c_int);
    }
    j = 0 as libc::c_int;
    while j < 4 as libc::c_int {
        q_tmp[j as usize] = quat[j as usize] as libc::c_double;
        j += 1;
    }
    sum = 0.0f64;
    j = 0 as libc::c_int;
    while j < 4 as libc::c_int {
        sum += q_tmp[j as usize] * q_tmp[j as usize];
        j += 1;
    }
    qlen = sqrt(sum);
    j = 0 as libc::c_int;
    while j < 4 as libc::c_int {
        q_tmp[j as usize] /= qlen;
        j += 1;
    }
    j = 0 as libc::c_int;
    while j < 4 as libc::c_int {
        (*data).dmp_quat[j as usize] = q_tmp[j as usize];
        j += 1;
    }
    rc_quaternion_to_tb_array(((*data).dmp_quat).as_mut_ptr(), ((*data).dmp_TaitBryan).as_mut_ptr());
    is_new_dmp_data = 1 as libc::c_int;
    if packet_len == 32 as libc::c_int {
        (*data).raw_accel[0 as libc::c_int as usize] =
            ((raw[(i + 0 as libc::c_int) as usize] as uint16_t as libc::c_int) << 8 as libc::c_int
                | raw[(i + 1 as libc::c_int) as usize] as libc::c_int) as int16_t;
        (*data).raw_accel[1 as libc::c_int as usize] =
            ((raw[(i + 2 as libc::c_int) as usize] as uint16_t as libc::c_int) << 8 as libc::c_int
                | raw[(i + 3 as libc::c_int) as usize] as libc::c_int) as int16_t;
        (*data).raw_accel[2 as libc::c_int as usize] =
            ((raw[(i + 4 as libc::c_int) as usize] as uint16_t as libc::c_int) << 8 as libc::c_int
                | raw[(i + 5 as libc::c_int) as usize] as libc::c_int) as int16_t;
        i += 6 as libc::c_int;
        (*data).accel[0 as libc::c_int as usize] =
            (*data).raw_accel[0 as libc::c_int as usize] as libc::c_int as libc::c_double * (*data).accel_to_ms2
                / accel_lengths[0 as libc::c_int as usize];
        (*data).accel[1 as libc::c_int as usize] =
            (*data).raw_accel[1 as libc::c_int as usize] as libc::c_int as libc::c_double * (*data).accel_to_ms2
                / accel_lengths[1 as libc::c_int as usize];
        (*data).accel[2 as libc::c_int as usize] =
            (*data).raw_accel[2 as libc::c_int as usize] as libc::c_int as libc::c_double * (*data).accel_to_ms2
                / accel_lengths[2 as libc::c_int as usize];
        (*data).raw_gyro[0 as libc::c_int as usize] =
            ((raw[(0 as libc::c_int + i) as usize] as int16_t as libc::c_int) << 8 as libc::c_int
                | raw[(1 as libc::c_int + i) as usize] as libc::c_int) as int16_t;
        (*data).raw_gyro[1 as libc::c_int as usize] =
            ((raw[(2 as libc::c_int + i) as usize] as int16_t as libc::c_int) << 8 as libc::c_int
                | raw[(3 as libc::c_int + i) as usize] as libc::c_int) as int16_t;
        (*data).raw_gyro[2 as libc::c_int as usize] =
            ((raw[(4 as libc::c_int + i) as usize] as int16_t as libc::c_int) << 8 as libc::c_int
                | raw[(5 as libc::c_int + i) as usize] as libc::c_int) as int16_t;
        i += 6 as libc::c_int;
        (*data).gyro[0 as libc::c_int as usize] =
            (*data).raw_gyro[0 as libc::c_int as usize] as libc::c_int as libc::c_double * (*data).gyro_to_degs;
        (*data).gyro[1 as libc::c_int as usize] =
            (*data).raw_gyro[1 as libc::c_int as usize] as libc::c_int as libc::c_double * (*data).gyro_to_degs;
        (*data).gyro[2 as libc::c_int as usize] =
            (*data).raw_gyro[2 as libc::c_int as usize] as libc::c_int as libc::c_double * (*data).gyro_to_degs;
    }
    let mut tap: libc::c_uchar = 0;
    tap = (0x3f as libc::c_int & raw[(i + 3 as libc::c_int) as usize] as libc::c_int) as libc::c_uchar;
    if raw[(i + 1 as libc::c_int) as usize] as libc::c_int & 0x1 as libc::c_int != 0 {
        let mut direction: libc::c_uchar = 0;
        let mut count: libc::c_uchar = 0;
        direction = (tap as libc::c_int >> 3 as libc::c_int) as libc::c_uchar;
        count = (tap as libc::c_int % 8 as libc::c_int + 1 as libc::c_int) as libc::c_uchar;
        (*data_ptr).last_tap_direction = direction as libc::c_int;
        (*data_ptr).last_tap_count = count as libc::c_int;
        (*data_ptr).tap_detected = 1 as libc::c_int;
    } else {
        (*data_ptr).tap_detected = 0 as libc::c_int;
    }
    if is_new_dmp_data != 0 && config.enable_magnetometer != 0 {
        __data_fusion(data);
    }
    if is_new_dmp_data != 0 {
        first_run = 0 as libc::c_int;
    }
    if is_new_dmp_data != 0 {
        return 0 as libc::c_int;
    } else {
        return -(1 as libc::c_int);
    };
}
unsafe extern "C" fn __data_fusion(mut data: *mut rc_mpu_data_t) -> libc::c_int {
    let mut tilt_tb: [libc::c_double; 3] = [0.; 3];
    let mut tilt_q: [libc::c_double; 4] = [0.; 4];
    let mut mag_vec: [libc::c_double; 3] = [0.; 3];
    static mut newMagYaw: libc::c_double = 0 as libc::c_int as libc::c_double;
    static mut newDMPYaw: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut lastDMPYaw: libc::c_double = 0.;
    let mut lastMagYaw: libc::c_double = 0.;
    let mut newYaw: libc::c_double = 0.;
    static mut dmp_spin_counter: libc::c_int = 0 as libc::c_int;
    static mut mag_spin_counter: libc::c_int = 0 as libc::c_int;
    static mut first_run: libc::c_int = 1 as libc::c_int;
    tilt_tb[0 as libc::c_int as usize] = (*data).dmp_TaitBryan[0 as libc::c_int as usize];
    tilt_tb[1 as libc::c_int as usize] = (*data).dmp_TaitBryan[1 as libc::c_int as usize];
    tilt_tb[2 as libc::c_int as usize] = 0.0f64;
    rc_quaternion_from_tb_array(tilt_tb.as_mut_ptr(), tilt_q.as_mut_ptr());
    if __mag_correct_orientation(mag_vec.as_mut_ptr()) != 0 {
        return -(1 as libc::c_int);
    }
    rc_quaternion_rotate_vector_array(mag_vec.as_mut_ptr(), tilt_q.as_mut_ptr());
    lastMagYaw = newMagYaw;
    newMagYaw = -atan2(mag_vec[1 as libc::c_int as usize], mag_vec[0 as libc::c_int as usize]);
    if newMagYaw.is_nan() as i32 != 0 {
        return -(1 as libc::c_int);
    }
    (*data).compass_heading_raw = newMagYaw;
    lastDMPYaw = newDMPYaw;
    newDMPYaw = (*data).dmp_TaitBryan[2 as libc::c_int as usize];
    if newMagYaw - lastMagYaw < -3.14159265358979323846f64 {
        mag_spin_counter += 1;
    } else if newMagYaw - lastMagYaw > 3.14159265358979323846f64 {
        mag_spin_counter -= 1;
    }
    if newDMPYaw - lastDMPYaw < -3.14159265358979323846f64 {
        dmp_spin_counter += 1;
    } else if newDMPYaw - lastDMPYaw > 3.14159265358979323846f64 {
        dmp_spin_counter -= 1;
    }
    if first_run != 0 {
        lastMagYaw = newMagYaw;
        lastDMPYaw = newDMPYaw;
        mag_spin_counter = 0 as libc::c_int;
        dmp_spin_counter = 0 as libc::c_int;
        let mut dt: libc::c_double = 1.0f64 / config.dmp_sample_rate as libc::c_double;
        rc_filter_first_order_lowpass(&mut low_pass, dt, config.compass_time_constant);
        rc_filter_first_order_highpass(&mut high_pass, dt, config.compass_time_constant);
        rc_filter_prefill_inputs(&mut low_pass, startMagYaw);
        rc_filter_prefill_outputs(&mut low_pass, startMagYaw);
        rc_filter_prefill_inputs(&mut high_pass, newDMPYaw);
        rc_filter_prefill_outputs(&mut high_pass, 0 as libc::c_int as libc::c_double);
        first_run = 0 as libc::c_int;
    }
    let mut lp: libc::c_double = rc_filter_march(
        &mut low_pass,
        newMagYaw + 2.0f64 * 3.14159265358979323846f64 * mag_spin_counter as libc::c_double,
    );
    let mut hp: libc::c_double = rc_filter_march(
        &mut high_pass,
        newDMPYaw + 2.0f64 * 3.14159265358979323846f64 * dmp_spin_counter as libc::c_double,
    );
    newYaw = lp + hp;
    newYaw = fmod(newYaw, 2.0f64 * 3.14159265358979323846f64);
    if newYaw > 3.14159265358979323846f64 {
        newYaw -= 2.0f64 * 3.14159265358979323846f64;
    } else if newYaw < -3.14159265358979323846f64 {
        newYaw += 2.0f64 * 3.14159265358979323846f64;
    }
    (*data).compass_heading = newYaw;
    (*data).fused_TaitBryan[2 as libc::c_int as usize] = newYaw;
    (*data).fused_TaitBryan[0 as libc::c_int as usize] = (*data).dmp_TaitBryan[0 as libc::c_int as usize];
    (*data).fused_TaitBryan[1 as libc::c_int as usize] = (*data).dmp_TaitBryan[1 as libc::c_int as usize];
    rc_quaternion_from_tb_array(
        ((*data).fused_TaitBryan).as_mut_ptr(),
        ((*data).fused_quat).as_mut_ptr(),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn __load_gyro_calibration() -> libc::c_int {
    let mut fd: *mut FILE = 0 as *mut FILE;
    let mut data: [uint8_t; 6] = [0; 6];
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    fd = fopen(
        b"/var/lib/robotcontrol/gyro.cal\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if fd.is_null() {
        fprintf(
            stderr,
            b"WARNING: no gyro calibration data found\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"Please run rc_calibrate_gyro\n\n\0" as *const u8 as *const libc::c_char,
        );
        x = 0 as libc::c_int;
        y = 0 as libc::c_int;
        z = 0 as libc::c_int;
    } else {
        if fscanf(
            fd,
            b"%d\n%d\n%d\n\0" as *const u8 as *const libc::c_char,
            &mut x as *mut libc::c_int,
            &mut y as *mut libc::c_int,
            &mut z as *mut libc::c_int,
        ) != 3 as libc::c_int
        {
            fprintf(
                stderr,
                b"ERROR loading gyro offsets, calibration file empty or malformed\n\0" as *const u8
                    as *const libc::c_char,
            );
            fprintf(
                stderr,
                b"please run rc_calibrate_gyro to make a new calibration file\n\0" as *const u8 as *const libc::c_char,
            );
            fprintf(
                stderr,
                b"using default offsets for now\n\0" as *const u8 as *const libc::c_char,
            );
            x = 0 as libc::c_int;
            y = 0 as libc::c_int;
            z = 0 as libc::c_int;
        }
        fclose(fd);
    }
    data[0 as libc::c_int as usize] = (-x / 4 as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    data[1 as libc::c_int as usize] = (-x / 4 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    data[2 as libc::c_int as usize] = (-y / 4 as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    data[3 as libc::c_int as usize] = (-y / 4 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    data[4 as libc::c_int as usize] = (-z / 4 as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    data[5 as libc::c_int as usize] = (-z / 4 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    if rc_i2c_write_bytes(
        config.i2c_bus,
        0x13 as libc::c_int as uint8_t,
        6 as libc::c_int as size_t,
        &mut *data.as_mut_ptr().offset(0 as libc::c_int as isize),
    ) != 0
    {
        fprintf(
            stderr,
            b"ERROR: failed to load gyro offsets into IMU register\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __load_mag_calibration() -> libc::c_int {
    let mut fd: *mut FILE = 0 as *mut FILE;
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    let mut sx: libc::c_double = 0.;
    let mut sy: libc::c_double = 0.;
    let mut sz: libc::c_double = 0.;
    fd = fopen(
        b"/var/lib/robotcontrol/mag.cal\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if fd.is_null() {
        fprintf(
            stderr,
            b"WARNING: no magnetometer calibration data found\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"Please run rc_calibrate_mag\n\n\0" as *const u8 as *const libc::c_char,
        );
        x = 0.0f64;
        y = 0.0f64;
        z = 0.0f64;
        sx = 1.0f64;
        sy = 1.0f64;
        sz = 1.0f64;
    } else {
        if fscanf(
            fd,
            b"%lf\n%lf\n%lf\n%lf\n%lf\n%lf\n\0" as *const u8 as *const libc::c_char,
            &mut x as *mut libc::c_double,
            &mut y as *mut libc::c_double,
            &mut z as *mut libc::c_double,
            &mut sx as *mut libc::c_double,
            &mut sy as *mut libc::c_double,
            &mut sz as *mut libc::c_double,
        ) != 6 as libc::c_int
        {
            fprintf(
                stderr,
                b"ERROR loading magnetometer calibration file, empty or malformed\n\0" as *const u8
                    as *const libc::c_char,
            );
            fprintf(
                stderr,
                b"please run rc_calibrate_mag to make a new calibration file\n\0" as *const u8 as *const libc::c_char,
            );
            fprintf(
                stderr,
                b"using default offsets for now\n\0" as *const u8 as *const libc::c_char,
            );
            x = 0.0f64;
            y = 0.0f64;
            z = 0.0f64;
            sx = 1.0f64;
            sy = 1.0f64;
            sz = 1.0f64;
        }
        fclose(fd);
    }
    mag_offsets[0 as libc::c_int as usize] = x;
    mag_offsets[1 as libc::c_int as usize] = y;
    mag_offsets[2 as libc::c_int as usize] = z;
    mag_scales[0 as libc::c_int as usize] = sx;
    mag_scales[1 as libc::c_int as usize] = sy;
    mag_scales[2 as libc::c_int as usize] = sz;
    return 0 as libc::c_int;
}
unsafe extern "C" fn __load_accel_calibration() -> libc::c_int {
    let mut fd: *mut FILE = 0 as *mut FILE;
    let mut raw: [uint8_t; 6] = [
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    ];
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    let mut sx: libc::c_double = 0.;
    let mut sy: libc::c_double = 0.;
    let mut sz: libc::c_double = 0.;
    let mut bias: [int16_t; 3] = [0; 3];
    let mut factory: [int16_t; 3] = [0; 3];
    fd = fopen(
        b"/var/lib/robotcontrol/accel.cal\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if fd.is_null() {
        fprintf(
            stderr,
            b"WARNING: no accelerometer calibration data found\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"Please run rc_calibrate_accel\n\n\0" as *const u8 as *const libc::c_char,
        );
        accel_lengths[0 as libc::c_int as usize] = 1.0f64;
        accel_lengths[1 as libc::c_int as usize] = 1.0f64;
        accel_lengths[2 as libc::c_int as usize] = 1.0f64;
        return 0 as libc::c_int;
    }
    if fscanf(
        fd,
        b"%lf\n%lf\n%lf\n%lf\n%lf\n%lf\n\0" as *const u8 as *const libc::c_char,
        &mut x as *mut libc::c_double,
        &mut y as *mut libc::c_double,
        &mut z as *mut libc::c_double,
        &mut sx as *mut libc::c_double,
        &mut sy as *mut libc::c_double,
        &mut sz as *mut libc::c_double,
    ) != 6 as libc::c_int
    {
        fprintf(
            stderr,
            b"ERROR loading accel offsets, calibration file empty or malformed\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"please run rc_calibrate_accel to make a new calibration file\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"using default offsets for now\n\0" as *const u8 as *const libc::c_char,
        );
        accel_lengths[0 as libc::c_int as usize] = 1.0f64;
        accel_lengths[1 as libc::c_int as usize] = 1.0f64;
        accel_lengths[2 as libc::c_int as usize] = 1.0f64;
        return 0 as libc::c_int;
    }
    fclose(fd);
    accel_lengths[0 as libc::c_int as usize] = sx;
    accel_lengths[1 as libc::c_int as usize] = sy;
    accel_lengths[2 as libc::c_int as usize] = sz;
    if rc_i2c_read_bytes(
        config.i2c_bus,
        0x77 as libc::c_int as uint8_t,
        2 as libc::c_int as size_t,
        &mut *raw.as_mut_ptr().offset(0 as libc::c_int as isize),
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if rc_i2c_read_bytes(
        config.i2c_bus,
        0x7a as libc::c_int as uint8_t,
        2 as libc::c_int as size_t,
        &mut *raw.as_mut_ptr().offset(2 as libc::c_int as isize),
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if rc_i2c_read_bytes(
        config.i2c_bus,
        0x7d as libc::c_int as uint8_t,
        2 as libc::c_int as size_t,
        &mut *raw.as_mut_ptr().offset(4 as libc::c_int as isize),
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    factory[0 as libc::c_int as usize] =
        ((raw[0 as libc::c_int as usize] as uint16_t as libc::c_int) << 7 as libc::c_int
            | raw[1 as libc::c_int as usize] as libc::c_int >> 1 as libc::c_int) as int16_t;
    factory[1 as libc::c_int as usize] =
        ((raw[2 as libc::c_int as usize] as uint16_t as libc::c_int) << 7 as libc::c_int
            | raw[3 as libc::c_int as usize] as libc::c_int >> 1 as libc::c_int) as int16_t;
    factory[2 as libc::c_int as usize] =
        ((raw[4 as libc::c_int as usize] as uint16_t as libc::c_int) << 7 as libc::c_int
            | raw[5 as libc::c_int as usize] as libc::c_int >> 1 as libc::c_int) as int16_t;
    bias[0 as libc::c_int as usize] =
        (factory[0 as libc::c_int as usize] as libc::c_int as libc::c_double - round(x / 0.0009765615f64)) as int16_t;
    bias[1 as libc::c_int as usize] =
        (factory[1 as libc::c_int as usize] as libc::c_int as libc::c_double - round(y / 0.0009765615f64)) as int16_t;
    bias[2 as libc::c_int as usize] =
        (factory[2 as libc::c_int as usize] as libc::c_int as libc::c_double - round(z / 0.0009765615f64)) as int16_t;
    raw[0 as libc::c_int as usize] =
        (bias[0 as libc::c_int as usize] as libc::c_int >> 7 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    raw[1 as libc::c_int as usize] =
        ((bias[0 as libc::c_int as usize] as libc::c_int) << 1 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    raw[2 as libc::c_int as usize] =
        (bias[1 as libc::c_int as usize] as libc::c_int >> 7 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    raw[3 as libc::c_int as usize] =
        ((bias[1 as libc::c_int as usize] as libc::c_int) << 1 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    raw[4 as libc::c_int as usize] =
        (bias[2 as libc::c_int as usize] as libc::c_int >> 7 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    raw[5 as libc::c_int as usize] =
        ((bias[2 as libc::c_int as usize] as libc::c_int) << 1 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    if rc_i2c_write_bytes(
        config.i2c_bus,
        0x77 as libc::c_int as uint8_t,
        2 as libc::c_int as size_t,
        &mut *raw.as_mut_ptr().offset(0 as libc::c_int as isize),
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"ERROR: failed to write X accel offsets into IMU register\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_i2c_write_bytes(
        config.i2c_bus,
        0x7a as libc::c_int as uint8_t,
        2 as libc::c_int as size_t,
        &mut *raw.as_mut_ptr().offset(2 as libc::c_int as isize),
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"ERROR: failed to write Y accel offsets into IMU register\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_i2c_write_bytes(
        config.i2c_bus,
        0x7d as libc::c_int as uint8_t,
        2 as libc::c_int as size_t,
        &mut *raw.as_mut_ptr().offset(4 as libc::c_int as isize),
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"ERROR: failed to write Z accel offsets into IMU register\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __write_gyro_cal_to_disk(mut offsets: *mut int16_t) -> libc::c_int {
    let mut fd: *mut FILE = 0 as *mut FILE;
    let mut ret: libc::c_int = 0;
    ret = mkdir(
        b"/var/lib/robotcontrol/\0" as *const u8 as *const libc::c_char,
        0o777 as libc::c_int as __mode_t,
    );
    if ret == -(1 as libc::c_int) && *__errno_location() != 17 as libc::c_int {
        perror(
            b"ERROR in rc_calibrate_gyro_routine making calibration file directory\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    remove(b"/var/lib/robotcontrol/gyro.cal\0" as *const u8 as *const libc::c_char);
    fd = fopen(
        b"/var/lib/robotcontrol/gyro.cal\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if fd.is_null() {
        perror(
            b"ERROR in rc_calibrate_gyro_routine opening calibration file for writing\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"most likely you ran this as root in the past and are now\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"running it as a normal user. try deleting the file\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"sudo rm /var/lib/robotcontrol/gyro.cal\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if fprintf(
        fd,
        b"%d\n%d\n%d\n\0" as *const u8 as *const libc::c_char,
        *offsets.offset(0 as libc::c_int as isize) as libc::c_int,
        *offsets.offset(1 as libc::c_int as isize) as libc::c_int,
        *offsets.offset(2 as libc::c_int as isize) as libc::c_int,
    ) < 0 as libc::c_int
    {
        perror(b"ERROR in rc_calibrate_gyro_routine writing to file\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"most likely you ran this as root in the past and are now\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"running it as a normal user. try deleting the file\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"sudo rm /var/lib/robotcontrol/gyro.cal\n\0" as *const u8 as *const libc::c_char,
        );
        fclose(fd);
        return -(1 as libc::c_int);
    }
    fclose(fd);
    if chmod(
        b"/var/lib/robotcontrol/gyro.cal\0" as *const u8 as *const libc::c_char,
        (0o400 as libc::c_int
            | 0o200 as libc::c_int
            | 0o100 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int) >> 3 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int
                >> 3 as libc::c_int) as __mode_t,
    ) == -(1 as libc::c_int)
    {
        perror(
            b"ERROR in rc_calibrate_gyro_routine setting correct permissions for file\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"writing file anyway, will probably still work\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"most likely you ran this as root in the past and are now\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"running it as a normal user. try deleting the file\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"sudo rm /var/lib/robotcontrol/gyro.cal\n\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __write_mag_cal_to_disk(
    mut offsets: *mut libc::c_double,
    mut scale: *mut libc::c_double,
) -> libc::c_int {
    let mut fd: *mut FILE = 0 as *mut FILE;
    let mut ret: libc::c_int = 0;
    ret = mkdir(
        b"/var/lib/robotcontrol/\0" as *const u8 as *const libc::c_char,
        0o777 as libc::c_int as __mode_t,
    );
    if ret == -(1 as libc::c_int) && *__errno_location() != 17 as libc::c_int {
        perror(
            b"ERROR in rc_calibrate_mag_routine making calibration file directory\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    remove(b"/var/lib/robotcontrol/mag.cal\0" as *const u8 as *const libc::c_char);
    fd = fopen(
        b"/var/lib/robotcontrol/mag.cal\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if fd.is_null() {
        perror(
            b"ERROR in rc_calibrate_mag_routine opening calibration file for writing\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"most likely you ran this as root in the past and are now\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"running it as a normal user. try deleting the file\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"sudo rm /var/lib/robotcontrol/mag.cal\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    ret = fprintf(
        fd,
        b"%.10f\n%.10f\n%.10f\n%.10f\n%.10f\n%.10f\n\0" as *const u8 as *const libc::c_char,
        *offsets.offset(0 as libc::c_int as isize),
        *offsets.offset(1 as libc::c_int as isize),
        *offsets.offset(2 as libc::c_int as isize),
        *scale.offset(0 as libc::c_int as isize),
        *scale.offset(1 as libc::c_int as isize),
        *scale.offset(2 as libc::c_int as isize),
    );
    if ret < 0 as libc::c_int {
        perror(b"ERROR in rc_calibrate_mag_routine writing to file\n\0" as *const u8 as *const libc::c_char);
        fclose(fd);
        return -(1 as libc::c_int);
    }
    fclose(fd);
    if chmod(
        b"/var/lib/robotcontrol/mag.cal\0" as *const u8 as *const libc::c_char,
        (0o400 as libc::c_int
            | 0o200 as libc::c_int
            | 0o100 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int) >> 3 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int
                >> 3 as libc::c_int) as __mode_t,
    ) == -(1 as libc::c_int)
    {
        perror(
            b"ERROR in rc_calibrate_mag_routine setting correct permissions for file\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"writing file anyway, will probably still work\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"most likely you ran this as root in the past and are now\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"running it as a normal user. try deleting the file\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"sudo rm /var/lib/robotcontrol/mag.cal\n\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __write_accel_cal_to_disk(
    mut center: *mut libc::c_double,
    mut lengths: *mut libc::c_double,
) -> libc::c_int {
    let mut fd: *mut FILE = 0 as *mut FILE;
    let mut ret: libc::c_int = 0;
    ret = mkdir(
        b"/var/lib/robotcontrol/\0" as *const u8 as *const libc::c_char,
        0o777 as libc::c_int as __mode_t,
    );
    if ret == -(1 as libc::c_int) && *__errno_location() != 17 as libc::c_int {
        perror(
            b"ERROR in rc_mpu_calibrate_accel_routine making calibration file directory\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    remove(b"/var/lib/robotcontrol/accel.cal\0" as *const u8 as *const libc::c_char);
    fd = fopen(
        b"/var/lib/robotcontrol/accel.cal\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if fd.is_null() {
        perror(
            b"ERROR in rc_mpu_calibrate_accel_routine opening calibration file for writing\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"most likely you ran this as root in the past and are now\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"running it as a normal user. try deleting the file\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"sudo rm /var/lib/robotcontrol/accel.cal\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    ret = fprintf(
        fd,
        b"%.10f\n%.10f\n%.10f\n%.10f\n%.10f\n%.10f\n\0" as *const u8 as *const libc::c_char,
        *center.offset(0 as libc::c_int as isize),
        *center.offset(1 as libc::c_int as isize),
        *center.offset(2 as libc::c_int as isize),
        *lengths.offset(0 as libc::c_int as isize),
        *lengths.offset(1 as libc::c_int as isize),
        *lengths.offset(2 as libc::c_int as isize),
    );
    if ret < 0 as libc::c_int {
        perror(b"ERROR in rc_mpu_calibrate_accel_routine writing to file\n\0" as *const u8 as *const libc::c_char);
        fclose(fd);
        return -(1 as libc::c_int);
    }
    fclose(fd);
    if chmod(
        b"/var/lib/robotcontrol/accel.cal\0" as *const u8 as *const libc::c_char,
        (0o400 as libc::c_int
            | 0o200 as libc::c_int
            | 0o100 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int) >> 3 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int
                >> 3 as libc::c_int) as __mode_t,
    ) == -(1 as libc::c_int)
    {
        perror(
            b"WARNING in rc_calibrate_accel_routine setting correct permissions for file\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"writing file anyway, will probably still work\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"most likely you ran this as root in the past and are now\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"running it as a normal user. try deleting the file\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"sudo rm /var/lib/robotcontrol/accel.cal\n\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_mpu_calibrate_gyro_routine(mut conf: rc_mpu_config_t) -> libc::c_int {
    let mut fifo_count: int16_t = 0;
    let mut samples: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut x: int16_t = 0;
    let mut y: int16_t = 0;
    let mut z: int16_t = 0;
    let mut vx: rc_vector_t = rc_vector_t {
        len: 0,
        d: 0 as *mut libc::c_double,
        initialized: 0,
    };
    let mut vy: rc_vector_t = rc_vector_t {
        len: 0,
        d: 0 as *mut libc::c_double,
        initialized: 0,
    };
    let mut vz: rc_vector_t = rc_vector_t {
        len: 0,
        d: 0 as *mut libc::c_double,
        initialized: 0,
    };
    let mut dev_x: libc::c_double = 0.;
    let mut dev_y: libc::c_double = 0.;
    let mut dev_z: libc::c_double = 0.;
    let mut c: uint8_t = 0;
    let mut data: [uint8_t; 6] = [0; 6];
    let mut gyro_sum: [int32_t; 3] = [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int];
    let mut offsets: [int16_t; 3] = [0; 3];
    was_last_steady = 1 as libc::c_int;
    config.i2c_bus = conf.i2c_bus;
    config.i2c_addr = conf.i2c_addr;
    if rc_i2c_get_lock(conf.i2c_bus) != 0 {
        fprintf(
            stderr,
            b"i2c bus claimed by another process\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"aborting gyro calibration()\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_i2c_init(conf.i2c_bus, conf.i2c_addr) == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"ERROR in rc_mpu_calibrate_gyro_routine, failed to init i2c bus\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_i2c_lock_bus(conf.i2c_bus);
    if __reset_mpu() == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"ERROR in rc_mpu_calibrate_gyro_routine, failed to reset MPU9250\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_i2c_write_byte(
        conf.i2c_bus,
        0x6b as libc::c_int as uint8_t,
        0x1 as libc::c_int as uint8_t,
    );
    rc_i2c_write_byte(
        conf.i2c_bus,
        0x6c as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    );
    rc_usleep(200000 as libc::c_int as libc::c_uint);
    rc_i2c_write_byte(
        conf.i2c_bus,
        0x38 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    );
    rc_i2c_write_byte(
        conf.i2c_bus,
        0x23 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    );
    rc_i2c_write_byte(
        conf.i2c_bus,
        0x6b as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    );
    rc_i2c_write_byte(
        conf.i2c_bus,
        0x24 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    );
    rc_i2c_write_byte(
        conf.i2c_bus,
        0x6a as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    );
    rc_i2c_write_byte(
        conf.i2c_bus,
        0x6a as libc::c_int as uint8_t,
        0xc as libc::c_int as uint8_t,
    );
    rc_usleep(15000 as libc::c_int as libc::c_uint);
    rc_i2c_write_byte(
        conf.i2c_bus,
        0x1a as libc::c_int as uint8_t,
        0x1 as libc::c_int as uint8_t,
    );
    rc_i2c_write_byte(
        conf.i2c_bus,
        0x19 as libc::c_int as uint8_t,
        0x4 as libc::c_int as uint8_t,
    );
    rc_i2c_write_byte(
        conf.i2c_bus,
        0x1b as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    );
    rc_i2c_write_byte(
        conf.i2c_bus,
        0x1c as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    );
    loop {
        rc_i2c_write_byte(
            conf.i2c_bus,
            0x6a as libc::c_int as uint8_t,
            0x40 as libc::c_int as uint8_t,
        );
        c = ((0x1 as libc::c_int) << 6 as libc::c_int
            | (0x1 as libc::c_int) << 5 as libc::c_int
            | (0x1 as libc::c_int) << 4 as libc::c_int) as uint8_t;
        rc_i2c_write_byte(conf.i2c_bus, 0x23 as libc::c_int as uint8_t, c);
        rc_usleep(400000 as libc::c_int as libc::c_uint);
        rc_i2c_write_byte(
            conf.i2c_bus,
            0x23 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
        );
        rc_i2c_read_bytes(
            conf.i2c_bus,
            0x72 as libc::c_int as uint8_t,
            2 as libc::c_int as size_t,
            &mut *data.as_mut_ptr().offset(0 as libc::c_int as isize),
        );
        fifo_count = ((data[0 as libc::c_int as usize] as uint16_t as libc::c_int) << 8 as libc::c_int
            | data[1 as libc::c_int as usize] as libc::c_int) as int16_t;
        samples = fifo_count as libc::c_int / 6 as libc::c_int;
        i = 0;
        x = 0;
        y = 0;
        z = 0;
        vx = rc_vector_empty();
        vy = rc_vector_empty();
        vz = rc_vector_empty();
        rc_vector_alloc(&mut vx, samples);
        rc_vector_alloc(&mut vy, samples);
        rc_vector_alloc(&mut vz, samples);
        dev_x = 0.;
        dev_y = 0.;
        dev_z = 0.;
        gyro_sum[0 as libc::c_int as usize] = 0 as libc::c_int;
        gyro_sum[1 as libc::c_int as usize] = 0 as libc::c_int;
        gyro_sum[2 as libc::c_int as usize] = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < samples {
            if rc_i2c_read_bytes(
                conf.i2c_bus,
                0x74 as libc::c_int as uint8_t,
                6 as libc::c_int as size_t,
                data.as_mut_ptr(),
            ) < 0 as libc::c_int
            {
                fprintf(
                    stderr,
                    b"ERROR: failed to read FIFO\n\0" as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            x = ((data[0 as libc::c_int as usize] as int16_t as libc::c_int) << 8 as libc::c_int
                | data[1 as libc::c_int as usize] as libc::c_int) as int16_t;
            y = ((data[2 as libc::c_int as usize] as int16_t as libc::c_int) << 8 as libc::c_int
                | data[3 as libc::c_int as usize] as libc::c_int) as int16_t;
            z = ((data[4 as libc::c_int as usize] as int16_t as libc::c_int) << 8 as libc::c_int
                | data[5 as libc::c_int as usize] as libc::c_int) as int16_t;
            gyro_sum[0 as libc::c_int as usize] += x as int32_t;
            gyro_sum[1 as libc::c_int as usize] += y as int32_t;
            gyro_sum[2 as libc::c_int as usize] += z as int32_t;
            *(vx.d).offset(i as isize) = x as libc::c_double;
            *(vy.d).offset(i as isize) = y as libc::c_double;
            *(vz.d).offset(i as isize) = z as libc::c_double;
            i += 1;
        }
        dev_x = rc_vector_std_dev(vx);
        dev_y = rc_vector_std_dev(vy);
        dev_z = rc_vector_std_dev(vz);
        rc_vector_free(&mut vx);
        rc_vector_free(&mut vy);
        rc_vector_free(&mut vz);
        if dev_x > 50 as libc::c_int as libc::c_double
            || dev_y > 50 as libc::c_int as libc::c_double
            || dev_z > 50 as libc::c_int as libc::c_double
        {
            printf(b"Gyro data too noisy, put me down on a solid surface!\n\0" as *const u8 as *const libc::c_char);
            printf(b"trying again\n\0" as *const u8 as *const libc::c_char);
            was_last_steady = 0 as libc::c_int;
        } else if was_last_steady == 0 as libc::c_int {
            was_last_steady = 1 as libc::c_int;
        } else {
            offsets[0 as libc::c_int as usize] = (gyro_sum[0 as libc::c_int as usize] / samples) as int16_t;
            offsets[1 as libc::c_int as usize] = (gyro_sum[1 as libc::c_int as usize] / samples) as int16_t;
            offsets[2 as libc::c_int as usize] = (gyro_sum[2 as libc::c_int as usize] / samples) as int16_t;
            if !(abs(offsets[0 as libc::c_int as usize] as libc::c_int) > 500 as libc::c_int
                || abs(offsets[1 as libc::c_int as usize] as libc::c_int) > 500 as libc::c_int
                || abs(offsets[2 as libc::c_int as usize] as libc::c_int) > 500 as libc::c_int)
            {
                break;
            }
            printf(b"Gyro data out of bounds, put me down on a solid surface!\n\0" as *const u8 as *const libc::c_char);
            printf(b"trying again\n\0" as *const u8 as *const libc::c_char);
        }
    }
    rc_i2c_unlock_bus(conf.i2c_bus);
    if __write_gyro_cal_to_disk(offsets.as_mut_ptr()) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_calibrate_gyro_routine, failed to write to disk\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_mpu_calibrate_mag_routine(mut conf: rc_mpu_config_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut new_scale: [libc::c_double; 3] = [0.; 3];
    let samples: libc::c_int = 200 as libc::c_int;
    let sample_time_us: libc::c_int = 12000000 as libc::c_int;
    let loop_wait_us: libc::c_int = sample_time_us / samples;
    let sample_rate_hz: libc::c_int = 1000000 as libc::c_int / loop_wait_us;
    let mut A: rc_matrix_t = rc_matrix_empty();
    let mut center: rc_vector_t = rc_vector_empty();
    let mut lengths: rc_vector_t = rc_vector_empty();
    let mut imu_data: rc_mpu_data_t = rc_mpu_data_t {
        accel: [0.; 3],
        gyro: [0.; 3],
        mag: [0.; 3],
        temp: 0.,
        raw_gyro: [0; 3],
        raw_accel: [0; 3],
        accel_to_ms2: 0.,
        gyro_to_degs: 0.,
        dmp_quat: [0.; 4],
        dmp_TaitBryan: [0.; 3],
        tap_detected: 0,
        last_tap_direction: 0,
        last_tap_count: 0,
        fused_quat: [0.; 4],
        fused_TaitBryan: [0.; 3],
        compass_heading: 0.,
        compass_heading_raw: 0.,
    };
    config = rc_mpu_default_config();
    config.enable_magnetometer = 1 as libc::c_int;
    config.i2c_bus = conf.i2c_bus;
    config.i2c_addr = conf.i2c_addr;
    if rc_i2c_get_lock(config.i2c_bus) != 0 {
        fprintf(
            stderr,
            b"i2c bus claimed by another process\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"aborting magnetometer calibration()\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_i2c_init(config.i2c_bus, config.i2c_addr) != 0 {
        fprintf(
            stderr,
            b"ERROR rc_calibrate_mag_routine failed at rc_i2c_init\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_i2c_lock_bus(config.i2c_bus);
    if __reset_mpu() < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: failed to reset MPU9250\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if __check_who_am_i() != 0 {
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    if __init_magnetometer(1 as libc::c_int) != 0 {
        fprintf(
            stderr,
            b"ERROR: failed to initialize_magnetometer\n\0" as *const u8 as *const libc::c_char,
        );
        rc_i2c_unlock_bus(config.i2c_bus);
        return -(1 as libc::c_int);
    }
    mag_offsets[0 as libc::c_int as usize] = 0.0f64;
    mag_offsets[1 as libc::c_int as usize] = 0.0f64;
    mag_offsets[2 as libc::c_int as usize] = 0.0f64;
    mag_scales[0 as libc::c_int as usize] = 1.0f64;
    mag_scales[1 as libc::c_int as usize] = 1.0f64;
    mag_scales[2 as libc::c_int as usize] = 1.0f64;
    if rc_matrix_alloc(&mut A, samples, 3 as libc::c_int) != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_calibrate_mag_routine, failed to alloc data matrix\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < samples {
        if rc_mpu_read_mag(&mut imu_data) < 0 as libc::c_int {
            fprintf(
                stderr,
                b"ERROR: failed to read magnetometer\n\0" as *const u8 as *const libc::c_char,
            );
            break;
        } else if fabs(imu_data.mag[0 as libc::c_int as usize]) < zero_tolerance
            && fabs(imu_data.mag[1 as libc::c_int as usize]) < zero_tolerance
            && fabs(imu_data.mag[2 as libc::c_int as usize]) < zero_tolerance
        {
            fprintf(
                stderr,
                b"ERROR: retreived all zeros from magnetometer\n\0" as *const u8 as *const libc::c_char,
            );
            break;
        } else {
            *(*(A.d).offset(i as isize)).offset(0 as libc::c_int as isize) = imu_data.mag[0 as libc::c_int as usize];
            *(*(A.d).offset(i as isize)).offset(1 as libc::c_int as isize) = imu_data.mag[1 as libc::c_int as usize];
            *(*(A.d).offset(i as isize)).offset(2 as libc::c_int as isize) = imu_data.mag[2 as libc::c_int as usize];
            i += 1;
            if i % (sample_rate_hz * 4 as libc::c_int) == sample_rate_hz * 2 as libc::c_int {
                printf(b"keep spinning\n\0" as *const u8 as *const libc::c_char);
            }
            if i % (sample_rate_hz * 4 as libc::c_int) == 0 as libc::c_int {
                printf(b"you're doing great\n\0" as *const u8 as *const libc::c_char);
            }
            rc_usleep(loop_wait_us as libc::c_uint);
        }
    }
    rc_mpu_power_off();
    rc_i2c_unlock_bus(config.i2c_bus);
    printf(b"\n\nOkay Stop!\n\0" as *const u8 as *const libc::c_char);
    printf(b"Calculating calibration constants.....\n\0" as *const u8 as *const libc::c_char);
    fflush(stdout);
    if i < samples {
        printf(b"exiting rc_calibrate_mag_routine without saving new data\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if rc_algebra_fit_ellipsoid(A, &mut center, &mut lengths) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"failed to fit ellipsoid to magnetometer data\n\0" as *const u8 as *const libc::c_char,
        );
        rc_matrix_free(&mut A);
        return -(1 as libc::c_int);
    }
    rc_matrix_free(&mut A);
    if fabs(*(center.d).offset(0 as libc::c_int as isize)) > 200 as libc::c_int as libc::c_double
        || fabs(*(center.d).offset(1 as libc::c_int as isize)) > 200 as libc::c_int as libc::c_double
        || fabs(*(center.d).offset(2 as libc::c_int as isize)) > 200 as libc::c_int as libc::c_double
    {
        fprintf(
            stderr,
            b"ERROR: center of fitted ellipsoid out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        rc_vector_free(&mut center);
        rc_vector_free(&mut lengths);
        return -(1 as libc::c_int);
    }
    if *(lengths.d).offset(0 as libc::c_int as isize) > 200 as libc::c_int as libc::c_double
        || *(lengths.d).offset(0 as libc::c_int as isize) < 5 as libc::c_int as libc::c_double
        || *(lengths.d).offset(1 as libc::c_int as isize) > 200 as libc::c_int as libc::c_double
        || *(lengths.d).offset(1 as libc::c_int as isize) < 5 as libc::c_int as libc::c_double
        || *(lengths.d).offset(2 as libc::c_int as isize) > 200 as libc::c_int as libc::c_double
        || *(lengths.d).offset(2 as libc::c_int as isize) < 5 as libc::c_int as libc::c_double
    {
        fprintf(
            stderr,
            b"WARNING: length of fitted ellipsoid out of bounds\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"Saving suspicious calibration data anyway in case this is intentional\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    new_scale[0 as libc::c_int as usize] = 70.0f64 / *(lengths.d).offset(0 as libc::c_int as isize);
    new_scale[1 as libc::c_int as usize] = 70.0f64 / *(lengths.d).offset(1 as libc::c_int as isize);
    new_scale[2 as libc::c_int as usize] = 70.0f64 / *(lengths.d).offset(2 as libc::c_int as isize);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Offsets X: %7.3f Y: %7.3f Z: %7.3f\n\0" as *const u8 as *const libc::c_char,
        *(center.d).offset(0 as libc::c_int as isize),
        *(center.d).offset(1 as libc::c_int as isize),
        *(center.d).offset(2 as libc::c_int as isize),
    );
    printf(
        b"Scales  X: %7.3f Y: %7.3f Z: %7.3f\n\0" as *const u8 as *const libc::c_char,
        new_scale[0 as libc::c_int as usize],
        new_scale[1 as libc::c_int as usize],
        new_scale[2 as libc::c_int as usize],
    );
    if __write_mag_cal_to_disk(center.d, new_scale.as_mut_ptr()) < 0 as libc::c_int {
        rc_vector_free(&mut center);
        rc_vector_free(&mut lengths);
        return -(1 as libc::c_int);
    }
    rc_vector_free(&mut center);
    rc_vector_free(&mut lengths);
    return 0 as libc::c_int;
}
unsafe extern "C" fn __collect_accel_samples(mut avg_raw: *mut libc::c_int) -> libc::c_int {
    let mut data: [uint8_t; 6] = [0; 6];
    let mut sum: [int32_t; 3] = [0; 3];
    let mut i: libc::c_int = 0;
    let mut samples: libc::c_int = 0;
    let mut fifo_count: libc::c_int = 0;
    let mut x: int16_t = 0;
    let mut y: int16_t = 0;
    let mut z: int16_t = 0;
    let mut dev_x: libc::c_double = 0.;
    let mut dev_y: libc::c_double = 0.;
    let mut dev_z: libc::c_double = 0.;
    let mut vx: rc_vector_t = rc_vector_empty();
    let mut vy: rc_vector_t = rc_vector_empty();
    let mut vz: rc_vector_t = rc_vector_empty();
    rc_i2c_write_byte(
        config.i2c_bus,
        0x6a as libc::c_int as uint8_t,
        0x40 as libc::c_int as uint8_t,
    );
    rc_i2c_write_byte(
        config.i2c_bus,
        0x23 as libc::c_int as uint8_t,
        ((0x1 as libc::c_int) << 3 as libc::c_int) as uint8_t,
    );
    rc_usleep(400000 as libc::c_int as libc::c_uint);
    rc_i2c_write_byte(
        config.i2c_bus,
        0x23 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    );
    rc_i2c_read_bytes(
        config.i2c_bus,
        0x72 as libc::c_int as uint8_t,
        2 as libc::c_int as size_t,
        &mut *data.as_mut_ptr().offset(0 as libc::c_int as isize),
    );
    fifo_count = (data[0 as libc::c_int as usize] as uint16_t as libc::c_int) << 8 as libc::c_int
        | data[1 as libc::c_int as usize] as libc::c_int;
    samples = fifo_count / 6 as libc::c_int;
    rc_vector_alloc(&mut vx, samples);
    rc_vector_alloc(&mut vy, samples);
    rc_vector_alloc(&mut vz, samples);
    sum[0 as libc::c_int as usize] = 0 as libc::c_int;
    sum[1 as libc::c_int as usize] = 0 as libc::c_int;
    sum[2 as libc::c_int as usize] = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < samples {
        if rc_i2c_read_bytes(
            config.i2c_bus,
            0x74 as libc::c_int as uint8_t,
            6 as libc::c_int as size_t,
            data.as_mut_ptr(),
        ) < 0 as libc::c_int
        {
            fprintf(
                stderr,
                b"ERROR in rc_mpu_calibrate_accel_routine, failed to read FIFO\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        x = ((data[0 as libc::c_int as usize] as int16_t as libc::c_int) << 8 as libc::c_int
            | data[1 as libc::c_int as usize] as libc::c_int) as int16_t;
        y = ((data[2 as libc::c_int as usize] as int16_t as libc::c_int) << 8 as libc::c_int
            | data[3 as libc::c_int as usize] as libc::c_int) as int16_t;
        z = ((data[4 as libc::c_int as usize] as int16_t as libc::c_int) << 8 as libc::c_int
            | data[5 as libc::c_int as usize] as libc::c_int) as int16_t;
        sum[0 as libc::c_int as usize] += x as int32_t;
        sum[1 as libc::c_int as usize] += y as int32_t;
        sum[2 as libc::c_int as usize] += z as int32_t;
        *(vx.d).offset(i as isize) = x as libc::c_double;
        *(vy.d).offset(i as isize) = y as libc::c_double;
        *(vz.d).offset(i as isize) = z as libc::c_double;
        i += 1;
    }
    dev_x = rc_vector_std_dev(vx);
    dev_y = rc_vector_std_dev(vy);
    dev_z = rc_vector_std_dev(vz);
    rc_vector_free(&mut vx);
    rc_vector_free(&mut vy);
    rc_vector_free(&mut vz);
    if dev_x > 100 as libc::c_int as libc::c_double
        || dev_y > 100 as libc::c_int as libc::c_double
        || dev_z > 100 as libc::c_int as libc::c_double
    {
        was_last_steady = 0 as libc::c_int;
        printf(b"data too noisy, please hold me still\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    if was_last_steady == 0 as libc::c_int {
        was_last_steady = 1 as libc::c_int;
        return 1 as libc::c_int;
    }
    *avg_raw.offset(0 as libc::c_int as isize) = sum[0 as libc::c_int as usize] / samples;
    *avg_raw.offset(1 as libc::c_int as isize) = sum[1 as libc::c_int as usize] / samples;
    *avg_raw.offset(2 as libc::c_int as isize) = sum[2 as libc::c_int as usize] / samples;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_mpu_calibrate_accel_routine(mut conf: rc_mpu_config_t) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut avg_raw: [[libc::c_int; 3]; 6] = [[0; 3]; 6];
    config.i2c_bus = conf.i2c_bus;
    config.i2c_addr = conf.i2c_addr;
    if rc_i2c_get_lock(config.i2c_bus) != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_mpu_calibrate_accel_routine, i2c bus claimed by another process\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if rc_i2c_init(config.i2c_bus, config.i2c_addr) != 0 {
        fprintf(
            stderr,
            b"ERROR in rc_mpu_calibrate_accel_routine, failed at rc_i2c_init\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_i2c_lock_bus(config.i2c_bus);
    if __reset_mpu() < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR in rc_mpu_calibrate_accel_routine failed to reset MPU9250\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_i2c_write_byte(
        config.i2c_bus,
        0x6b as libc::c_int as uint8_t,
        0x1 as libc::c_int as uint8_t,
    );
    rc_i2c_write_byte(
        config.i2c_bus,
        0x6c as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    );
    rc_usleep(200000 as libc::c_int as libc::c_uint);
    rc_i2c_write_byte(
        config.i2c_bus,
        0x38 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    );
    rc_i2c_write_byte(
        config.i2c_bus,
        0x23 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    );
    rc_i2c_write_byte(
        config.i2c_bus,
        0x6b as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    );
    rc_i2c_write_byte(
        config.i2c_bus,
        0x24 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    );
    rc_i2c_write_byte(
        config.i2c_bus,
        0x6a as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    );
    rc_i2c_write_byte(
        config.i2c_bus,
        0x6a as libc::c_int as uint8_t,
        0xc as libc::c_int as uint8_t,
    );
    rc_usleep(15000 as libc::c_int as libc::c_uint);
    rc_i2c_write_byte(
        config.i2c_bus,
        0x1a as libc::c_int as uint8_t,
        0x1 as libc::c_int as uint8_t,
    );
    rc_i2c_write_byte(
        config.i2c_bus,
        0x19 as libc::c_int as uint8_t,
        0x4 as libc::c_int as uint8_t,
    );
    rc_i2c_write_byte(
        config.i2c_bus,
        0x1b as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    );
    rc_i2c_write_byte(
        config.i2c_bus,
        0x1c as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    );
    printf(b"\nOrient Z pointing up and hold as still as possible\n\0" as *const u8 as *const libc::c_char);
    printf(b"When ready, press any key to sample accelerometer\n\0" as *const u8 as *const libc::c_char);
    getchar();
    ret = 1 as libc::c_int;
    was_last_steady = 0 as libc::c_int;
    while ret != 0 {
        ret = __collect_accel_samples((avg_raw[0 as libc::c_int as usize]).as_mut_ptr());
        if ret == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    printf(b"success\n\0" as *const u8 as *const libc::c_char);
    printf(b"\nOrient Z pointing down and hold as still as possible\n\0" as *const u8 as *const libc::c_char);
    printf(b"When ready, press any key to sample accelerometer\n\0" as *const u8 as *const libc::c_char);
    getchar();
    ret = 1 as libc::c_int;
    was_last_steady = 0 as libc::c_int;
    while ret != 0 {
        ret = __collect_accel_samples((avg_raw[1 as libc::c_int as usize]).as_mut_ptr());
        if ret == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    printf(b"success\n\0" as *const u8 as *const libc::c_char);
    printf(b"\nOrient X pointing up and hold as still as possible\n\0" as *const u8 as *const libc::c_char);
    printf(b"When ready, press any key to sample accelerometer\n\0" as *const u8 as *const libc::c_char);
    getchar();
    ret = 1 as libc::c_int;
    was_last_steady = 0 as libc::c_int;
    while ret != 0 {
        ret = __collect_accel_samples((avg_raw[2 as libc::c_int as usize]).as_mut_ptr());
        if ret == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    printf(b"success\n\0" as *const u8 as *const libc::c_char);
    printf(b"\nOrient X pointing down and hold as still as possible\n\0" as *const u8 as *const libc::c_char);
    printf(b"When ready, press any key to sample accelerometer\n\0" as *const u8 as *const libc::c_char);
    getchar();
    ret = 1 as libc::c_int;
    was_last_steady = 0 as libc::c_int;
    while ret != 0 {
        ret = __collect_accel_samples((avg_raw[3 as libc::c_int as usize]).as_mut_ptr());
        if ret == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    printf(b"success\n\0" as *const u8 as *const libc::c_char);
    printf(b"\nOrient Y pointing up and hold as still as possible\n\0" as *const u8 as *const libc::c_char);
    printf(b"When ready, press any key to sample accelerometer\n\0" as *const u8 as *const libc::c_char);
    getchar();
    ret = 1 as libc::c_int;
    was_last_steady = 0 as libc::c_int;
    while ret != 0 {
        ret = __collect_accel_samples((avg_raw[4 as libc::c_int as usize]).as_mut_ptr());
        if ret == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    printf(b"success\n\0" as *const u8 as *const libc::c_char);
    printf(b"\nOrient Y pointing down and hold as still as possible\n\0" as *const u8 as *const libc::c_char);
    printf(b"When ready, press any key to sample accelerometer\n\0" as *const u8 as *const libc::c_char);
    getchar();
    ret = 1 as libc::c_int;
    was_last_steady = 0 as libc::c_int;
    while ret != 0 {
        ret = __collect_accel_samples((avg_raw[5 as libc::c_int as usize]).as_mut_ptr());
        if ret == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    printf(b"success\n\0" as *const u8 as *const libc::c_char);
    rc_mpu_power_off();
    rc_i2c_unlock_bus(config.i2c_bus);
    let mut A: rc_matrix_t = rc_matrix_empty();
    let mut center: rc_vector_t = rc_vector_empty();
    let mut lengths: rc_vector_t = rc_vector_empty();
    if rc_matrix_alloc(&mut A, 6 as libc::c_int, 3 as libc::c_int) != 0 {
        fprintf(
            stderr,
            b"ERROR: in rc_mpu_calibrate_accel_routine, failed to alloc data matrix\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            *(*(A.d).offset(i as isize)).offset(j as isize) =
                avg_raw[i as usize][j as usize] as libc::c_double / 16384.0f64;
            j += 1;
        }
        i += 1;
    }
    if rc_algebra_fit_ellipsoid(A, &mut center, &mut lengths) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"failed to fit ellipsoid to magnetometer data\n\0" as *const u8 as *const libc::c_char,
        );
        rc_matrix_free(&mut A);
        return -(1 as libc::c_int);
    }
    rc_matrix_free(&mut A);
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if fabs(*(center.d).offset(i as isize)) > 0.3f64 {
            fprintf(
                stderr,
                b"ERROR in rc_mpu_calibrate_accel_routine, center of fitted ellipsoid out of bounds\n\0" as *const u8
                    as *const libc::c_char,
            );
            fprintf(
                stderr,
                b"most likely the unit was held in incorrect orientation during data collection\n\0" as *const u8
                    as *const libc::c_char,
            );
            rc_vector_free(&mut center);
            rc_vector_free(&mut lengths);
            return -(1 as libc::c_int);
        }
        if (*(center.d).offset(i as isize)).is_nan() as i32 != 0
            || (*(lengths.d).offset(i as isize)).is_nan() as i32 != 0
        {
            fprintf(
                stderr,
                b"ERROR in rc_mpu_calibrate_accel_routine, data fitting produced NaN\n\0" as *const u8
                    as *const libc::c_char,
            );
            fprintf(
                stderr,
                b"most likely the unit was held in incorrect orientation during data collection\n\0" as *const u8
                    as *const libc::c_char,
            );
            rc_vector_free(&mut center);
            rc_vector_free(&mut lengths);
            return -(1 as libc::c_int);
        }
        if *(lengths.d).offset(i as isize) > 1.3f64 || *(lengths.d).offset(i as isize) < 0.7f64 {
            fprintf(
                stderr,
                b"ERROR in rc_mpu_calibrate_accel_routine, scale out of bounds\n\0" as *const u8 as *const libc::c_char,
            );
            fprintf(
                stderr,
                b"most likely the unit was held in incorrect orientation during data collection\n\0" as *const u8
                    as *const libc::c_char,
            );
            rc_vector_free(&mut center);
            rc_vector_free(&mut lengths);
            return -(1 as libc::c_int);
        }
        i += 1;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Offsets X: %7.3f Y: %7.3f Z: %7.3f\n\0" as *const u8 as *const libc::c_char,
        *(center.d).offset(0 as libc::c_int as isize),
        *(center.d).offset(1 as libc::c_int as isize),
        *(center.d).offset(2 as libc::c_int as isize),
    );
    printf(
        b"Scales  X: %7.3f Y: %7.3f Z: %7.3f\n\0" as *const u8 as *const libc::c_char,
        *(lengths.d).offset(0 as libc::c_int as isize),
        *(lengths.d).offset(1 as libc::c_int as isize),
        *(lengths.d).offset(2 as libc::c_int as isize),
    );
    if __write_accel_cal_to_disk(center.d, lengths.d) == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"ERROR in rc_mpu_calibrate_accel_routine, failed to write to disk\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc_vector_free(&mut center);
    rc_vector_free(&mut lengths);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_mpu_is_gyro_calibrated() -> libc::c_int {
    if access(
        b"/var/lib/robotcontrol/gyro.cal\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) == 0
    {
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn rc_mpu_is_mag_calibrated() -> libc::c_int {
    if access(
        b"/var/lib/robotcontrol/mag.cal\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) == 0
    {
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn rc_mpu_is_accel_calibrated() -> libc::c_int {
    if access(
        b"/var/lib/robotcontrol/accel.cal\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) == 0
    {
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn rc_mpu_nanos_since_last_dmp_interrupt() -> int64_t {
    if last_interrupt_timestamp_nanos == 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int) as int64_t;
    }
    return (rc_nanos_since_epoch()).wrapping_sub(last_interrupt_timestamp_nanos) as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn rc_mpu_nanos_since_last_tap() -> int64_t {
    if last_tap_timestamp_nanos == 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int) as int64_t;
    }
    return (rc_nanos_since_epoch()).wrapping_sub(last_tap_timestamp_nanos) as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn rc_mpu_block_until_dmp_data() -> libc::c_int {
    if imu_shutdown_flag != 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: call to rc_mpu_block_until_dmp_data after shutting down mpu\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if thread_running_flag == 0 {
        fprintf(
            stderr,
            b"ERROR: call to rc_mpu_block_until_dmp_data when DMP handler not running\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    pthread_mutex_lock(&mut read_mutex);
    pthread_cond_wait(&mut read_condition, &mut read_mutex);
    pthread_mutex_unlock(&mut read_mutex);
    if imu_shutdown_flag != 0 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rc_mpu_block_until_tap() -> libc::c_int {
    if imu_shutdown_flag != 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR: call to rc_mpu_block_until_tap after shutting down mpu\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if thread_running_flag == 0 {
        fprintf(
            stderr,
            b"ERROR: call to rc_mpu_block_until_tap when DMP handler not running\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    pthread_mutex_lock(&mut tap_mutex);
    pthread_cond_wait(&mut tap_condition, &mut tap_mutex);
    pthread_mutex_unlock(&mut tap_mutex);
    if imu_shutdown_flag != 0 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __mag_correct_orientation(mut mag_vec: *mut libc::c_double) -> libc::c_int {
    match config.orient as libc::c_uint {
        136 => {
            *mag_vec.offset(0 as libc::c_int as isize) = (*data_ptr).mag[0 as libc::c_int as usize];
            *mag_vec.offset(1 as libc::c_int as isize) = (*data_ptr).mag[1 as libc::c_int as usize];
            *mag_vec.offset(2 as libc::c_int as isize) = (*data_ptr).mag[2 as libc::c_int as usize];
        }
        396 => {
            *mag_vec.offset(0 as libc::c_int as isize) = -(*data_ptr).mag[0 as libc::c_int as usize];
            *mag_vec.offset(1 as libc::c_int as isize) = (*data_ptr).mag[1 as libc::c_int as usize];
            *mag_vec.offset(2 as libc::c_int as isize) = -(*data_ptr).mag[2 as libc::c_int as usize];
        }
        14 => {
            *mag_vec.offset(0 as libc::c_int as isize) = -(*data_ptr).mag[2 as libc::c_int as usize];
            *mag_vec.offset(1 as libc::c_int as isize) = (*data_ptr).mag[1 as libc::c_int as usize];
            *mag_vec.offset(2 as libc::c_int as isize) = (*data_ptr).mag[0 as libc::c_int as usize];
        }
        266 => {
            *mag_vec.offset(0 as libc::c_int as isize) = (*data_ptr).mag[2 as libc::c_int as usize];
            *mag_vec.offset(1 as libc::c_int as isize) = (*data_ptr).mag[1 as libc::c_int as usize];
            *mag_vec.offset(2 as libc::c_int as isize) = -(*data_ptr).mag[0 as libc::c_int as usize];
        }
        112 => {
            *mag_vec.offset(0 as libc::c_int as isize) = (*data_ptr).mag[0 as libc::c_int as usize];
            *mag_vec.offset(1 as libc::c_int as isize) = -(*data_ptr).mag[2 as libc::c_int as usize];
            *mag_vec.offset(2 as libc::c_int as isize) = (*data_ptr).mag[1 as libc::c_int as usize];
        }
        336 => {
            *mag_vec.offset(0 as libc::c_int as isize) = (*data_ptr).mag[0 as libc::c_int as usize];
            *mag_vec.offset(1 as libc::c_int as isize) = (*data_ptr).mag[2 as libc::c_int as usize];
            *mag_vec.offset(2 as libc::c_int as isize) = -(*data_ptr).mag[1 as libc::c_int as usize];
        }
        133 => {
            *mag_vec.offset(0 as libc::c_int as isize) = -(*data_ptr).mag[1 as libc::c_int as usize];
            *mag_vec.offset(1 as libc::c_int as isize) = (*data_ptr).mag[0 as libc::c_int as usize];
            *mag_vec.offset(2 as libc::c_int as isize) = (*data_ptr).mag[2 as libc::c_int as usize];
        }
        161 => {
            *mag_vec.offset(0 as libc::c_int as isize) = (*data_ptr).mag[1 as libc::c_int as usize];
            *mag_vec.offset(1 as libc::c_int as isize) = -(*data_ptr).mag[0 as libc::c_int as usize];
            *mag_vec.offset(2 as libc::c_int as isize) = (*data_ptr).mag[2 as libc::c_int as usize];
        }
        _ => {
            fprintf(
                stderr,
                b"ERROR: reading magnetometer, invalid orientation\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
