use std::mem::zeroed;
use crate::sbc::{sbc_decode, sbc_get_frame_size, SBC_MODE_MONO, sbc_probe, sbc_reset, sbc_t};

pub mod bits;
pub mod sbc;

pub const SBC_HEADER_SIZE: usize = 4;
pub const SBC_PROBE_SIZE: usize = SBC_HEADER_SIZE;

pub const SBC_MAX_SUBBANDS: usize = 8;
pub const SBC_MAX_BLOCKS: usize = 16;
pub const SBC_MAX_SAMPLES: usize = SBC_MAX_BLOCKS * SBC_MAX_SUBBANDS;

pub const SBC_MSBC_SAMPLES: usize = 120;
pub const SBC_MSBC_SIZE: usize = 57;

pub struct Decoder {
    buffer: Vec<u8>,
    index: usize,
    sbc: Box<sbc_t>
}

unsafe impl Send for Decoder {}

impl Decoder {
    pub fn new(data: Vec<u8>) -> Self {
        let mut sbc: Box<sbc_t> = unsafe { Box::new(zeroed()) };
        unsafe { sbc_reset(sbc.as_mut()) };
        Self { buffer: data, index: 0, sbc }
    }

    pub fn next_frame(&mut self) -> Option<Vec<i16>> {
        let remaining = &self.buffer[self.index..];
        if remaining.len() < SBC_PROBE_SIZE { return None; }
        assert_eq!(remaining.len().min(SBC_PROBE_SIZE), SBC_PROBE_SIZE);
        unsafe {
            let mut frame = zeroed();
            assert_eq!(sbc_probe(remaining.as_ptr().cast(), &mut frame), 0);
            assert_ne!(frame.mode, SBC_MODE_MONO);
            let frame_size = sbc_get_frame_size(&frame) as usize;

            if remaining.len() < frame_size { return None; }
            assert_eq!(remaining.len().min(frame_size), frame_size);

            let mut pcm: Vec<i16> = vec![0; (2 * frame.nblocks * frame.nsubbands) as usize];

            assert_eq!(sbc_decode(
                &mut *self.sbc,
                remaining.as_ptr().cast(),
                remaining.len() as _,
                &mut frame,
                pcm.as_mut_ptr(),
                2,
                pcm.as_mut_ptr().add(1),
                2
            ), 0);
            self.index += frame_size;

            Some(pcm)
        }

    }

}


#[cfg(test)]
mod tests {
    use std::mem::size_of;
    use super::*;

    #[test]
    fn it_works() {
        let data = std::fs::read("../bluefang/target/sbc/output.sbc").unwrap();
        let mut decoder = Decoder::new(data);
        let mut count = 0;
        while let Some(_) = decoder.next_frame() {
            count += 1;
        }
        println!("Decoded {} frames", count);
        assert_eq!(count, 10416);
    }

}
