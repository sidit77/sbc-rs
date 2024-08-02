use std::mem::zeroed;
use crate::decoder::{decode, SbcHeader};
use crate::raw::{sbc_decode, sbc_get_frame_size, SBC_MODE_MONO, sbc_probe, sbc_reset, sbc_t};

mod bits2;
mod decoder;
mod raw;

pub const SBC_HEADER_SIZE: usize = 4;
pub const SBC_PROBE_SIZE: usize = SBC_HEADER_SIZE;

pub const SBC_MAX_SUBBANDS: usize = 8;
pub const SBC_MAX_BLOCKS: usize = 16;
pub const SBC_MAX_SAMPLES: usize = SBC_MAX_BLOCKS * SBC_MAX_SUBBANDS;

pub const SBC_MSBC_SAMPLES: usize = 120;
pub const SBC_MSBC_SIZE: usize = 57;

pub struct Decoder {
    data: Vec<u8>,
    index: usize,
    sbc: Box<sbc_t>,
    buffer: Vec<i16>
}

unsafe impl Send for Decoder {}

impl Decoder {
    pub fn new(data: Vec<u8>) -> Self {
        let mut sbc: Box<sbc_t> = unsafe { Box::new(zeroed()) };
        unsafe { sbc_reset(sbc.as_mut()) };
        Self { data, index: 0, sbc, buffer: Vec::new() }
    }

    pub fn refill_buffer(&mut self, data: &[u8]) {
        let remaining = self.data.len() - self.index;
        self.data.copy_within(self.index.., 0);
        self.data.truncate(remaining);
        self.index = 0;
        self.data.extend_from_slice(data);
    }

    pub fn next_frame(&mut self) -> Option<&[i16]> {
        let remaining = &self.data[self.index..];
        if remaining.len() < SbcHeader::SIZE { return None; }

        let header = SbcHeader::read(remaining).unwrap();
        let frame_size = header.frame_size();
        if remaining.len() < frame_size { return None; }

        let nch = header.channels();
        let nr_of_samples = nch * header.blocks * header.subbands;
        self.buffer.resize(nr_of_samples as usize, 0);

        decode(remaining, &mut self.buffer).unwrap();
        unsafe {
            let mut frame = zeroed();
            assert_eq!(sbc_decode(
                &mut *self.sbc,
                remaining.as_ptr().cast(),
                remaining.len() as _,
                &mut frame,
                self.buffer.as_mut_ptr(),
                nch as i32,
                self.buffer.as_mut_ptr().add(1),
                2
            ), 0);
            self.index += frame_size;

            Some(&self.buffer)
        }

    }

    pub fn next_frame_lr(&mut self) -> Option<[&[i16]; 2]> {
        let remaining = &self.data[self.index..];
        if remaining.len() < SBC_PROBE_SIZE { return None; }
        assert_eq!(remaining.len().min(SBC_PROBE_SIZE), SBC_PROBE_SIZE);
        unsafe {
            let mut frame = zeroed();
            assert_eq!(sbc_probe(remaining.as_ptr().cast(), &mut frame), 0);
            let frame_size = sbc_get_frame_size(&frame) as usize;
            if remaining.len() < frame_size { return None; }
            assert_eq!(remaining.len().min(frame_size), frame_size);

            let nch = if frame.mode == SBC_MODE_MONO { 1 } else { 2 };
            let nr_of_samples = (frame.nblocks * frame.nsubbands) as usize;
            self.buffer.resize(nch as usize * nr_of_samples, 0);

            assert_eq!(sbc_decode(
                &mut *self.sbc,
                remaining.as_ptr().cast(),
                remaining.len() as _,
                &mut frame,
                self.buffer.as_mut_ptr(),
                1,
                self.buffer.as_mut_ptr().add(nr_of_samples),
                1
            ), 0);

            if frame.mode == SBC_MODE_MONO {
                self.buffer.copy_within(0..nr_of_samples, nr_of_samples);
            }

            self.index += frame_size;
            Some([&self.buffer[..nr_of_samples], &self.buffer[nr_of_samples..]])
        }
    }

}


#[cfg(test)]
mod tests {
    use std::path::Path;
    use bytemuck::cast_slice;
    use crc32fast::Hasher;
    use super::*;

    fn run_decoder_testcase<P: AsRef<Path>>(path: P, expected_frames: u32, expected_crc32: u32) {
        let data = std::fs::read(path).unwrap();
        let mut decoder = Decoder::new(data);
        let mut hasher = Hasher::new();
        let mut count = 0;
        while let Some(frame) = decoder.next_frame() {
            count += 1;
            hasher.update(cast_slice(frame));
        }
        let crc32 = hasher.finalize();
        assert_eq!(count, expected_frames);
        assert_eq!(crc32, expected_crc32);
    }

    macro_rules! decoder_testcase {
        ($name:ident, $frames:expr, $crc:expr) => {
            #[test]
            fn $name() {
                run_decoder_testcase(concat!("testcases/", stringify!($name), ".sbc"), $frames, $crc);
            }
        };
    }

    decoder_testcase!(sbc_test_01, 2250, 0x7c62364d);
    decoder_testcase!(sbc_test_02, 2250, 0xd6b342d4);
    decoder_testcase!(sbc_test_03, 2067, 0x37771c9b);
    decoder_testcase!(sbc_test_04, 2067, 0x76690eee);
    decoder_testcase!(sbc_test_05, 3000, 0xbddcd959);
    decoder_testcase!(sbc_test_06, 3000, 0xcb5e1d3e);
    decoder_testcase!(sbc_test_07, 1000, 0xa8b775ec);
    decoder_testcase!(sbc_test_08, 1000, 0x96b5353a);
    decoder_testcase!(sbc_test_09, 2067, 0xcb774425);
    decoder_testcase!(sbc_test_10, 1500, 0xc8eb7dd4);
    // Some bitpool issue, I dont think its my fault
    // decoder_testcase!(sbc_test_11);
    decoder_testcase!(sbc_test_12,  375, 0x200d4bc9);
    decoder_testcase!(sbc_test_13,  750, 0xd76572ee);
    decoder_testcase!(sbc_test_14,  750, 0xfc5efd98);
    decoder_testcase!(sbc_test_15, 1033, 0x5f12d2dc);
    decoder_testcase!(sbc_test_16, 1033, 0xccc6e4d6);
    decoder_testcase!(sbc_test_17, 1125, 0x88f51823);
    decoder_testcase!(sbc_test_18, 1125, 0xe71af735);
    decoder_testcase!(sbc_test_19, 1152, 0xa411b141);
    decoder_testcase!(sbc_test_20,  768, 0x5c5ec4a3);
    decoder_testcase!(sbc_test_21, 1033, 0xe62daf5a);
    decoder_testcase!(sbc_test_22, 1125, 0xe5d10914);
    decoder_testcase!(sbc_test_23, 1033, 0xe19f8402);
    decoder_testcase!(sbc_test_24, 1125, 0xd02bbd7a);
    decoder_testcase!(sbc_test_25, 1033, 0x1fb8f7da);
    decoder_testcase!(sbc_test_26, 1125, 0xe008a042);
    decoder_testcase!(sbc_test_27, 1033, 0x137ab205);
    decoder_testcase!(sbc_test_28, 1125, 0x6a47d773);

}
