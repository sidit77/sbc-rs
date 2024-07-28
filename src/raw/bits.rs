#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, clippy::missing_safety_doc)]

use std::ffi::{c_uchar, c_uint, c_void};
use std::mem::size_of;

const ACCU_NBITS: u32 = size_of::<bits_accu_t>() as u32 * 8;

pub type __uint8_t = c_uchar;
pub type uint8_t = __uint8_t;
pub type sbc_bits_mode = c_uint;
pub const SBC_BITS_WRITE: sbc_bits_mode = 1;
pub const SBC_BITS_READ: sbc_bits_mode = 0;
pub type bits_accu_t = c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sbc_bits {
    pub mode: sbc_bits_mode,
    pub data: sbc_bits_data,
    pub accu: sbc_bits_accu,
    pub error: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sbc_bits_accu {
    pub v: bits_accu_t,
    pub nleft: c_uint,
    pub nover: c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sbc_bits_data {
    pub p: *mut uint8_t,
    pub nbytes: c_uint,
    pub nleft: c_uint,
}
pub type sbc_bits_t = sbc_bits;
unsafe extern "C" fn load_accu_slow(bits: *mut sbc_bits) {
    while ((*bits).accu.nleft) < ACCU_NBITS -7 && (*bits).data.nleft != 0
    {
        let fresh0 = (*bits).data.p;
        (*bits).data.p = ((*bits).data.p).offset(1);
        (*bits).accu.v = (*bits).accu.v << 8 | (*fresh0 as c_uint);
        (*bits)
            .accu
            .nleft = ((*bits).accu.nleft).wrapping_add(8);
        (*bits).data.nleft = ((*bits).data.nleft).wrapping_sub(1);
    }
    if ((*bits).accu.nleft) < ACCU_NBITS - 7 {
        (*bits).error = true;
    }
    {
        let nover: c_uint = ((ACCU_NBITS - (*bits).accu.nleft) >> 3) << 3;
        if (*bits).accu.nleft >= (*bits).accu.nover {
            (*bits).accu.nover = ((*bits).accu.nover).wrapping_add(nover);
        } else {
            (*bits).accu.nover = u32::MAX;
        }
        if nover < ACCU_NBITS
        {
            (*bits).accu.v <<= nover;
        } else {
            (*bits).accu.v = 0 as bits_accu_t;
        }
        (*bits).accu.nleft = ((*bits).accu.nleft).wrapping_add(nover);
    }
}
#[inline]
unsafe extern "C" fn load_accu(bits: *mut sbc_bits) {
    let mut nbytes = (size_of::<bits_accu_t>() as c_uint).wrapping_sub((*bits).accu.nleft + 7) >> 3;
    if nbytes > (*bits).data.nleft {
        load_accu_slow(bits);
        return;
    }
    (*bits).accu.nleft = ((*bits).accu.nleft).wrapping_add(nbytes << 3);
    (*bits).data.nleft = ((*bits).data.nleft).wrapping_sub(nbytes);
    loop {
        let fresh1 = nbytes;
        nbytes = nbytes.wrapping_sub(1);
        if fresh1 == 0 {
            break;
        }
        let fresh2 = (*bits).data.p;
        (*bits).data.p = ((*bits).data.p).offset(1);
        (*bits).accu.v = (*bits).accu.v << 8 | (*fresh2 as c_uint);
    };
}
#[inline]
unsafe extern "C" fn flush_accu(bits: *mut sbc_bits) {
    let nbytes = (size_of::<bits_accu_t>() as c_uint - ((*bits).accu.nleft + 7)) >> 3;
    let mut nflush: c_uint = if nbytes < (*bits).data.nleft {
        nbytes
    } else {
        (*bits).data.nleft
    };
    (*bits).data.nleft = ((*bits).data.nleft).wrapping_sub(nflush);
    let mut shr: c_uint = ACCU_NBITS - 8 - (*bits).accu.nleft;
    loop {
        let fresh3 = nflush;
        nflush = nflush.wrapping_sub(1);
        if fresh3 == 0 {
            break;
        }
        let fresh4 = (*bits).data.p;
        (*bits).data.p = ((*bits).data.p).offset(1);
        *fresh4 = ((*bits).accu.v >> shr) as uint8_t;
        shr -= 8;
    }
    (*bits).accu.v &= ((1) << (*bits).accu.nleft) - 1;
    (*bits).accu.nleft = ((*bits).accu.nleft).wrapping_add(nbytes << 3);
}
#[no_mangle]
pub unsafe extern "C" fn sbc_setup_bits(
    bits: *mut sbc_bits,
    mode: sbc_bits_mode,
    data: *mut c_void,
    size: c_uint,
) {
    *bits = {
        
        sbc_bits {
            mode,
            data: {
                
                sbc_bits_data {
                    p: data as *mut uint8_t,
                    nbytes: size,
                    nleft: size,
                }
            },
            accu: {
                
                sbc_bits_accu {
                    v: 0,
                    nleft: if mode == SBC_BITS_READ { 0 } else { 8 * size_of::<bits_accu_t>() as c_uint },
                    nover: 0,
                }
            },
            error: false,
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn sbc_tell_bits(bits: *mut sbc_bits_t) -> c_uint {
    let mut nbytes: c_uint = ((*bits).data.nbytes)
        .wrapping_sub((*bits).data.nleft);
    if (*bits).mode == SBC_BITS_WRITE {
        nbytes += size_of::<bits_accu_t>() as c_uint;
    }
    8 * nbytes -
        if (*bits).accu.nleft < (*bits).accu.nover {
            0
        } else {
            ((*bits).accu.nleft) - ((*bits).accu.nover)
        }
}
#[no_mangle]
pub unsafe extern "C" fn __sbc_get_bits(
    bits: *mut sbc_bits,
    mut n: c_uint,
) -> c_uint {
    if n > 32 {
        n = 32;
    }
    if (*bits).accu.nleft == 0 {
        load_accu(bits);
    }
    if (*bits).accu.nleft >= n {
        (*bits).accu.nleft = ((*bits).accu.nleft).wrapping_sub(n);
        return (*bits).accu.v >> (*bits).accu.nleft & ((1 << n) - 1);
    }
    n = n.wrapping_sub((*bits).accu.nleft);
    let v: c_uint = (*bits).accu.v
        & ((1 << (*bits).accu.nleft) - 1);
    (*bits).accu.nleft = 0;
    load_accu(bits);
    (*bits).accu.nleft = ((*bits).accu.nleft).wrapping_sub(n);
    v << n | (*bits).accu.v >> (*bits).accu.nleft & ((1 << n) - 1)
}
#[no_mangle]
pub unsafe extern "C" fn __sbc_put_bits(
    bits: *mut sbc_bits,
    v: c_uint,
    mut n: c_uint,
) {
    if n > 32 {
        n = 32;
    }
    if (*bits).accu.nleft == 0 {
        flush_accu(bits);
    }
    let m: c_uint = if (*bits).accu.nleft < n {
        (*bits).accu.nleft
    } else {
        n
    };
    n = n.wrapping_sub(m);
    (*bits).accu.nleft = ((*bits).accu.nleft).wrapping_sub(m);
    (*bits)
        .accu
        .v = (*bits).accu.v << m | v >> n & ((1 << m) - 1);
    if n == 0 {
        return;
    }
    flush_accu(bits);
    (*bits).accu.nleft = ((*bits).accu.nleft).wrapping_sub(n);
    (*bits)
        .accu
        .v = (*bits).accu.v << n
        | v & ((1 << n) - 1);
}
#[no_mangle]
pub unsafe extern "C" fn sbc_flush_bits(bits: *mut sbc_bits) {
    flush_accu(bits);
}
