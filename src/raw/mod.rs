#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, clippy::missing_safety_doc)]

mod bits;

use std::ffi::{c_int, c_short, c_uchar, c_uint, c_void};
use crate::bits2::{Bits, Mode};

pub type __uint8_t = c_uchar;
pub type __int16_t = c_short;
pub type __int32_t = c_int;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type sbc_freq = c_uint;
pub const SBC_FREQ_48K: sbc_freq = 3;
pub const SBC_FREQ_44K1: sbc_freq = 2;
pub const SBC_FREQ_32K: sbc_freq = 1;
pub const SBC_FREQ_16K: sbc_freq = 0;
pub type sbc_mode = c_uint;
pub const SBC_MODE_JOINT_STEREO: sbc_mode = 3;
pub const SBC_MODE_STEREO: sbc_mode = 2;
pub const SBC_MODE_DUAL_CHANNEL: sbc_mode = 1;
pub const SBC_MODE_MONO: sbc_mode = 0;
pub type sbc_bam = c_uint;
pub const SBC_BAM_SNR: sbc_bam = 1;
pub const SBC_BAM_LOUDNESS: sbc_bam = 0;
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct sbc_frame {
    pub msbc: bool,
    pub freq: sbc_freq,
    pub mode: sbc_mode,
    pub bam: sbc_bam,
    pub nblocks: c_int,
    pub nsubbands: c_int,
    pub bitpool: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sbc_dstate {
    pub idx: c_int,
    pub v: [[[int16_t; 10]; 8]; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sbc_estate {
    pub idx: c_int,
    pub x: [[[int16_t; 5]; 8]; 2],
    pub y: [int32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sbc {
    pub nchannels: c_int,
    pub nblocks: c_int,
    pub nsubbands: c_int,
    pub c2rust_unnamed: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub dstates: [sbc_dstate; 2],
    pub estates: [sbc_estate; 2],
}
pub type sbc_t = sbc;

static msbc_frame: sbc_frame = sbc_frame {
    msbc: 1 as c_int != 0,
    freq: SBC_FREQ_16K,
    mode: SBC_MODE_MONO,
    bam: SBC_BAM_LOUDNESS,
    nblocks: 15 as c_int,
    nsubbands: 8 as c_int,
    bitpool: 26 as c_int,
};
unsafe extern "C" fn compute_crc(
    frame: *const sbc_frame,
    data: *const uint8_t,
    size: c_uint,
) -> c_int {
    static mut t: [uint8_t; 256] = [
        0x00, 0x1d, 0x3a, 0x27, 0x74, 0x69, 0x4e, 0x53, 0xe8, 0xf5, 0xd2, 0xcf, 0x9c, 0x81, 0xa6, 0xbb,
        0xcd, 0xd0, 0xf7, 0xea, 0xb9, 0xa4, 0x83, 0x9e, 0x25, 0x38, 0x1f, 0x02, 0x51, 0x4c, 0x6b, 0x76,
        0x87, 0x9a, 0xbd, 0xa0, 0xf3, 0xee, 0xc9, 0xd4, 0x6f, 0x72, 0x55, 0x48, 0x1b, 0x06, 0x21, 0x3c,
        0x4a, 0x57, 0x70, 0x6d, 0x3e, 0x23, 0x04, 0x19, 0xa2, 0xbf, 0x98, 0x85, 0xd6, 0xcb, 0xec, 0xf1,
        0x13, 0x0e, 0x29, 0x34, 0x67, 0x7a, 0x5d, 0x40, 0xfb, 0xe6, 0xc1, 0xdc, 0x8f, 0x92, 0xb5, 0xa8,
        0xde, 0xc3, 0xe4, 0xf9, 0xaa, 0xb7, 0x90, 0x8d, 0x36, 0x2b, 0x0c, 0x11, 0x42, 0x5f, 0x78, 0x65,
        0x94, 0x89, 0xae, 0xb3, 0xe0, 0xfd, 0xda, 0xc7, 0x7c, 0x61, 0x46, 0x5b, 0x08, 0x15, 0x32, 0x2f,
        0x59, 0x44, 0x63, 0x7e, 0x2d, 0x30, 0x17, 0x0a, 0xb1, 0xac, 0x8b, 0x96, 0xc5, 0xd8, 0xff, 0xe2,
        0x26, 0x3b, 0x1c, 0x01, 0x52, 0x4f, 0x68, 0x75, 0xce, 0xd3, 0xf4, 0xe9, 0xba, 0xa7, 0x80, 0x9d,
        0xeb, 0xf6, 0xd1, 0xcc, 0x9f, 0x82, 0xa5, 0xb8, 0x03, 0x1e, 0x39, 0x24, 0x77, 0x6a, 0x4d, 0x50,
        0xa1, 0xbc, 0x9b, 0x86, 0xd5, 0xc8, 0xef, 0xf2, 0x49, 0x54, 0x73, 0x6e, 0x3d, 0x20, 0x07, 0x1a,
        0x6c, 0x71, 0x56, 0x4b, 0x18, 0x05, 0x22, 0x3f, 0x84, 0x99, 0xbe, 0xa3, 0xf0, 0xed, 0xca, 0xd7,
        0x35, 0x28, 0x0f, 0x12, 0x41, 0x5c, 0x7b, 0x66, 0xdd, 0xc0, 0xe7, 0xfa, 0xa9, 0xb4, 0x93, 0x8e,
        0xf8, 0xe5, 0xc2, 0xdf, 0x8c, 0x91, 0xb6, 0xab, 0x10, 0x0d, 0x2a, 0x37, 0x64, 0x79, 0x5e, 0x43,
        0xb2, 0xaf, 0x88, 0x95, 0xc6, 0xdb, 0xfc, 0xe1, 0x5a, 0x47, 0x60, 0x7d, 0x2e, 0x33, 0x14, 0x09,
        0x7f, 0x62, 0x45, 0x58, 0x0b, 0x16, 0x31, 0x2c, 0x97, 0x8a, 0xad, 0xb0, 0xe3, 0xfe, 0xd9, 0xc4,
    ];
    let nch: c_int = 1 as c_int
        + ((*frame).mode as c_uint != SBC_MODE_MONO as c_int as c_uint)
        as c_int;
    let nsb: c_int = (*frame).nsubbands;
    let nbit: c_uint = (nch * nsb * 4 as c_int
        + (if (*frame).mode as c_uint
        == SBC_MODE_JOINT_STEREO as c_int as c_uint
    {
        nsb
    } else {
        0 as c_int
    })) as c_uint;
    if size
        < ((4 as c_int * 8 as c_int) as c_uint)
        .wrapping_add(nbit)
        .wrapping_add(7 as c_int as c_uint) >> 3 as c_int
    {
        return -(1 as c_int);
    }
    let mut crc: uint8_t = 0xf as c_int as uint8_t;
    crc = t[(crc as c_int ^ *data.offset(1 as c_int as isize) as c_int)
        as usize];
    crc = t[(crc as c_int ^ *data.offset(2 as c_int as isize) as c_int)
        as usize];
    let mut i = 4 as c_int as c_uint;
    while i
        < (4 as c_int as c_uint)
        .wrapping_add(nbit.wrapping_div(8 as c_int as c_uint))
    {
        crc = t[(crc as c_int ^ *data.offset(i as isize) as c_int) as usize];
        i = i.wrapping_add(1);
    }
    if nbit.wrapping_rem(8 as c_int as c_uint) != 0 {
        crc = ((crc as c_int) << 4 as c_int
            ^ t[(crc as c_int >> 4 as c_int
            ^ *data.offset(i as isize) as c_int >> 4 as c_int) as usize]
            as c_int) as uint8_t;
    }
    crc as c_int
}
unsafe extern "C" fn check_frame(frame: *const sbc_frame) -> bool {
    if ((*frame).nblocks - 4 as c_int) as c_uint
        > 12 as c_int as c_uint
        || !(*frame).msbc && (*frame).nblocks % 4 as c_int != 0 as c_int
    {
        return 0 as c_int != 0;
    }
    if ((*frame).nsubbands - 4 as c_int) as c_uint
        > 4 as c_int as c_uint
        || (*frame).nsubbands % 4 as c_int != 0 as c_int
    {
        return 0 as c_int != 0;
    }
    let two_channels: bool = (*frame).mode as c_uint
        != SBC_MODE_MONO as c_int as c_uint;
    let dual_mode: bool = (*frame).mode as c_uint
        == SBC_MODE_DUAL_CHANNEL as c_int as c_uint;
    let joint_mode: bool = (*frame).mode as c_uint
        == SBC_MODE_JOINT_STEREO as c_int as c_uint;
    let stereo_mode: bool = joint_mode as c_int != 0
        || (*frame).mode as c_uint
        == SBC_MODE_STEREO as c_int as c_uint;
    let max_bits: c_int = ((16 as c_int * (*frame).nsubbands
        * (*frame).nblocks) << two_channels as c_int)
        - 4 as c_int * 8 as c_int
        - ((4 as c_int * (*frame).nsubbands) << two_channels as c_int)
        - (if joint_mode as c_int != 0 {
        (*frame).nsubbands
    } else {
        0 as c_int
    });
    let max_bitpool: c_int = if max_bits
        / ((*frame).nblocks << dual_mode as c_int)
        < ((16 as c_int) << stereo_mode as c_int) * (*frame).nsubbands
    {
        max_bits / ((*frame).nblocks << dual_mode as c_int)
    } else {
        ((16 as c_int) << stereo_mode as c_int) * (*frame).nsubbands
    };
    (*frame).bitpool <= max_bitpool
}
unsafe extern "C" fn compute_nbits(
    frame: *const sbc_frame,
    scale_factors: *const [c_int; 8],
    nbits: *mut [c_int; 8],
) {
    static mut loudness_offset_4: [[c_int; 4]; 4] = [
        [-(1 as c_int), 0 as c_int, 0 as c_int, 0 as c_int],
        [-(2 as c_int), 0 as c_int, 0 as c_int, 1 as c_int],
        [-(2 as c_int), 0 as c_int, 0 as c_int, 1 as c_int],
        [-(2 as c_int), 0 as c_int, 0 as c_int, 1 as c_int],
    ];
    static mut loudness_offset_8: [[c_int; 8]; 4] = [
        [
            -(2 as c_int),
            0 as c_int,
            0 as c_int,
            0 as c_int,
            0 as c_int,
            0 as c_int,
            0 as c_int,
            1 as c_int,
        ],
        [
            -(3 as c_int),
            0 as c_int,
            0 as c_int,
            0 as c_int,
            0 as c_int,
            0 as c_int,
            1 as c_int,
            2 as c_int,
        ],
        [
            -(4 as c_int),
            0 as c_int,
            0 as c_int,
            0 as c_int,
            0 as c_int,
            0 as c_int,
            1 as c_int,
            2 as c_int,
        ],
        [
            -(4 as c_int),
            0 as c_int,
            0 as c_int,
            0 as c_int,
            0 as c_int,
            0 as c_int,
            1 as c_int,
            2 as c_int,
        ],
    ];
    let loudness_offset: *const c_int = if (*frame).nsubbands
        == 4 as c_int
    {
        (loudness_offset_4[(*frame).freq as usize]).as_ptr()
    } else {
        (loudness_offset_8[(*frame).freq as usize]).as_ptr()
    };
    let stereo_mode: bool = (*frame).mode as c_uint
        == SBC_MODE_STEREO as c_int as c_uint
        || (*frame).mode as c_uint
        == SBC_MODE_JOINT_STEREO as c_int as c_uint;
    let nsubbands: c_int = (*frame).nsubbands;
    let nchannels: c_int = 1 as c_int + stereo_mode as c_int;
    let mut bitneeds: [[c_int; 8]; 2] = [[0; 8]; 2];
    let mut max_bitneed: c_int = 0 as c_int;
    let mut ich: c_int = 0 as c_int;
    while ich < nchannels {
        let mut isb: c_int = 0 as c_int;
        while isb < nsubbands {
            let mut bitneed: c_int;
            let scf: c_int = (*scale_factors
                .offset(ich as isize))[isb as usize];
            if (*frame).bam as c_uint
                == SBC_BAM_LOUDNESS as c_int as c_uint
            {
                bitneed = if scf != 0 {
                    scf - *loudness_offset.offset(isb as isize)
                } else {
                    -(5 as c_int)
                };
                bitneed >>= (bitneed > 0 as c_int) as c_int;
            } else {
                bitneed = scf;
            }
            if bitneed > max_bitneed {
                max_bitneed = bitneed;
            }
            bitneeds[ich as usize][isb as usize] = bitneed;
            isb += 1;
        }
        ich += 1;
    }
    let bitpool: c_int = (*frame).bitpool;
    let mut bitcount: c_int = 0 as c_int;
    let mut bitslice: c_int = max_bitneed + 1 as c_int;
    let mut bc: c_int = 0 as c_int;
    while bc < bitpool {
        let fresh0 = bitslice;
        bitslice -= 1;
        let bs: c_int = fresh0;
        bitcount = bc;
        if bitcount == bitpool {
            break;
        }
        let mut ich_0: c_int = 0 as c_int;
        while ich_0 < nchannels {
            let mut isb_0: c_int = 0 as c_int;
            while isb_0 < nsubbands {
                let bn: c_int = bitneeds[ich_0 as usize][isb_0 as usize];
                bc
                    += (bn >= bs && bn < bs + 15 as c_int) as c_int
                    + (bn == bs) as c_int;
                isb_0 += 1;
            }
            ich_0 += 1;
        }
    }
    let mut ich_1: c_int = 0 as c_int;
    while ich_1 < nchannels {
        let mut isb_1: c_int = 0 as c_int;
        while isb_1 < nsubbands {
            let nbit: c_int = bitneeds[ich_1 as usize][isb_1 as usize]
                - bitslice;
            (*nbits
                .offset(
                    ich_1 as isize,
                ))[isb_1
                as usize] = if nbit < 2 as c_int {
                0 as c_int
            } else if nbit > 16 as c_int {
                16 as c_int
            } else {
                nbit
            };
            isb_1 += 1;
        }
        ich_1 += 1;
    }
    let mut isb_2: c_int = 0 as c_int;
    while isb_2 < nsubbands && bitcount < bitpool {
        let mut ich_2: c_int = 0 as c_int;
        while ich_2 < nchannels && bitcount < bitpool {
            let n: c_int = if (*nbits.offset(ich_2 as isize))[isb_2 as usize]
                != 0
                && (*nbits.offset(ich_2 as isize))[isb_2 as usize] < 16 as c_int
            {
                1 as c_int
            } else if bitneeds[ich_2 as usize][isb_2 as usize]
                == bitslice + 1 as c_int && bitpool > bitcount + 1 as c_int
            {
                2 as c_int
            } else {
                0 as c_int
            };
            (*nbits.offset(ich_2 as isize))[isb_2 as usize] += n;
            bitcount += n;
            ich_2 += 1;
        }
        isb_2 += 1;
    }
    let mut isb_3: c_int = 0 as c_int;
    while isb_3 < nsubbands && bitcount < bitpool {
        let mut ich_3: c_int = 0 as c_int;
        while ich_3 < nchannels && bitcount < bitpool {
            let n_0: c_int = ((*nbits.offset(ich_3 as isize))[isb_3 as usize]
                < 16 as c_int) as c_int;
            (*nbits.offset(ich_3 as isize))[isb_3 as usize] += n_0;
            bitcount += n_0;
            ich_3 += 1;
        }
        isb_3 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sbc_get_freq_hz(freq: sbc_freq) -> c_int {
    static mut freq_hz: [c_int; 4] = [
        16000 as c_int,
        32000 as c_int,
        44100 as c_int,
        48000 as c_int,
    ];
    freq_hz[freq as usize]
}
#[no_mangle]
pub unsafe extern "C" fn sbc_get_frame_size(
    frame: *const sbc_frame,
) -> c_uint {
    if !check_frame(frame) {
        return 0 as c_int as c_uint;
    }
    let two_channels: bool = (*frame).mode as c_uint
        != SBC_MODE_MONO as c_int as c_uint;
    let dual_mode: bool = (*frame).mode as c_uint
        == SBC_MODE_DUAL_CHANNEL as c_int as c_uint;
    let joint_mode: bool = (*frame).mode as c_uint
        == SBC_MODE_JOINT_STEREO as c_int as c_uint;
    let nbits: c_uint = (((4 as c_int * (*frame).nsubbands) << two_channels as c_int)
        + (((*frame).nblocks * (*frame).bitpool) << dual_mode as c_int)
        + (if joint_mode as c_int != 0 {
        (*frame).nsubbands
    } else {
        0 as c_int
    })) as c_uint;
    (4 as c_int as c_uint)
        .wrapping_add(
            nbits.wrapping_add(7 as c_int as c_uint) >> 3 as c_int,
        )
}
#[no_mangle]
pub unsafe extern "C" fn sbc_get_frame_bitrate(
    frame: *const sbc_frame,
) -> c_uint {
    if !check_frame(frame) {
        return 0 as c_int as c_uint;
    }
    let nsamples: c_uint = ((*frame).nblocks * (*frame).nsubbands)
        as c_uint;
    let nbits: c_uint = (8 as c_int as c_uint)
        .wrapping_mul(sbc_get_frame_size(frame));
    nbits
        .wrapping_mul(sbc_get_freq_hz((*frame).freq) as c_uint)
        .wrapping_div(nsamples)
}
#[no_mangle]
pub unsafe extern "C" fn sbc_get_frame_bps(freq: sbc_freq) -> c_int {
    static mut freq_hz: [c_int; 4] = [
        16000 as c_int,
        32000 as c_int,
        44100 as c_int,
        48000 as c_int,
    ];
    freq_hz[freq as usize]
}
#[no_mangle]
pub unsafe extern "C" fn sbc_reset(sbc: *mut sbc) {
    *sbc = {

        sbc {
            nchannels: 0,
            nblocks: 0,
            nsubbands: 0,
            c2rust_unnamed: C2RustUnnamed {
                dstates: [sbc_dstate {
                    idx: 0,
                    v: [[[0; 10]; 8]; 2],
                }; 2],
            },
        }
    };
}
unsafe extern "C" fn decode_header(
    bits: &mut Bits,
    frame: *mut sbc_frame,
    crc: *mut c_int,
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
    let syncword = bits.get_bits(8);
    (*frame).msbc = syncword == 0xad;
    if (*frame).msbc {
        bits.advance(16);
        *frame = msbc_frame;
    } else if syncword == 0x9c {
        (*frame).freq = dec_freq[bits.get_bits(2) as usize];
        (*frame).nblocks = ((1 + bits.get_bits(2)) << 2) as c_int;
        (*frame).mode = dec_mode[bits.get_bits(2) as usize];
        (*frame).bam = dec_bam[bits.get_bits(1) as usize];
        (*frame).nsubbands = ((1 + bits.get_bits(1)) << 2) as c_int;
        (*frame).bitpool = bits.get_bits(8) as c_int;
    } else {
        return false
    }
    if !crc.is_null() {
        *crc = bits.get_bits(8) as c_int;
    }
    check_frame(frame)
}
unsafe extern "C" fn decode_frame(
    bits: &mut Bits,
    frame: *const sbc_frame,
    sb_samples: *mut [int16_t; 128],
    sb_scale: *mut c_int,
) {
    static mut range_scale: [c_int; 16] = [
        0xfffffff as c_int,
        0x5555556 as c_int,
        0x2492492 as c_int,
        0x1111111 as c_int,
        0x842108 as c_int,
        0x410410 as c_int,
        0x204081 as c_int,
        0x101010 as c_int,
        0x80402 as c_int,
        0x40100 as c_int,
        0x20040 as c_int,
        0x10010 as c_int,
        0x8004 as c_int,
        0x4001 as c_int,
        0x2000 as c_int,
        0x1000 as c_int,
    ];
    let mut mjoint: c_uint = 0;
    if (*frame).mode as c_uint
        == SBC_MODE_JOINT_STEREO as c_int as c_uint
        && (*frame).nsubbands == 4 as c_int
    {
        let v: c_uint = bits.get_bits(4);
        mjoint = ((0 as c_int) << 3 as c_int) as c_uint
            | (v & 0x2 as c_int as c_uint) << 1 as c_int
            | (v & 0x4 as c_int as c_uint) >> 1 as c_int
            | (v & 0x8 as c_int as c_uint) >> 3 as c_int;
    } else if (*frame).mode as c_uint
        == SBC_MODE_JOINT_STEREO as c_int as c_uint
    {
        let v_0 = bits.get_bits(8) as c_uint;
        mjoint = ((0 as c_int) << 7 as c_int) as c_uint
            | (v_0 & 0x2 as c_int as c_uint) << 5 as c_int
            | (v_0 & 0x4 as c_int as c_uint) << 3 as c_int
            | (v_0 & 0x8 as c_int as c_uint) << 1 as c_int
            | (v_0 & 0x10 as c_int as c_uint) >> 1 as c_int
            | (v_0 & 0x20 as c_int as c_uint) >> 3 as c_int
            | (v_0 & 0x40 as c_int as c_uint) >> 5 as c_int
            | (v_0 & 0x80 as c_int as c_uint) >> 7 as c_int;
    }
    let nchannels: c_int = 1 as c_int
        + ((*frame).mode as c_uint != SBC_MODE_MONO as c_int as c_uint)
        as c_int;
    let nsubbands: c_int = (*frame).nsubbands;
    let mut scale_factors: [[c_int; 8]; 2] = [[0; 8]; 2];
    let mut nbits: [[c_int; 8]; 2] = [[0; 8]; 2];
    let mut ich: c_int = 0 as c_int;
    while ich < nchannels {
        let mut isb: c_int = 0 as c_int;
        while isb < nsubbands {
            scale_factors[ich as usize][isb as usize] = bits.get_bits(4) as c_int;
            isb += 1;
        }
        ich += 1;
    }
    compute_nbits(
        frame,
        scale_factors.as_mut_ptr() as *const [c_int; 8],
        nbits.as_mut_ptr(),
    );
    if (*frame).mode as c_uint
        == SBC_MODE_DUAL_CHANNEL as c_int as c_uint
    {
        compute_nbits(
            frame,
            scale_factors.as_mut_ptr().offset(1 as c_int as isize)
                as *const [c_int; 8],
            nbits.as_mut_ptr().offset(1 as c_int as isize),
        );
    }
    let mut ich_0: c_int = 0 as c_int;
    while ich_0 < nchannels {
        let mut max_scf: c_int = 0 as c_int;
        let mut isb_0: c_int = 0 as c_int;
        while isb_0 < nsubbands {
            let scf: c_int = (scale_factors[ich_0 as usize][isb_0 as usize]
                as c_uint)
                .wrapping_add(mjoint >> isb_0 & 1 as c_int as c_uint)
                as c_int;
            if scf > max_scf {
                max_scf = scf;
            }
            isb_0 += 1;
        }
        *sb_scale
            .offset(
                ich_0 as isize,
            ) = 15 as c_int - max_scf - (17 as c_int - 16 as c_int);
        ich_0 += 1;
    }
    if (*frame).mode as c_uint
        == SBC_MODE_JOINT_STEREO as c_int as c_uint
    {
        let fresh1 = &mut (*sb_scale.offset(1 as c_int as isize));
        *fresh1 = if *sb_scale.offset(0 as c_int as isize)
            < *sb_scale.offset(1 as c_int as isize)
        {
            *sb_scale.offset(0 as c_int as isize)
        } else {
            *sb_scale.offset(1 as c_int as isize)
        };
        *sb_scale.offset(0 as c_int as isize) = *fresh1;
    }
    let mut iblk: c_int = 0 as c_int;
    while iblk < (*frame).nblocks {
        let mut ich_1: c_int = 0 as c_int;
        while ich_1 < nchannels {
            let mut p_sb_samples: *mut int16_t = (*sb_samples.offset(ich_1 as isize))
                .as_mut_ptr()
                .offset((iblk * nsubbands) as isize);
            let mut isb_1: c_int = 0 as c_int;
            while isb_1 < nsubbands {
                let nbit: c_int = nbits[ich_1 as usize][isb_1 as usize];
                let scf_0: c_int = scale_factors[ich_1
                    as usize][isb_1 as usize];
                if nbit == 0 {
                    let fresh2 = p_sb_samples;
                    p_sb_samples = p_sb_samples.offset(1);
                    *fresh2 = 0 as c_int as int16_t;
                } else {
                    let mut s = bits.get_bits(nbit as _) as c_int;
                    s = (s << 1 as c_int | 1 as c_int)
                        * range_scale[(nbit - 1 as c_int) as usize];
                    let fresh3 = p_sb_samples;
                    p_sb_samples = p_sb_samples.offset(1);
                    *fresh3 = ((s - ((1 as c_int) << 28 as c_int)) >> (28 as c_int
                        - (scf_0 + 1 as c_int
                        + *sb_scale.offset(ich_1 as isize)))) as int16_t;
                }
                isb_1 += 1;
            }
            ich_1 += 1;
        }
        iblk += 1;
    }
    let mut isb_2: c_int = 0 as c_int;
    while isb_2 < nsubbands {
        if mjoint >> isb_2 & 1 as c_int as c_uint != 0 as c_int as c_uint
        {
            let mut iblk_0: c_int = 0 as c_int;
            while iblk_0 < (*frame).nblocks {
                let s0: int16_t = (*sb_samples
                    .offset(
                        0 as c_int as isize,
                    ))[(iblk_0 * nsubbands + isb_2) as usize];
                let s1: int16_t = (*sb_samples
                    .offset(
                        1 as c_int as isize,
                    ))[(iblk_0 * nsubbands + isb_2) as usize];
                (*sb_samples
                    .offset(
                        0 as c_int as isize,
                    ))[(iblk_0 * nsubbands + isb_2)
                    as usize] = (s0 as c_int + s1 as c_int) as int16_t;
                (*sb_samples
                    .offset(
                        1 as c_int as isize,
                    ))[(iblk_0 * nsubbands + isb_2)
                    as usize] = (s0 as c_int - s1 as c_int) as int16_t;
                iblk_0 += 1;
            }
        }
        isb_2 += 1;
    }
    let padding_nbits = 8 - bits.pos() % 8;
    if padding_nbits < 8 {
        bits.get_bits_fixed(padding_nbits as u32, 0);
    }
}
#[inline]
unsafe extern "C" fn dct4(
    in_0: *const int16_t,
    scale: c_int,
    out0: *mut [int16_t; 10],
    out1: *mut [int16_t; 10],
    idx: c_int,
) {
    static mut cos8: [int16_t; 4] = [
        8192 as c_int as int16_t,
        7568 as c_int as int16_t,
        5793 as c_int as int16_t,
        3135 as c_int as int16_t,
    ];
    let s03: int16_t = ((*in_0.offset(0 as c_int as isize) as c_int
        + *in_0.offset(3 as c_int as isize) as c_int) >> 1 as c_int)
        as int16_t;
    let d03: int16_t = ((*in_0.offset(0 as c_int as isize) as c_int
        - *in_0.offset(3 as c_int as isize) as c_int) >> 1 as c_int)
        as int16_t;
    let s12: int16_t = ((*in_0.offset(1 as c_int as isize) as c_int
        + *in_0.offset(2 as c_int as isize) as c_int) >> 1 as c_int)
        as int16_t;
    let d12: int16_t = ((*in_0.offset(1 as c_int as isize) as c_int
        - *in_0.offset(2 as c_int as isize) as c_int) >> 1 as c_int)
        as int16_t;
    let mut a0: c_int = (s03 as c_int - s12 as c_int)
        * cos8[2 as c_int as usize] as c_int;
    let mut b1: c_int = -(s03 as c_int + s12 as c_int)
        << 13 as c_int;
    let mut a1: c_int = d03 as c_int
        * cos8[3 as c_int as usize] as c_int
        - d12 as c_int * cos8[1 as c_int as usize] as c_int;
    let mut b0: c_int = -(d03 as c_int)
        * cos8[1 as c_int as usize] as c_int
        - d12 as c_int * cos8[3 as c_int as usize] as c_int;
    let shr: c_int = 12 as c_int + scale;
    a0 = (a0 + ((1 as c_int) << (shr - 1 as c_int))) >> shr;
    b0 = (b0 + ((1 as c_int) << (shr - 1 as c_int))) >> shr;
    a1 = (a1 + ((1 as c_int) << (shr - 1 as c_int))) >> shr;
    b1 = (b1 + ((1 as c_int) << (shr - 1 as c_int))) >> shr;
    (*out0
        .offset(
            0 as c_int as isize,
        ))[idx
        as usize] = (if a0 > 32767 as c_int {
        32767 as c_int
    } else if a0 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        a0
    }) as int16_t;
    (*out0
        .offset(
            3 as c_int as isize,
        ))[idx
        as usize] = (if -a1 > 32767 as c_int {
        32767 as c_int
    } else if -a1 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        -a1
    }) as int16_t;
    (*out0
        .offset(
            1 as c_int as isize,
        ))[idx
        as usize] = (if a1 > 32767 as c_int {
        32767 as c_int
    } else if a1 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        a1
    }) as int16_t;
    (*out0
        .offset(
            2 as c_int as isize,
        ))[idx
        as usize] = (if 0 as c_int > 32767 as c_int {
        32767 as c_int
    } else if (0 as c_int) < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        0 as c_int
    }) as int16_t;
    (*out1
        .offset(
            0 as c_int as isize,
        ))[idx
        as usize] = (if -a0 > 32767 as c_int {
        32767 as c_int
    } else if -a0 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        -a0
    }) as int16_t;
    (*out1
        .offset(
            3 as c_int as isize,
        ))[idx
        as usize] = (if b0 > 32767 as c_int {
        32767 as c_int
    } else if b0 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        b0
    }) as int16_t;
    (*out1
        .offset(
            1 as c_int as isize,
        ))[idx
        as usize] = (if b0 > 32767 as c_int {
        32767 as c_int
    } else if b0 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        b0
    }) as int16_t;
    (*out1
        .offset(
            2 as c_int as isize,
        ))[idx
        as usize] = (if b1 > 32767 as c_int {
        32767 as c_int
    } else if b1 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        b1
    }) as int16_t;
}
#[inline]
unsafe extern "C" fn dct8(
    in_0: *const int16_t,
    scale: c_int,
    out0: *mut [int16_t; 10],
    out1: *mut [int16_t; 10],
    idx: c_int,
) {
    static mut cos16: [int16_t; 8] = [
        8192 as c_int as int16_t,
        8035 as c_int as int16_t,
        7568 as c_int as int16_t,
        6811 as c_int as int16_t,
        5793 as c_int as int16_t,
        4551 as c_int as int16_t,
        3135 as c_int as int16_t,
        1598 as c_int as int16_t,
    ];
    let s07: int16_t = ((*in_0.offset(0 as c_int as isize) as c_int
        + *in_0.offset(7 as c_int as isize) as c_int) >> 1 as c_int)
        as int16_t;
    let d07: int16_t = ((*in_0.offset(0 as c_int as isize) as c_int
        - *in_0.offset(7 as c_int as isize) as c_int) >> 1 as c_int)
        as int16_t;
    let s16: int16_t = ((*in_0.offset(1 as c_int as isize) as c_int
        + *in_0.offset(6 as c_int as isize) as c_int) >> 1 as c_int)
        as int16_t;
    let d16: int16_t = ((*in_0.offset(1 as c_int as isize) as c_int
        - *in_0.offset(6 as c_int as isize) as c_int) >> 1 as c_int)
        as int16_t;
    let s25: int16_t = ((*in_0.offset(2 as c_int as isize) as c_int
        + *in_0.offset(5 as c_int as isize) as c_int) >> 1 as c_int)
        as int16_t;
    let d25: int16_t = ((*in_0.offset(2 as c_int as isize) as c_int
        - *in_0.offset(5 as c_int as isize) as c_int) >> 1 as c_int)
        as int16_t;
    let s34: int16_t = ((*in_0.offset(3 as c_int as isize) as c_int
        + *in_0.offset(4 as c_int as isize) as c_int) >> 1 as c_int)
        as int16_t;
    let d34: int16_t = ((*in_0.offset(3 as c_int as isize) as c_int
        - *in_0.offset(4 as c_int as isize) as c_int) >> 1 as c_int)
        as int16_t;
    let mut a0: c_int = (s07 as c_int + s34 as c_int
        - (s25 as c_int + s16 as c_int))
        * cos16[4 as c_int as usize] as c_int;
    let mut b3: c_int = (-(s07 as c_int + s34 as c_int)
        - (s25 as c_int + s16 as c_int)) << 13 as c_int;
    let mut a2: c_int = (s07 as c_int - s34 as c_int)
        * cos16[6 as c_int as usize] as c_int
        + (s25 as c_int - s16 as c_int)
        * cos16[2 as c_int as usize] as c_int;
    let mut b1: c_int = (s34 as c_int - s07 as c_int)
        * cos16[2 as c_int as usize] as c_int
        + (s25 as c_int - s16 as c_int)
        * cos16[6 as c_int as usize] as c_int;
    let mut a1: c_int = d07 as c_int
        * cos16[5 as c_int as usize] as c_int
        - d16 as c_int * cos16[1 as c_int as usize] as c_int
        + d25 as c_int * cos16[7 as c_int as usize] as c_int
        + d34 as c_int * cos16[3 as c_int as usize] as c_int;
    let mut b2: c_int = -(d07 as c_int)
        * cos16[1 as c_int as usize] as c_int
        - d16 as c_int * cos16[3 as c_int as usize] as c_int
        - d25 as c_int * cos16[5 as c_int as usize] as c_int
        - d34 as c_int * cos16[7 as c_int as usize] as c_int;
    let mut a3: c_int = d07 as c_int
        * cos16[7 as c_int as usize] as c_int
        - d16 as c_int * cos16[5 as c_int as usize] as c_int
        + d25 as c_int * cos16[3 as c_int as usize] as c_int
        - d34 as c_int * cos16[1 as c_int as usize] as c_int;
    let mut b0: c_int = -(d07 as c_int)
        * cos16[3 as c_int as usize] as c_int
        + d16 as c_int * cos16[7 as c_int as usize] as c_int
        + d25 as c_int * cos16[1 as c_int as usize] as c_int
        + d34 as c_int * cos16[5 as c_int as usize] as c_int;
    let shr: c_int = 12 as c_int + scale;
    a0 = (a0 + ((1 as c_int) << (shr - 1 as c_int))) >> shr;
    b0 = (b0 + ((1 as c_int) << (shr - 1 as c_int))) >> shr;
    a1 = (a1 + ((1 as c_int) << (shr - 1 as c_int))) >> shr;
    b1 = (b1 + ((1 as c_int) << (shr - 1 as c_int))) >> shr;
    a2 = (a2 + ((1 as c_int) << (shr - 1 as c_int))) >> shr;
    b2 = (b2 + ((1 as c_int) << (shr - 1 as c_int))) >> shr;
    a3 = (a3 + ((1 as c_int) << (shr - 1 as c_int))) >> shr;
    b3 = (b3 + ((1 as c_int) << (shr - 1 as c_int))) >> shr;
    (*out0
        .offset(
            0 as c_int as isize,
        ))[idx
        as usize] = (if a0 > 32767 as c_int {
        32767 as c_int
    } else if a0 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        a0
    }) as int16_t;
    (*out0
        .offset(
            7 as c_int as isize,
        ))[idx
        as usize] = (if -a1 > 32767 as c_int {
        32767 as c_int
    } else if -a1 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        -a1
    }) as int16_t;
    (*out0
        .offset(
            1 as c_int as isize,
        ))[idx
        as usize] = (if a1 > 32767 as c_int {
        32767 as c_int
    } else if a1 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        a1
    }) as int16_t;
    (*out0
        .offset(
            6 as c_int as isize,
        ))[idx
        as usize] = (if -a2 > 32767 as c_int {
        32767 as c_int
    } else if -a2 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        -a2
    }) as int16_t;
    (*out0
        .offset(
            2 as c_int as isize,
        ))[idx
        as usize] = (if a2 > 32767 as c_int {
        32767 as c_int
    } else if a2 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        a2
    }) as int16_t;
    (*out0
        .offset(
            5 as c_int as isize,
        ))[idx
        as usize] = (if -a3 > 32767 as c_int {
        32767 as c_int
    } else if -a3 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        -a3
    }) as int16_t;
    (*out0
        .offset(
            3 as c_int as isize,
        ))[idx
        as usize] = (if a3 > 32767 as c_int {
        32767 as c_int
    } else if a3 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        a3
    }) as int16_t;
    (*out0
        .offset(
            4 as c_int as isize,
        ))[idx
        as usize] = (if 0 as c_int > 32767 as c_int {
        32767 as c_int
    } else if (0 as c_int) < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        0 as c_int
    }) as int16_t;
    (*out1
        .offset(
            0 as c_int as isize,
        ))[idx
        as usize] = (if -a0 > 32767 as c_int {
        32767 as c_int
    } else if -a0 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        -a0
    }) as int16_t;
    (*out1
        .offset(
            7 as c_int as isize,
        ))[idx
        as usize] = (if b0 > 32767 as c_int {
        32767 as c_int
    } else if b0 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        b0
    }) as int16_t;
    (*out1
        .offset(
            1 as c_int as isize,
        ))[idx
        as usize] = (if b0 > 32767 as c_int {
        32767 as c_int
    } else if b0 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        b0
    }) as int16_t;
    (*out1
        .offset(
            6 as c_int as isize,
        ))[idx
        as usize] = (if b1 > 32767 as c_int {
        32767 as c_int
    } else if b1 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        b1
    }) as int16_t;
    (*out1
        .offset(
            2 as c_int as isize,
        ))[idx
        as usize] = (if b1 > 32767 as c_int {
        32767 as c_int
    } else if b1 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        b1
    }) as int16_t;
    (*out1
        .offset(
            5 as c_int as isize,
        ))[idx
        as usize] = (if b2 > 32767 as c_int {
        32767 as c_int
    } else if b2 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        b2
    }) as int16_t;
    (*out1
        .offset(
            3 as c_int as isize,
        ))[idx
        as usize] = (if b2 > 32767 as c_int {
        32767 as c_int
    } else if b2 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        b2
    }) as int16_t;
    (*out1
        .offset(
            4 as c_int as isize,
        ))[idx
        as usize] = (if b3 > 32767 as c_int {
        32767 as c_int
    } else if b3 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        b3
    }) as int16_t;
}
#[inline]
unsafe extern "C" fn apply_window(
    in_0: *const [int16_t; 10],
    n: c_int,
    window: *const [int16_t; 20],
    offset: c_int,
    mut out: *mut int16_t,
    pitch: c_int,
) {
    let mut u: *const int16_t = in_0 as *const int16_t;
    let mut i: c_int = 0 as c_int;
    while i < n {
        let mut w: *const int16_t = (*window.offset(i as isize))
            .as_ptr()
            .offset(offset as isize);
        let mut s: c_int;
        let fresh4 = u;
        u = u.offset(1);
        let fresh5 = w;
        w = w.offset(1);
        s = *fresh4 as c_int * *fresh5 as c_int;
        let fresh6 = u;
        u = u.offset(1);
        let fresh7 = w;
        w = w.offset(1);
        s += *fresh6 as c_int * *fresh7 as c_int;
        let fresh8 = u;
        u = u.offset(1);
        let fresh9 = w;
        w = w.offset(1);
        s += *fresh8 as c_int * *fresh9 as c_int;
        let fresh10 = u;
        u = u.offset(1);
        let fresh11 = w;
        w = w.offset(1);
        s += *fresh10 as c_int * *fresh11 as c_int;
        let fresh12 = u;
        u = u.offset(1);
        let fresh13 = w;
        w = w.offset(1);
        s += *fresh12 as c_int * *fresh13 as c_int;
        let fresh14 = u;
        u = u.offset(1);
        let fresh15 = w;
        w = w.offset(1);
        s += *fresh14 as c_int * *fresh15 as c_int;
        let fresh16 = u;
        u = u.offset(1);
        let fresh17 = w;
        w = w.offset(1);
        s += *fresh16 as c_int * *fresh17 as c_int;
        let fresh18 = u;
        u = u.offset(1);
        let fresh19 = w;
        w = w.offset(1);
        s += *fresh18 as c_int * *fresh19 as c_int;
        let fresh20 = u;
        u = u.offset(1);
        let fresh21 = w;
        w = w.offset(1);
        s += *fresh20 as c_int * *fresh21 as c_int;
        let fresh22 = u;
        u = u.offset(1);
        let fresh23 = w;
        s += *fresh22 as c_int * *fresh23 as c_int;
        *out = (if (s + ((1 as c_int) << 12 as c_int)) >> 13 as c_int
            > 32767 as c_int
        {
            32767 as c_int
        } else if ((s + ((1 as c_int) << 12 as c_int)) >> 13 as c_int)
            < -(32767 as c_int) - 1 as c_int
        {
            -(32767 as c_int) - 1 as c_int
        } else {
            (s + ((1 as c_int) << 12 as c_int)) >> 13 as c_int
        }) as int16_t;
        out = out.offset(pitch as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sbc_synthesize_4_c(
    state: *mut sbc_dstate,
    in_0: *const int16_t,
    scale: c_int,
    out: *mut int16_t,
    pitch: c_int,
) {
    static mut window: [[int16_t; 20]; 4] = [
        [
            0 as c_int as int16_t,
            -(126 as c_int) as int16_t,
            -(358 as c_int) as int16_t,
            -(848 as c_int) as int16_t,
            -(4443 as c_int) as int16_t,
            -(9644 as c_int) as int16_t,
            4443 as c_int as int16_t,
            -(848 as c_int) as int16_t,
            358 as c_int as int16_t,
            -(126 as c_int) as int16_t,
            0 as c_int as int16_t,
            -(126 as c_int) as int16_t,
            -(358 as c_int) as int16_t,
            -(848 as c_int) as int16_t,
            -(4443 as c_int) as int16_t,
            -(9644 as c_int) as int16_t,
            4443 as c_int as int16_t,
            -(848 as c_int) as int16_t,
            358 as c_int as int16_t,
            -(126 as c_int) as int16_t,
        ],
        [
            -(18 as c_int) as int16_t,
            -(128 as c_int) as int16_t,
            -(670 as c_int) as int16_t,
            -(201 as c_int) as int16_t,
            -(6389 as c_int) as int16_t,
            -(9235 as c_int) as int16_t,
            2544 as c_int as int16_t,
            -(1055 as c_int) as int16_t,
            100 as c_int as int16_t,
            -(90 as c_int) as int16_t,
            -(18 as c_int) as int16_t,
            -(128 as c_int) as int16_t,
            -(670 as c_int) as int16_t,
            -(201 as c_int) as int16_t,
            -(6389 as c_int) as int16_t,
            -(9235 as c_int) as int16_t,
            2544 as c_int as int16_t,
            -(1055 as c_int) as int16_t,
            100 as c_int as int16_t,
            -(90 as c_int) as int16_t,
        ],
        [
            -(49 as c_int) as int16_t,
            -(61 as c_int) as int16_t,
            -(946 as c_int) as int16_t,
            944 as c_int as int16_t,
            -(8082 as c_int) as int16_t,
            -(8082 as c_int) as int16_t,
            944 as c_int as int16_t,
            -(946 as c_int) as int16_t,
            -(61 as c_int) as int16_t,
            -(49 as c_int) as int16_t,
            -(49 as c_int) as int16_t,
            -(61 as c_int) as int16_t,
            -(946 as c_int) as int16_t,
            944 as c_int as int16_t,
            -(8082 as c_int) as int16_t,
            -(8082 as c_int) as int16_t,
            944 as c_int as int16_t,
            -(946 as c_int) as int16_t,
            -(61 as c_int) as int16_t,
            -(49 as c_int) as int16_t,
        ],
        [
            -(90 as c_int) as int16_t,
            100 as c_int as int16_t,
            -(1055 as c_int) as int16_t,
            2544 as c_int as int16_t,
            -(9235 as c_int) as int16_t,
            -(6389 as c_int) as int16_t,
            -(201 as c_int) as int16_t,
            -(670 as c_int) as int16_t,
            -(128 as c_int) as int16_t,
            -(18 as c_int) as int16_t,
            -(90 as c_int) as int16_t,
            100 as c_int as int16_t,
            -(1055 as c_int) as int16_t,
            2544 as c_int as int16_t,
            -(9235 as c_int) as int16_t,
            -(6389 as c_int) as int16_t,
            -(201 as c_int) as int16_t,
            -(670 as c_int) as int16_t,
            -(128 as c_int) as int16_t,
            -(18 as c_int) as int16_t,
        ],
    ];
    let dct_idx: c_int = if (*state).idx != 0 {
        10 as c_int - (*state).idx
    } else {
        0 as c_int
    };
    let odd: c_int = dct_idx & 1 as c_int;
    dct4(
        in_0,
        scale,
        ((*state).v[odd as usize]).as_mut_ptr(),
        ((*state).v[(odd == 0) as c_int as usize]).as_mut_ptr(),
        dct_idx,
    );
    apply_window(
        ((*state).v[odd as usize]).as_mut_ptr() as *const [int16_t; 10],
        4 as c_int,
        window.as_ptr(),
        (*state).idx,
        out,
        pitch,
    );
    (*state)
        .idx = if (*state).idx < 9 as c_int {
        (*state).idx + 1 as c_int
    } else {
        0 as c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn sbc_synthesize_8_c(
    state: *mut sbc_dstate,
    in_0: *const int16_t,
    scale: c_int,
    out: *mut int16_t,
    pitch: c_int,
) {
    static mut window: [[int16_t; 20]; 8] = [
        [
            0 as c_int as int16_t,
            -(132 as c_int) as int16_t,
            -(371 as c_int) as int16_t,
            -(848 as c_int) as int16_t,
            -(4456 as c_int) as int16_t,
            -(9631 as c_int) as int16_t,
            4456 as c_int as int16_t,
            -(848 as c_int) as int16_t,
            371 as c_int as int16_t,
            -(132 as c_int) as int16_t,
            0 as c_int as int16_t,
            -(132 as c_int) as int16_t,
            -(371 as c_int) as int16_t,
            -(848 as c_int) as int16_t,
            -(4456 as c_int) as int16_t,
            -(9631 as c_int) as int16_t,
            4456 as c_int as int16_t,
            -(848 as c_int) as int16_t,
            371 as c_int as int16_t,
            -(132 as c_int) as int16_t,
        ],
        [
            -(10 as c_int) as int16_t,
            -(138 as c_int) as int16_t,
            -(526 as c_int) as int16_t,
            -(580 as c_int) as int16_t,
            -(5438 as c_int) as int16_t,
            -(9528 as c_int) as int16_t,
            3486 as c_int as int16_t,
            -(1004 as c_int) as int16_t,
            229 as c_int as int16_t,
            -(117 as c_int) as int16_t,
            -(10 as c_int) as int16_t,
            -(138 as c_int) as int16_t,
            -(526 as c_int) as int16_t,
            -(580 as c_int) as int16_t,
            -(5438 as c_int) as int16_t,
            -(9528 as c_int) as int16_t,
            3486 as c_int as int16_t,
            -(1004 as c_int) as int16_t,
            229 as c_int as int16_t,
            -(117 as c_int) as int16_t,
        ],
        [
            -(22 as c_int) as int16_t,
            -(131 as c_int) as int16_t,
            -(685 as c_int) as int16_t,
            -(192 as c_int) as int16_t,
            -(6395 as c_int) as int16_t,
            -(9224 as c_int) as int16_t,
            2561 as c_int as int16_t,
            -(1063 as c_int) as int16_t,
            108 as c_int as int16_t,
            -(97 as c_int) as int16_t,
            -(22 as c_int) as int16_t,
            -(131 as c_int) as int16_t,
            -(685 as c_int) as int16_t,
            -(192 as c_int) as int16_t,
            -(6395 as c_int) as int16_t,
            -(9224 as c_int) as int16_t,
            2561 as c_int as int16_t,
            -(1063 as c_int) as int16_t,
            108 as c_int as int16_t,
            -(97 as c_int) as int16_t,
        ],
        [
            -(36 as c_int) as int16_t,
            -(106 as c_int) as int16_t,
            -(835 as c_int) as int16_t,
            322 as c_int as int16_t,
            -(7287 as c_int) as int16_t,
            -(8734 as c_int) as int16_t,
            1711 as c_int as int16_t,
            -(1042 as c_int) as int16_t,
            12 as c_int as int16_t,
            -(75 as c_int) as int16_t,
            -(36 as c_int) as int16_t,
            -(106 as c_int) as int16_t,
            -(835 as c_int) as int16_t,
            322 as c_int as int16_t,
            -(7287 as c_int) as int16_t,
            -(8734 as c_int) as int16_t,
            1711 as c_int as int16_t,
            -(1042 as c_int) as int16_t,
            12 as c_int as int16_t,
            -(75 as c_int) as int16_t,
        ],
        [
            -(54 as c_int) as int16_t,
            -(59 as c_int) as int16_t,
            -(960 as c_int) as int16_t,
            959 as c_int as int16_t,
            -(8078 as c_int) as int16_t,
            -(8078 as c_int) as int16_t,
            959 as c_int as int16_t,
            -(960 as c_int) as int16_t,
            -(59 as c_int) as int16_t,
            -(54 as c_int) as int16_t,
            -(54 as c_int) as int16_t,
            -(59 as c_int) as int16_t,
            -(960 as c_int) as int16_t,
            959 as c_int as int16_t,
            -(8078 as c_int) as int16_t,
            -(8078 as c_int) as int16_t,
            959 as c_int as int16_t,
            -(960 as c_int) as int16_t,
            -(59 as c_int) as int16_t,
            -(54 as c_int) as int16_t,
        ],
        [
            -(75 as c_int) as int16_t,
            12 as c_int as int16_t,
            -(1042 as c_int) as int16_t,
            1711 as c_int as int16_t,
            -(8734 as c_int) as int16_t,
            -(7287 as c_int) as int16_t,
            322 as c_int as int16_t,
            -(835 as c_int) as int16_t,
            -(106 as c_int) as int16_t,
            -(36 as c_int) as int16_t,
            -(75 as c_int) as int16_t,
            12 as c_int as int16_t,
            -(1042 as c_int) as int16_t,
            1711 as c_int as int16_t,
            -(8734 as c_int) as int16_t,
            -(7287 as c_int) as int16_t,
            322 as c_int as int16_t,
            -(835 as c_int) as int16_t,
            -(106 as c_int) as int16_t,
            -(36 as c_int) as int16_t,
        ],
        [
            -(97 as c_int) as int16_t,
            108 as c_int as int16_t,
            -(1063 as c_int) as int16_t,
            2561 as c_int as int16_t,
            -(9224 as c_int) as int16_t,
            -(6395 as c_int) as int16_t,
            -(192 as c_int) as int16_t,
            -(685 as c_int) as int16_t,
            -(131 as c_int) as int16_t,
            -(22 as c_int) as int16_t,
            -(97 as c_int) as int16_t,
            108 as c_int as int16_t,
            -(1063 as c_int) as int16_t,
            2561 as c_int as int16_t,
            -(9224 as c_int) as int16_t,
            -(6395 as c_int) as int16_t,
            -(192 as c_int) as int16_t,
            -(685 as c_int) as int16_t,
            -(131 as c_int) as int16_t,
            -(22 as c_int) as int16_t,
        ],
        [
            -(117 as c_int) as int16_t,
            229 as c_int as int16_t,
            -(1004 as c_int) as int16_t,
            3486 as c_int as int16_t,
            -(9528 as c_int) as int16_t,
            -(5438 as c_int) as int16_t,
            -(580 as c_int) as int16_t,
            -(526 as c_int) as int16_t,
            -(138 as c_int) as int16_t,
            -(10 as c_int) as int16_t,
            -(117 as c_int) as int16_t,
            229 as c_int as int16_t,
            -(1004 as c_int) as int16_t,
            3486 as c_int as int16_t,
            -(9528 as c_int) as int16_t,
            -(5438 as c_int) as int16_t,
            -(580 as c_int) as int16_t,
            -(526 as c_int) as int16_t,
            -(138 as c_int) as int16_t,
            -(10 as c_int) as int16_t,
        ],
    ];
    let dct_idx: c_int = if (*state).idx != 0 {
        10 as c_int - (*state).idx
    } else {
        0 as c_int
    };
    let odd: c_int = dct_idx & 1 as c_int;
    dct8(
        in_0,
        scale,
        ((*state).v[odd as usize]).as_mut_ptr(),
        ((*state).v[(odd == 0) as c_int as usize]).as_mut_ptr(),
        dct_idx,
    );
    apply_window(
        ((*state).v[odd as usize]).as_mut_ptr() as *const [int16_t; 10],
        8 as c_int,
        window.as_ptr(),
        (*state).idx,
        out,
        pitch,
    );
    (*state)
        .idx = if (*state).idx < 9 as c_int {
        (*state).idx + 1 as c_int
    } else {
        0 as c_int
    };
}
#[inline]
unsafe extern "C" fn synthesize(
    state: *mut sbc_dstate,
    nblocks: c_int,
    nsubbands: c_int,
    mut in_0: *const int16_t,
    scale: c_int,
    mut out: *mut int16_t,
    pitch: c_int,
) {
    let mut iblk: c_int = 0 as c_int;
    while iblk < nblocks {
        if nsubbands == 4 as c_int {
            sbc_synthesize_4_c(state, in_0, scale, out, pitch);
        } else {
            sbc_synthesize_8_c(state, in_0, scale, out, pitch);
        }
        in_0 = in_0.offset(nsubbands as isize);
        out = out.offset((nsubbands * pitch) as isize);
        iblk += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sbc_probe(
    data: *const c_void,
    frame: *mut sbc_frame,
) -> c_int {
    let mut bits = Bits::new(Mode::Read, data as *mut u8, 4);
    if !decode_header(&mut bits, frame, std::ptr::null_mut::<c_int>()) || bits.has_error() {
        -1
    } else {
        0
    }
}
#[no_mangle]
pub unsafe extern "C" fn sbc_decode(
    sbc: *mut sbc,
    data: *const c_void,
    size: c_uint,
    frame: *mut sbc_frame,
    pcml: *mut int16_t,
    pitchl: c_int,
    pcmr: *mut int16_t,
    pitchr: c_int,
) -> c_int {
    let mut crc: c_int = 0;
    if !data.is_null() {
        if size < 4 as c_int as c_uint {
            return -(1 as c_int);
        }
        let mut bits = Bits::new(Mode::Read, data as *mut u8, 4);
        if !decode_header(&mut bits, frame, &mut crc) || bits.has_error()
        {
            return -1;
        }
        if size < sbc_get_frame_size(frame)
            || compute_crc(frame, data as *const uint8_t, size) != crc
        {
            return -(1 as c_int);
        }
    }
    let mut sb_samples: [[int16_t; 128]; 2] = [[0; 128]; 2];
    let mut sb_scale: [c_int; 2] = [0; 2];
    if !data.is_null() {
        let mut bits = Bits::new(Mode::Read, data.offset(4) as *mut u8, sbc_get_frame_size(frame) as usize - 4);
        decode_frame(&mut bits, frame, sb_samples.as_mut_ptr(), sb_scale.as_mut_ptr());
        (*sbc)
            .nchannels = 1 as c_int
            + ((*frame).mode as c_uint
            != SBC_MODE_MONO as c_int as c_uint) as c_int;
        (*sbc).nblocks = (*frame).nblocks;
        (*sbc).nsubbands = (*frame).nsubbands;
    } else {
        let nsamples: c_int = (*sbc).nblocks * (*sbc).nsubbands;
        let mut ich: c_int = 0 as c_int;
        while ich < (*sbc).nchannels {
            std::ptr::write_bytes(
                sb_samples[ich as usize].as_mut_ptr(),
                0,
                nsamples as usize
            );
            sb_scale[ich as usize] = 0 as c_int;
            ich += 1;
        }
    }
    synthesize(
        &mut *((*sbc).c2rust_unnamed.dstates)
            .as_mut_ptr()
            .offset(0 as c_int as isize),
        (*sbc).nblocks,
        (*sbc).nsubbands,
        (sb_samples[0 as c_int as usize]).as_mut_ptr(),
        sb_scale[0 as c_int as usize],
        pcml,
        pitchl,
    );
    if (*frame).mode as c_uint != SBC_MODE_MONO as c_int as c_uint {
        synthesize(
            &mut *((*sbc).c2rust_unnamed.dstates)
                .as_mut_ptr()
                .offset(1 as c_int as isize),
            (*sbc).nblocks,
            (*sbc).nsubbands,
            (sb_samples[1 as c_int as usize]).as_mut_ptr(),
            sb_scale[1 as c_int as usize],
            pcmr,
            pitchr,
        );
    }
    0 as c_int
}
/*
unsafe extern "C" fn compute_scale_factors_js(
    frame: *const sbc_frame,
    sb_samples: *const [int16_t; 128],
    scale_factors: *mut [c_int; 8],
    mjoint: *mut c_uint,
) {
    *mjoint = 0 as c_int as c_uint;
    let mut isb: c_int = 0 as c_int;
    while isb < (*frame).nsubbands {
        let mut m: [c_uint; 2] = [0, 0];
        let mut mj: [c_uint; 2] = [0, 0];
        let mut iblk: c_int = 0 as c_int;
        while iblk < (*frame).nblocks {
            let s0: c_int = (*sb_samples
                .offset(
                    0 as c_int as isize,
                ))[(iblk * (*frame).nsubbands + isb) as usize] as c_int;
            let s1: c_int = (*sb_samples
                .offset(
                    1 as c_int as isize,
                ))[(iblk * (*frame).nsubbands + isb) as usize] as c_int;
            m[0 as c_int as usize]
                |= (if s0 < 0 as c_int { !s0 } else { s0 }) as c_uint;
            m[1 as c_int as usize]
                |= (if s1 < 0 as c_int { !s1 } else { s1 }) as c_uint;
            mj[0 as c_int as usize]
                |= (if s0 + s1 < 0 as c_int { !(s0 + s1) } else { s0 + s1 })
                as c_uint;
            mj[1 as c_int as usize]
                |= (if s0 - s1 < 0 as c_int { !(s0 - s1) } else { s0 - s1 })
                as c_uint;
            iblk += 1;
        }
        let mut scf0: c_int = (if m[0 as c_int as usize] != 0 {
            (8 as c_int as c_ulong)
                .wrapping_mul(::core::mem::size_of::<c_uint>() as c_ulong)
                .wrapping_sub(1 as c_int as c_ulong)
                .wrapping_sub(
                    (m[0 as c_int as usize]).leading_zeros() as i32
                        as c_ulong,
                )
        } else {
            0 as c_int as c_ulong
        }) as c_int;
        let mut scf1: c_int = (if m[1 as c_int as usize] != 0 {
            (8 as c_int as c_ulong)
                .wrapping_mul(::core::mem::size_of::<c_uint>() as c_ulong)
                .wrapping_sub(1 as c_int as c_ulong)
                .wrapping_sub(
                    (m[1 as c_int as usize]).leading_zeros() as i32
                        as c_ulong,
                )
        } else {
            0 as c_int as c_ulong
        }) as c_int;
        let js0: c_int = (if mj[0 as c_int as usize] != 0 {
            (8 as c_int as c_ulong)
                .wrapping_mul(::core::mem::size_of::<c_uint>() as c_ulong)
                .wrapping_sub(1 as c_int as c_ulong)
                .wrapping_sub(
                    (mj[0 as c_int as usize]).leading_zeros() as i32
                        as c_ulong,
                )
        } else {
            0 as c_int as c_ulong
        }) as c_int;
        let js1: c_int = (if mj[1 as c_int as usize] != 0 {
            (8 as c_int as c_ulong)
                .wrapping_mul(::core::mem::size_of::<c_uint>() as c_ulong)
                .wrapping_sub(1 as c_int as c_ulong)
                .wrapping_sub(
                    (mj[1 as c_int as usize]).leading_zeros() as i32
                        as c_ulong,
                )
        } else {
            0 as c_int as c_ulong
        }) as c_int;
        if isb < (*frame).nsubbands - 1 as c_int && js0 + js1 < scf0 + scf1 {
            *mjoint |= ((1 as c_int) << isb) as c_uint;
            scf0 = js0;
            scf1 = js1;
        }
        (*scale_factors.offset(0 as c_int as isize))[isb as usize] = scf0;
        (*scale_factors.offset(1 as c_int as isize))[isb as usize] = scf1;
        isb += 1;
    }
}
unsafe extern "C" fn compute_scale_factors(
    frame: *const sbc_frame,
    sb_samples: *const [int16_t; 128],
    scale_factors: *mut [c_int; 8],
) {
    let mut ich: c_int = 0 as c_int;
    while ich
        < 1 as c_int
        + ((*frame).mode as c_uint
        != SBC_MODE_MONO as c_int as c_uint) as c_int
    {
        let mut isb: c_int = 0 as c_int;
        while isb < (*frame).nsubbands {
            let mut m: c_uint = 0 as c_int as c_uint;
            let mut iblk: c_int = 0 as c_int;
            while iblk < (*frame).nblocks {
                let s: c_int = (*sb_samples
                    .offset(ich as isize))[(iblk * (*frame).nsubbands + isb) as usize]
                    as c_int;
                m |= (if s < 0 as c_int { !s } else { s }) as c_uint;
                iblk += 1;
            }
            let scf: c_int = (if m != 0 {
                (8 as c_int as c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<c_uint>() as c_ulong,
                    )
                    .wrapping_sub(1 as c_int as c_ulong)
                    .wrapping_sub(m.leading_zeros() as i32 as c_ulong)
            } else {
                0 as c_int as c_ulong
            }) as c_int;
            (*scale_factors.offset(ich as isize))[isb as usize] = scf;
            isb += 1;
        }
        ich += 1;
    }
}

unsafe extern "C" fn encode_header(
    bits: *mut sbc_bits_t,
    frame: *const sbc_frame,
) {
    static mut enc_freq: [c_int; 4] = [
        0 as c_int,
        1 as c_int,
        2 as c_int,
        3 as c_int,
    ];
    static mut enc_mode: [c_int; 4] = [
        0 as c_int,
        1 as c_int,
        2 as c_int,
        3 as c_int,
    ];
    static mut enc_bam: [c_int; 2] = [0 as c_int, 1 as c_int];
    let mut __bits: *mut sbc_bits_t = bits;
    sbc_put_bits(
        __bits,
        (if (*frame).msbc as c_int != 0 {
            0xad as c_int
        } else {
            0x9c as c_int
        }) as c_uint,
        8 as c_int as c_uint,
    );
    if !(*frame).msbc {
        sbc_put_bits(
            __bits,
            enc_freq[(*frame).freq as usize] as c_uint,
            2 as c_int as c_uint,
        );
        sbc_put_bits(
            __bits,
            (((*frame).nblocks >> 2 as c_int) - 1 as c_int) as c_uint,
            2 as c_int as c_uint,
        );
        sbc_put_bits(
            __bits,
            enc_mode[(*frame).mode as usize] as c_uint,
            2 as c_int as c_uint,
        );
        sbc_put_bits(
            __bits,
            enc_bam[(*frame).bam as usize] as c_uint,
            1 as c_int as c_uint,
        );
        sbc_put_bits(
            __bits,
            (((*frame).nsubbands >> 2 as c_int) - 1 as c_int)
                as c_uint,
            1 as c_int as c_uint,
        );
        sbc_put_bits(
            __bits,
            (*frame).bitpool as c_uint,
            8 as c_int as c_uint,
        );
    } else {
        sbc_put_bits(
            __bits,
            0 as c_int as c_uint,
            16 as c_int as c_uint,
        );
    }
    sbc_put_bits(
        __bits,
        0 as c_int as c_uint,
        8 as c_int as c_uint,
    );
}

unsafe extern "C" fn put_crc(
    frame: *const sbc_frame,
    data: *mut c_void,
    size: c_uint,
) -> c_int {
    let crc: c_int = compute_crc(frame, data as *const uint8_t, size);
    if crc < 0 as c_int {
        -1
    } else {
        *(data as *mut uint8_t).offset(3 as c_int as isize) = crc as uint8_t;
        0
    }
}

unsafe extern "C" fn encode_frame(
    bits: *mut sbc_bits_t,
    frame: *const sbc_frame,
    sb_samples: *mut [int16_t; 128],
) {
    let mut __bits: *mut sbc_bits_t = bits;
    let mut scale_factors: [[c_int; 8]; 2] = [[0; 8]; 2];
    let mut mjoint: c_uint = 0 as c_int as c_uint;
    if (*frame).mode as c_uint
        == SBC_MODE_JOINT_STEREO as c_int as c_uint
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
    if (*frame).mode as c_uint
        == SBC_MODE_DUAL_CHANNEL as c_int as c_uint
    {
        compute_scale_factors(
            frame,
            sb_samples.offset(1 as c_int as isize) as *const [int16_t; 128],
            scale_factors.as_mut_ptr().offset(1 as c_int as isize),
        );
    }
    if (*frame).mode as c_uint
        == SBC_MODE_JOINT_STEREO as c_int as c_uint
        && (*frame).nsubbands == 4 as c_int
    {
        sbc_put_bits(
            __bits,
            (mjoint & 0x1 as c_int as c_uint) << 3 as c_int
                | (mjoint & 0x2 as c_int as c_uint) << 1 as c_int
                | (mjoint & 0x4 as c_int as c_uint) >> 1 as c_int
                | (0 as c_int >> 3 as c_int) as c_uint,
            4 as c_int as c_uint,
        );
    } else if (*frame).mode as c_uint
        == SBC_MODE_JOINT_STEREO as c_int as c_uint
    {
        sbc_put_bits(
            __bits,
            (mjoint & 0x1 as c_int as c_uint) << 7 as c_int
                | (mjoint & 0x2 as c_int as c_uint) << 5 as c_int
                | (mjoint & 0x4 as c_int as c_uint) << 3 as c_int
                | (mjoint & 0x8 as c_int as c_uint) << 1 as c_int
                | (mjoint & 0x10 as c_int as c_uint) >> 1 as c_int
                | (mjoint & 0x20 as c_int as c_uint) >> 3 as c_int
                | (mjoint & 0x40 as c_int as c_uint) >> 5 as c_int
                | (0 as c_int >> 7 as c_int) as c_uint,
            8 as c_int as c_uint,
        );
    }
    let nchannels: c_int = 1 as c_int
        + ((*frame).mode as c_uint != SBC_MODE_MONO as c_int as c_uint)
        as c_int;
    let nsubbands: c_int = (*frame).nsubbands;
    let mut nbits: [[c_int; 8]; 2] = [[0; 8]; 2];
    let mut ich: c_int = 0 as c_int;
    while ich < nchannels {
        let mut isb: c_int = 0 as c_int;
        while isb < nsubbands {
            sbc_put_bits(
                __bits,
                scale_factors[ich as usize][isb as usize] as c_uint,
                4 as c_int as c_uint,
            );
            isb += 1;
        }
        ich += 1;
    }
    compute_nbits(
        frame,
        scale_factors.as_mut_ptr() as *const [c_int; 8],
        nbits.as_mut_ptr(),
    );
    if (*frame).mode as c_uint
        == SBC_MODE_DUAL_CHANNEL as c_int as c_uint
    {
        compute_nbits(
            frame,
            scale_factors.as_mut_ptr().offset(1 as c_int as isize)
                as *const [c_int; 8],
            nbits.as_mut_ptr().offset(1 as c_int as isize),
        );
    }
    let mut isb_0: c_int = 0 as c_int;
    while isb_0 < nsubbands {
        if mjoint >> isb_0 & 1 as c_int as c_uint != 0 as c_int as c_uint
        {
            let mut iblk: c_int = 0 as c_int;
            while iblk < (*frame).nblocks {
                let s0: int16_t = (*sb_samples
                    .offset(
                        0 as c_int as isize,
                    ))[(iblk * nsubbands + isb_0) as usize];
                let s1: int16_t = (*sb_samples
                    .offset(
                        1 as c_int as isize,
                    ))[(iblk * nsubbands + isb_0) as usize];
                (*sb_samples
                    .offset(
                        0 as c_int as isize,
                    ))[(iblk * nsubbands + isb_0)
                    as usize] = ((s0 as c_int + s1 as c_int) >> 1 as c_int) as int16_t;
                (*sb_samples
                    .offset(
                        1 as c_int as isize,
                    ))[(iblk * nsubbands + isb_0)
                    as usize] = ((s0 as c_int - s1 as c_int) >> 1 as c_int) as int16_t;
                iblk += 1;
            }
        }
        isb_0 += 1;
    }
    let mut iblk_0: c_int = 0 as c_int;
    while iblk_0 < (*frame).nblocks {
        let mut ich_0: c_int = 0 as c_int;
        while ich_0 < nchannels {
            let mut isb_1: c_int = 0 as c_int;
            while isb_1 < nsubbands {
                let nbit: c_int = nbits[ich_0 as usize][isb_1 as usize];
                let scf: c_int = scale_factors[ich_0 as usize][isb_1 as usize];
                if nbit != 0 {
                    let s: c_int = (*sb_samples
                        .offset(ich_0 as isize))[(iblk_0 * nsubbands + isb_1) as usize]
                        as c_int;
                    let range: c_int = !((2147483647 as c_int
                        as c_uint)
                        .wrapping_mul(2 as c_uint)
                        .wrapping_add(1 as c_uint) << nbit) as c_int;
                    sbc_put_bits(
                        __bits,
                        ((((s * range) >> (scf + 1 as c_int)) + range) >> 1 as c_int) as c_uint,
                        nbit as c_uint,
                    );
                }
                isb_1 += 1;
            }
            ich_0 += 1;
        }
        iblk_0 += 1;
    }
    let padding_nbits: c_int = (8 as c_int as c_uint)
        .wrapping_sub(
            (sbc_tell_bits(bits)).wrapping_rem(8 as c_int as c_uint),
        ) as c_int;
    sbc_put_bits(
        __bits,
        0 as c_int as c_uint,
        (if padding_nbits < 8 as c_int { padding_nbits } else { 0 as c_int })
            as c_uint,
    );
}

#[allow(clippy::eq_op)]
unsafe extern "C" fn analyze_4(
    state: *mut sbc_estate,
    in_0: *const int16_t,
    pitch: c_int,
    mut out: *mut int16_t,
) {
    static mut window: [[[int16_t; 10]; 4]; 2] = [
        [
            [
                0 as c_int as int16_t,
                358 as c_int as int16_t,
                4443 as c_int as int16_t,
                -(4443 as c_int) as int16_t,
                -(358 as c_int) as int16_t,
                0 as c_int as int16_t,
                358 as c_int as int16_t,
                4443 as c_int as int16_t,
                -(4443 as c_int) as int16_t,
                -(358 as c_int) as int16_t,
            ],
            [
                49 as c_int as int16_t,
                946 as c_int as int16_t,
                8082 as c_int as int16_t,
                -(944 as c_int) as int16_t,
                61 as c_int as int16_t,
                49 as c_int as int16_t,
                946 as c_int as int16_t,
                8082 as c_int as int16_t,
                -(944 as c_int) as int16_t,
                61 as c_int as int16_t,
            ],
            [
                18 as c_int as int16_t,
                670 as c_int as int16_t,
                6389 as c_int as int16_t,
                -(2544 as c_int) as int16_t,
                -(100 as c_int) as int16_t,
                18 as c_int as int16_t,
                670 as c_int as int16_t,
                6389 as c_int as int16_t,
                -(2544 as c_int) as int16_t,
                -(100 as c_int) as int16_t,
            ],
            [
                90 as c_int as int16_t,
                1055 as c_int as int16_t,
                9235 as c_int as int16_t,
                201 as c_int as int16_t,
                128 as c_int as int16_t,
                90 as c_int as int16_t,
                1055 as c_int as int16_t,
                9235 as c_int as int16_t,
                201 as c_int as int16_t,
                128 as c_int as int16_t,
            ],
        ],
        [
            [
                126 as c_int as int16_t,
                848 as c_int as int16_t,
                9644 as c_int as int16_t,
                848 as c_int as int16_t,
                126 as c_int as int16_t,
                126 as c_int as int16_t,
                848 as c_int as int16_t,
                9644 as c_int as int16_t,
                848 as c_int as int16_t,
                126 as c_int as int16_t,
            ],
            [
                61 as c_int as int16_t,
                -(944 as c_int) as int16_t,
                8082 as c_int as int16_t,
                946 as c_int as int16_t,
                49 as c_int as int16_t,
                61 as c_int as int16_t,
                -(944 as c_int) as int16_t,
                8082 as c_int as int16_t,
                946 as c_int as int16_t,
                49 as c_int as int16_t,
            ],
            [
                128 as c_int as int16_t,
                201 as c_int as int16_t,
                9235 as c_int as int16_t,
                1055 as c_int as int16_t,
                90 as c_int as int16_t,
                128 as c_int as int16_t,
                201 as c_int as int16_t,
                9235 as c_int as int16_t,
                1055 as c_int as int16_t,
                90 as c_int as int16_t,
            ],
            [
                -(100 as c_int) as int16_t,
                -(2544 as c_int) as int16_t,
                6389 as c_int as int16_t,
                670 as c_int as int16_t,
                18 as c_int as int16_t,
                -(100 as c_int) as int16_t,
                -(2544 as c_int) as int16_t,
                6389 as c_int as int16_t,
                670 as c_int as int16_t,
                18 as c_int as int16_t,
            ],
        ],
    ];
    let idx: c_int = (*state).idx >> 1 as c_int;
    let odd: c_int = (*state).idx & 1 as c_int;
    let x: *mut [int16_t; 5] = ((*state).x[odd as usize]).as_mut_ptr();
    let in_idx: c_int = if idx != 0 {
        5 as c_int - idx
    } else {
        0 as c_int
    };
    (*x
        .offset(
            0 as c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((3 as c_int - 0 as c_int) * pitch) as isize);
    (*x
        .offset(
            1 as c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((3 as c_int - 2 as c_int) * pitch) as isize);
    (*x
        .offset(
            2 as c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((3 as c_int - 1 as c_int) * pitch) as isize);
    (*x
        .offset(
            3 as c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((3 as c_int - 3 as c_int) * pitch) as isize);
    let w0: *const [int16_t; 10] = (window[0 as c_int
        as usize][0 as c_int as usize])
        .as_ptr()
        .offset(idx as isize) as *const [int16_t; 10];
    let w1: *const [int16_t; 10] = (window[1 as c_int
        as usize][0 as c_int as usize])
        .as_ptr()
        .offset(idx as isize) as *const [int16_t; 10];




    let mut y: [int16_t; 4] = [0; 4];
    let y0: c_int = (*x.offset(0 as c_int as isize))[0 as c_int as usize] as c_int
        * (*w0.offset(0 as c_int as isize))[0 as c_int as usize]
        as c_int
        + (*x.offset(0 as c_int as isize))[1 as c_int as usize]
        as c_int
        * (*w0.offset(0 as c_int as isize))[1 as c_int as usize]
        as c_int
        + (*x.offset(0 as c_int as isize))[2 as c_int as usize]
        as c_int
        * (*w0.offset(0 as c_int as isize))[2 as c_int as usize]
        as c_int
        + (*x.offset(0 as c_int as isize))[3 as c_int as usize]
        as c_int
        * (*w0.offset(0 as c_int as isize))[3 as c_int as usize]
        as c_int
        + (*x.offset(0 as c_int as isize))[4 as c_int as usize]
        as c_int
        * (*w0.offset(0 as c_int as isize))[4 as c_int as usize]
        as c_int + (*state).y[0 as c_int as usize];
    (*state)
        .y[0 as c_int
        as usize] = (*x.offset(0 as c_int as isize))[0 as c_int as usize]
        as c_int
        * (*w1.offset(0 as c_int as isize))[0 as c_int as usize]
        as c_int
        + (*x.offset(0 as c_int as isize))[1 as c_int as usize]
        as c_int
        * (*w1.offset(0 as c_int as isize))[1 as c_int as usize]
        as c_int
        + (*x.offset(0 as c_int as isize))[2 as c_int as usize]
        as c_int
        * (*w1.offset(0 as c_int as isize))[2 as c_int as usize]
        as c_int
        + (*x.offset(0 as c_int as isize))[3 as c_int as usize]
        as c_int
        * (*w1.offset(0 as c_int as isize))[3 as c_int as usize]
        as c_int
        + (*x.offset(0 as c_int as isize))[4 as c_int as usize]
        as c_int
        * (*w1.offset(0 as c_int as isize))[4 as c_int as usize]
        as c_int;
    let y1: c_int = (*x.offset(2 as c_int as isize))[0 as c_int as usize] as c_int
        * (*w0.offset(2 as c_int as isize))[0 as c_int as usize]
        as c_int
        + (*x.offset(2 as c_int as isize))[1 as c_int as usize]
        as c_int
        * (*w0.offset(2 as c_int as isize))[1 as c_int as usize]
        as c_int
        + (*x.offset(2 as c_int as isize))[2 as c_int as usize]
        as c_int
        * (*w0.offset(2 as c_int as isize))[2 as c_int as usize]
        as c_int
        + (*x.offset(2 as c_int as isize))[3 as c_int as usize]
        as c_int
        * (*w0.offset(2 as c_int as isize))[3 as c_int as usize]
        as c_int
        + (*x.offset(2 as c_int as isize))[4 as c_int as usize]
        as c_int
        * (*w0.offset(2 as c_int as isize))[4 as c_int as usize]
        as c_int
        + (*x.offset(3 as c_int as isize))[0 as c_int as usize]
        as c_int
        * (*w0.offset(3 as c_int as isize))[0 as c_int as usize]
        as c_int
        + (*x.offset(3 as c_int as isize))[1 as c_int as usize]
        as c_int
        * (*w0.offset(3 as c_int as isize))[1 as c_int as usize]
        as c_int
        + (*x.offset(3 as c_int as isize))[2 as c_int as usize]
        as c_int
        * (*w0.offset(3 as c_int as isize))[2 as c_int as usize]
        as c_int
        + (*x.offset(3 as c_int as isize))[3 as c_int as usize]
        as c_int
        * (*w0.offset(3 as c_int as isize))[3 as c_int as usize]
        as c_int
        + (*x.offset(3 as c_int as isize))[4 as c_int as usize]
        as c_int
        * (*w0.offset(3 as c_int as isize))[4 as c_int as usize]
        as c_int;
    let y2: c_int = (*state).y[1 as c_int as usize];
    (*state)
        .y[1 as c_int
        as usize] = (*x.offset(2 as c_int as isize))[0 as c_int as usize]
        as c_int
        * (*w1.offset(2 as c_int as isize))[0 as c_int as usize]
        as c_int
        + (*x.offset(2 as c_int as isize))[1 as c_int as usize]
        as c_int
        * (*w1.offset(2 as c_int as isize))[1 as c_int as usize]
        as c_int
        + (*x.offset(2 as c_int as isize))[2 as c_int as usize]
        as c_int
        * (*w1.offset(2 as c_int as isize))[2 as c_int as usize]
        as c_int
        + (*x.offset(2 as c_int as isize))[3 as c_int as usize]
        as c_int
        * (*w1.offset(2 as c_int as isize))[3 as c_int as usize]
        as c_int
        + (*x.offset(2 as c_int as isize))[4 as c_int as usize]
        as c_int
        * (*w1.offset(2 as c_int as isize))[4 as c_int as usize]
        as c_int
        - (*x.offset(3 as c_int as isize))[0 as c_int as usize]
        as c_int
        * (*w1.offset(3 as c_int as isize))[0 as c_int as usize]
        as c_int
        - (*x.offset(3 as c_int as isize))[1 as c_int as usize]
        as c_int
        * (*w1.offset(3 as c_int as isize))[1 as c_int as usize]
        as c_int
        - (*x.offset(3 as c_int as isize))[2 as c_int as usize]
        as c_int
        * (*w1.offset(3 as c_int as isize))[2 as c_int as usize]
        as c_int
        - (*x.offset(3 as c_int as isize))[3 as c_int as usize]
        as c_int
        * (*w1.offset(3 as c_int as isize))[3 as c_int as usize]
        as c_int
        - (*x.offset(3 as c_int as isize))[4 as c_int as usize]
        as c_int
        * (*w1.offset(3 as c_int as isize))[4 as c_int as usize]
        as c_int;
    let y3: c_int = (*x.offset(1 as c_int as isize))[0 as c_int as usize] as c_int
        * (*w0.offset(1 as c_int as isize))[0 as c_int as usize]
        as c_int
        + (*x.offset(1 as c_int as isize))[1 as c_int as usize]
        as c_int
        * (*w0.offset(1 as c_int as isize))[1 as c_int as usize]
        as c_int
        + (*x.offset(1 as c_int as isize))[2 as c_int as usize]
        as c_int
        * (*w0.offset(1 as c_int as isize))[2 as c_int as usize]
        as c_int
        + (*x.offset(1 as c_int as isize))[3 as c_int as usize]
        as c_int
        * (*w0.offset(1 as c_int as isize))[3 as c_int as usize]
        as c_int
        + (*x.offset(1 as c_int as isize))[4 as c_int as usize]
        as c_int
        * (*w0.offset(1 as c_int as isize))[4 as c_int as usize]
        as c_int;
    y[0 as c_int
        as usize] = (if (y0 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int > 32767 as c_int
    {
        32767 as c_int
    } else if ((y0 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int)
        < -(32767 as c_int) - 1 as c_int
    {
        -(32767 as c_int) - 1 as c_int
    } else {
        (y0 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int
    }) as int16_t;
    y[1 as c_int
        as usize] = (if (y1 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int > 32767 as c_int
    {
        32767 as c_int
    } else if ((y1 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int)
        < -(32767 as c_int) - 1 as c_int
    {
        -(32767 as c_int) - 1 as c_int
    } else {
        (y1 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int
    }) as int16_t;
    y[2 as c_int
        as usize] = (if (y2 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int > 32767 as c_int
    {
        32767 as c_int
    } else if ((y2 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int)
        < -(32767 as c_int) - 1 as c_int
    {
        -(32767 as c_int) - 1 as c_int
    } else {
        (y2 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int
    }) as int16_t;
    y[3 as c_int
        as usize] = (if (y3 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int > 32767 as c_int
    {
        32767 as c_int
    } else if ((y3 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int)
        < -(32767 as c_int) - 1 as c_int
    {
        -(32767 as c_int) - 1 as c_int
    } else {
        (y3 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int
    }) as int16_t;
    (*state)
        .idx = if (*state).idx < 9 as c_int {
        (*state).idx + 1 as c_int
    } else {
        0 as c_int
    };
    static mut cos8: [int16_t; 4] = [
        8192 as c_int as int16_t,
        7568 as c_int as int16_t,
        5793 as c_int as int16_t,
        3135 as c_int as int16_t,
    ];
    let s0 = y[0 as c_int as usize] as c_int
        * cos8[2 as c_int as usize] as c_int
        + y[1 as c_int as usize] as c_int
        * cos8[1 as c_int as usize] as c_int
        + y[2 as c_int as usize] as c_int
        * cos8[3 as c_int as usize] as c_int
        + ((y[3 as c_int as usize] as c_int) << 13 as c_int);
    let s1 = -(y[0 as c_int as usize] as c_int)
        * cos8[2 as c_int as usize] as c_int
        + y[1 as c_int as usize] as c_int
        * cos8[3 as c_int as usize] as c_int
        - y[2 as c_int as usize] as c_int
        * cos8[1 as c_int as usize] as c_int
        + ((y[3 as c_int as usize] as c_int) << 13 as c_int);
    let s2 = -(y[0 as c_int as usize] as c_int)
        * cos8[2 as c_int as usize] as c_int
        - y[1 as c_int as usize] as c_int
        * cos8[3 as c_int as usize] as c_int
        + y[2 as c_int as usize] as c_int
        * cos8[1 as c_int as usize] as c_int
        + ((y[3 as c_int as usize] as c_int) << 13 as c_int);
    let s3 = y[0 as c_int as usize] as c_int
        * cos8[2 as c_int as usize] as c_int
        - y[1 as c_int as usize] as c_int
        * cos8[1 as c_int as usize] as c_int
        - y[2 as c_int as usize] as c_int
        * cos8[3 as c_int as usize] as c_int
        + ((y[3 as c_int as usize] as c_int) << 13 as c_int);
    let fresh24 = out;
    out = out.offset(1);
    *fresh24 = (if (s0 + ((1 as c_int) << 12 as c_int)) >> 13 as c_int
        > 32767 as c_int
    {
        32767 as c_int
    } else if ((s0 + ((1 as c_int) << 12 as c_int)) >> 13 as c_int)
        < -(32767 as c_int) - 1 as c_int
    {
        -(32767 as c_int) - 1 as c_int
    } else {
        (s0 + ((1 as c_int) << 12 as c_int)) >> 13 as c_int
    }) as int16_t;
    let fresh25 = out;
    out = out.offset(1);
    *fresh25 = (if (s1 + ((1 as c_int) << 12 as c_int)) >> 13 as c_int
        > 32767 as c_int
    {
        32767 as c_int
    } else if ((s1 + ((1 as c_int) << 12 as c_int)) >> 13 as c_int)
        < -(32767 as c_int) - 1 as c_int
    {
        -(32767 as c_int) - 1 as c_int
    } else {
        (s1 + ((1 as c_int) << 12 as c_int)) >> 13 as c_int
    }) as int16_t;
    let fresh26 = out;
    out = out.offset(1);
    *fresh26 = (if (s2 + ((1 as c_int) << 12 as c_int)) >> 13 as c_int
        > 32767 as c_int
    {
        32767 as c_int
    } else if ((s2 + ((1 as c_int) << 12 as c_int)) >> 13 as c_int)
        < -(32767 as c_int) - 1 as c_int
    {
        -(32767 as c_int) - 1 as c_int
    } else {
        (s2 + ((1 as c_int) << 12 as c_int)) >> 13 as c_int
    }) as int16_t;
    let fresh27 = out;
    *fresh27 = (if (s3 + ((1 as c_int) << 12 as c_int)) >> 13 as c_int
        > 32767 as c_int
    {
        32767 as c_int
    } else if ((s3 + ((1 as c_int) << 12 as c_int)) >> 13 as c_int)
        < -(32767 as c_int) - 1 as c_int
    {
        -(32767 as c_int) - 1 as c_int
    } else {
        (s3 + ((1 as c_int) << 12 as c_int)) >> 13 as c_int
    }) as int16_t;
}

#[allow(clippy::eq_op)]
unsafe extern "C" fn analyze_8(
    state: *mut sbc_estate,
    in_0: *const int16_t,
    pitch: c_int,
    mut out: *mut int16_t,
) {
    static mut window: [[[int16_t; 10]; 8]; 2] = [
        [
            [
                0 as c_int as int16_t,
                185 as c_int as int16_t,
                2228 as c_int as int16_t,
                -(2228 as c_int) as int16_t,
                -(185 as c_int) as int16_t,
                0 as c_int as int16_t,
                185 as c_int as int16_t,
                2228 as c_int as int16_t,
                -(2228 as c_int) as int16_t,
                -(185 as c_int) as int16_t,
            ],
            [
                27 as c_int as int16_t,
                480 as c_int as int16_t,
                4039 as c_int as int16_t,
                -(480 as c_int) as int16_t,
                30 as c_int as int16_t,
                27 as c_int as int16_t,
                480 as c_int as int16_t,
                4039 as c_int as int16_t,
                -(480 as c_int) as int16_t,
                30 as c_int as int16_t,
            ],
            [
                5 as c_int as int16_t,
                263 as c_int as int16_t,
                2719 as c_int as int16_t,
                -(1743 as c_int) as int16_t,
                -(115 as c_int) as int16_t,
                5 as c_int as int16_t,
                263 as c_int as int16_t,
                2719 as c_int as int16_t,
                -(1743 as c_int) as int16_t,
                -(115 as c_int) as int16_t,
            ],
            [
                58 as c_int as int16_t,
                502 as c_int as int16_t,
                4764 as c_int as int16_t,
                290 as c_int as int16_t,
                69 as c_int as int16_t,
                58 as c_int as int16_t,
                502 as c_int as int16_t,
                4764 as c_int as int16_t,
                290 as c_int as int16_t,
                69 as c_int as int16_t,
            ],
            [
                11 as c_int as int16_t,
                343 as c_int as int16_t,
                3197 as c_int as int16_t,
                -(1280 as c_int) as int16_t,
                -(54 as c_int) as int16_t,
                11 as c_int as int16_t,
                343 as c_int as int16_t,
                3197 as c_int as int16_t,
                -(1280 as c_int) as int16_t,
                -(54 as c_int) as int16_t,
            ],
            [
                48 as c_int as int16_t,
                532 as c_int as int16_t,
                4612 as c_int as int16_t,
                96 as c_int as int16_t,
                65 as c_int as int16_t,
                48 as c_int as int16_t,
                532 as c_int as int16_t,
                4612 as c_int as int16_t,
                96 as c_int as int16_t,
                65 as c_int as int16_t,
            ],
            [
                18 as c_int as int16_t,
                418 as c_int as int16_t,
                3644 as c_int as int16_t,
                -(856 as c_int) as int16_t,
                -(6 as c_int) as int16_t,
                18 as c_int as int16_t,
                418 as c_int as int16_t,
                3644 as c_int as int16_t,
                -(856 as c_int) as int16_t,
                -(6 as c_int) as int16_t,
            ],
            [
                37 as c_int as int16_t,
                521 as c_int as int16_t,
                4367 as c_int as int16_t,
                -(161 as c_int) as int16_t,
                53 as c_int as int16_t,
                37 as c_int as int16_t,
                521 as c_int as int16_t,
                4367 as c_int as int16_t,
                -(161 as c_int) as int16_t,
                53 as c_int as int16_t,
            ],
        ],
        [
            [
                66 as c_int as int16_t,
                424 as c_int as int16_t,
                4815 as c_int as int16_t,
                424 as c_int as int16_t,
                66 as c_int as int16_t,
                66 as c_int as int16_t,
                424 as c_int as int16_t,
                4815 as c_int as int16_t,
                424 as c_int as int16_t,
                66 as c_int as int16_t,
            ],
            [
                30 as c_int as int16_t,
                -(480 as c_int) as int16_t,
                4039 as c_int as int16_t,
                480 as c_int as int16_t,
                27 as c_int as int16_t,
                30 as c_int as int16_t,
                -(480 as c_int) as int16_t,
                4039 as c_int as int16_t,
                480 as c_int as int16_t,
                27 as c_int as int16_t,
            ],
            [
                69 as c_int as int16_t,
                290 as c_int as int16_t,
                4764 as c_int as int16_t,
                502 as c_int as int16_t,
                58 as c_int as int16_t,
                69 as c_int as int16_t,
                290 as c_int as int16_t,
                4764 as c_int as int16_t,
                502 as c_int as int16_t,
                58 as c_int as int16_t,
            ],
            [
                -(115 as c_int) as int16_t,
                -(1743 as c_int) as int16_t,
                2719 as c_int as int16_t,
                263 as c_int as int16_t,
                5 as c_int as int16_t,
                -(115 as c_int) as int16_t,
                -(1743 as c_int) as int16_t,
                2719 as c_int as int16_t,
                263 as c_int as int16_t,
                5 as c_int as int16_t,
            ],
            [
                65 as c_int as int16_t,
                96 as c_int as int16_t,
                4612 as c_int as int16_t,
                532 as c_int as int16_t,
                48 as c_int as int16_t,
                65 as c_int as int16_t,
                96 as c_int as int16_t,
                4612 as c_int as int16_t,
                532 as c_int as int16_t,
                48 as c_int as int16_t,
            ],
            [
                -(54 as c_int) as int16_t,
                -(1280 as c_int) as int16_t,
                3197 as c_int as int16_t,
                343 as c_int as int16_t,
                11 as c_int as int16_t,
                -(54 as c_int) as int16_t,
                -(1280 as c_int) as int16_t,
                3197 as c_int as int16_t,
                343 as c_int as int16_t,
                11 as c_int as int16_t,
            ],
            [
                53 as c_int as int16_t,
                -(161 as c_int) as int16_t,
                4367 as c_int as int16_t,
                521 as c_int as int16_t,
                37 as c_int as int16_t,
                53 as c_int as int16_t,
                -(161 as c_int) as int16_t,
                4367 as c_int as int16_t,
                521 as c_int as int16_t,
                37 as c_int as int16_t,
            ],
            [
                -(6 as c_int) as int16_t,
                -(856 as c_int) as int16_t,
                3644 as c_int as int16_t,
                418 as c_int as int16_t,
                18 as c_int as int16_t,
                -(6 as c_int) as int16_t,
                -(856 as c_int) as int16_t,
                3644 as c_int as int16_t,
                418 as c_int as int16_t,
                18 as c_int as int16_t,
            ],
        ],
    ];
    let idx: c_int = (*state).idx >> 1 as c_int;
    let odd: c_int = (*state).idx & 1 as c_int;
    let x: *mut [int16_t; 5] = ((*state).x[odd as usize]).as_mut_ptr();
    let in_idx: c_int = if idx != 0 {
        5 as c_int - idx
    } else {
        0 as c_int
    };
    (*x
        .offset(
            0 as c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((7 as c_int - 0 as c_int) * pitch) as isize);
    (*x
        .offset(
            1 as c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((7 as c_int - 4 as c_int) * pitch) as isize);
    (*x
        .offset(
            2 as c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((7 as c_int - 1 as c_int) * pitch) as isize);
    (*x
        .offset(
            3 as c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((7 as c_int - 7 as c_int) * pitch) as isize);
    (*x
        .offset(
            4 as c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((7 as c_int - 2 as c_int) * pitch) as isize);
    (*x
        .offset(
            5 as c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((7 as c_int - 6 as c_int) * pitch) as isize);
    (*x
        .offset(
            6 as c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((7 as c_int - 3 as c_int) * pitch) as isize);
    (*x
        .offset(
            7 as c_int as isize,
        ))[in_idx
        as usize] = *in_0
        .offset(((7 as c_int - 5 as c_int) * pitch) as isize);
    let w0: *const [int16_t; 10] = (window[0 as c_int
        as usize][0 as c_int as usize])
        .as_ptr()
        .offset(idx as isize) as *const [int16_t; 10];
    let w1: *const [int16_t; 10] = (window[1 as c_int
        as usize][0 as c_int as usize])
        .as_ptr()
        .offset(idx as isize) as *const [int16_t; 10];
    let mut y: [int16_t; 8] = [0; 8];
    let y0 = (*x.offset(0 as c_int as isize))[0 as c_int as usize] as c_int
        * (*w0.offset(0 as c_int as isize))[0 as c_int as usize]
        as c_int
        + (*x.offset(0 as c_int as isize))[1 as c_int as usize]
        as c_int
        * (*w0.offset(0 as c_int as isize))[1 as c_int as usize]
        as c_int
        + (*x.offset(0 as c_int as isize))[2 as c_int as usize]
        as c_int
        * (*w0.offset(0 as c_int as isize))[2 as c_int as usize]
        as c_int
        + (*x.offset(0 as c_int as isize))[3 as c_int as usize]
        as c_int
        * (*w0.offset(0 as c_int as isize))[3 as c_int as usize]
        as c_int
        + (*x.offset(0 as c_int as isize))[4 as c_int as usize]
        as c_int
        * (*w0.offset(0 as c_int as isize))[4 as c_int as usize]
        as c_int + (*state).y[0 as c_int as usize];
    (*state)
        .y[0 as c_int
        as usize] = (*x.offset(0 as c_int as isize))[0 as c_int as usize]
        as c_int
        * (*w1.offset(0 as c_int as isize))[0 as c_int as usize]
        as c_int
        + (*x.offset(0 as c_int as isize))[1 as c_int as usize]
        as c_int
        * (*w1.offset(0 as c_int as isize))[1 as c_int as usize]
        as c_int
        + (*x.offset(0 as c_int as isize))[2 as c_int as usize]
        as c_int
        * (*w1.offset(0 as c_int as isize))[2 as c_int as usize]
        as c_int
        + (*x.offset(0 as c_int as isize))[3 as c_int as usize]
        as c_int
        * (*w1.offset(0 as c_int as isize))[3 as c_int as usize]
        as c_int
        + (*x.offset(0 as c_int as isize))[4 as c_int as usize]
        as c_int
        * (*w1.offset(0 as c_int as isize))[4 as c_int as usize]
        as c_int;
    let y1 = (*x.offset(2 as c_int as isize))[0 as c_int as usize] as c_int
        * (*w0.offset(2 as c_int as isize))[0 as c_int as usize]
        as c_int
        + (*x.offset(2 as c_int as isize))[1 as c_int as usize]
        as c_int
        * (*w0.offset(2 as c_int as isize))[1 as c_int as usize]
        as c_int
        + (*x.offset(2 as c_int as isize))[2 as c_int as usize]
        as c_int
        * (*w0.offset(2 as c_int as isize))[2 as c_int as usize]
        as c_int
        + (*x.offset(2 as c_int as isize))[3 as c_int as usize]
        as c_int
        * (*w0.offset(2 as c_int as isize))[3 as c_int as usize]
        as c_int
        + (*x.offset(2 as c_int as isize))[4 as c_int as usize]
        as c_int
        * (*w0.offset(2 as c_int as isize))[4 as c_int as usize]
        as c_int
        + (*x.offset(3 as c_int as isize))[0 as c_int as usize]
        as c_int
        * (*w0.offset(3 as c_int as isize))[0 as c_int as usize]
        as c_int
        + (*x.offset(3 as c_int as isize))[1 as c_int as usize]
        as c_int
        * (*w0.offset(3 as c_int as isize))[1 as c_int as usize]
        as c_int
        + (*x.offset(3 as c_int as isize))[2 as c_int as usize]
        as c_int
        * (*w0.offset(3 as c_int as isize))[2 as c_int as usize]
        as c_int
        + (*x.offset(3 as c_int as isize))[3 as c_int as usize]
        as c_int
        * (*w0.offset(3 as c_int as isize))[3 as c_int as usize]
        as c_int
        + (*x.offset(3 as c_int as isize))[4 as c_int as usize]
        as c_int
        * (*w0.offset(3 as c_int as isize))[4 as c_int as usize]
        as c_int;
    let y4 = (*state).y[1 as c_int as usize];
    (*state)
        .y[1 as c_int
        as usize] = (*x.offset(2 as c_int as isize))[0 as c_int as usize]
        as c_int
        * (*w1.offset(2 as c_int as isize))[0 as c_int as usize]
        as c_int
        + (*x.offset(2 as c_int as isize))[1 as c_int as usize]
        as c_int
        * (*w1.offset(2 as c_int as isize))[1 as c_int as usize]
        as c_int
        + (*x.offset(2 as c_int as isize))[2 as c_int as usize]
        as c_int
        * (*w1.offset(2 as c_int as isize))[2 as c_int as usize]
        as c_int
        + (*x.offset(2 as c_int as isize))[3 as c_int as usize]
        as c_int
        * (*w1.offset(2 as c_int as isize))[3 as c_int as usize]
        as c_int
        + (*x.offset(2 as c_int as isize))[4 as c_int as usize]
        as c_int
        * (*w1.offset(2 as c_int as isize))[4 as c_int as usize]
        as c_int
        - (*x.offset(3 as c_int as isize))[0 as c_int as usize]
        as c_int
        * (*w1.offset(3 as c_int as isize))[0 as c_int as usize]
        as c_int
        - (*x.offset(3 as c_int as isize))[1 as c_int as usize]
        as c_int
        * (*w1.offset(3 as c_int as isize))[1 as c_int as usize]
        as c_int
        - (*x.offset(3 as c_int as isize))[2 as c_int as usize]
        as c_int
        * (*w1.offset(3 as c_int as isize))[2 as c_int as usize]
        as c_int
        - (*x.offset(3 as c_int as isize))[3 as c_int as usize]
        as c_int
        * (*w1.offset(3 as c_int as isize))[3 as c_int as usize]
        as c_int
        - (*x.offset(3 as c_int as isize))[4 as c_int as usize]
        as c_int
        * (*w1.offset(3 as c_int as isize))[4 as c_int as usize]
        as c_int;
    let y2 = (*x.offset(4 as c_int as isize))[0 as c_int as usize] as c_int
        * (*w0.offset(4 as c_int as isize))[0 as c_int as usize]
        as c_int
        + (*x.offset(4 as c_int as isize))[1 as c_int as usize]
        as c_int
        * (*w0.offset(4 as c_int as isize))[1 as c_int as usize]
        as c_int
        + (*x.offset(4 as c_int as isize))[2 as c_int as usize]
        as c_int
        * (*w0.offset(4 as c_int as isize))[2 as c_int as usize]
        as c_int
        + (*x.offset(4 as c_int as isize))[3 as c_int as usize]
        as c_int
        * (*w0.offset(4 as c_int as isize))[3 as c_int as usize]
        as c_int
        + (*x.offset(4 as c_int as isize))[4 as c_int as usize]
        as c_int
        * (*w0.offset(4 as c_int as isize))[4 as c_int as usize]
        as c_int
        + (*x.offset(5 as c_int as isize))[0 as c_int as usize]
        as c_int
        * (*w0.offset(5 as c_int as isize))[0 as c_int as usize]
        as c_int
        + (*x.offset(5 as c_int as isize))[1 as c_int as usize]
        as c_int
        * (*w0.offset(5 as c_int as isize))[1 as c_int as usize]
        as c_int
        + (*x.offset(5 as c_int as isize))[2 as c_int as usize]
        as c_int
        * (*w0.offset(5 as c_int as isize))[2 as c_int as usize]
        as c_int
        + (*x.offset(5 as c_int as isize))[3 as c_int as usize]
        as c_int
        * (*w0.offset(5 as c_int as isize))[3 as c_int as usize]
        as c_int
        + (*x.offset(5 as c_int as isize))[4 as c_int as usize]
        as c_int
        * (*w0.offset(5 as c_int as isize))[4 as c_int as usize]
        as c_int;
    let y5 = (*state).y[2 as c_int as usize];
    (*state)
        .y[2 as c_int
        as usize] = (*x.offset(4 as c_int as isize))[0 as c_int as usize]
        as c_int
        * (*w1.offset(4 as c_int as isize))[0 as c_int as usize]
        as c_int
        + (*x.offset(4 as c_int as isize))[1 as c_int as usize]
        as c_int
        * (*w1.offset(4 as c_int as isize))[1 as c_int as usize]
        as c_int
        + (*x.offset(4 as c_int as isize))[2 as c_int as usize]
        as c_int
        * (*w1.offset(4 as c_int as isize))[2 as c_int as usize]
        as c_int
        + (*x.offset(4 as c_int as isize))[3 as c_int as usize]
        as c_int
        * (*w1.offset(4 as c_int as isize))[3 as c_int as usize]
        as c_int
        + (*x.offset(4 as c_int as isize))[4 as c_int as usize]
        as c_int
        * (*w1.offset(4 as c_int as isize))[4 as c_int as usize]
        as c_int
        - (*x.offset(5 as c_int as isize))[0 as c_int as usize]
        as c_int
        * (*w1.offset(5 as c_int as isize))[0 as c_int as usize]
        as c_int
        - (*x.offset(5 as c_int as isize))[1 as c_int as usize]
        as c_int
        * (*w1.offset(5 as c_int as isize))[1 as c_int as usize]
        as c_int
        - (*x.offset(5 as c_int as isize))[2 as c_int as usize]
        as c_int
        * (*w1.offset(5 as c_int as isize))[2 as c_int as usize]
        as c_int
        - (*x.offset(5 as c_int as isize))[3 as c_int as usize]
        as c_int
        * (*w1.offset(5 as c_int as isize))[3 as c_int as usize]
        as c_int
        - (*x.offset(5 as c_int as isize))[4 as c_int as usize]
        as c_int
        * (*w1.offset(5 as c_int as isize))[4 as c_int as usize]
        as c_int;
    let y3 = (*x.offset(6 as c_int as isize))[0 as c_int as usize] as c_int
        * (*w0.offset(6 as c_int as isize))[0 as c_int as usize]
        as c_int
        + (*x.offset(6 as c_int as isize))[1 as c_int as usize]
        as c_int
        * (*w0.offset(6 as c_int as isize))[1 as c_int as usize]
        as c_int
        + (*x.offset(6 as c_int as isize))[2 as c_int as usize]
        as c_int
        * (*w0.offset(6 as c_int as isize))[2 as c_int as usize]
        as c_int
        + (*x.offset(6 as c_int as isize))[3 as c_int as usize]
        as c_int
        * (*w0.offset(6 as c_int as isize))[3 as c_int as usize]
        as c_int
        + (*x.offset(6 as c_int as isize))[4 as c_int as usize]
        as c_int
        * (*w0.offset(6 as c_int as isize))[4 as c_int as usize]
        as c_int
        + (*x.offset(7 as c_int as isize))[0 as c_int as usize]
        as c_int
        * (*w0.offset(7 as c_int as isize))[0 as c_int as usize]
        as c_int
        + (*x.offset(7 as c_int as isize))[1 as c_int as usize]
        as c_int
        * (*w0.offset(7 as c_int as isize))[1 as c_int as usize]
        as c_int
        + (*x.offset(7 as c_int as isize))[2 as c_int as usize]
        as c_int
        * (*w0.offset(7 as c_int as isize))[2 as c_int as usize]
        as c_int
        + (*x.offset(7 as c_int as isize))[3 as c_int as usize]
        as c_int
        * (*w0.offset(7 as c_int as isize))[3 as c_int as usize]
        as c_int
        + (*x.offset(7 as c_int as isize))[4 as c_int as usize]
        as c_int
        * (*w0.offset(7 as c_int as isize))[4 as c_int as usize]
        as c_int;
    let y6 = (*state).y[3 as c_int as usize];
    (*state)
        .y[3 as c_int
        as usize] = (*x.offset(6 as c_int as isize))[0 as c_int as usize]
        as c_int
        * (*w1.offset(6 as c_int as isize))[0 as c_int as usize]
        as c_int
        + (*x.offset(6 as c_int as isize))[1 as c_int as usize]
        as c_int
        * (*w1.offset(6 as c_int as isize))[1 as c_int as usize]
        as c_int
        + (*x.offset(6 as c_int as isize))[2 as c_int as usize]
        as c_int
        * (*w1.offset(6 as c_int as isize))[2 as c_int as usize]
        as c_int
        + (*x.offset(6 as c_int as isize))[3 as c_int as usize]
        as c_int
        * (*w1.offset(6 as c_int as isize))[3 as c_int as usize]
        as c_int
        + (*x.offset(6 as c_int as isize))[4 as c_int as usize]
        as c_int
        * (*w1.offset(6 as c_int as isize))[4 as c_int as usize]
        as c_int
        - (*x.offset(7 as c_int as isize))[0 as c_int as usize]
        as c_int
        * (*w1.offset(7 as c_int as isize))[0 as c_int as usize]
        as c_int
        - (*x.offset(7 as c_int as isize))[1 as c_int as usize]
        as c_int
        * (*w1.offset(7 as c_int as isize))[1 as c_int as usize]
        as c_int
        - (*x.offset(7 as c_int as isize))[2 as c_int as usize]
        as c_int
        * (*w1.offset(7 as c_int as isize))[2 as c_int as usize]
        as c_int
        - (*x.offset(7 as c_int as isize))[3 as c_int as usize]
        as c_int
        * (*w1.offset(7 as c_int as isize))[3 as c_int as usize]
        as c_int
        - (*x.offset(7 as c_int as isize))[4 as c_int as usize]
        as c_int
        * (*w1.offset(7 as c_int as isize))[4 as c_int as usize]
        as c_int;
    let y7 = (*x.offset(1 as c_int as isize))[0 as c_int as usize] as c_int
        * (*w0.offset(1 as c_int as isize))[0 as c_int as usize]
        as c_int
        + (*x.offset(1 as c_int as isize))[1 as c_int as usize]
        as c_int
        * (*w0.offset(1 as c_int as isize))[1 as c_int as usize]
        as c_int
        + (*x.offset(1 as c_int as isize))[2 as c_int as usize]
        as c_int
        * (*w0.offset(1 as c_int as isize))[2 as c_int as usize]
        as c_int
        + (*x.offset(1 as c_int as isize))[3 as c_int as usize]
        as c_int
        * (*w0.offset(1 as c_int as isize))[3 as c_int as usize]
        as c_int
        + (*x.offset(1 as c_int as isize))[4 as c_int as usize]
        as c_int
        * (*w0.offset(1 as c_int as isize))[4 as c_int as usize]
        as c_int;
    y[0 as c_int
        as usize] = (if (y0 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int > 32767 as c_int
    {
        32767 as c_int
    } else if ((y0 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int)
        < -(32767 as c_int) - 1 as c_int
    {
        -(32767 as c_int) - 1 as c_int
    } else {
        (y0 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int
    }) as int16_t;
    y[1 as c_int
        as usize] = (if (y1 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int > 32767 as c_int
    {
        32767 as c_int
    } else if ((y1 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int)
        < -(32767 as c_int) - 1 as c_int
    {
        -(32767 as c_int) - 1 as c_int
    } else {
        (y1 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int
    }) as int16_t;
    y[2 as c_int
        as usize] = (if (y2 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int > 32767 as c_int
    {
        32767 as c_int
    } else if ((y2 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int)
        < -(32767 as c_int) - 1 as c_int
    {
        -(32767 as c_int) - 1 as c_int
    } else {
        (y2 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int
    }) as int16_t;
    y[3 as c_int
        as usize] = (if (y3 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int > 32767 as c_int
    {
        32767 as c_int
    } else if ((y3 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int)
        < -(32767 as c_int) - 1 as c_int
    {
        -(32767 as c_int) - 1 as c_int
    } else {
        (y3 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int
    }) as int16_t;
    y[4 as c_int
        as usize] = (if (y4 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int > 32767 as c_int
    {
        32767 as c_int
    } else if ((y4 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int)
        < -(32767 as c_int) - 1 as c_int
    {
        -(32767 as c_int) - 1 as c_int
    } else {
        (y4 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int
    }) as int16_t;
    y[5 as c_int
        as usize] = (if (y5 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int > 32767 as c_int
    {
        32767 as c_int
    } else if ((y5 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int)
        < -(32767 as c_int) - 1 as c_int
    {
        -(32767 as c_int) - 1 as c_int
    } else {
        (y5 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int
    }) as int16_t;
    y[6 as c_int
        as usize] = (if (y6 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int > 32767 as c_int
    {
        32767 as c_int
    } else if ((y6 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int)
        < -(32767 as c_int) - 1 as c_int
    {
        -(32767 as c_int) - 1 as c_int
    } else {
        (y6 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int
    }) as int16_t;
    y[7 as c_int
        as usize] = (if (y7 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int > 32767 as c_int
    {
        32767 as c_int
    } else if ((y7 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int)
        < -(32767 as c_int) - 1 as c_int
    {
        -(32767 as c_int) - 1 as c_int
    } else {
        (y7 + ((1 as c_int) << 14 as c_int)) >> 15 as c_int
    }) as int16_t;
    (*state)
        .idx = if (*state).idx < 9 as c_int {
        (*state).idx + 1 as c_int
    } else {
        0 as c_int
    };
    static mut cosmat: [[int16_t; 8]; 8] = [
        [
            5793 as c_int as int16_t,
            6811 as c_int as int16_t,
            7568 as c_int as int16_t,
            8035 as c_int as int16_t,
            4551 as c_int as int16_t,
            3135 as c_int as int16_t,
            1598 as c_int as int16_t,
            8192 as c_int as int16_t,
        ],
        [
            -(5793 as c_int) as int16_t,
            -(1598 as c_int) as int16_t,
            3135 as c_int as int16_t,
            6811 as c_int as int16_t,
            -(8035 as c_int) as int16_t,
            -(7568 as c_int) as int16_t,
            -(4551 as c_int) as int16_t,
            8192 as c_int as int16_t,
        ],
        [
            -(5793 as c_int) as int16_t,
            -(8035 as c_int) as int16_t,
            -(3135 as c_int) as int16_t,
            4551 as c_int as int16_t,
            1598 as c_int as int16_t,
            7568 as c_int as int16_t,
            6811 as c_int as int16_t,
            8192 as c_int as int16_t,
        ],
        [
            5793 as c_int as int16_t,
            -(4551 as c_int) as int16_t,
            -(7568 as c_int) as int16_t,
            1598 as c_int as int16_t,
            6811 as c_int as int16_t,
            -(3135 as c_int) as int16_t,
            -(8035 as c_int) as int16_t,
            8192 as c_int as int16_t,
        ],
        [
            5793 as c_int as int16_t,
            4551 as c_int as int16_t,
            -(7568 as c_int) as int16_t,
            -(1598 as c_int) as int16_t,
            -(6811 as c_int) as int16_t,
            -(3135 as c_int) as int16_t,
            8035 as c_int as int16_t,
            8192 as c_int as int16_t,
        ],
        [
            -(5793 as c_int) as int16_t,
            8035 as c_int as int16_t,
            -(3135 as c_int) as int16_t,
            -(4551 as c_int) as int16_t,
            -(1598 as c_int) as int16_t,
            7568 as c_int as int16_t,
            -(6811 as c_int) as int16_t,
            8192 as c_int as int16_t,
        ],
        [
            -(5793 as c_int) as int16_t,
            1598 as c_int as int16_t,
            3135 as c_int as int16_t,
            -(6811 as c_int) as int16_t,
            8035 as c_int as int16_t,
            -(7568 as c_int) as int16_t,
            4551 as c_int as int16_t,
            8192 as c_int as int16_t,
        ],
        [
            5793 as c_int as int16_t,
            -(6811 as c_int) as int16_t,
            7568 as c_int as int16_t,
            -(8035 as c_int) as int16_t,
            -(4551 as c_int) as int16_t,
            3135 as c_int as int16_t,
            -(1598 as c_int) as int16_t,
            8192 as c_int as int16_t,
        ],
    ];
    let mut i: c_int = 0 as c_int;
    while i < 8 as c_int {
        let s: c_int = y[0 as c_int as usize] as c_int
            * cosmat[i as usize][0 as c_int as usize] as c_int
            + y[1 as c_int as usize] as c_int
            * cosmat[i as usize][1 as c_int as usize] as c_int
            + y[2 as c_int as usize] as c_int
            * cosmat[i as usize][2 as c_int as usize] as c_int
            + y[3 as c_int as usize] as c_int
            * cosmat[i as usize][3 as c_int as usize] as c_int
            + y[4 as c_int as usize] as c_int
            * cosmat[i as usize][4 as c_int as usize] as c_int
            + y[5 as c_int as usize] as c_int
            * cosmat[i as usize][5 as c_int as usize] as c_int
            + y[6 as c_int as usize] as c_int
            * cosmat[i as usize][6 as c_int as usize] as c_int
            + y[7 as c_int as usize] as c_int
            * cosmat[i as usize][7 as c_int as usize] as c_int;
        let fresh28 = out;
        out = out.offset(1);
        *fresh28 = (if (s + ((1 as c_int) << 12 as c_int)) >> 13 as c_int
            > 32767 as c_int
        {
            32767 as c_int
        } else if ((s + ((1 as c_int) << 12 as c_int)) >> 13 as c_int)
            < -(32767 as c_int) - 1 as c_int
        {
            -(32767 as c_int) - 1 as c_int
        } else {
            (s + ((1 as c_int) << 12 as c_int)) >> 13 as c_int
        }) as int16_t;
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn analyze(
    state: *mut sbc_estate,
    frame: *const sbc_frame,
    mut in_0: *const int16_t,
    pitch: c_int,
    mut out: *mut int16_t,
) {
    let mut iblk: c_int = 0 as c_int;
    while iblk < (*frame).nblocks {
        if (*frame).nsubbands == 4 as c_int {
            analyze_4(state, in_0, pitch, out);
        } else {
            analyze_8(state, in_0, pitch, out);
        }
        in_0 = in_0.offset(((*frame).nsubbands * pitch) as isize);
        out = out.offset((*frame).nsubbands as isize);
        iblk += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn sbc_encode(
    sbc: *mut sbc,
    pcml: *const int16_t,
    pitchl: c_int,
    pcmr: *const int16_t,
    pitchr: c_int,
    mut frame: *const sbc_frame,
    data: *mut c_void,
    size: c_uint,
) -> c_int {
    if (*frame).msbc {
        frame = &msbc_frame;
    }
    if !check_frame(frame) || size < sbc_get_frame_size(frame) {
        return -(1 as c_int);
    }
    let mut sb_samples: [[int16_t; 128]; 2] = [[0; 128]; 2];
    analyze(
        &mut *((*sbc).c2rust_unnamed.estates)
            .as_mut_ptr()
            .offset(0 as c_int as isize),
        frame,
        pcml,
        pitchl,
        (sb_samples[0 as c_int as usize]).as_mut_ptr(),
    );
    if (*frame).mode as c_uint != SBC_MODE_MONO as c_int as c_uint {
        analyze(
            &mut *((*sbc).c2rust_unnamed.estates)
                .as_mut_ptr()
                .offset(1 as c_int as isize),
            frame,
            pcmr,
            pitchr,
            (sb_samples[1 as c_int as usize]).as_mut_ptr(),
        );
    }
    let mut bits: sbc_bits_t = sbc_bits_t {
        mode: SBC_BITS_READ,
        data: sbc_bits_data {
            p: std::ptr::null_mut::<uint8_t>(),
            nbytes: 0,
            nleft: 0,
        },
        accu: sbc_bits_accu {
            v: 0,
            nleft: 0,
            nover: 0,
        },
        error: false,
    };
    sbc_setup_bits(
        &mut bits,
        SBC_BITS_WRITE,
        data.add(4),
        sbc_get_frame_size(frame) - 4,
    );
    encode_frame(&mut bits, frame, sb_samples.as_mut_ptr());
    sbc_flush_bits(&mut bits);
    sbc_setup_bits(&mut bits, SBC_BITS_WRITE, data, 4 as c_int as c_uint);
    encode_header(&mut bits, frame);
    sbc_flush_bits(&mut bits);
    put_crc(frame, data, size);
    0 as c_int
}
*/