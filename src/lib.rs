mod bits;
mod sbc;

const SBC_HEADER_SIZE: usize = 4;
const SBC_PROBE_SIZE: usize = SBC_HEADER_SIZE;

const SBC_MAX_SUBBANDS: usize = 8;
const SBC_MAX_BLOCKS: usize = 16;
const SBC_MAX_SAMPLES: usize = SBC_MAX_BLOCKS * SBC_MAX_SUBBANDS;

const SBC_MSBC_SAMPLES: usize = 120;
const SBC_MSBC_SIZE: usize = 57;

#[cfg(test)]
mod tests {
    use std::mem::zeroed;
    use crate::sbc::{sbc_decode, sbc_get_frame_size, sbc_probe, sbc_reset};
    use super::*;

    #[test]
    fn it_works() {
        let data = std::fs::read("../bluefang/target/sbc/output.sbc").unwrap();
        let mut index = 0;
        unsafe {
            let mut sbc = zeroed();
            sbc_reset(&mut sbc);

            let mut pcm = [0i16; 2 * SBC_MAX_SAMPLES];

            let mut number_of_frames = 0;
            while index < data.len() {
                let remaining = &data[index..];
                assert_eq!(remaining.len().min(SBC_PROBE_SIZE), SBC_PROBE_SIZE);
                let mut frame = zeroed();
                assert_eq!(sbc_probe(remaining.as_ptr().cast(), &mut frame), 0);
                let frame_size = sbc_get_frame_size(&frame) as usize;
                assert_eq!(remaining.len().min(frame_size), frame_size);

                assert_eq!(sbc_decode(
                    &mut sbc,
                    remaining.as_ptr().cast(),
                    remaining.len() as _,
                    &mut frame,
                    pcm.as_mut_ptr(),
                    2,
                    pcm.as_mut_ptr().add(1),
                    2
                ), 0);

                index += frame_size;
                number_of_frames += 1;
            }

            println!("Decoded {} frames", number_of_frames);
        }
    }
}
