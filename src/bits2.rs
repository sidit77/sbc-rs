use std::mem::size_of;
use std::slice;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Mode {
    Read,
    Write,
}

type BitAccuT = u32;

#[derive(Copy, Clone)]
pub struct Accu {
    pub v: BitAccuT,
    pub nleft: usize,
    pub nover: usize,
}

impl Accu {
    const BYTES: usize = size_of::<BitAccuT>();
    const BITS: usize = Self::BYTES * 8;
}

#[derive(Copy, Clone)]
pub struct Bits<'a> {
    mode: Mode,
    data: &'a [u8],
    pos: usize,
    accu: Accu,
    error: bool,
}

impl<'a> Bits<'a> {

    #[inline]
    pub fn new(mode: Mode, data: *mut u8, size: usize) -> Self {
        Self {
            mode,
            data: unsafe { slice::from_raw_parts(data, size)},
            pos: 0,
            accu: Accu {
                v: 0,
                nleft: if mode == Mode::Read { 0 } else { Accu::BITS },
                nover: 0,
            },
            error: false,
        }
    }

    #[inline]
    pub fn has_error(&self) -> bool {
        self.error
    }

    #[inline]
    pub fn pos(&self) -> usize {
        let mut nbytes = self.pos;
        if self.mode == Mode::Write {
            nbytes += Accu::BYTES;
        }
        8 * nbytes - match self.accu.nleft < self.accu.nover {
            true => 0,
            false => self.accu.nleft - self.accu.nover
        }
    }

    #[inline]
    pub fn get_bits(&mut self, n: u32) -> u32 {
        if self.accu.nleft < n as usize {
            return self.get_bits_internal(n);
        }
        self.accu.nleft -= n as usize;
        (self.accu.v >> self.accu.nleft) & ((1u32 << n) - 1)
    }

    #[inline]
    pub fn advance(&mut self, n: u32) {
        self.get_bits(n);
    }

    //pub fn put_bits(&mut self, n: u32, v: u32) {
    //    if self.accu.nleft < n {
    //        unsafe { __sbc_put_bits(&mut self.inner, v, n) };
    //    } else {
    //        self.inner.accu.nleft -= n;
    //        self.inner.accu.v = (self.inner.accu.v << n) | v & ((1u32 << n) - 1);
    //    }
    //}

    #[inline]
    pub fn get_bits_fixed(&mut self, n: u32, v: u32) {
        if self.get_bits(n) != v {
            self.error = true;
        }
    }
}

impl<'a> Bits<'a> {

    #[inline]
    fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }

    fn get_bits_internal(&mut self, mut n: u32) -> u32 {
        n = n.min(u32::BITS);
        if self.accu.nleft == 0 {
            self.load_accu();
        }
        if self.accu.nleft >= n as usize {
            self.accu.nleft -= n as usize;
            self.accu.v >> self.accu.nleft & ((1 << n) - 1)
        } else {
            n -= self.accu.nleft as u32;
            let v = self.accu.v & ((1 << self.accu.nleft) - 1);
            self.accu.nleft = 0;
            self.load_accu();
            self.accu.nleft -= n as usize;
            v << n | self.accu.v >> self.accu.nleft & ((1 << n) - 1)
        }
    }

    fn load_accu(&mut self) {
        debug_assert_eq!(self.accu.nleft, 0);
        let mut nbytes = Accu::BYTES;
        if nbytes > self.remaining() {
            self.load_accu_slow();
        } else {
            self.accu.nleft += nbytes << 3;
            while nbytes > 0 {
                self.accu.v = self.accu.v << 8 | self.data[self.pos] as u32;
                self.pos += 1;
                nbytes -= 1;
            }
        }
    }

    #[inline]
    fn load_accu_slow(&mut self) {
        while self.accu.nleft < Accu::BITS - 7 && self.remaining() > 0
        {
            self.accu.v = self.accu.v << 8 | (self.data[self.pos] as u32);
            self.pos += 1;
            self.accu.nleft += 8;
        }
        if self.accu.nleft < Accu::BITS - 7 {
            let nover = ((Accu::BITS - self.accu.nleft) >> 3) << 3;
            if self.accu.nleft >= self.accu.nover {
                self.accu.nover += nover;
            } else {
                self.accu.nover = usize::MAX;
            }
            self.accu.v = self.accu.v.checked_shl(nover as u32).unwrap_or(0);
            self.accu.nleft += nover;
        }
    }
}