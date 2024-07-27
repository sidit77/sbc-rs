use std::mem::zeroed;

use crate::bits::{__sbc_get_bits, __sbc_put_bits, sbc_bits, SBC_BITS_READ, SBC_BITS_WRITE, sbc_setup_bits, sbc_tell_bits};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Mode {
    Read,
    Write,
}

//struct BitsData {
//    p: *mut u8,
//    nbytes: usize,
//    nleft: usize,
//}
//
//type BitAccuT = u32;
//
//struct BitsAccu {
//    v: BitAccuT,
//    nleft: usize,
//    nover: usize,
//}
//
//impl BitsAccu {
//    const BITS: usize = size_of::<BitAccuT>() * 8;
//}

pub struct Bits {
    inner: sbc_bits,
}

impl Bits {

    pub fn new(mode: Mode, data: *mut u8, size: usize) -> Self {
        unsafe {
            let mut inner = zeroed();
            sbc_setup_bits(
                &mut inner,
                match mode {
                    Mode::Read => SBC_BITS_READ,
                    Mode::Write => SBC_BITS_WRITE,
                },
                data as _,
                size as _,
            );
            Self { inner }
        }
    }

    pub fn has_error(&self) -> bool {
        self.inner.error
    }

    pub fn pos(&mut self) -> u32 {
        unsafe { sbc_tell_bits(&mut self.inner)}
    }

    pub fn get_bits(&mut self, n: u32) -> u32 {
        if self.inner.accu.nleft < n {
            return unsafe { __sbc_get_bits(&mut self.inner, n) };
        }
        self.inner.accu.nleft -= n;
        (self.inner.accu.v >> self.inner.accu.nleft) & ((1u32 << n) - 1)
    }

    pub fn advance(&mut self, n: u32) {
        self.get_bits(n);
    }

    pub fn put_bits(&mut self, n: u32, v: u32) {
        if self.inner.accu.nleft < n {
            unsafe { __sbc_put_bits(&mut self.inner, v, n) };
        } else {
            self.inner.accu.nleft -= n;
            self.inner.accu.v = (self.inner.accu.v << n) | v & ((1u32 << n) - 1);
        }
    }

    pub fn get_bits_fixed(&mut self, n: u32, v: u32) {
        if self.get_bits(n) != v {
            self.inner.error = true;
        }
    }
}