#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type sbc_bits_mode = libc::c_uint;
pub const SBC_BITS_WRITE: sbc_bits_mode = 1;
pub const SBC_BITS_READ: sbc_bits_mode = 0;
pub type bits_accu_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sbc_bits {
    pub mode: sbc_bits_mode,
    pub data: C2RustUnnamed_0,
    pub accu: C2RustUnnamed,
    pub error: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub v: bits_accu_t,
    pub nleft: libc::c_uint,
    pub nover: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub p: *mut uint8_t,
    pub nbytes: libc::c_uint,
    pub nleft: libc::c_uint,
}
pub type sbc_bits_t = sbc_bits;
unsafe extern "C" fn load_accu_slow(mut bits: *mut sbc_bits) {
    while ((*bits).accu.nleft as libc::c_ulong)
        < (8 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<bits_accu_t>() as libc::c_ulong)
        .wrapping_sub(7 as libc::c_int as libc::c_ulong) && (*bits).data.nleft != 0
    {
        let fresh0 = (*bits).data.p;
        (*bits).data.p = ((*bits).data.p).offset(1);
        (*bits).accu.v = (*bits).accu.v << 8 as libc::c_int | *fresh0 as libc::c_uint;
        (*bits)
            .accu
            .nleft = ((*bits).accu.nleft).wrapping_add(8 as libc::c_int as libc::c_uint);
        (*bits).data.nleft = ((*bits).data.nleft).wrapping_sub(1);
        (*bits).data.nleft;
    }
    if ((*bits).accu.nleft as libc::c_ulong)
        < (8 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<bits_accu_t>() as libc::c_ulong)
        .wrapping_sub(7 as libc::c_int as libc::c_ulong)
    {
        let mut nover: libc::c_uint = (((8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<bits_accu_t>() as libc::c_ulong)
            .wrapping_sub((*bits).accu.nleft as libc::c_ulong) >> 3 as libc::c_int)
            << 3 as libc::c_int) as libc::c_uint;
        if (*bits).accu.nleft >= (*bits).accu.nover {
            (*bits).accu.nover = ((*bits).accu.nover).wrapping_add(nover);
        } else {
            (*bits)
                .accu
                .nover = (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint);
        }
        if (nover as libc::c_ulong)
            < (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<bits_accu_t>() as libc::c_ulong)
        {
            (*bits).accu.v <<= nover;
        } else {
            (*bits).accu.v = 0 as libc::c_int as bits_accu_t;
        }
        (*bits).accu.nleft = ((*bits).accu.nleft).wrapping_add(nover);
    }
}
#[inline]
unsafe extern "C" fn load_accu(mut bits: *mut sbc_bits) {
    let mut nbytes: libc::c_uint = (::core::mem::size_of::<bits_accu_t>()
        as libc::c_ulong)
        .wrapping_sub(
            (((*bits).accu.nleft).wrapping_add(7 as libc::c_int as libc::c_uint)
                >> 3 as libc::c_int) as libc::c_ulong,
        ) as libc::c_uint;
    if nbytes > (*bits).data.nleft {
        load_accu_slow(bits);
        return;
    }
    (*bits).accu.nleft = ((*bits).accu.nleft).wrapping_add(nbytes << 3 as libc::c_int);
    (*bits).data.nleft = ((*bits).data.nleft).wrapping_sub(nbytes);
    loop {
        let fresh1 = nbytes;
        nbytes = nbytes.wrapping_sub(1);
        if !(fresh1 != 0) {
            break;
        }
        let fresh2 = (*bits).data.p;
        (*bits).data.p = ((*bits).data.p).offset(1);
        (*bits).accu.v = (*bits).accu.v << 8 as libc::c_int | *fresh2 as libc::c_uint;
    };
}
#[inline]
unsafe extern "C" fn flush_accu(mut bits: *mut sbc_bits) {
    let mut nbytes: libc::c_uint = (::core::mem::size_of::<bits_accu_t>()
        as libc::c_ulong)
        .wrapping_sub(
            (((*bits).accu.nleft).wrapping_add(7 as libc::c_int as libc::c_uint)
                >> 3 as libc::c_int) as libc::c_ulong,
        ) as libc::c_uint;
    let mut nflush: libc::c_uint = if nbytes < (*bits).data.nleft {
        nbytes
    } else {
        (*bits).data.nleft
    };
    (*bits).data.nleft = ((*bits).data.nleft).wrapping_sub(nflush);
    let mut shr: libc::c_int = (8 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<bits_accu_t>() as libc::c_ulong)
        .wrapping_sub(8 as libc::c_int as libc::c_ulong)
        .wrapping_sub((*bits).accu.nleft as libc::c_ulong) as libc::c_int;
    loop {
        let fresh3 = nflush;
        nflush = nflush.wrapping_sub(1);
        if !(fresh3 != 0) {
            break;
        }
        let fresh4 = (*bits).data.p;
        (*bits).data.p = ((*bits).data.p).offset(1);
        *fresh4 = ((*bits).accu.v >> shr) as uint8_t;
        shr -= 8 as libc::c_int;
    }
    (*bits).accu.v
        &= (((1 as libc::c_int) << (*bits).accu.nleft) - 1 as libc::c_int)
        as libc::c_uint;
    (*bits).accu.nleft = ((*bits).accu.nleft).wrapping_add(nbytes << 3 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn sbc_setup_bits(
    mut bits: *mut sbc_bits,
    mut mode: sbc_bits_mode,
    mut data: *mut libc::c_void,
    mut size: libc::c_uint,
) {
    *bits = {
        let mut init = sbc_bits {
            mode: mode,
            data: {
                let mut init = C2RustUnnamed_0 {
                    p: data as *mut uint8_t,
                    nbytes: size,
                    nleft: size,
                };
                init
            },
            accu: {
                let mut init = C2RustUnnamed {
                    v: 0,
                    nleft: (if mode as libc::c_uint
                        == SBC_BITS_READ as libc::c_int as libc::c_uint
                    {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (8 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<bits_accu_t>() as libc::c_ulong,
                            )
                    }) as libc::c_uint,
                    nover: 0,
                };
                init
            },
            error: false,
        };
        init
    };
}
#[no_mangle]
pub unsafe extern "C" fn sbc_tell_bits(mut bits: *mut sbc_bits_t) -> libc::c_uint {
    let mut nbytes: libc::c_uint = ((*bits).data.nbytes)
        .wrapping_sub((*bits).data.nleft);
    if (*bits).mode as libc::c_uint == SBC_BITS_WRITE as libc::c_int as libc::c_uint {
        nbytes = (nbytes as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<bits_accu_t>() as libc::c_ulong)
            as libc::c_uint as libc::c_uint;
    }
    return (8 as libc::c_int as libc::c_uint)
        .wrapping_mul(nbytes)
        .wrapping_sub(
            (if (*bits).accu.nleft < (*bits).accu.nover {
                0 as libc::c_int as libc::c_uint
            } else {
                ((*bits).accu.nleft).wrapping_sub((*bits).accu.nover)
            }),
        );
}
#[no_mangle]
pub unsafe extern "C" fn __sbc_get_bits(
    mut bits: *mut sbc_bits,
    mut n: libc::c_uint,
) -> libc::c_uint {
    if n > 32 as libc::c_int as libc::c_uint {
        n = 32 as libc::c_int as libc::c_uint;
    }
    if (*bits).accu.nleft == 0 {
        load_accu(bits);
    }
    if (*bits).accu.nleft >= n {
        (*bits).accu.nleft = ((*bits).accu.nleft).wrapping_sub(n);
        return (*bits).accu.v >> (*bits).accu.nleft
            & ((1 as libc::c_uint) << n).wrapping_sub(1 as libc::c_int as libc::c_uint);
    }
    n = n.wrapping_sub((*bits).accu.nleft);
    let mut v: libc::c_uint = (*bits).accu.v
        & ((1 as libc::c_uint) << (*bits).accu.nleft)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    (*bits).accu.nleft = 0 as libc::c_int as libc::c_uint;
    load_accu(bits);
    (*bits).accu.nleft = ((*bits).accu.nleft).wrapping_sub(n);
    return v << n
        | (*bits).accu.v >> (*bits).accu.nleft
        & ((1 as libc::c_uint) << n).wrapping_sub(1 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn __sbc_put_bits(
    mut bits: *mut sbc_bits,
    mut v: libc::c_uint,
    mut n: libc::c_uint,
) {
    if n > 32 as libc::c_int as libc::c_uint {
        n = 32 as libc::c_int as libc::c_uint;
    }
    if (*bits).accu.nleft == 0 {
        flush_accu(bits);
    }
    let mut m: libc::c_int = (if (*bits).accu.nleft < n {
        (*bits).accu.nleft
    } else {
        n
    }) as libc::c_int;
    n = n.wrapping_sub(m as libc::c_uint);
    (*bits).accu.nleft = ((*bits).accu.nleft).wrapping_sub(m as libc::c_uint);
    (*bits)
        .accu
        .v = (*bits).accu.v << m
        | v >> n
        & ((1 as libc::c_uint) << m).wrapping_sub(1 as libc::c_int as libc::c_uint);
    if n <= 0 as libc::c_int as libc::c_uint {
        return;
    }
    flush_accu(bits);
    (*bits).accu.nleft = ((*bits).accu.nleft).wrapping_sub(n);
    (*bits)
        .accu
        .v = (*bits).accu.v << n
        | v & ((1 as libc::c_uint) << n).wrapping_sub(1 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn sbc_flush_bits(mut bits: *mut sbc_bits) {
    flush_accu(bits);
}
