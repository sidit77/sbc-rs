#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn sbc_setup_bits(
        bits: *mut sbc_bits_t,
        mode: sbc_bits_mode,
        data: *mut libc::c_void,
        size: libc::c_uint,
    );
    fn sbc_tell_bits(bits: *mut sbc_bits_t) -> libc::c_uint;
    fn sbc_flush_bits(bits: *mut sbc_bits_t);
    fn __sbc_get_bits(_: *mut sbc_bits_t, _: libc::c_uint) -> libc::c_uint;
    fn __sbc_put_bits(_: *mut sbc_bits_t, _: libc::c_uint, _: libc::c_uint);
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __int32_t = libc::c_int;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uintptr_t = libc::c_ulong;
pub type sbc_freq = libc::c_uint;
pub const SBC_NUM_FREQ: sbc_freq = 4;
pub const SBC_FREQ_48K: sbc_freq = 3;
pub const SBC_FREQ_44K1: sbc_freq = 2;
pub const SBC_FREQ_32K: sbc_freq = 1;
pub const SBC_FREQ_16K: sbc_freq = 0;
pub type sbc_mode = libc::c_uint;
pub const SBC_NUM_MODE: sbc_mode = 4;
pub const SBC_MODE_JOINT_STEREO: sbc_mode = 3;
pub const SBC_MODE_STEREO: sbc_mode = 2;
pub const SBC_MODE_DUAL_CHANNEL: sbc_mode = 1;
pub const SBC_MODE_MONO: sbc_mode = 0;
pub type sbc_bam = libc::c_uint;
pub const SBC_NUM_BAM: sbc_bam = 2;
pub const SBC_BAM_SNR: sbc_bam = 1;
pub const SBC_BAM_LOUDNESS: sbc_bam = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sbc_frame {
    pub msbc: bool,
    pub freq: sbc_freq,
    pub mode: sbc_mode,
    pub bam: sbc_bam,
    pub nblocks: libc::c_int,
    pub nsubbands: libc::c_int,
    pub bitpool: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sbc_dstate {
    pub idx: libc::c_int,
    pub v: [[[int16_t; 10]; 8]; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sbc_estate {
    pub idx: libc::c_int,
    pub x: [[[int16_t; 5]; 8]; 2],
    pub y: [int32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sbc {
    pub nchannels: libc::c_int,
    pub nblocks: libc::c_int,
    pub nsubbands: libc::c_int,
    pub c2rust_unnamed: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub dstates: [sbc_dstate; 2],
    pub estates: [sbc_estate; 2],
}
pub type sbc_t = sbc;
pub type sbc_bits_t = sbc_bits;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sbc_bits {
    pub mode: sbc_bits_mode,
    pub data: C2RustUnnamed_1,
    pub accu: C2RustUnnamed_0,
    pub error: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub v: bits_accu_t,
    pub nleft: libc::c_uint,
    pub nover: libc::c_uint,
}
pub type bits_accu_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub p: *mut uint8_t,
    pub nbytes: libc::c_uint,
    pub nleft: libc::c_uint,
}
pub type sbc_bits_mode = libc::c_uint;
pub const SBC_BITS_WRITE: sbc_bits_mode = 1;
pub const SBC_BITS_READ: sbc_bits_mode = 0;
#[inline]
unsafe extern "C" fn sbc_bits_error(mut bits: *mut sbc_bits) -> bool {
    return (*bits).error;
}
#[inline]
unsafe extern "C" fn sbc_get_bits(
    mut bits: *mut sbc_bits,
    mut n: libc::c_uint,
) -> libc::c_uint {
    if (*bits).accu.nleft < n {
        return __sbc_get_bits(bits, n);
    }
    (*bits).accu.nleft = ((*bits).accu.nleft).wrapping_sub(n);
    return (*bits).accu.v >> (*bits).accu.nleft
        & ((1 as libc::c_uint) << n).wrapping_sub(1 as libc::c_int as libc::c_uint);
}
#[inline]
unsafe extern "C" fn sbc_put_bits(
    mut bits: *mut sbc_bits,
    mut v: libc::c_uint,
    mut n: libc::c_uint,
) {
    if (*bits).accu.nleft < n {
        __sbc_put_bits(bits, v, n);
    } else {
        (*bits).accu.nleft = ((*bits).accu.nleft).wrapping_sub(n);
        (*bits)
            .accu
            .v = (*bits).accu.v << n
            | v
            & ((1 as libc::c_uint) << n)
            .wrapping_sub(1 as libc::c_int as libc::c_uint);
    };
}
#[inline]
unsafe extern "C" fn sbc_get_fixed(
    mut bits: *mut sbc_bits,
    mut n: libc::c_uint,
    mut v: libc::c_uint,
) {
    if sbc_get_bits(bits, n) != v {
        (*bits).error = 1 as libc::c_int != 0;
    }
}
static mut msbc_frame: sbc_frame = {
    let mut init = sbc_frame {
        msbc: 1 as libc::c_int != 0,
        freq: SBC_FREQ_16K,
        mode: SBC_MODE_MONO,
        bam: SBC_BAM_LOUDNESS,
        nblocks: 15 as libc::c_int,
        nsubbands: 8 as libc::c_int,
        bitpool: 26 as libc::c_int,
    };
    init
};
unsafe extern "C" fn compute_crc(
    mut frame: *const sbc_frame,
    mut data: *const uint8_t,
    mut size: libc::c_uint,
) -> libc::c_int {
    static mut t: [uint8_t; 256] = [
        0 as libc::c_int as uint8_t,
        0x1d as libc::c_int as uint8_t,
        0x3a as libc::c_int as uint8_t,
        0x27 as libc::c_int as uint8_t,
        0x74 as libc::c_int as uint8_t,
        0x69 as libc::c_int as uint8_t,
        0x4e as libc::c_int as uint8_t,
        0x53 as libc::c_int as uint8_t,
        0xe8 as libc::c_int as uint8_t,
        0xf5 as libc::c_int as uint8_t,
        0xd2 as libc::c_int as uint8_t,
        0xcf as libc::c_int as uint8_t,
        0x9c as libc::c_int as uint8_t,
        0x81 as libc::c_int as uint8_t,
        0xa6 as libc::c_int as uint8_t,
        0xbb as libc::c_int as uint8_t,
        0xcd as libc::c_int as uint8_t,
        0xd0 as libc::c_int as uint8_t,
        0xf7 as libc::c_int as uint8_t,
        0xea as libc::c_int as uint8_t,
        0xb9 as libc::c_int as uint8_t,
        0xa4 as libc::c_int as uint8_t,
        0x83 as libc::c_int as uint8_t,
        0x9e as libc::c_int as uint8_t,
        0x25 as libc::c_int as uint8_t,
        0x38 as libc::c_int as uint8_t,
        0x1f as libc::c_int as uint8_t,
        0x2 as libc::c_int as uint8_t,
        0x51 as libc::c_int as uint8_t,
        0x4c as libc::c_int as uint8_t,
        0x6b as libc::c_int as uint8_t,
        0x76 as libc::c_int as uint8_t,
        0x87 as libc::c_int as uint8_t,
        0x9a as libc::c_int as uint8_t,
        0xbd as libc::c_int as uint8_t,
        0xa0 as libc::c_int as uint8_t,
        0xf3 as libc::c_int as uint8_t,
        0xee as libc::c_int as uint8_t,
        0xc9 as libc::c_int as uint8_t,
        0xd4 as libc::c_int as uint8_t,
        0x6f as libc::c_int as uint8_t,
        0x72 as libc::c_int as uint8_t,
        0x55 as libc::c_int as uint8_t,
        0x48 as libc::c_int as uint8_t,
        0x1b as libc::c_int as uint8_t,
        0x6 as libc::c_int as uint8_t,
        0x21 as libc::c_int as uint8_t,
        0x3c as libc::c_int as uint8_t,
        0x4a as libc::c_int as uint8_t,
        0x57 as libc::c_int as uint8_t,
        0x70 as libc::c_int as uint8_t,
        0x6d as libc::c_int as uint8_t,
        0x3e as libc::c_int as uint8_t,
        0x23 as libc::c_int as uint8_t,
        0x4 as libc::c_int as uint8_t,
        0x19 as libc::c_int as uint8_t,
        0xa2 as libc::c_int as uint8_t,
        0xbf as libc::c_int as uint8_t,
        0x98 as libc::c_int as uint8_t,
        0x85 as libc::c_int as uint8_t,
        0xd6 as libc::c_int as uint8_t,
        0xcb as libc::c_int as uint8_t,
        0xec as libc::c_int as uint8_t,
        0xf1 as libc::c_int as uint8_t,
        0x13 as libc::c_int as uint8_t,
        0xe as libc::c_int as uint8_t,
        0x29 as libc::c_int as uint8_t,
        0x34 as libc::c_int as uint8_t,
        0x67 as libc::c_int as uint8_t,
        0x7a as libc::c_int as uint8_t,
        0x5d as libc::c_int as uint8_t,
        0x40 as libc::c_int as uint8_t,
        0xfb as libc::c_int as uint8_t,
        0xe6 as libc::c_int as uint8_t,
        0xc1 as libc::c_int as uint8_t,
        0xdc as libc::c_int as uint8_t,
        0x8f as libc::c_int as uint8_t,
        0x92 as libc::c_int as uint8_t,
        0xb5 as libc::c_int as uint8_t,
        0xa8 as libc::c_int as uint8_t,
        0xde as libc::c_int as uint8_t,
        0xc3 as libc::c_int as uint8_t,
        0xe4 as libc::c_int as uint8_t,
        0xf9 as libc::c_int as uint8_t,
        0xaa as libc::c_int as uint8_t,
        0xb7 as libc::c_int as uint8_t,
        0x90 as libc::c_int as uint8_t,
        0x8d as libc::c_int as uint8_t,
        0x36 as libc::c_int as uint8_t,
        0x2b as libc::c_int as uint8_t,
        0xc as libc::c_int as uint8_t,
        0x11 as libc::c_int as uint8_t,
        0x42 as libc::c_int as uint8_t,
        0x5f as libc::c_int as uint8_t,
        0x78 as libc::c_int as uint8_t,
        0x65 as libc::c_int as uint8_t,
        0x94 as libc::c_int as uint8_t,
        0x89 as libc::c_int as uint8_t,
        0xae as libc::c_int as uint8_t,
        0xb3 as libc::c_int as uint8_t,
        0xe0 as libc::c_int as uint8_t,
        0xfd as libc::c_int as uint8_t,
        0xda as libc::c_int as uint8_t,
        0xc7 as libc::c_int as uint8_t,
        0x7c as libc::c_int as uint8_t,
        0x61 as libc::c_int as uint8_t,
        0x46 as libc::c_int as uint8_t,
        0x5b as libc::c_int as uint8_t,
        0x8 as libc::c_int as uint8_t,
        0x15 as libc::c_int as uint8_t,
        0x32 as libc::c_int as uint8_t,
        0x2f as libc::c_int as uint8_t,
        0x59 as libc::c_int as uint8_t,
        0x44 as libc::c_int as uint8_t,
        0x63 as libc::c_int as uint8_t,
        0x7e as libc::c_int as uint8_t,
        0x2d as libc::c_int as uint8_t,
        0x30 as libc::c_int as uint8_t,
        0x17 as libc::c_int as uint8_t,
        0xa as libc::c_int as uint8_t,
        0xb1 as libc::c_int as uint8_t,
        0xac as libc::c_int as uint8_t,
        0x8b as libc::c_int as uint8_t,
        0x96 as libc::c_int as uint8_t,
        0xc5 as libc::c_int as uint8_t,
        0xd8 as libc::c_int as uint8_t,
        0xff as libc::c_int as uint8_t,
        0xe2 as libc::c_int as uint8_t,
        0x26 as libc::c_int as uint8_t,
        0x3b as libc::c_int as uint8_t,
        0x1c as libc::c_int as uint8_t,
        0x1 as libc::c_int as uint8_t,
        0x52 as libc::c_int as uint8_t,
        0x4f as libc::c_int as uint8_t,
        0x68 as libc::c_int as uint8_t,
        0x75 as libc::c_int as uint8_t,
        0xce as libc::c_int as uint8_t,
        0xd3 as libc::c_int as uint8_t,
        0xf4 as libc::c_int as uint8_t,
        0xe9 as libc::c_int as uint8_t,
        0xba as libc::c_int as uint8_t,
        0xa7 as libc::c_int as uint8_t,
        0x80 as libc::c_int as uint8_t,
        0x9d as libc::c_int as uint8_t,
        0xeb as libc::c_int as uint8_t,
        0xf6 as libc::c_int as uint8_t,
        0xd1 as libc::c_int as uint8_t,
        0xcc as libc::c_int as uint8_t,
        0x9f as libc::c_int as uint8_t,
        0x82 as libc::c_int as uint8_t,
        0xa5 as libc::c_int as uint8_t,
        0xb8 as libc::c_int as uint8_t,
        0x3 as libc::c_int as uint8_t,
        0x1e as libc::c_int as uint8_t,
        0x39 as libc::c_int as uint8_t,
        0x24 as libc::c_int as uint8_t,
        0x77 as libc::c_int as uint8_t,
        0x6a as libc::c_int as uint8_t,
        0x4d as libc::c_int as uint8_t,
        0x50 as libc::c_int as uint8_t,
        0xa1 as libc::c_int as uint8_t,
        0xbc as libc::c_int as uint8_t,
        0x9b as libc::c_int as uint8_t,
        0x86 as libc::c_int as uint8_t,
        0xd5 as libc::c_int as uint8_t,
        0xc8 as libc::c_int as uint8_t,
        0xef as libc::c_int as uint8_t,
        0xf2 as libc::c_int as uint8_t,
        0x49 as libc::c_int as uint8_t,
        0x54 as libc::c_int as uint8_t,
        0x73 as libc::c_int as uint8_t,
        0x6e as libc::c_int as uint8_t,
        0x3d as libc::c_int as uint8_t,
        0x20 as libc::c_int as uint8_t,
        0x7 as libc::c_int as uint8_t,
        0x1a as libc::c_int as uint8_t,
        0x6c as libc::c_int as uint8_t,
        0x71 as libc::c_int as uint8_t,
        0x56 as libc::c_int as uint8_t,
        0x4b as libc::c_int as uint8_t,
        0x18 as libc::c_int as uint8_t,
        0x5 as libc::c_int as uint8_t,
        0x22 as libc::c_int as uint8_t,
        0x3f as libc::c_int as uint8_t,
        0x84 as libc::c_int as uint8_t,
        0x99 as libc::c_int as uint8_t,
        0xbe as libc::c_int as uint8_t,
        0xa3 as libc::c_int as uint8_t,
        0xf0 as libc::c_int as uint8_t,
        0xed as libc::c_int as uint8_t,
        0xca as libc::c_int as uint8_t,
        0xd7 as libc::c_int as uint8_t,
        0x35 as libc::c_int as uint8_t,
        0x28 as libc::c_int as uint8_t,
        0xf as libc::c_int as uint8_t,
        0x12 as libc::c_int as uint8_t,
        0x41 as libc::c_int as uint8_t,
        0x5c as libc::c_int as uint8_t,
        0x7b as libc::c_int as uint8_t,
        0x66 as libc::c_int as uint8_t,
        0xdd as libc::c_int as uint8_t,
        0xc0 as libc::c_int as uint8_t,
        0xe7 as libc::c_int as uint8_t,
        0xfa as libc::c_int as uint8_t,
        0xa9 as libc::c_int as uint8_t,
        0xb4 as libc::c_int as uint8_t,
        0x93 as libc::c_int as uint8_t,
        0x8e as libc::c_int as uint8_t,
        0xf8 as libc::c_int as uint8_t,
        0xe5 as libc::c_int as uint8_t,
        0xc2 as libc::c_int as uint8_t,
        0xdf as libc::c_int as uint8_t,
        0x8c as libc::c_int as uint8_t,
        0x91 as libc::c_int as uint8_t,
        0xb6 as libc::c_int as uint8_t,
        0xab as libc::c_int as uint8_t,
        0x10 as libc::c_int as uint8_t,
        0xd as libc::c_int as uint8_t,
        0x2a as libc::c_int as uint8_t,
        0x37 as libc::c_int as uint8_t,
        0x64 as libc::c_int as uint8_t,
        0x79 as libc::c_int as uint8_t,
        0x5e as libc::c_int as uint8_t,
        0x43 as libc::c_int as uint8_t,
        0xb2 as libc::c_int as uint8_t,
        0xaf as libc::c_int as uint8_t,
        0x88 as libc::c_int as uint8_t,
        0x95 as libc::c_int as uint8_t,
        0xc6 as libc::c_int as uint8_t,
        0xdb as libc::c_int as uint8_t,
        0xfc as libc::c_int as uint8_t,
        0xe1 as libc::c_int as uint8_t,
        0x5a as libc::c_int as uint8_t,
        0x47 as libc::c_int as uint8_t,
        0x60 as libc::c_int as uint8_t,
        0x7d as libc::c_int as uint8_t,
        0x2e as libc::c_int as uint8_t,
        0x33 as libc::c_int as uint8_t,
        0x14 as libc::c_int as uint8_t,
        0x9 as libc::c_int as uint8_t,
        0x7f as libc::c_int as uint8_t,
        0x62 as libc::c_int as uint8_t,
        0x45 as libc::c_int as uint8_t,
        0x58 as libc::c_int as uint8_t,
        0xb as libc::c_int as uint8_t,
        0x16 as libc::c_int as uint8_t,
        0x31 as libc::c_int as uint8_t,
        0x2c as libc::c_int as uint8_t,
        0x97 as libc::c_int as uint8_t,
        0x8a as libc::c_int as uint8_t,
        0xad as libc::c_int as uint8_t,
        0xb0 as libc::c_int as uint8_t,
        0xe3 as libc::c_int as uint8_t,
        0xfe as libc::c_int as uint8_t,
        0xd9 as libc::c_int as uint8_t,
        0xc4 as libc::c_int as uint8_t,
    ];
    let mut nch: libc::c_int = 1 as libc::c_int
        + ((*frame).mode as libc::c_uint != SBC_MODE_MONO as libc::c_int as libc::c_uint)
        as libc::c_int;
    let mut nsb: libc::c_int = (*frame).nsubbands;
    let mut i: libc::c_uint = 0;
    let mut nbit: libc::c_uint = (nch * nsb * 4 as libc::c_int
        + (if (*frame).mode as libc::c_uint
        == SBC_MODE_JOINT_STEREO as libc::c_int as libc::c_uint
    {
        nsb
    } else {
        0 as libc::c_int
    })) as libc::c_uint;
    if size
        < ((4 as libc::c_int * 8 as libc::c_int) as libc::c_uint)
        .wrapping_add(nbit)
        .wrapping_add(7 as libc::c_int as libc::c_uint) >> 3 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    let mut crc: uint8_t = 0xf as libc::c_int as uint8_t;
    crc = t[(crc as libc::c_int ^ *data.offset(1 as libc::c_int as isize) as libc::c_int)
        as usize];
    crc = t[(crc as libc::c_int ^ *data.offset(2 as libc::c_int as isize) as libc::c_int)
        as usize];
    i = 4 as libc::c_int as libc::c_uint;
    while i
        < (4 as libc::c_int as libc::c_uint)
        .wrapping_add(nbit.wrapping_div(8 as libc::c_int as libc::c_uint))
    {
        crc = t[(crc as libc::c_int ^ *data.offset(i as isize) as libc::c_int) as usize];
        i = i.wrapping_add(1);
        i;
    }
    if nbit.wrapping_rem(8 as libc::c_int as libc::c_uint) != 0 {
        crc = ((crc as libc::c_int) << 4 as libc::c_int
            ^ t[(crc as libc::c_int >> 4 as libc::c_int
            ^ *data.offset(i as isize) as libc::c_int >> 4 as libc::c_int) as usize]
            as libc::c_int) as uint8_t;
    }
    return crc as libc::c_int;
}
unsafe extern "C" fn check_frame(mut frame: *const sbc_frame) -> bool {
    if ((*frame).nblocks - 4 as libc::c_int) as libc::c_uint
        > 12 as libc::c_int as libc::c_uint
        || !(*frame).msbc && (*frame).nblocks % 4 as libc::c_int != 0 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    if ((*frame).nsubbands - 4 as libc::c_int) as libc::c_uint
        > 4 as libc::c_int as libc::c_uint
        || (*frame).nsubbands % 4 as libc::c_int != 0 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    let mut two_channels: bool = (*frame).mode as libc::c_uint
        != SBC_MODE_MONO as libc::c_int as libc::c_uint;
    let mut dual_mode: bool = (*frame).mode as libc::c_uint
        == SBC_MODE_DUAL_CHANNEL as libc::c_int as libc::c_uint;
    let mut joint_mode: bool = (*frame).mode as libc::c_uint
        == SBC_MODE_JOINT_STEREO as libc::c_int as libc::c_uint;
    let mut stereo_mode: bool = joint_mode as libc::c_int != 0
        || (*frame).mode as libc::c_uint
        == SBC_MODE_STEREO as libc::c_int as libc::c_uint;
    let mut max_bits: libc::c_int = (16 as libc::c_int * (*frame).nsubbands
        * (*frame).nblocks << two_channels as libc::c_int)
        - 4 as libc::c_int * 8 as libc::c_int
        - (4 as libc::c_int * (*frame).nsubbands << two_channels as libc::c_int)
        - (if joint_mode as libc::c_int != 0 {
        (*frame).nsubbands
    } else {
        0 as libc::c_int
    });
    let mut max_bitpool: libc::c_int = if max_bits
        / ((*frame).nblocks << dual_mode as libc::c_int)
        < ((16 as libc::c_int) << stereo_mode as libc::c_int) * (*frame).nsubbands
    {
        max_bits / ((*frame).nblocks << dual_mode as libc::c_int)
    } else {
        ((16 as libc::c_int) << stereo_mode as libc::c_int) * (*frame).nsubbands
    };
    return (*frame).bitpool <= max_bitpool;
}
unsafe extern "C" fn compute_nbits(
    mut frame: *const sbc_frame,
    mut scale_factors: *const [libc::c_int; 8],
    mut nbits: *mut [libc::c_int; 8],
) {
    static mut loudness_offset_4: [[libc::c_int; 4]; 4] = [
        [-(1 as libc::c_int), 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int],
        [-(2 as libc::c_int), 0 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int],
        [-(2 as libc::c_int), 0 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int],
        [-(2 as libc::c_int), 0 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int],
    ];
    static mut loudness_offset_8: [[libc::c_int; 8]; 4] = [
        [
            -(2 as libc::c_int),
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
        ],
        [
            -(3 as libc::c_int),
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
            2 as libc::c_int,
        ],
        [
            -(4 as libc::c_int),
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
            2 as libc::c_int,
        ],
        [
            -(4 as libc::c_int),
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
            2 as libc::c_int,
        ],
    ];
    let mut loudness_offset: *const libc::c_int = if (*frame).nsubbands
        == 4 as libc::c_int
    {
        (loudness_offset_4[(*frame).freq as usize]).as_ptr()
    } else {
        (loudness_offset_8[(*frame).freq as usize]).as_ptr()
    };
    let mut stereo_mode: bool = (*frame).mode as libc::c_uint
        == SBC_MODE_STEREO as libc::c_int as libc::c_uint
        || (*frame).mode as libc::c_uint
        == SBC_MODE_JOINT_STEREO as libc::c_int as libc::c_uint;
    let mut nsubbands: libc::c_int = (*frame).nsubbands;
    let mut nchannels: libc::c_int = 1 as libc::c_int + stereo_mode as libc::c_int;
    let mut bitneeds: [[libc::c_int; 8]; 2] = [[0; 8]; 2];
    let mut max_bitneed: libc::c_int = 0 as libc::c_int;
    let mut ich: libc::c_int = 0 as libc::c_int;
    while ich < nchannels {
        let mut isb: libc::c_int = 0 as libc::c_int;
        while isb < nsubbands {
            let mut bitneed: libc::c_int = 0;
            let mut scf: libc::c_int = (*scale_factors
                .offset(ich as isize))[isb as usize];
            if (*frame).bam as libc::c_uint
                == SBC_BAM_LOUDNESS as libc::c_int as libc::c_uint
            {
                bitneed = if scf != 0 {
                    scf - *loudness_offset.offset(isb as isize)
                } else {
                    -(5 as libc::c_int)
                };
                bitneed >>= (bitneed > 0 as libc::c_int) as libc::c_int;
            } else {
                bitneed = scf;
            }
            if bitneed > max_bitneed {
                max_bitneed = bitneed;
            }
            bitneeds[ich as usize][isb as usize] = bitneed;
            isb += 1;
            isb;
        }
        ich += 1;
        ich;
    }
    let mut bitpool: libc::c_int = (*frame).bitpool;
    let mut bitcount: libc::c_int = 0 as libc::c_int;
    let mut bitslice: libc::c_int = max_bitneed + 1 as libc::c_int;
    let mut bc: libc::c_int = 0 as libc::c_int;
    while bc < bitpool {
        let fresh0 = bitslice;
        bitslice = bitslice - 1;
        let mut bs: libc::c_int = fresh0;
        bitcount = bc;
        if bitcount == bitpool {
            break;
        }
        let mut ich_0: libc::c_int = 0 as libc::c_int;
        while ich_0 < nchannels {
            let mut isb_0: libc::c_int = 0 as libc::c_int;
            while isb_0 < nsubbands {
                let mut bn: libc::c_int = bitneeds[ich_0 as usize][isb_0 as usize];
                bc
                    += (bn >= bs && bn < bs + 15 as libc::c_int) as libc::c_int
                    + (bn == bs) as libc::c_int;
                isb_0 += 1;
                isb_0;
            }
            ich_0 += 1;
            ich_0;
        }
    }
    let mut ich_1: libc::c_int = 0 as libc::c_int;
    while ich_1 < nchannels {
        let mut isb_1: libc::c_int = 0 as libc::c_int;
        while isb_1 < nsubbands {
            let mut nbit: libc::c_int = bitneeds[ich_1 as usize][isb_1 as usize]
                - bitslice;
            (*nbits
                .offset(
                    ich_1 as isize,
                ))[isb_1
                as usize] = if nbit < 2 as libc::c_int {
                0 as libc::c_int
            } else if nbit > 16 as libc::c_int {
                16 as libc::c_int
            } else {
                nbit
            };
            isb_1 += 1;
            isb_1;
        }
        ich_1 += 1;
        ich_1;
    }
    let mut isb_2: libc::c_int = 0 as libc::c_int;
    while isb_2 < nsubbands && bitcount < bitpool {
        let mut ich_2: libc::c_int = 0 as libc::c_int;
        while ich_2 < nchannels && bitcount < bitpool {
            let mut n: libc::c_int = if (*nbits.offset(ich_2 as isize))[isb_2 as usize]
                != 0
                && (*nbits.offset(ich_2 as isize))[isb_2 as usize] < 16 as libc::c_int
            {
                1 as libc::c_int
            } else if bitneeds[ich_2 as usize][isb_2 as usize]
                == bitslice + 1 as libc::c_int && bitpool > bitcount + 1 as libc::c_int
            {
                2 as libc::c_int
            } else {
                0 as libc::c_int
            };
            (*nbits.offset(ich_2 as isize))[isb_2 as usize] += n;
            bitcount += n;
            ich_2 += 1;
            ich_2;
        }
        isb_2 += 1;
        isb_2;
    }
    let mut isb_3: libc::c_int = 0 as libc::c_int;
    while isb_3 < nsubbands && bitcount < bitpool {
        let mut ich_3: libc::c_int = 0 as libc::c_int;
        while ich_3 < nchannels && bitcount < bitpool {
            let mut n_0: libc::c_int = ((*nbits.offset(ich_3 as isize))[isb_3 as usize]
                < 16 as libc::c_int) as libc::c_int;
            (*nbits.offset(ich_3 as isize))[isb_3 as usize] += n_0;
            bitcount += n_0;
            ich_3 += 1;
            ich_3;
        }
        isb_3 += 1;
        isb_3;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sbc_get_freq_hz(mut freq: sbc_freq) -> libc::c_int {
    static mut freq_hz: [libc::c_int; 4] = [
        16000 as libc::c_int,
        32000 as libc::c_int,
        44100 as libc::c_int,
        48000 as libc::c_int,
    ];
    return freq_hz[freq as usize];
}
#[no_mangle]
pub unsafe extern "C" fn sbc_get_frame_size(
    mut frame: *const sbc_frame,
) -> libc::c_uint {
    if !check_frame(frame) {
        return 0 as libc::c_int as libc::c_uint;
    }
    let mut two_channels: bool = (*frame).mode as libc::c_uint
        != SBC_MODE_MONO as libc::c_int as libc::c_uint;
    let mut dual_mode: bool = (*frame).mode as libc::c_uint
        == SBC_MODE_DUAL_CHANNEL as libc::c_int as libc::c_uint;
    let mut joint_mode: bool = (*frame).mode as libc::c_uint
        == SBC_MODE_JOINT_STEREO as libc::c_int as libc::c_uint;
    let mut nbits: libc::c_uint = ((4 as libc::c_int * (*frame).nsubbands
        << two_channels as libc::c_int)
        + ((*frame).nblocks * (*frame).bitpool << dual_mode as libc::c_int)
        + (if joint_mode as libc::c_int != 0 {
        (*frame).nsubbands
    } else {
        0 as libc::c_int
    })) as libc::c_uint;
    return (4 as libc::c_int as libc::c_uint)
        .wrapping_add(
            nbits.wrapping_add(7 as libc::c_int as libc::c_uint) >> 3 as libc::c_int,
        );
}
#[no_mangle]
pub unsafe extern "C" fn sbc_get_frame_bitrate(
    mut frame: *const sbc_frame,
) -> libc::c_uint {
    if !check_frame(frame) {
        return 0 as libc::c_int as libc::c_uint;
    }
    let mut nsamples: libc::c_uint = ((*frame).nblocks * (*frame).nsubbands)
        as libc::c_uint;
    let mut nbits: libc::c_uint = (8 as libc::c_int as libc::c_uint)
        .wrapping_mul(sbc_get_frame_size(frame));
    return nbits
        .wrapping_mul(sbc_get_freq_hz((*frame).freq) as libc::c_uint)
        .wrapping_div(nsamples);
}
#[no_mangle]
pub unsafe extern "C" fn sbc_get_frame_bps(mut freq: sbc_freq) -> libc::c_int {
    static mut freq_hz: [libc::c_int; 4] = [
        16000 as libc::c_int,
        32000 as libc::c_int,
        44100 as libc::c_int,
        48000 as libc::c_int,
    ];
    return freq_hz[freq as usize];
}
#[no_mangle]
pub unsafe extern "C" fn sbc_reset(mut sbc: *mut sbc) {
    *sbc = {
        let mut init = sbc {
            nchannels: 0,
            nblocks: 0,
            nsubbands: 0,
            c2rust_unnamed: C2RustUnnamed {
                dstates: [sbc_dstate {
                    idx: 0,
                    v: [[[0; 10]; 8]; 2],
                }; 2],
            },
        };
        init
    };
}
unsafe extern "C" fn decode_header(
    mut bits: *mut sbc_bits_t,
    mut frame: *mut sbc_frame,
    mut crc: *mut libc::c_int,
) -> bool {
    static mut dec_freq: [sbc_freq; 4] = [
        SBC_FREQ_16K,
        SBC_FREQ_32K,
        SBC_FREQ_44K1,
        SBC_FREQ_48K,
    ];
    static mut dec_mode: [sbc_mode; 4] = [
        SBC_MODE_MONO,
        SBC_MODE_DUAL_CHANNEL,
        SBC_MODE_STEREO,
        SBC_MODE_JOINT_STEREO,
    ];
    static mut dec_bam: [sbc_bam; 2] = [SBC_BAM_LOUDNESS, SBC_BAM_SNR];
    let mut __bits: *mut sbc_bits_t = bits;
    let mut syncword: libc::c_int = sbc_get_bits(
        __bits,
        8 as libc::c_int as libc::c_uint,
    ) as libc::c_int;
    (*frame).msbc = syncword == 0xad as libc::c_int;
    if (*frame).msbc {
        sbc_get_bits(__bits, 16 as libc::c_int as libc::c_uint);
        *frame = msbc_frame;
    } else if syncword == 0x9c as libc::c_int {
        (*frame)
            .freq = dec_freq[sbc_get_bits(__bits, 2 as libc::c_int as libc::c_uint)
            as usize];
        (*frame)
            .nblocks = ((1 as libc::c_int as libc::c_uint)
            .wrapping_add(sbc_get_bits(__bits, 2 as libc::c_int as libc::c_uint))
            << 2 as libc::c_int) as libc::c_int;
        (*frame)
            .mode = dec_mode[sbc_get_bits(__bits, 2 as libc::c_int as libc::c_uint)
            as usize];
        (*frame)
            .bam = dec_bam[sbc_get_bits(__bits, 1 as libc::c_int as libc::c_uint)
            as usize];
        (*frame)
            .nsubbands = ((1 as libc::c_int as libc::c_uint)
            .wrapping_add(sbc_get_bits(__bits, 1 as libc::c_int as libc::c_uint))
            << 2 as libc::c_int) as libc::c_int;
        (*frame)
            .bitpool = sbc_get_bits(__bits, 8 as libc::c_int as libc::c_uint)
            as libc::c_int;
    } else {
        return 0 as libc::c_int != 0
    }
    if !crc.is_null() {
        *crc = sbc_get_bits(__bits, 8 as libc::c_int as libc::c_uint) as libc::c_int;
    }
    return check_frame(frame);
}
unsafe extern "C" fn decode_frame(
    mut bits: *mut sbc_bits_t,
    mut frame: *const sbc_frame,
    mut sb_samples: *mut [int16_t; 128],
    mut sb_scale: *mut libc::c_int,
) {
    static mut range_scale: [libc::c_int; 16] = [
        0xfffffff as libc::c_int,
        0x5555556 as libc::c_int,
        0x2492492 as libc::c_int,
        0x1111111 as libc::c_int,
        0x842108 as libc::c_int,
        0x410410 as libc::c_int,
        0x204081 as libc::c_int,
        0x101010 as libc::c_int,
        0x80402 as libc::c_int,
        0x40100 as libc::c_int,
        0x20040 as libc::c_int,
        0x10010 as libc::c_int,
        0x8004 as libc::c_int,
        0x4001 as libc::c_int,
        0x2000 as libc::c_int,
        0x1000 as libc::c_int,
    ];
    let mut __bits: *mut sbc_bits_t = bits;
    let mut mjoint: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if (*frame).mode as libc::c_uint
        == SBC_MODE_JOINT_STEREO as libc::c_int as libc::c_uint
        && (*frame).nsubbands == 4 as libc::c_int
    {
        let mut v: libc::c_uint = sbc_get_bits(__bits, 4 as libc::c_int as libc::c_uint);
        mjoint = ((0 as libc::c_int) << 3 as libc::c_int) as libc::c_uint
            | (v & 0x2 as libc::c_int as libc::c_uint) << 1 as libc::c_int
            | (v & 0x4 as libc::c_int as libc::c_uint) >> 1 as libc::c_int
            | (v & 0x8 as libc::c_int as libc::c_uint) >> 3 as libc::c_int;
    } else if (*frame).mode as libc::c_uint
        == SBC_MODE_JOINT_STEREO as libc::c_int as libc::c_uint
    {
        let mut v_0: libc::c_uint = sbc_get_bits(
            __bits,
            8 as libc::c_int as libc::c_uint,
        );
        mjoint = ((0 as libc::c_int) << 7 as libc::c_int) as libc::c_uint
            | (v_0 & 0x2 as libc::c_int as libc::c_uint) << 5 as libc::c_int
            | (v_0 & 0x4 as libc::c_int as libc::c_uint) << 3 as libc::c_int
            | (v_0 & 0x8 as libc::c_int as libc::c_uint) << 1 as libc::c_int
            | (v_0 & 0x10 as libc::c_int as libc::c_uint) >> 1 as libc::c_int
            | (v_0 & 0x20 as libc::c_int as libc::c_uint) >> 3 as libc::c_int
            | (v_0 & 0x40 as libc::c_int as libc::c_uint) >> 5 as libc::c_int
            | (v_0 & 0x80 as libc::c_int as libc::c_uint) >> 7 as libc::c_int;
    }
    let mut nchannels: libc::c_int = 1 as libc::c_int
        + ((*frame).mode as libc::c_uint != SBC_MODE_MONO as libc::c_int as libc::c_uint)
        as libc::c_int;
    let mut nsubbands: libc::c_int = (*frame).nsubbands;
    let mut scale_factors: [[libc::c_int; 8]; 2] = [[0; 8]; 2];
    let mut nbits: [[libc::c_int; 8]; 2] = [[0; 8]; 2];
    let mut ich: libc::c_int = 0 as libc::c_int;
    while ich < nchannels {
        let mut isb: libc::c_int = 0 as libc::c_int;
        while isb < nsubbands {
            scale_factors[ich
                as usize][isb
                as usize] = sbc_get_bits(__bits, 4 as libc::c_int as libc::c_uint)
                as libc::c_int;
            isb += 1;
            isb;
        }
        ich += 1;
        ich;
    }
    compute_nbits(
        frame,
        scale_factors.as_mut_ptr() as *const [libc::c_int; 8],
        nbits.as_mut_ptr(),
    );
    if (*frame).mode as libc::c_uint
        == SBC_MODE_DUAL_CHANNEL as libc::c_int as libc::c_uint
    {
        compute_nbits(
            frame,
            scale_factors.as_mut_ptr().offset(1 as libc::c_int as isize)
                as *const [libc::c_int; 8],
            nbits.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
    }
    let mut ich_0: libc::c_int = 0 as libc::c_int;
    while ich_0 < nchannels {
        let mut max_scf: libc::c_int = 0 as libc::c_int;
        let mut isb_0: libc::c_int = 0 as libc::c_int;
        while isb_0 < nsubbands {
            let mut scf: libc::c_int = (scale_factors[ich_0 as usize][isb_0 as usize]
                as libc::c_uint)
                .wrapping_add(mjoint >> isb_0 & 1 as libc::c_int as libc::c_uint)
                as libc::c_int;
            if scf > max_scf {
                max_scf = scf;
            }
            isb_0 += 1;
            isb_0;
        }
        *sb_scale
            .offset(
                ich_0 as isize,
            ) = 15 as libc::c_int - max_scf - (17 as libc::c_int - 16 as libc::c_int);
        ich_0 += 1;
        ich_0;
    }
    if (*frame).mode as libc::c_uint
        == SBC_MODE_JOINT_STEREO as libc::c_int as libc::c_uint
    {
        let ref mut fresh1 = *sb_scale.offset(1 as libc::c_int as isize);
        *fresh1 = if *sb_scale.offset(0 as libc::c_int as isize)
            < *sb_scale.offset(1 as libc::c_int as isize)
        {
            *sb_scale.offset(0 as libc::c_int as isize)
        } else {
            *sb_scale.offset(1 as libc::c_int as isize)
        };
        *sb_scale.offset(0 as libc::c_int as isize) = *fresh1;
    }
    let mut iblk: libc::c_int = 0 as libc::c_int;
    while iblk < (*frame).nblocks {
        let mut ich_1: libc::c_int = 0 as libc::c_int;
        while ich_1 < nchannels {
            let mut p_sb_samples: *mut int16_t = (*sb_samples.offset(ich_1 as isize))
                .as_mut_ptr()
                .offset((iblk * nsubbands) as isize);
            let mut isb_1: libc::c_int = 0 as libc::c_int;
            while isb_1 < nsubbands {
                let mut nbit: libc::c_int = nbits[ich_1 as usize][isb_1 as usize];
                let mut scf_0: libc::c_int = scale_factors[ich_1
                    as usize][isb_1 as usize];
                if nbit == 0 {
                    let fresh2 = p_sb_samples;
                    p_sb_samples = p_sb_samples.offset(1);
                    *fresh2 = 0 as libc::c_int as int16_t;
                } else {
                    let mut s: libc::c_int = sbc_get_bits(__bits, nbit as libc::c_uint)
                        as libc::c_int;
                    s = (s << 1 as libc::c_int | 1 as libc::c_int)
                        * range_scale[(nbit - 1 as libc::c_int) as usize];
                    let fresh3 = p_sb_samples;
                    p_sb_samples = p_sb_samples.offset(1);
                    *fresh3 = (s - ((1 as libc::c_int) << 28 as libc::c_int)
                        >> 28 as libc::c_int
                        - (scf_0 + 1 as libc::c_int
                        + *sb_scale.offset(ich_1 as isize))) as int16_t;
                }
                isb_1 += 1;
                isb_1;
            }
            ich_1 += 1;
            ich_1;
        }
        iblk += 1;
        iblk;
    }
    let mut isb_2: libc::c_int = 0 as libc::c_int;
    while isb_2 < nsubbands {
        if !(mjoint >> isb_2 & 1 as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint)
        {
            let mut iblk_0: libc::c_int = 0 as libc::c_int;
            while iblk_0 < (*frame).nblocks {
                let mut s0: int16_t = (*sb_samples
                    .offset(
                        0 as libc::c_int as isize,
                    ))[(iblk_0 * nsubbands + isb_2) as usize];
                let mut s1: int16_t = (*sb_samples
                    .offset(
                        1 as libc::c_int as isize,
                    ))[(iblk_0 * nsubbands + isb_2) as usize];
                (*sb_samples
                    .offset(
                        0 as libc::c_int as isize,
                    ))[(iblk_0 * nsubbands + isb_2)
                    as usize] = (s0 as libc::c_int + s1 as libc::c_int) as int16_t;
                (*sb_samples
                    .offset(
                        1 as libc::c_int as isize,
                    ))[(iblk_0 * nsubbands + isb_2)
                    as usize] = (s0 as libc::c_int - s1 as libc::c_int) as int16_t;
                iblk_0 += 1;
                iblk_0;
            }
        }
        isb_2 += 1;
        isb_2;
    }
    let mut padding_nbits: libc::c_int = (8 as libc::c_int as libc::c_uint)
        .wrapping_sub(
            (sbc_tell_bits(bits)).wrapping_rem(8 as libc::c_int as libc::c_uint),
        ) as libc::c_int;
    if padding_nbits < 8 as libc::c_int {
        sbc_get_fixed(
            __bits,
            padding_nbits as libc::c_uint,
            0 as libc::c_int as libc::c_uint,
        );
    }
}
#[inline]
unsafe extern "C" fn dct4(
    mut in_0: *const int16_t,
    mut scale: libc::c_int,
    mut out0: *mut [int16_t; 10],
    mut out1: *mut [int16_t; 10],
    mut idx: libc::c_int,
) {
    static mut cos8: [int16_t; 4] = [
        8192 as libc::c_int as int16_t,
        7568 as libc::c_int as int16_t,
        5793 as libc::c_int as int16_t,
        3135 as libc::c_int as int16_t,
    ];
    let mut s03: int16_t = (*in_0.offset(0 as libc::c_int as isize) as libc::c_int
        + *in_0.offset(3 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int)
        as int16_t;
    let mut d03: int16_t = (*in_0.offset(0 as libc::c_int as isize) as libc::c_int
        - *in_0.offset(3 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int)
        as int16_t;
    let mut s12: int16_t = (*in_0.offset(1 as libc::c_int as isize) as libc::c_int
        + *in_0.offset(2 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int)
        as int16_t;
    let mut d12: int16_t = (*in_0.offset(1 as libc::c_int as isize) as libc::c_int
        - *in_0.offset(2 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int)
        as int16_t;
    let mut a0: libc::c_int = (s03 as libc::c_int - s12 as libc::c_int)
        * cos8[2 as libc::c_int as usize] as libc::c_int;
    let mut b1: libc::c_int = -(s03 as libc::c_int + s12 as libc::c_int)
        << 13 as libc::c_int;
    let mut a1: libc::c_int = d03 as libc::c_int
        * cos8[3 as libc::c_int as usize] as libc::c_int
        - d12 as libc::c_int * cos8[1 as libc::c_int as usize] as libc::c_int;
    let mut b0: libc::c_int = -(d03 as libc::c_int)
        * cos8[1 as libc::c_int as usize] as libc::c_int
        - d12 as libc::c_int * cos8[3 as libc::c_int as usize] as libc::c_int;
    let mut shr: libc::c_int = 12 as libc::c_int + scale;
    a0 = a0 + ((1 as libc::c_int) << shr - 1 as libc::c_int) >> shr;
    b0 = b0 + ((1 as libc::c_int) << shr - 1 as libc::c_int) >> shr;
    a1 = a1 + ((1 as libc::c_int) << shr - 1 as libc::c_int) >> shr;
    b1 = b1 + ((1 as libc::c_int) << shr - 1 as libc::c_int) >> shr;
    (*out0
        .offset(
            0 as libc::c_int as isize,
        ))[idx
        as usize] = (if a0 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if a0 < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        a0
    }) as int16_t;
    (*out0
        .offset(
            3 as libc::c_int as isize,
        ))[idx
        as usize] = (if -a1 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if -a1 < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        -a1
    }) as int16_t;
    (*out0
        .offset(
            1 as libc::c_int as isize,
        ))[idx
        as usize] = (if a1 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if a1 < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        a1
    }) as int16_t;
    (*out0
        .offset(
            2 as libc::c_int as isize,
        ))[idx
        as usize] = (if 0 as libc::c_int > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if (0 as libc::c_int) < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        0 as libc::c_int
    }) as int16_t;
    (*out1
        .offset(
            0 as libc::c_int as isize,
        ))[idx
        as usize] = (if -a0 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if -a0 < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        -a0
    }) as int16_t;
    (*out1
        .offset(
            3 as libc::c_int as isize,
        ))[idx
        as usize] = (if b0 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if b0 < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        b0
    }) as int16_t;
    (*out1
        .offset(
            1 as libc::c_int as isize,
        ))[idx
        as usize] = (if b0 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if b0 < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        b0
    }) as int16_t;
    (*out1
        .offset(
            2 as libc::c_int as isize,
        ))[idx
        as usize] = (if b1 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if b1 < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        b1
    }) as int16_t;
}
#[inline]
unsafe extern "C" fn dct8(
    mut in_0: *const int16_t,
    mut scale: libc::c_int,
    mut out0: *mut [int16_t; 10],
    mut out1: *mut [int16_t; 10],
    mut idx: libc::c_int,
) {
    static mut cos16: [int16_t; 8] = [
        8192 as libc::c_int as int16_t,
        8035 as libc::c_int as int16_t,
        7568 as libc::c_int as int16_t,
        6811 as libc::c_int as int16_t,
        5793 as libc::c_int as int16_t,
        4551 as libc::c_int as int16_t,
        3135 as libc::c_int as int16_t,
        1598 as libc::c_int as int16_t,
    ];
    let mut s07: int16_t = (*in_0.offset(0 as libc::c_int as isize) as libc::c_int
        + *in_0.offset(7 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int)
        as int16_t;
    let mut d07: int16_t = (*in_0.offset(0 as libc::c_int as isize) as libc::c_int
        - *in_0.offset(7 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int)
        as int16_t;
    let mut s16: int16_t = (*in_0.offset(1 as libc::c_int as isize) as libc::c_int
        + *in_0.offset(6 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int)
        as int16_t;
    let mut d16: int16_t = (*in_0.offset(1 as libc::c_int as isize) as libc::c_int
        - *in_0.offset(6 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int)
        as int16_t;
    let mut s25: int16_t = (*in_0.offset(2 as libc::c_int as isize) as libc::c_int
        + *in_0.offset(5 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int)
        as int16_t;
    let mut d25: int16_t = (*in_0.offset(2 as libc::c_int as isize) as libc::c_int
        - *in_0.offset(5 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int)
        as int16_t;
    let mut s34: int16_t = (*in_0.offset(3 as libc::c_int as isize) as libc::c_int
        + *in_0.offset(4 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int)
        as int16_t;
    let mut d34: int16_t = (*in_0.offset(3 as libc::c_int as isize) as libc::c_int
        - *in_0.offset(4 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int)
        as int16_t;
    let mut a0: libc::c_int = (s07 as libc::c_int + s34 as libc::c_int
        - (s25 as libc::c_int + s16 as libc::c_int))
        * cos16[4 as libc::c_int as usize] as libc::c_int;
    let mut b3: libc::c_int = -(s07 as libc::c_int + s34 as libc::c_int)
        - (s25 as libc::c_int + s16 as libc::c_int) << 13 as libc::c_int;
    let mut a2: libc::c_int = (s07 as libc::c_int - s34 as libc::c_int)
        * cos16[6 as libc::c_int as usize] as libc::c_int
        + (s25 as libc::c_int - s16 as libc::c_int)
        * cos16[2 as libc::c_int as usize] as libc::c_int;
    let mut b1: libc::c_int = (s34 as libc::c_int - s07 as libc::c_int)
        * cos16[2 as libc::c_int as usize] as libc::c_int
        + (s25 as libc::c_int - s16 as libc::c_int)
        * cos16[6 as libc::c_int as usize] as libc::c_int;
    let mut a1: libc::c_int = d07 as libc::c_int
        * cos16[5 as libc::c_int as usize] as libc::c_int
        - d16 as libc::c_int * cos16[1 as libc::c_int as usize] as libc::c_int
        + d25 as libc::c_int * cos16[7 as libc::c_int as usize] as libc::c_int
        + d34 as libc::c_int * cos16[3 as libc::c_int as usize] as libc::c_int;
    let mut b2: libc::c_int = -(d07 as libc::c_int)
        * cos16[1 as libc::c_int as usize] as libc::c_int
        - d16 as libc::c_int * cos16[3 as libc::c_int as usize] as libc::c_int
        - d25 as libc::c_int * cos16[5 as libc::c_int as usize] as libc::c_int
        - d34 as libc::c_int * cos16[7 as libc::c_int as usize] as libc::c_int;
    let mut a3: libc::c_int = d07 as libc::c_int
        * cos16[7 as libc::c_int as usize] as libc::c_int
        - d16 as libc::c_int * cos16[5 as libc::c_int as usize] as libc::c_int
        + d25 as libc::c_int * cos16[3 as libc::c_int as usize] as libc::c_int
        - d34 as libc::c_int * cos16[1 as libc::c_int as usize] as libc::c_int;
    let mut b0: libc::c_int = -(d07 as libc::c_int)
        * cos16[3 as libc::c_int as usize] as libc::c_int
        + d16 as libc::c_int * cos16[7 as libc::c_int as usize] as libc::c_int
        + d25 as libc::c_int * cos16[1 as libc::c_int as usize] as libc::c_int
        + d34 as libc::c_int * cos16[5 as libc::c_int as usize] as libc::c_int;
    let mut shr: libc::c_int = 12 as libc::c_int + scale;
    a0 = a0 + ((1 as libc::c_int) << shr - 1 as libc::c_int) >> shr;
    b0 = b0 + ((1 as libc::c_int) << shr - 1 as libc::c_int) >> shr;
    a1 = a1 + ((1 as libc::c_int) << shr - 1 as libc::c_int) >> shr;
    b1 = b1 + ((1 as libc::c_int) << shr - 1 as libc::c_int) >> shr;
    a2 = a2 + ((1 as libc::c_int) << shr - 1 as libc::c_int) >> shr;
    b2 = b2 + ((1 as libc::c_int) << shr - 1 as libc::c_int) >> shr;
    a3 = a3 + ((1 as libc::c_int) << shr - 1 as libc::c_int) >> shr;
    b3 = b3 + ((1 as libc::c_int) << shr - 1 as libc::c_int) >> shr;
    (*out0
        .offset(
            0 as libc::c_int as isize,
        ))[idx
        as usize] = (if a0 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if a0 < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        a0
    }) as int16_t;
    (*out0
        .offset(
            7 as libc::c_int as isize,
        ))[idx
        as usize] = (if -a1 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if -a1 < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        -a1
    }) as int16_t;
    (*out0
        .offset(
            1 as libc::c_int as isize,
        ))[idx
        as usize] = (if a1 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if a1 < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        a1
    }) as int16_t;
    (*out0
        .offset(
            6 as libc::c_int as isize,
        ))[idx
        as usize] = (if -a2 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if -a2 < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        -a2
    }) as int16_t;
    (*out0
        .offset(
            2 as libc::c_int as isize,
        ))[idx
        as usize] = (if a2 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if a2 < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        a2
    }) as int16_t;
    (*out0
        .offset(
            5 as libc::c_int as isize,
        ))[idx
        as usize] = (if -a3 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if -a3 < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        -a3
    }) as int16_t;
    (*out0
        .offset(
            3 as libc::c_int as isize,
        ))[idx
        as usize] = (if a3 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if a3 < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        a3
    }) as int16_t;
    (*out0
        .offset(
            4 as libc::c_int as isize,
        ))[idx
        as usize] = (if 0 as libc::c_int > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if (0 as libc::c_int) < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        0 as libc::c_int
    }) as int16_t;
    (*out1
        .offset(
            0 as libc::c_int as isize,
        ))[idx
        as usize] = (if -a0 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if -a0 < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        -a0
    }) as int16_t;
    (*out1
        .offset(
            7 as libc::c_int as isize,
        ))[idx
        as usize] = (if b0 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if b0 < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        b0
    }) as int16_t;
    (*out1
        .offset(
            1 as libc::c_int as isize,
        ))[idx
        as usize] = (if b0 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if b0 < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        b0
    }) as int16_t;
    (*out1
        .offset(
            6 as libc::c_int as isize,
        ))[idx
        as usize] = (if b1 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if b1 < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        b1
    }) as int16_t;
    (*out1
        .offset(
            2 as libc::c_int as isize,
        ))[idx
        as usize] = (if b1 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if b1 < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        b1
    }) as int16_t;
    (*out1
        .offset(
            5 as libc::c_int as isize,
        ))[idx
        as usize] = (if b2 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if b2 < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        b2
    }) as int16_t;
    (*out1
        .offset(
            3 as libc::c_int as isize,
        ))[idx
        as usize] = (if b2 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if b2 < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        b2
    }) as int16_t;
    (*out1
        .offset(
            4 as libc::c_int as isize,
        ))[idx
        as usize] = (if b3 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if b3 < -(32767 as libc::c_int) - 1 as libc::c_int {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        b3
    }) as int16_t;
}
#[inline]
unsafe extern "C" fn apply_window(
    mut in_0: *const [int16_t; 10],
    mut n: libc::c_int,
    mut window: *const [int16_t; 20],
    mut offset: libc::c_int,
    mut out: *mut int16_t,
    mut pitch: libc::c_int,
) {
    let mut u: *const int16_t = in_0 as *const int16_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let mut w: *const int16_t = (*window.offset(i as isize))
            .as_ptr()
            .offset(offset as isize);
        let mut s: libc::c_int = 0;
        let fresh4 = u;
        u = u.offset(1);
        let fresh5 = w;
        w = w.offset(1);
        s = *fresh4 as libc::c_int * *fresh5 as libc::c_int;
        let fresh6 = u;
        u = u.offset(1);
        let fresh7 = w;
        w = w.offset(1);
        s += *fresh6 as libc::c_int * *fresh7 as libc::c_int;
        let fresh8 = u;
        u = u.offset(1);
        let fresh9 = w;
        w = w.offset(1);
        s += *fresh8 as libc::c_int * *fresh9 as libc::c_int;
        let fresh10 = u;
        u = u.offset(1);
        let fresh11 = w;
        w = w.offset(1);
        s += *fresh10 as libc::c_int * *fresh11 as libc::c_int;
        let fresh12 = u;
        u = u.offset(1);
        let fresh13 = w;
        w = w.offset(1);
        s += *fresh12 as libc::c_int * *fresh13 as libc::c_int;
        let fresh14 = u;
        u = u.offset(1);
        let fresh15 = w;
        w = w.offset(1);
        s += *fresh14 as libc::c_int * *fresh15 as libc::c_int;
        let fresh16 = u;
        u = u.offset(1);
        let fresh17 = w;
        w = w.offset(1);
        s += *fresh16 as libc::c_int * *fresh17 as libc::c_int;
        let fresh18 = u;
        u = u.offset(1);
        let fresh19 = w;
        w = w.offset(1);
        s += *fresh18 as libc::c_int * *fresh19 as libc::c_int;
        let fresh20 = u;
        u = u.offset(1);
        let fresh21 = w;
        w = w.offset(1);
        s += *fresh20 as libc::c_int * *fresh21 as libc::c_int;
        let fresh22 = u;
        u = u.offset(1);
        let fresh23 = w;
        w = w.offset(1);
        s += *fresh22 as libc::c_int * *fresh23 as libc::c_int;
        *out = (if s + ((1 as libc::c_int) << 12 as libc::c_int) >> 13 as libc::c_int
            > 32767 as libc::c_int
        {
            32767 as libc::c_int
        } else if (s + ((1 as libc::c_int) << 12 as libc::c_int) >> 13 as libc::c_int)
            < -(32767 as libc::c_int) - 1 as libc::c_int
        {
            -(32767 as libc::c_int) - 1 as libc::c_int
        } else {
            s + ((1 as libc::c_int) << 12 as libc::c_int) >> 13 as libc::c_int
        }) as int16_t;
        out = out.offset(pitch as isize);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sbc_synthesize_4_c(
    mut state: *mut sbc_dstate,
    mut in_0: *const int16_t,
    mut scale: libc::c_int,
    mut out: *mut int16_t,
    mut pitch: libc::c_int,
) {
    static mut window: [[int16_t; 20]; 4] = [
        [
            0 as libc::c_int as int16_t,
            -(126 as libc::c_int) as int16_t,
            -(358 as libc::c_int) as int16_t,
            -(848 as libc::c_int) as int16_t,
            -(4443 as libc::c_int) as int16_t,
            -(9644 as libc::c_int) as int16_t,
            4443 as libc::c_int as int16_t,
            -(848 as libc::c_int) as int16_t,
            358 as libc::c_int as int16_t,
            -(126 as libc::c_int) as int16_t,
            0 as libc::c_int as int16_t,
            -(126 as libc::c_int) as int16_t,
            -(358 as libc::c_int) as int16_t,
            -(848 as libc::c_int) as int16_t,
            -(4443 as libc::c_int) as int16_t,
            -(9644 as libc::c_int) as int16_t,
            4443 as libc::c_int as int16_t,
            -(848 as libc::c_int) as int16_t,
            358 as libc::c_int as int16_t,
            -(126 as libc::c_int) as int16_t,
        ],
        [
            -(18 as libc::c_int) as int16_t,
            -(128 as libc::c_int) as int16_t,
            -(670 as libc::c_int) as int16_t,
            -(201 as libc::c_int) as int16_t,
            -(6389 as libc::c_int) as int16_t,
            -(9235 as libc::c_int) as int16_t,
            2544 as libc::c_int as int16_t,
            -(1055 as libc::c_int) as int16_t,
            100 as libc::c_int as int16_t,
            -(90 as libc::c_int) as int16_t,
            -(18 as libc::c_int) as int16_t,
            -(128 as libc::c_int) as int16_t,
            -(670 as libc::c_int) as int16_t,
            -(201 as libc::c_int) as int16_t,
            -(6389 as libc::c_int) as int16_t,
            -(9235 as libc::c_int) as int16_t,
            2544 as libc::c_int as int16_t,
            -(1055 as libc::c_int) as int16_t,
            100 as libc::c_int as int16_t,
            -(90 as libc::c_int) as int16_t,
        ],
        [
            -(49 as libc::c_int) as int16_t,
            -(61 as libc::c_int) as int16_t,
            -(946 as libc::c_int) as int16_t,
            944 as libc::c_int as int16_t,
            -(8082 as libc::c_int) as int16_t,
            -(8082 as libc::c_int) as int16_t,
            944 as libc::c_int as int16_t,
            -(946 as libc::c_int) as int16_t,
            -(61 as libc::c_int) as int16_t,
            -(49 as libc::c_int) as int16_t,
            -(49 as libc::c_int) as int16_t,
            -(61 as libc::c_int) as int16_t,
            -(946 as libc::c_int) as int16_t,
            944 as libc::c_int as int16_t,
            -(8082 as libc::c_int) as int16_t,
            -(8082 as libc::c_int) as int16_t,
            944 as libc::c_int as int16_t,
            -(946 as libc::c_int) as int16_t,
            -(61 as libc::c_int) as int16_t,
            -(49 as libc::c_int) as int16_t,
        ],
        [
            -(90 as libc::c_int) as int16_t,
            100 as libc::c_int as int16_t,
            -(1055 as libc::c_int) as int16_t,
            2544 as libc::c_int as int16_t,
            -(9235 as libc::c_int) as int16_t,
            -(6389 as libc::c_int) as int16_t,
            -(201 as libc::c_int) as int16_t,
            -(670 as libc::c_int) as int16_t,
            -(128 as libc::c_int) as int16_t,
            -(18 as libc::c_int) as int16_t,
            -(90 as libc::c_int) as int16_t,
            100 as libc::c_int as int16_t,
            -(1055 as libc::c_int) as int16_t,
            2544 as libc::c_int as int16_t,
            -(9235 as libc::c_int) as int16_t,
            -(6389 as libc::c_int) as int16_t,
            -(201 as libc::c_int) as int16_t,
            -(670 as libc::c_int) as int16_t,
            -(128 as libc::c_int) as int16_t,
            -(18 as libc::c_int) as int16_t,
        ],
    ];
    let mut dct_idx: libc::c_int = if (*state).idx != 0 {
        10 as libc::c_int - (*state).idx
    } else {
        0 as libc::c_int
    };
    let mut odd: libc::c_int = dct_idx & 1 as libc::c_int;
    dct4(
        in_0,
        scale,
        ((*state).v[odd as usize]).as_mut_ptr(),
        ((*state).v[(odd == 0) as libc::c_int as usize]).as_mut_ptr(),
        dct_idx,
    );
    apply_window(
        ((*state).v[odd as usize]).as_mut_ptr() as *const [int16_t; 10],
        4 as libc::c_int,
        window.as_ptr(),
        (*state).idx,
        out,
        pitch,
    );
    (*state)
        .idx = if (*state).idx < 9 as libc::c_int {
        (*state).idx + 1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn sbc_synthesize_8_c(
    mut state: *mut sbc_dstate,
    mut in_0: *const int16_t,
    mut scale: libc::c_int,
    mut out: *mut int16_t,
    mut pitch: libc::c_int,
) {
    static mut window: [[int16_t; 20]; 8] = [
        [
            0 as libc::c_int as int16_t,
            -(132 as libc::c_int) as int16_t,
            -(371 as libc::c_int) as int16_t,
            -(848 as libc::c_int) as int16_t,
            -(4456 as libc::c_int) as int16_t,
            -(9631 as libc::c_int) as int16_t,
            4456 as libc::c_int as int16_t,
            -(848 as libc::c_int) as int16_t,
            371 as libc::c_int as int16_t,
            -(132 as libc::c_int) as int16_t,
            0 as libc::c_int as int16_t,
            -(132 as libc::c_int) as int16_t,
            -(371 as libc::c_int) as int16_t,
            -(848 as libc::c_int) as int16_t,
            -(4456 as libc::c_int) as int16_t,
            -(9631 as libc::c_int) as int16_t,
            4456 as libc::c_int as int16_t,
            -(848 as libc::c_int) as int16_t,
            371 as libc::c_int as int16_t,
            -(132 as libc::c_int) as int16_t,
        ],
        [
            -(10 as libc::c_int) as int16_t,
            -(138 as libc::c_int) as int16_t,
            -(526 as libc::c_int) as int16_t,
            -(580 as libc::c_int) as int16_t,
            -(5438 as libc::c_int) as int16_t,
            -(9528 as libc::c_int) as int16_t,
            3486 as libc::c_int as int16_t,
            -(1004 as libc::c_int) as int16_t,
            229 as libc::c_int as int16_t,
            -(117 as libc::c_int) as int16_t,
            -(10 as libc::c_int) as int16_t,
            -(138 as libc::c_int) as int16_t,
            -(526 as libc::c_int) as int16_t,
            -(580 as libc::c_int) as int16_t,
            -(5438 as libc::c_int) as int16_t,
            -(9528 as libc::c_int) as int16_t,
            3486 as libc::c_int as int16_t,
            -(1004 as libc::c_int) as int16_t,
            229 as libc::c_int as int16_t,
            -(117 as libc::c_int) as int16_t,
        ],
        [
            -(22 as libc::c_int) as int16_t,
            -(131 as libc::c_int) as int16_t,
            -(685 as libc::c_int) as int16_t,
            -(192 as libc::c_int) as int16_t,
            -(6395 as libc::c_int) as int16_t,
            -(9224 as libc::c_int) as int16_t,
            2561 as libc::c_int as int16_t,
            -(1063 as libc::c_int) as int16_t,
            108 as libc::c_int as int16_t,
            -(97 as libc::c_int) as int16_t,
            -(22 as libc::c_int) as int16_t,
            -(131 as libc::c_int) as int16_t,
            -(685 as libc::c_int) as int16_t,
            -(192 as libc::c_int) as int16_t,
            -(6395 as libc::c_int) as int16_t,
            -(9224 as libc::c_int) as int16_t,
            2561 as libc::c_int as int16_t,
            -(1063 as libc::c_int) as int16_t,
            108 as libc::c_int as int16_t,
            -(97 as libc::c_int) as int16_t,
        ],
        [
            -(36 as libc::c_int) as int16_t,
            -(106 as libc::c_int) as int16_t,
            -(835 as libc::c_int) as int16_t,
            322 as libc::c_int as int16_t,
            -(7287 as libc::c_int) as int16_t,
            -(8734 as libc::c_int) as int16_t,
            1711 as libc::c_int as int16_t,
            -(1042 as libc::c_int) as int16_t,
            12 as libc::c_int as int16_t,
            -(75 as libc::c_int) as int16_t,
            -(36 as libc::c_int) as int16_t,
            -(106 as libc::c_int) as int16_t,
            -(835 as libc::c_int) as int16_t,
            322 as libc::c_int as int16_t,
            -(7287 as libc::c_int) as int16_t,
            -(8734 as libc::c_int) as int16_t,
            1711 as libc::c_int as int16_t,
            -(1042 as libc::c_int) as int16_t,
            12 as libc::c_int as int16_t,
            -(75 as libc::c_int) as int16_t,
        ],
        [
            -(54 as libc::c_int) as int16_t,
            -(59 as libc::c_int) as int16_t,
            -(960 as libc::c_int) as int16_t,
            959 as libc::c_int as int16_t,
            -(8078 as libc::c_int) as int16_t,
            -(8078 as libc::c_int) as int16_t,
            959 as libc::c_int as int16_t,
            -(960 as libc::c_int) as int16_t,
            -(59 as libc::c_int) as int16_t,
            -(54 as libc::c_int) as int16_t,
            -(54 as libc::c_int) as int16_t,
            -(59 as libc::c_int) as int16_t,
            -(960 as libc::c_int) as int16_t,
            959 as libc::c_int as int16_t,
            -(8078 as libc::c_int) as int16_t,
            -(8078 as libc::c_int) as int16_t,
            959 as libc::c_int as int16_t,
            -(960 as libc::c_int) as int16_t,
            -(59 as libc::c_int) as int16_t,
            -(54 as libc::c_int) as int16_t,
        ],
        [
            -(75 as libc::c_int) as int16_t,
            12 as libc::c_int as int16_t,
            -(1042 as libc::c_int) as int16_t,
            1711 as libc::c_int as int16_t,
            -(8734 as libc::c_int) as int16_t,
            -(7287 as libc::c_int) as int16_t,
            322 as libc::c_int as int16_t,
            -(835 as libc::c_int) as int16_t,
            -(106 as libc::c_int) as int16_t,
            -(36 as libc::c_int) as int16_t,
            -(75 as libc::c_int) as int16_t,
            12 as libc::c_int as int16_t,
            -(1042 as libc::c_int) as int16_t,
            1711 as libc::c_int as int16_t,
            -(8734 as libc::c_int) as int16_t,
            -(7287 as libc::c_int) as int16_t,
            322 as libc::c_int as int16_t,
            -(835 as libc::c_int) as int16_t,
            -(106 as libc::c_int) as int16_t,
            -(36 as libc::c_int) as int16_t,
        ],
        [
            -(97 as libc::c_int) as int16_t,
            108 as libc::c_int as int16_t,
            -(1063 as libc::c_int) as int16_t,
            2561 as libc::c_int as int16_t,
            -(9224 as libc::c_int) as int16_t,
            -(6395 as libc::c_int) as int16_t,
            -(192 as libc::c_int) as int16_t,
            -(685 as libc::c_int) as int16_t,
            -(131 as libc::c_int) as int16_t,
            -(22 as libc::c_int) as int16_t,
            -(97 as libc::c_int) as int16_t,
            108 as libc::c_int as int16_t,
            -(1063 as libc::c_int) as int16_t,
            2561 as libc::c_int as int16_t,
            -(9224 as libc::c_int) as int16_t,
            -(6395 as libc::c_int) as int16_t,
            -(192 as libc::c_int) as int16_t,
            -(685 as libc::c_int) as int16_t,
            -(131 as libc::c_int) as int16_t,
            -(22 as libc::c_int) as int16_t,
        ],
        [
            -(117 as libc::c_int) as int16_t,
            229 as libc::c_int as int16_t,
            -(1004 as libc::c_int) as int16_t,
            3486 as libc::c_int as int16_t,
            -(9528 as libc::c_int) as int16_t,
            -(5438 as libc::c_int) as int16_t,
            -(580 as libc::c_int) as int16_t,
            -(526 as libc::c_int) as int16_t,
            -(138 as libc::c_int) as int16_t,
            -(10 as libc::c_int) as int16_t,
            -(117 as libc::c_int) as int16_t,
            229 as libc::c_int as int16_t,
            -(1004 as libc::c_int) as int16_t,
            3486 as libc::c_int as int16_t,
            -(9528 as libc::c_int) as int16_t,
            -(5438 as libc::c_int) as int16_t,
            -(580 as libc::c_int) as int16_t,
            -(526 as libc::c_int) as int16_t,
            -(138 as libc::c_int) as int16_t,
            -(10 as libc::c_int) as int16_t,
        ],
    ];
    let mut dct_idx: libc::c_int = if (*state).idx != 0 {
        10 as libc::c_int - (*state).idx
    } else {
        0 as libc::c_int
    };
    let mut odd: libc::c_int = dct_idx & 1 as libc::c_int;
    dct8(
        in_0,
        scale,
        ((*state).v[odd as usize]).as_mut_ptr(),
        ((*state).v[(odd == 0) as libc::c_int as usize]).as_mut_ptr(),
        dct_idx,
    );
    apply_window(
        ((*state).v[odd as usize]).as_mut_ptr() as *const [int16_t; 10],
        8 as libc::c_int,
        window.as_ptr(),
        (*state).idx,
        out,
        pitch,
    );
    (*state)
        .idx = if (*state).idx < 9 as libc::c_int {
        (*state).idx + 1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn synthesize(
    mut state: *mut sbc_dstate,
    mut nblocks: libc::c_int,
    mut nsubbands: libc::c_int,
    mut in_0: *const int16_t,
    mut scale: libc::c_int,
    mut out: *mut int16_t,
    mut pitch: libc::c_int,
) {
    let mut iblk: libc::c_int = 0 as libc::c_int;
    while iblk < nblocks {
        if nsubbands == 4 as libc::c_int {
            sbc_synthesize_4_c(state, in_0, scale, out, pitch);
        } else {
            sbc_synthesize_8_c(state, in_0, scale, out, pitch);
        }
        in_0 = in_0.offset(nsubbands as isize);
        out = out.offset((nsubbands * pitch) as isize);
        iblk += 1;
        iblk;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sbc_probe(
    mut data: *const libc::c_void,
    mut frame: *mut sbc_frame,
) -> libc::c_int {
    let mut bits: sbc_bits_t = sbc_bits_t {
        mode: SBC_BITS_READ,
        data: C2RustUnnamed_1 {
            p: 0 as *mut uint8_t,
            nbytes: 0,
            nleft: 0,
        },
        accu: C2RustUnnamed_0 {
            v: 0,
            nleft: 0,
            nover: 0,
        },
        error: false,
    };
    sbc_setup_bits(
        &mut bits,
        SBC_BITS_READ,
        data as *mut libc::c_void,
        4 as libc::c_int as libc::c_uint,
    );
    return if !decode_header(&mut bits, frame, 0 as *mut libc::c_int)
        || sbc_bits_error(&mut bits) as libc::c_int != 0
    {
        -(1 as libc::c_int)
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn sbc_decode(
    mut sbc: *mut sbc,
    mut data: *const libc::c_void,
    mut size: libc::c_uint,
    mut frame: *mut sbc_frame,
    mut pcml: *mut int16_t,
    mut pitchl: libc::c_int,
    mut pcmr: *mut int16_t,
    mut pitchr: libc::c_int,
) -> libc::c_int {
    let mut bits: sbc_bits_t = sbc_bits_t {
        mode: SBC_BITS_READ,
        data: C2RustUnnamed_1 {
            p: 0 as *mut uint8_t,
            nbytes: 0,
            nleft: 0,
        },
        accu: C2RustUnnamed_0 {
            v: 0,
            nleft: 0,
            nover: 0,
        },
        error: false,
    };
    let mut crc: libc::c_int = 0;
    if !data.is_null() {
        if size < 4 as libc::c_int as libc::c_uint {
            return -(1 as libc::c_int);
        }
        sbc_setup_bits(
            &mut bits,
            SBC_BITS_READ,
            data as *mut libc::c_void,
            4 as libc::c_int as libc::c_uint,
        );
        if !decode_header(&mut bits, frame, &mut crc)
            || sbc_bits_error(&mut bits) as libc::c_int != 0
        {
            return -(1 as libc::c_int);
        }
        if size < sbc_get_frame_size(frame)
            || compute_crc(frame, data as *const uint8_t, size) != crc
        {
            return -(1 as libc::c_int);
        }
    }
    let mut sb_samples: [[int16_t; 128]; 2] = [[0; 128]; 2];
    let mut sb_scale: [libc::c_int; 2] = [0; 2];
    if !data.is_null() {
        sbc_setup_bits(
            &mut bits,
            SBC_BITS_READ,
            (data as uintptr_t).wrapping_add(4 as libc::c_int as libc::c_ulong)
                as *mut libc::c_void,
            (sbc_get_frame_size(frame)).wrapping_sub(4 as libc::c_int as libc::c_uint),
        );
        decode_frame(&mut bits, frame, sb_samples.as_mut_ptr(), sb_scale.as_mut_ptr());
        (*sbc)
            .nchannels = 1 as libc::c_int
            + ((*frame).mode as libc::c_uint
            != SBC_MODE_MONO as libc::c_int as libc::c_uint) as libc::c_int;
        (*sbc).nblocks = (*frame).nblocks;
        (*sbc).nsubbands = (*frame).nsubbands;
    } else {
        let mut nsamples: libc::c_int = (*sbc).nblocks * (*sbc).nsubbands;
        let mut ich: libc::c_int = 0 as libc::c_int;
        while ich < (*sbc).nchannels {
            memset(
                (sb_samples[ich as usize]).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                (nsamples as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
            );
            sb_scale[ich as usize] = 0 as libc::c_int;
            ich += 1;
            ich;
        }
    }
    synthesize(
        &mut *((*sbc).c2rust_unnamed.dstates)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        (*sbc).nblocks,
        (*sbc).nsubbands,
        (sb_samples[0 as libc::c_int as usize]).as_mut_ptr(),
        sb_scale[0 as libc::c_int as usize],
        pcml,
        pitchl,
    );
    if (*frame).mode as libc::c_uint != SBC_MODE_MONO as libc::c_int as libc::c_uint {
        synthesize(
            &mut *((*sbc).c2rust_unnamed.dstates)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize),
            (*sbc).nblocks,
            (*sbc).nsubbands,
            (sb_samples[1 as libc::c_int as usize]).as_mut_ptr(),
            sb_scale[1 as libc::c_int as usize],
            pcmr,
            pitchr,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn compute_scale_factors_js(
    mut frame: *const sbc_frame,
    mut sb_samples: *const [int16_t; 128],
    mut scale_factors: *mut [libc::c_int; 8],
    mut mjoint: *mut libc::c_uint,
) {
    *mjoint = 0 as libc::c_int as libc::c_uint;
    let mut isb: libc::c_int = 0 as libc::c_int;
    while isb < (*frame).nsubbands {
        let mut m: [libc::c_uint; 2] = [0, 0];
        let mut mj: [libc::c_uint; 2] = [0, 0];
        let mut iblk: libc::c_int = 0 as libc::c_int;
        while iblk < (*frame).nblocks {
            let mut s0: libc::c_int = (*sb_samples
                .offset(
                    0 as libc::c_int as isize,
                ))[(iblk * (*frame).nsubbands + isb) as usize] as libc::c_int;
            let mut s1: libc::c_int = (*sb_samples
                .offset(
                    1 as libc::c_int as isize,
                ))[(iblk * (*frame).nsubbands + isb) as usize] as libc::c_int;
            m[0 as libc::c_int as usize]
                |= (if s0 < 0 as libc::c_int { !s0 } else { s0 }) as libc::c_uint;
            m[1 as libc::c_int as usize]
                |= (if s1 < 0 as libc::c_int { !s1 } else { s1 }) as libc::c_uint;
            mj[0 as libc::c_int as usize]
                |= (if s0 + s1 < 0 as libc::c_int { !(s0 + s1) } else { s0 + s1 })
                as libc::c_uint;
            mj[1 as libc::c_int as usize]
                |= (if s0 - s1 < 0 as libc::c_int { !(s0 - s1) } else { s0 - s1 })
                as libc::c_uint;
            iblk += 1;
            iblk;
        }
        let mut scf0: libc::c_int = (if m[0 as libc::c_int as usize] != 0 {
            (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(
                    (m[0 as libc::c_int as usize]).leading_zeros() as i32
                        as libc::c_ulong,
                )
        } else {
            0 as libc::c_int as libc::c_ulong
        }) as libc::c_int;
        let mut scf1: libc::c_int = (if m[1 as libc::c_int as usize] != 0 {
            (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(
                    (m[1 as libc::c_int as usize]).leading_zeros() as i32
                        as libc::c_ulong,
                )
        } else {
            0 as libc::c_int as libc::c_ulong
        }) as libc::c_int;
        let mut js0: libc::c_int = (if mj[0 as libc::c_int as usize] != 0 {
            (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(
                    (mj[0 as libc::c_int as usize]).leading_zeros() as i32
                        as libc::c_ulong,
                )
        } else {
            0 as libc::c_int as libc::c_ulong
        }) as libc::c_int;
        let mut js1: libc::c_int = (if mj[1 as libc::c_int as usize] != 0 {
            (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(
                    (mj[1 as libc::c_int as usize]).leading_zeros() as i32
                        as libc::c_ulong,
                )
        } else {
            0 as libc::c_int as libc::c_ulong
        }) as libc::c_int;
        if isb < (*frame).nsubbands - 1 as libc::c_int && js0 + js1 < scf0 + scf1 {
            *mjoint |= ((1 as libc::c_int) << isb) as libc::c_uint;
            scf0 = js0;
            scf1 = js1;
        }
        (*scale_factors.offset(0 as libc::c_int as isize))[isb as usize] = scf0;
        (*scale_factors.offset(1 as libc::c_int as isize))[isb as usize] = scf1;
        isb += 1;
        isb;
    }
}
unsafe extern "C" fn compute_scale_factors(
    mut frame: *const sbc_frame,
    mut sb_samples: *const [int16_t; 128],
    mut scale_factors: *mut [libc::c_int; 8],
) {
    let mut ich: libc::c_int = 0 as libc::c_int;
    while ich
        < 1 as libc::c_int
        + ((*frame).mode as libc::c_uint
        != SBC_MODE_MONO as libc::c_int as libc::c_uint) as libc::c_int
    {
        let mut isb: libc::c_int = 0 as libc::c_int;
        while isb < (*frame).nsubbands {
            let mut m: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            let mut iblk: libc::c_int = 0 as libc::c_int;
            while iblk < (*frame).nblocks {
                let mut s: libc::c_int = (*sb_samples
                    .offset(ich as isize))[(iblk * (*frame).nsubbands + isb) as usize]
                    as libc::c_int;
                m |= (if s < 0 as libc::c_int { !s } else { s }) as libc::c_uint;
                iblk += 1;
                iblk;
            }
            let mut scf: libc::c_int = (if m != 0 {
                (8 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(m.leading_zeros() as i32 as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }) as libc::c_int;
            (*scale_factors.offset(ich as isize))[isb as usize] = scf;
            isb += 1;
            isb;
        }
        ich += 1;
        ich;
    }
}
unsafe extern "C" fn encode_header(
    mut bits: *mut sbc_bits_t,
    mut frame: *const sbc_frame,
) {
    static mut enc_freq: [libc::c_int; 4] = [
        0 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
    ];
    static mut enc_mode: [libc::c_int; 4] = [
        0 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
    ];
    static mut enc_bam: [libc::c_int; 2] = [0 as libc::c_int, 1 as libc::c_int];
    let mut __bits: *mut sbc_bits_t = bits;
    sbc_put_bits(
        __bits,
        (if (*frame).msbc as libc::c_int != 0 {
            0xad as libc::c_int
        } else {
            0x9c as libc::c_int
        }) as libc::c_uint,
        8 as libc::c_int as libc::c_uint,
    );
    if !(*frame).msbc {
        sbc_put_bits(
            __bits,
            enc_freq[(*frame).freq as usize] as libc::c_uint,
            2 as libc::c_int as libc::c_uint,
        );
        sbc_put_bits(
            __bits,
            (((*frame).nblocks >> 2 as libc::c_int) - 1 as libc::c_int) as libc::c_uint,
            2 as libc::c_int as libc::c_uint,
        );
        sbc_put_bits(
            __bits,
            enc_mode[(*frame).mode as usize] as libc::c_uint,
            2 as libc::c_int as libc::c_uint,
        );
        sbc_put_bits(
            __bits,
            enc_bam[(*frame).bam as usize] as libc::c_uint,
            1 as libc::c_int as libc::c_uint,
        );
        sbc_put_bits(
            __bits,
            (((*frame).nsubbands >> 2 as libc::c_int) - 1 as libc::c_int)
                as libc::c_uint,
            1 as libc::c_int as libc::c_uint,
        );
        sbc_put_bits(
            __bits,
            (*frame).bitpool as libc::c_uint,
            8 as libc::c_int as libc::c_uint,
        );
    } else {
        sbc_put_bits(
            __bits,
            0 as libc::c_int as libc::c_uint,
            16 as libc::c_int as libc::c_uint,
        );
    }
    sbc_put_bits(
        __bits,
        0 as libc::c_int as libc::c_uint,
        8 as libc::c_int as libc::c_uint,
    );
}
unsafe extern "C" fn put_crc(
    mut frame: *const sbc_frame,
    mut data: *mut libc::c_void,
    mut size: libc::c_uint,
) -> libc::c_int {
    let mut crc: libc::c_int = compute_crc(frame, data as *const uint8_t, size);
    if crc < 0 as libc::c_int {
        -(1 as libc::c_int);
    } else {
        *(data as *mut uint8_t).offset(3 as libc::c_int as isize) = crc as uint8_t;
    };
    return 0 as libc::c_int;
}
unsafe extern "C" fn encode_frame(
    mut bits: *mut sbc_bits_t,
    mut frame: *const sbc_frame,
    mut sb_samples: *mut [int16_t; 128],
) {
    let mut __bits: *mut sbc_bits_t = bits;
    let mut scale_factors: [[libc::c_int; 8]; 2] = [[0; 8]; 2];
    let mut mjoint: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if (*frame).mode as libc::c_uint
        == SBC_MODE_JOINT_STEREO as libc::c_int as libc::c_uint
    {
        compute_scale_factors_js(
            frame,
            sb_samples as *const [int16_t; 128],
            scale_factors.as_mut_ptr(),
            &mut mjoint,
        );
    } else {
        compute_scale_factors(
            frame,
            sb_samples as *const [int16_t; 128],
            scale_factors.as_mut_ptr(),
        );
    }
    if (*frame).mode as libc::c_uint
        == SBC_MODE_DUAL_CHANNEL as libc::c_int as libc::c_uint
    {
        compute_scale_factors(
            frame,
            sb_samples.offset(1 as libc::c_int as isize) as *const [int16_t; 128],
            scale_factors.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
    }
    if (*frame).mode as libc::c_uint
        == SBC_MODE_JOINT_STEREO as libc::c_int as libc::c_uint
        && (*frame).nsubbands == 4 as libc::c_int
    {
        sbc_put_bits(
            __bits,
            (mjoint & 0x1 as libc::c_int as libc::c_uint) << 3 as libc::c_int
                | (mjoint & 0x2 as libc::c_int as libc::c_uint) << 1 as libc::c_int
                | (mjoint & 0x4 as libc::c_int as libc::c_uint) >> 1 as libc::c_int
                | (0 as libc::c_int >> 3 as libc::c_int) as libc::c_uint,
            4 as libc::c_int as libc::c_uint,
        );
    } else if (*frame).mode as libc::c_uint
        == SBC_MODE_JOINT_STEREO as libc::c_int as libc::c_uint
    {
        sbc_put_bits(
            __bits,
            (mjoint & 0x1 as libc::c_int as libc::c_uint) << 7 as libc::c_int
                | (mjoint & 0x2 as libc::c_int as libc::c_uint) << 5 as libc::c_int
                | (mjoint & 0x4 as libc::c_int as libc::c_uint) << 3 as libc::c_int
                | (mjoint & 0x8 as libc::c_int as libc::c_uint) << 1 as libc::c_int
                | (mjoint & 0x10 as libc::c_int as libc::c_uint) >> 1 as libc::c_int
                | (mjoint & 0x20 as libc::c_int as libc::c_uint) >> 3 as libc::c_int
                | (mjoint & 0x40 as libc::c_int as libc::c_uint) >> 5 as libc::c_int
                | (0 as libc::c_int >> 7 as libc::c_int) as libc::c_uint,
            8 as libc::c_int as libc::c_uint,
        );
    }
    let mut nchannels: libc::c_int = 1 as libc::c_int
        + ((*frame).mode as libc::c_uint != SBC_MODE_MONO as libc::c_int as libc::c_uint)
        as libc::c_int;
    let mut nsubbands: libc::c_int = (*frame).nsubbands;
    let mut nbits: [[libc::c_int; 8]; 2] = [[0; 8]; 2];
    let mut ich: libc::c_int = 0 as libc::c_int;
    while ich < nchannels {
        let mut isb: libc::c_int = 0 as libc::c_int;
        while isb < nsubbands {
            sbc_put_bits(
                __bits,
                scale_factors[ich as usize][isb as usize] as libc::c_uint,
                4 as libc::c_int as libc::c_uint,
            );
            isb += 1;
            isb;
        }
        ich += 1;
        ich;
    }
    compute_nbits(
        frame,
        scale_factors.as_mut_ptr() as *const [libc::c_int; 8],
        nbits.as_mut_ptr(),
    );
    if (*frame).mode as libc::c_uint
        == SBC_MODE_DUAL_CHANNEL as libc::c_int as libc::c_uint
    {
        compute_nbits(
            frame,
            scale_factors.as_mut_ptr().offset(1 as libc::c_int as isize)
                as *const [libc::c_int; 8],
            nbits.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
    }
    let mut isb_0: libc::c_int = 0 as libc::c_int;
    while isb_0 < nsubbands {
        if !(mjoint >> isb_0 & 1 as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint)
        {
            let mut iblk: libc::c_int = 0 as libc::c_int;
            while iblk < (*frame).nblocks {
                let mut s0: int16_t = (*sb_samples
                    .offset(
                        0 as libc::c_int as isize,
                    ))[(iblk * nsubbands + isb_0) as usize];
                let mut s1: int16_t = (*sb_samples
                    .offset(
                        1 as libc::c_int as isize,
                    ))[(iblk * nsubbands + isb_0) as usize];
                (*sb_samples
                    .offset(
                        0 as libc::c_int as isize,
                    ))[(iblk * nsubbands + isb_0)
                    as usize] = (s0 as libc::c_int + s1 as libc::c_int
                    >> 1 as libc::c_int) as int16_t;
                (*sb_samples
                    .offset(
                        1 as libc::c_int as isize,
                    ))[(iblk * nsubbands + isb_0)
                    as usize] = (s0 as libc::c_int - s1 as libc::c_int
                    >> 1 as libc::c_int) as int16_t;
                iblk += 1;
                iblk;
            }
        }
        isb_0 += 1;
        isb_0;
    }
    let mut iblk_0: libc::c_int = 0 as libc::c_int;
    while iblk_0 < (*frame).nblocks {
        let mut ich_0: libc::c_int = 0 as libc::c_int;
        while ich_0 < nchannels {
            let mut isb_1: libc::c_int = 0 as libc::c_int;
            while isb_1 < nsubbands {
                let mut nbit: libc::c_int = nbits[ich_0 as usize][isb_1 as usize];
                let mut scf: libc::c_int = scale_factors[ich_0 as usize][isb_1 as usize];
                if !(nbit == 0) {
                    let mut s: libc::c_int = (*sb_samples
                        .offset(ich_0 as isize))[(iblk_0 * nsubbands + isb_1) as usize]
                        as libc::c_int;
                    let mut range: libc::c_int = !((2147483647 as libc::c_int
                        as libc::c_uint)
                        .wrapping_mul(2 as libc::c_uint)
                        .wrapping_add(1 as libc::c_uint) << nbit) as libc::c_int;
                    sbc_put_bits(
                        __bits,
                        ((s * range >> scf + 1 as libc::c_int) + range
                            >> 1 as libc::c_int) as libc::c_uint,
                        nbit as libc::c_uint,
                    );
                }
                isb_1 += 1;
                isb_1;
            }
            ich_0 += 1;
            ich_0;
        }
        iblk_0 += 1;
        iblk_0;
    }
    let mut padding_nbits: libc::c_int = (8 as libc::c_int as libc::c_uint)
        .wrapping_sub(
            (sbc_tell_bits(bits)).wrapping_rem(8 as libc::c_int as libc::c_uint),
        ) as libc::c_int;
    sbc_put_bits(
        __bits,
        0 as libc::c_int as libc::c_uint,
        (if padding_nbits < 8 as libc::c_int { padding_nbits } else { 0 as libc::c_int })
            as libc::c_uint,
    );
}
unsafe extern "C" fn analyze_4(
    mut state: *mut sbc_estate,
    mut in_0: *const int16_t,
    mut pitch: libc::c_int,
    mut out: *mut int16_t,
) {
    static mut window: [[[int16_t; 10]; 4]; 2] = [
        [
            [
                0 as libc::c_int as int16_t,
                358 as libc::c_int as int16_t,
                4443 as libc::c_int as int16_t,
                -(4443 as libc::c_int) as int16_t,
                -(358 as libc::c_int) as int16_t,
                0 as libc::c_int as int16_t,
                358 as libc::c_int as int16_t,
                4443 as libc::c_int as int16_t,
                -(4443 as libc::c_int) as int16_t,
                -(358 as libc::c_int) as int16_t,
            ],
            [
                49 as libc::c_int as int16_t,
                946 as libc::c_int as int16_t,
                8082 as libc::c_int as int16_t,
                -(944 as libc::c_int) as int16_t,
                61 as libc::c_int as int16_t,
                49 as libc::c_int as int16_t,
                946 as libc::c_int as int16_t,
                8082 as libc::c_int as int16_t,
                -(944 as libc::c_int) as int16_t,
                61 as libc::c_int as int16_t,
            ],
            [
                18 as libc::c_int as int16_t,
                670 as libc::c_int as int16_t,
                6389 as libc::c_int as int16_t,
                -(2544 as libc::c_int) as int16_t,
                -(100 as libc::c_int) as int16_t,
                18 as libc::c_int as int16_t,
                670 as libc::c_int as int16_t,
                6389 as libc::c_int as int16_t,
                -(2544 as libc::c_int) as int16_t,
                -(100 as libc::c_int) as int16_t,
            ],
            [
                90 as libc::c_int as int16_t,
                1055 as libc::c_int as int16_t,
                9235 as libc::c_int as int16_t,
                201 as libc::c_int as int16_t,
                128 as libc::c_int as int16_t,
                90 as libc::c_int as int16_t,
                1055 as libc::c_int as int16_t,
                9235 as libc::c_int as int16_t,
                201 as libc::c_int as int16_t,
                128 as libc::c_int as int16_t,
            ],
        ],
        [
            [
                126 as libc::c_int as int16_t,
                848 as libc::c_int as int16_t,
                9644 as libc::c_int as int16_t,
                848 as libc::c_int as int16_t,
                126 as libc::c_int as int16_t,
                126 as libc::c_int as int16_t,
                848 as libc::c_int as int16_t,
                9644 as libc::c_int as int16_t,
                848 as libc::c_int as int16_t,
                126 as libc::c_int as int16_t,
            ],
            [
                61 as libc::c_int as int16_t,
                -(944 as libc::c_int) as int16_t,
                8082 as libc::c_int as int16_t,
                946 as libc::c_int as int16_t,
                49 as libc::c_int as int16_t,
                61 as libc::c_int as int16_t,
                -(944 as libc::c_int) as int16_t,
                8082 as libc::c_int as int16_t,
                946 as libc::c_int as int16_t,
                49 as libc::c_int as int16_t,
            ],
            [
                128 as libc::c_int as int16_t,
                201 as libc::c_int as int16_t,
                9235 as libc::c_int as int16_t,
                1055 as libc::c_int as int16_t,
                90 as libc::c_int as int16_t,
                128 as libc::c_int as int16_t,
                201 as libc::c_int as int16_t,
                9235 as libc::c_int as int16_t,
                1055 as libc::c_int as int16_t,
                90 as libc::c_int as int16_t,
            ],
            [
                -(100 as libc::c_int) as int16_t,
                -(2544 as libc::c_int) as int16_t,
                6389 as libc::c_int as int16_t,
                670 as libc::c_int as int16_t,
                18 as libc::c_int as int16_t,
                -(100 as libc::c_int) as int16_t,
                -(2544 as libc::c_int) as int16_t,
                6389 as libc::c_int as int16_t,
                670 as libc::c_int as int16_t,
                18 as libc::c_int as int16_t,
            ],
        ],
    ];
    let mut idx: libc::c_int = (*state).idx >> 1 as libc::c_int;
    let mut odd: libc::c_int = (*state).idx & 1 as libc::c_int;
    let mut x: *mut [int16_t; 5] = ((*state).x[odd as usize]).as_mut_ptr();
    let mut in_idx: libc::c_int = if idx != 0 {
        5 as libc::c_int - idx
    } else {
        0 as libc::c_int
    };
    (*x
        .offset(
            0 as libc::c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((3 as libc::c_int - 0 as libc::c_int) * pitch) as isize);
    (*x
        .offset(
            1 as libc::c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((3 as libc::c_int - 2 as libc::c_int) * pitch) as isize);
    (*x
        .offset(
            2 as libc::c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((3 as libc::c_int - 1 as libc::c_int) * pitch) as isize);
    (*x
        .offset(
            3 as libc::c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((3 as libc::c_int - 3 as libc::c_int) * pitch) as isize);
    let mut w0: *const [int16_t; 10] = (window[0 as libc::c_int
        as usize][0 as libc::c_int as usize])
        .as_ptr()
        .offset(idx as isize) as *const [int16_t; 10];
    let mut w1: *const [int16_t; 10] = (window[1 as libc::c_int
        as usize][0 as libc::c_int as usize])
        .as_ptr()
        .offset(idx as isize) as *const [int16_t; 10];
    let mut y0: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut y3: libc::c_int = 0;
    let mut y: [int16_t; 4] = [0; 4];
    y0 = (*x.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int
        * (*w0.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(0 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(0 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int + (*state).y[0 as libc::c_int as usize];
    (*state)
        .y[0 as libc::c_int
        as usize] = (*x.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(0 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(0 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int;
    y1 = (*x.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int
        * (*w0.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(2 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(2 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(3 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(3 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int;
    y2 = (*state).y[1 as libc::c_int as usize];
    (*state)
        .y[1 as libc::c_int
        as usize] = (*x.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(2 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(2 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        - (*x.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        - (*x.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        - (*x.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        - (*x.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        - (*x.offset(3 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(3 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int;
    y3 = (*x.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int
        * (*w0.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(1 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(1 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int;
    y[0 as libc::c_int
        as usize] = (if y0 + ((1 as libc::c_int) << 14 as libc::c_int)
        >> 15 as libc::c_int > 32767 as libc::c_int
    {
        32767 as libc::c_int
    } else if (y0 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int)
        < -(32767 as libc::c_int) - 1 as libc::c_int
    {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        y0 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int
    }) as int16_t;
    y[1 as libc::c_int
        as usize] = (if y1 + ((1 as libc::c_int) << 14 as libc::c_int)
        >> 15 as libc::c_int > 32767 as libc::c_int
    {
        32767 as libc::c_int
    } else if (y1 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int)
        < -(32767 as libc::c_int) - 1 as libc::c_int
    {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        y1 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int
    }) as int16_t;
    y[2 as libc::c_int
        as usize] = (if y2 + ((1 as libc::c_int) << 14 as libc::c_int)
        >> 15 as libc::c_int > 32767 as libc::c_int
    {
        32767 as libc::c_int
    } else if (y2 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int)
        < -(32767 as libc::c_int) - 1 as libc::c_int
    {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        y2 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int
    }) as int16_t;
    y[3 as libc::c_int
        as usize] = (if y3 + ((1 as libc::c_int) << 14 as libc::c_int)
        >> 15 as libc::c_int > 32767 as libc::c_int
    {
        32767 as libc::c_int
    } else if (y3 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int)
        < -(32767 as libc::c_int) - 1 as libc::c_int
    {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        y3 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int
    }) as int16_t;
    (*state)
        .idx = if (*state).idx < 9 as libc::c_int {
        (*state).idx + 1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    static mut cos8: [int16_t; 4] = [
        8192 as libc::c_int as int16_t,
        7568 as libc::c_int as int16_t,
        5793 as libc::c_int as int16_t,
        3135 as libc::c_int as int16_t,
    ];
    let mut s0: libc::c_int = 0;
    let mut s1: libc::c_int = 0;
    let mut s2: libc::c_int = 0;
    let mut s3: libc::c_int = 0;
    s0 = y[0 as libc::c_int as usize] as libc::c_int
        * cos8[2 as libc::c_int as usize] as libc::c_int
        + y[1 as libc::c_int as usize] as libc::c_int
        * cos8[1 as libc::c_int as usize] as libc::c_int
        + y[2 as libc::c_int as usize] as libc::c_int
        * cos8[3 as libc::c_int as usize] as libc::c_int
        + ((y[3 as libc::c_int as usize] as libc::c_int) << 13 as libc::c_int);
    s1 = -(y[0 as libc::c_int as usize] as libc::c_int)
        * cos8[2 as libc::c_int as usize] as libc::c_int
        + y[1 as libc::c_int as usize] as libc::c_int
        * cos8[3 as libc::c_int as usize] as libc::c_int
        - y[2 as libc::c_int as usize] as libc::c_int
        * cos8[1 as libc::c_int as usize] as libc::c_int
        + ((y[3 as libc::c_int as usize] as libc::c_int) << 13 as libc::c_int);
    s2 = -(y[0 as libc::c_int as usize] as libc::c_int)
        * cos8[2 as libc::c_int as usize] as libc::c_int
        - y[1 as libc::c_int as usize] as libc::c_int
        * cos8[3 as libc::c_int as usize] as libc::c_int
        + y[2 as libc::c_int as usize] as libc::c_int
        * cos8[1 as libc::c_int as usize] as libc::c_int
        + ((y[3 as libc::c_int as usize] as libc::c_int) << 13 as libc::c_int);
    s3 = y[0 as libc::c_int as usize] as libc::c_int
        * cos8[2 as libc::c_int as usize] as libc::c_int
        - y[1 as libc::c_int as usize] as libc::c_int
        * cos8[1 as libc::c_int as usize] as libc::c_int
        - y[2 as libc::c_int as usize] as libc::c_int
        * cos8[3 as libc::c_int as usize] as libc::c_int
        + ((y[3 as libc::c_int as usize] as libc::c_int) << 13 as libc::c_int);
    let fresh24 = out;
    out = out.offset(1);
    *fresh24 = (if s0 + ((1 as libc::c_int) << 12 as libc::c_int) >> 13 as libc::c_int
        > 32767 as libc::c_int
    {
        32767 as libc::c_int
    } else if (s0 + ((1 as libc::c_int) << 12 as libc::c_int) >> 13 as libc::c_int)
        < -(32767 as libc::c_int) - 1 as libc::c_int
    {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        s0 + ((1 as libc::c_int) << 12 as libc::c_int) >> 13 as libc::c_int
    }) as int16_t;
    let fresh25 = out;
    out = out.offset(1);
    *fresh25 = (if s1 + ((1 as libc::c_int) << 12 as libc::c_int) >> 13 as libc::c_int
        > 32767 as libc::c_int
    {
        32767 as libc::c_int
    } else if (s1 + ((1 as libc::c_int) << 12 as libc::c_int) >> 13 as libc::c_int)
        < -(32767 as libc::c_int) - 1 as libc::c_int
    {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        s1 + ((1 as libc::c_int) << 12 as libc::c_int) >> 13 as libc::c_int
    }) as int16_t;
    let fresh26 = out;
    out = out.offset(1);
    *fresh26 = (if s2 + ((1 as libc::c_int) << 12 as libc::c_int) >> 13 as libc::c_int
        > 32767 as libc::c_int
    {
        32767 as libc::c_int
    } else if (s2 + ((1 as libc::c_int) << 12 as libc::c_int) >> 13 as libc::c_int)
        < -(32767 as libc::c_int) - 1 as libc::c_int
    {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        s2 + ((1 as libc::c_int) << 12 as libc::c_int) >> 13 as libc::c_int
    }) as int16_t;
    let fresh27 = out;
    out = out.offset(1);
    *fresh27 = (if s3 + ((1 as libc::c_int) << 12 as libc::c_int) >> 13 as libc::c_int
        > 32767 as libc::c_int
    {
        32767 as libc::c_int
    } else if (s3 + ((1 as libc::c_int) << 12 as libc::c_int) >> 13 as libc::c_int)
        < -(32767 as libc::c_int) - 1 as libc::c_int
    {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        s3 + ((1 as libc::c_int) << 12 as libc::c_int) >> 13 as libc::c_int
    }) as int16_t;
}
unsafe extern "C" fn analyze_8(
    mut state: *mut sbc_estate,
    mut in_0: *const int16_t,
    mut pitch: libc::c_int,
    mut out: *mut int16_t,
) {
    static mut window: [[[int16_t; 10]; 8]; 2] = [
        [
            [
                0 as libc::c_int as int16_t,
                185 as libc::c_int as int16_t,
                2228 as libc::c_int as int16_t,
                -(2228 as libc::c_int) as int16_t,
                -(185 as libc::c_int) as int16_t,
                0 as libc::c_int as int16_t,
                185 as libc::c_int as int16_t,
                2228 as libc::c_int as int16_t,
                -(2228 as libc::c_int) as int16_t,
                -(185 as libc::c_int) as int16_t,
            ],
            [
                27 as libc::c_int as int16_t,
                480 as libc::c_int as int16_t,
                4039 as libc::c_int as int16_t,
                -(480 as libc::c_int) as int16_t,
                30 as libc::c_int as int16_t,
                27 as libc::c_int as int16_t,
                480 as libc::c_int as int16_t,
                4039 as libc::c_int as int16_t,
                -(480 as libc::c_int) as int16_t,
                30 as libc::c_int as int16_t,
            ],
            [
                5 as libc::c_int as int16_t,
                263 as libc::c_int as int16_t,
                2719 as libc::c_int as int16_t,
                -(1743 as libc::c_int) as int16_t,
                -(115 as libc::c_int) as int16_t,
                5 as libc::c_int as int16_t,
                263 as libc::c_int as int16_t,
                2719 as libc::c_int as int16_t,
                -(1743 as libc::c_int) as int16_t,
                -(115 as libc::c_int) as int16_t,
            ],
            [
                58 as libc::c_int as int16_t,
                502 as libc::c_int as int16_t,
                4764 as libc::c_int as int16_t,
                290 as libc::c_int as int16_t,
                69 as libc::c_int as int16_t,
                58 as libc::c_int as int16_t,
                502 as libc::c_int as int16_t,
                4764 as libc::c_int as int16_t,
                290 as libc::c_int as int16_t,
                69 as libc::c_int as int16_t,
            ],
            [
                11 as libc::c_int as int16_t,
                343 as libc::c_int as int16_t,
                3197 as libc::c_int as int16_t,
                -(1280 as libc::c_int) as int16_t,
                -(54 as libc::c_int) as int16_t,
                11 as libc::c_int as int16_t,
                343 as libc::c_int as int16_t,
                3197 as libc::c_int as int16_t,
                -(1280 as libc::c_int) as int16_t,
                -(54 as libc::c_int) as int16_t,
            ],
            [
                48 as libc::c_int as int16_t,
                532 as libc::c_int as int16_t,
                4612 as libc::c_int as int16_t,
                96 as libc::c_int as int16_t,
                65 as libc::c_int as int16_t,
                48 as libc::c_int as int16_t,
                532 as libc::c_int as int16_t,
                4612 as libc::c_int as int16_t,
                96 as libc::c_int as int16_t,
                65 as libc::c_int as int16_t,
            ],
            [
                18 as libc::c_int as int16_t,
                418 as libc::c_int as int16_t,
                3644 as libc::c_int as int16_t,
                -(856 as libc::c_int) as int16_t,
                -(6 as libc::c_int) as int16_t,
                18 as libc::c_int as int16_t,
                418 as libc::c_int as int16_t,
                3644 as libc::c_int as int16_t,
                -(856 as libc::c_int) as int16_t,
                -(6 as libc::c_int) as int16_t,
            ],
            [
                37 as libc::c_int as int16_t,
                521 as libc::c_int as int16_t,
                4367 as libc::c_int as int16_t,
                -(161 as libc::c_int) as int16_t,
                53 as libc::c_int as int16_t,
                37 as libc::c_int as int16_t,
                521 as libc::c_int as int16_t,
                4367 as libc::c_int as int16_t,
                -(161 as libc::c_int) as int16_t,
                53 as libc::c_int as int16_t,
            ],
        ],
        [
            [
                66 as libc::c_int as int16_t,
                424 as libc::c_int as int16_t,
                4815 as libc::c_int as int16_t,
                424 as libc::c_int as int16_t,
                66 as libc::c_int as int16_t,
                66 as libc::c_int as int16_t,
                424 as libc::c_int as int16_t,
                4815 as libc::c_int as int16_t,
                424 as libc::c_int as int16_t,
                66 as libc::c_int as int16_t,
            ],
            [
                30 as libc::c_int as int16_t,
                -(480 as libc::c_int) as int16_t,
                4039 as libc::c_int as int16_t,
                480 as libc::c_int as int16_t,
                27 as libc::c_int as int16_t,
                30 as libc::c_int as int16_t,
                -(480 as libc::c_int) as int16_t,
                4039 as libc::c_int as int16_t,
                480 as libc::c_int as int16_t,
                27 as libc::c_int as int16_t,
            ],
            [
                69 as libc::c_int as int16_t,
                290 as libc::c_int as int16_t,
                4764 as libc::c_int as int16_t,
                502 as libc::c_int as int16_t,
                58 as libc::c_int as int16_t,
                69 as libc::c_int as int16_t,
                290 as libc::c_int as int16_t,
                4764 as libc::c_int as int16_t,
                502 as libc::c_int as int16_t,
                58 as libc::c_int as int16_t,
            ],
            [
                -(115 as libc::c_int) as int16_t,
                -(1743 as libc::c_int) as int16_t,
                2719 as libc::c_int as int16_t,
                263 as libc::c_int as int16_t,
                5 as libc::c_int as int16_t,
                -(115 as libc::c_int) as int16_t,
                -(1743 as libc::c_int) as int16_t,
                2719 as libc::c_int as int16_t,
                263 as libc::c_int as int16_t,
                5 as libc::c_int as int16_t,
            ],
            [
                65 as libc::c_int as int16_t,
                96 as libc::c_int as int16_t,
                4612 as libc::c_int as int16_t,
                532 as libc::c_int as int16_t,
                48 as libc::c_int as int16_t,
                65 as libc::c_int as int16_t,
                96 as libc::c_int as int16_t,
                4612 as libc::c_int as int16_t,
                532 as libc::c_int as int16_t,
                48 as libc::c_int as int16_t,
            ],
            [
                -(54 as libc::c_int) as int16_t,
                -(1280 as libc::c_int) as int16_t,
                3197 as libc::c_int as int16_t,
                343 as libc::c_int as int16_t,
                11 as libc::c_int as int16_t,
                -(54 as libc::c_int) as int16_t,
                -(1280 as libc::c_int) as int16_t,
                3197 as libc::c_int as int16_t,
                343 as libc::c_int as int16_t,
                11 as libc::c_int as int16_t,
            ],
            [
                53 as libc::c_int as int16_t,
                -(161 as libc::c_int) as int16_t,
                4367 as libc::c_int as int16_t,
                521 as libc::c_int as int16_t,
                37 as libc::c_int as int16_t,
                53 as libc::c_int as int16_t,
                -(161 as libc::c_int) as int16_t,
                4367 as libc::c_int as int16_t,
                521 as libc::c_int as int16_t,
                37 as libc::c_int as int16_t,
            ],
            [
                -(6 as libc::c_int) as int16_t,
                -(856 as libc::c_int) as int16_t,
                3644 as libc::c_int as int16_t,
                418 as libc::c_int as int16_t,
                18 as libc::c_int as int16_t,
                -(6 as libc::c_int) as int16_t,
                -(856 as libc::c_int) as int16_t,
                3644 as libc::c_int as int16_t,
                418 as libc::c_int as int16_t,
                18 as libc::c_int as int16_t,
            ],
        ],
    ];
    let mut idx: libc::c_int = (*state).idx >> 1 as libc::c_int;
    let mut odd: libc::c_int = (*state).idx & 1 as libc::c_int;
    let mut x: *mut [int16_t; 5] = ((*state).x[odd as usize]).as_mut_ptr();
    let mut in_idx: libc::c_int = if idx != 0 {
        5 as libc::c_int - idx
    } else {
        0 as libc::c_int
    };
    (*x
        .offset(
            0 as libc::c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((7 as libc::c_int - 0 as libc::c_int) * pitch) as isize);
    (*x
        .offset(
            1 as libc::c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((7 as libc::c_int - 4 as libc::c_int) * pitch) as isize);
    (*x
        .offset(
            2 as libc::c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((7 as libc::c_int - 1 as libc::c_int) * pitch) as isize);
    (*x
        .offset(
            3 as libc::c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((7 as libc::c_int - 7 as libc::c_int) * pitch) as isize);
    (*x
        .offset(
            4 as libc::c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((7 as libc::c_int - 2 as libc::c_int) * pitch) as isize);
    (*x
        .offset(
            5 as libc::c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((7 as libc::c_int - 6 as libc::c_int) * pitch) as isize);
    (*x
        .offset(
            6 as libc::c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((7 as libc::c_int - 3 as libc::c_int) * pitch) as isize);
    (*x
        .offset(
            7 as libc::c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((7 as libc::c_int - 5 as libc::c_int) * pitch) as isize);
    let mut w0: *const [int16_t; 10] = (window[0 as libc::c_int
        as usize][0 as libc::c_int as usize])
        .as_ptr()
        .offset(idx as isize) as *const [int16_t; 10];
    let mut w1: *const [int16_t; 10] = (window[1 as libc::c_int
        as usize][0 as libc::c_int as usize])
        .as_ptr()
        .offset(idx as isize) as *const [int16_t; 10];
    let mut y0: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut y3: libc::c_int = 0;
    let mut y4: libc::c_int = 0;
    let mut y5: libc::c_int = 0;
    let mut y6: libc::c_int = 0;
    let mut y7: libc::c_int = 0;
    let mut y: [int16_t; 8] = [0; 8];
    y0 = (*x.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int
        * (*w0.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(0 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(0 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int + (*state).y[0 as libc::c_int as usize];
    (*state)
        .y[0 as libc::c_int
        as usize] = (*x.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(0 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(0 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int;
    y1 = (*x.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int
        * (*w0.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(2 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(2 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(3 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(3 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int;
    y4 = (*state).y[1 as libc::c_int as usize];
    (*state)
        .y[1 as libc::c_int
        as usize] = (*x.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(2 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(2 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        - (*x.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        - (*x.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        - (*x.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        - (*x.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        - (*x.offset(3 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(3 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int;
    y2 = (*x.offset(4 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int
        * (*w0.offset(4 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(4 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(4 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(4 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(4 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(4 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(4 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(4 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(4 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(5 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(5 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(5 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(5 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(5 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(5 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(5 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(5 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(5 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(5 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int;
    y5 = (*state).y[2 as libc::c_int as usize];
    (*state)
        .y[2 as libc::c_int
        as usize] = (*x.offset(4 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(4 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(4 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(4 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(4 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(4 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(4 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(4 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(4 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(4 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        - (*x.offset(5 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(5 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        - (*x.offset(5 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(5 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        - (*x.offset(5 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(5 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        - (*x.offset(5 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(5 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        - (*x.offset(5 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(5 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int;
    y3 = (*x.offset(6 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int
        * (*w0.offset(6 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(6 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(6 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(6 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(6 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(6 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(6 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(6 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(6 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(7 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(7 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(7 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(7 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(7 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(7 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(7 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(7 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(7 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(7 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int;
    y6 = (*state).y[3 as libc::c_int as usize];
    (*state)
        .y[3 as libc::c_int
        as usize] = (*x.offset(6 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(6 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(6 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(6 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(6 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(6 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(6 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(6 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(6 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(6 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        - (*x.offset(7 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(7 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        - (*x.offset(7 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(7 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        - (*x.offset(7 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(7 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        - (*x.offset(7 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(7 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        - (*x.offset(7 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        * (*w1.offset(7 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int;
    y7 = (*x.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int
        * (*w0.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize]
        as libc::c_int
        + (*x.offset(1 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int
        * (*w0.offset(1 as libc::c_int as isize))[4 as libc::c_int as usize]
        as libc::c_int;
    y[0 as libc::c_int
        as usize] = (if y0 + ((1 as libc::c_int) << 14 as libc::c_int)
        >> 15 as libc::c_int > 32767 as libc::c_int
    {
        32767 as libc::c_int
    } else if (y0 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int)
        < -(32767 as libc::c_int) - 1 as libc::c_int
    {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        y0 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int
    }) as int16_t;
    y[1 as libc::c_int
        as usize] = (if y1 + ((1 as libc::c_int) << 14 as libc::c_int)
        >> 15 as libc::c_int > 32767 as libc::c_int
    {
        32767 as libc::c_int
    } else if (y1 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int)
        < -(32767 as libc::c_int) - 1 as libc::c_int
    {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        y1 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int
    }) as int16_t;
    y[2 as libc::c_int
        as usize] = (if y2 + ((1 as libc::c_int) << 14 as libc::c_int)
        >> 15 as libc::c_int > 32767 as libc::c_int
    {
        32767 as libc::c_int
    } else if (y2 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int)
        < -(32767 as libc::c_int) - 1 as libc::c_int
    {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        y2 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int
    }) as int16_t;
    y[3 as libc::c_int
        as usize] = (if y3 + ((1 as libc::c_int) << 14 as libc::c_int)
        >> 15 as libc::c_int > 32767 as libc::c_int
    {
        32767 as libc::c_int
    } else if (y3 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int)
        < -(32767 as libc::c_int) - 1 as libc::c_int
    {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        y3 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int
    }) as int16_t;
    y[4 as libc::c_int
        as usize] = (if y4 + ((1 as libc::c_int) << 14 as libc::c_int)
        >> 15 as libc::c_int > 32767 as libc::c_int
    {
        32767 as libc::c_int
    } else if (y4 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int)
        < -(32767 as libc::c_int) - 1 as libc::c_int
    {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        y4 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int
    }) as int16_t;
    y[5 as libc::c_int
        as usize] = (if y5 + ((1 as libc::c_int) << 14 as libc::c_int)
        >> 15 as libc::c_int > 32767 as libc::c_int
    {
        32767 as libc::c_int
    } else if (y5 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int)
        < -(32767 as libc::c_int) - 1 as libc::c_int
    {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        y5 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int
    }) as int16_t;
    y[6 as libc::c_int
        as usize] = (if y6 + ((1 as libc::c_int) << 14 as libc::c_int)
        >> 15 as libc::c_int > 32767 as libc::c_int
    {
        32767 as libc::c_int
    } else if (y6 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int)
        < -(32767 as libc::c_int) - 1 as libc::c_int
    {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        y6 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int
    }) as int16_t;
    y[7 as libc::c_int
        as usize] = (if y7 + ((1 as libc::c_int) << 14 as libc::c_int)
        >> 15 as libc::c_int > 32767 as libc::c_int
    {
        32767 as libc::c_int
    } else if (y7 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int)
        < -(32767 as libc::c_int) - 1 as libc::c_int
    {
        -(32767 as libc::c_int) - 1 as libc::c_int
    } else {
        y7 + ((1 as libc::c_int) << 14 as libc::c_int) >> 15 as libc::c_int
    }) as int16_t;
    (*state)
        .idx = if (*state).idx < 9 as libc::c_int {
        (*state).idx + 1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    static mut cosmat: [[int16_t; 8]; 8] = [
        [
            5793 as libc::c_int as int16_t,
            6811 as libc::c_int as int16_t,
            7568 as libc::c_int as int16_t,
            8035 as libc::c_int as int16_t,
            4551 as libc::c_int as int16_t,
            3135 as libc::c_int as int16_t,
            1598 as libc::c_int as int16_t,
            8192 as libc::c_int as int16_t,
        ],
        [
            -(5793 as libc::c_int) as int16_t,
            -(1598 as libc::c_int) as int16_t,
            3135 as libc::c_int as int16_t,
            6811 as libc::c_int as int16_t,
            -(8035 as libc::c_int) as int16_t,
            -(7568 as libc::c_int) as int16_t,
            -(4551 as libc::c_int) as int16_t,
            8192 as libc::c_int as int16_t,
        ],
        [
            -(5793 as libc::c_int) as int16_t,
            -(8035 as libc::c_int) as int16_t,
            -(3135 as libc::c_int) as int16_t,
            4551 as libc::c_int as int16_t,
            1598 as libc::c_int as int16_t,
            7568 as libc::c_int as int16_t,
            6811 as libc::c_int as int16_t,
            8192 as libc::c_int as int16_t,
        ],
        [
            5793 as libc::c_int as int16_t,
            -(4551 as libc::c_int) as int16_t,
            -(7568 as libc::c_int) as int16_t,
            1598 as libc::c_int as int16_t,
            6811 as libc::c_int as int16_t,
            -(3135 as libc::c_int) as int16_t,
            -(8035 as libc::c_int) as int16_t,
            8192 as libc::c_int as int16_t,
        ],
        [
            5793 as libc::c_int as int16_t,
            4551 as libc::c_int as int16_t,
            -(7568 as libc::c_int) as int16_t,
            -(1598 as libc::c_int) as int16_t,
            -(6811 as libc::c_int) as int16_t,
            -(3135 as libc::c_int) as int16_t,
            8035 as libc::c_int as int16_t,
            8192 as libc::c_int as int16_t,
        ],
        [
            -(5793 as libc::c_int) as int16_t,
            8035 as libc::c_int as int16_t,
            -(3135 as libc::c_int) as int16_t,
            -(4551 as libc::c_int) as int16_t,
            -(1598 as libc::c_int) as int16_t,
            7568 as libc::c_int as int16_t,
            -(6811 as libc::c_int) as int16_t,
            8192 as libc::c_int as int16_t,
        ],
        [
            -(5793 as libc::c_int) as int16_t,
            1598 as libc::c_int as int16_t,
            3135 as libc::c_int as int16_t,
            -(6811 as libc::c_int) as int16_t,
            8035 as libc::c_int as int16_t,
            -(7568 as libc::c_int) as int16_t,
            4551 as libc::c_int as int16_t,
            8192 as libc::c_int as int16_t,
        ],
        [
            5793 as libc::c_int as int16_t,
            -(6811 as libc::c_int) as int16_t,
            7568 as libc::c_int as int16_t,
            -(8035 as libc::c_int) as int16_t,
            -(4551 as libc::c_int) as int16_t,
            3135 as libc::c_int as int16_t,
            -(1598 as libc::c_int) as int16_t,
            8192 as libc::c_int as int16_t,
        ],
    ];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        let mut s: libc::c_int = y[0 as libc::c_int as usize] as libc::c_int
            * cosmat[i as usize][0 as libc::c_int as usize] as libc::c_int
            + y[1 as libc::c_int as usize] as libc::c_int
            * cosmat[i as usize][1 as libc::c_int as usize] as libc::c_int
            + y[2 as libc::c_int as usize] as libc::c_int
            * cosmat[i as usize][2 as libc::c_int as usize] as libc::c_int
            + y[3 as libc::c_int as usize] as libc::c_int
            * cosmat[i as usize][3 as libc::c_int as usize] as libc::c_int
            + y[4 as libc::c_int as usize] as libc::c_int
            * cosmat[i as usize][4 as libc::c_int as usize] as libc::c_int
            + y[5 as libc::c_int as usize] as libc::c_int
            * cosmat[i as usize][5 as libc::c_int as usize] as libc::c_int
            + y[6 as libc::c_int as usize] as libc::c_int
            * cosmat[i as usize][6 as libc::c_int as usize] as libc::c_int
            + y[7 as libc::c_int as usize] as libc::c_int
            * cosmat[i as usize][7 as libc::c_int as usize] as libc::c_int;
        let fresh28 = out;
        out = out.offset(1);
        *fresh28 = (if s + ((1 as libc::c_int) << 12 as libc::c_int) >> 13 as libc::c_int
            > 32767 as libc::c_int
        {
            32767 as libc::c_int
        } else if (s + ((1 as libc::c_int) << 12 as libc::c_int) >> 13 as libc::c_int)
            < -(32767 as libc::c_int) - 1 as libc::c_int
        {
            -(32767 as libc::c_int) - 1 as libc::c_int
        } else {
            s + ((1 as libc::c_int) << 12 as libc::c_int) >> 13 as libc::c_int
        }) as int16_t;
        i += 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn analyze(
    mut state: *mut sbc_estate,
    mut frame: *const sbc_frame,
    mut in_0: *const int16_t,
    mut pitch: libc::c_int,
    mut out: *mut int16_t,
) {
    let mut iblk: libc::c_int = 0 as libc::c_int;
    while iblk < (*frame).nblocks {
        if (*frame).nsubbands == 4 as libc::c_int {
            analyze_4(state, in_0, pitch, out);
        } else {
            analyze_8(state, in_0, pitch, out);
        }
        in_0 = in_0.offset(((*frame).nsubbands * pitch) as isize);
        out = out.offset((*frame).nsubbands as isize);
        iblk += 1;
        iblk;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sbc_encode(
    mut sbc: *mut sbc,
    mut pcml: *const int16_t,
    mut pitchl: libc::c_int,
    mut pcmr: *const int16_t,
    mut pitchr: libc::c_int,
    mut frame: *const sbc_frame,
    mut data: *mut libc::c_void,
    mut size: libc::c_uint,
) -> libc::c_int {
    if (*frame).msbc {
        frame = &msbc_frame;
    }
    if !check_frame(frame) || size < sbc_get_frame_size(frame) {
        return -(1 as libc::c_int);
    }
    let mut sb_samples: [[int16_t; 128]; 2] = [[0; 128]; 2];
    analyze(
        &mut *((*sbc).c2rust_unnamed.estates)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        frame,
        pcml,
        pitchl,
        (sb_samples[0 as libc::c_int as usize]).as_mut_ptr(),
    );
    if (*frame).mode as libc::c_uint != SBC_MODE_MONO as libc::c_int as libc::c_uint {
        analyze(
            &mut *((*sbc).c2rust_unnamed.estates)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize),
            frame,
            pcmr,
            pitchr,
            (sb_samples[1 as libc::c_int as usize]).as_mut_ptr(),
        );
    }
    let mut bits: sbc_bits_t = sbc_bits_t {
        mode: SBC_BITS_READ,
        data: C2RustUnnamed_1 {
            p: 0 as *mut uint8_t,
            nbytes: 0,
            nleft: 0,
        },
        accu: C2RustUnnamed_0 {
            v: 0,
            nleft: 0,
            nover: 0,
        },
        error: false,
    };
    sbc_setup_bits(
        &mut bits,
        SBC_BITS_WRITE,
        (data as uintptr_t).wrapping_add(4 as libc::c_int as libc::c_ulong)
            as *mut libc::c_void,
        (sbc_get_frame_size(frame)).wrapping_sub(4 as libc::c_int as libc::c_uint),
    );
    encode_frame(&mut bits, frame, sb_samples.as_mut_ptr());
    sbc_flush_bits(&mut bits);
    sbc_setup_bits(&mut bits, SBC_BITS_WRITE, data, 4 as libc::c_int as libc::c_uint);
    encode_header(&mut bits, frame);
    sbc_flush_bits(&mut bits);
    put_crc(frame, data, size);
    return 0 as libc::c_int;
}
