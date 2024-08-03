use std::ffi::c_int;
use std::panic::Location;

use crate::bits2::{Bits, Mode};
use crate::decoder::crc::compute_crc;
use crate::decoder::frame::decode_frame;
use crate::decoder::synthesize::synthesize;
use crate::raw::{int16_t, sbc};

macro_rules! ensure {
    ($cond:expr) => {
        if !$cond {
            return Err(SbcError::new());
        }
    };
}

mod header;
mod frame;
mod crc;
mod synthesize;

const MAX_CHANNELS: usize = 2;
const MAX_SUBBANDS: usize = 8;
const MAX_BLOCKS: usize = 16;
const MAX_SAMPLES: usize = MAX_BLOCKS * MAX_SUBBANDS;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Frequency {
    Hz16k = 0,
    Hz32k = 1,
    Hz44k = 2,
    Hz48k = 3,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ChannelMode {
    Mono,
    DualChannel,
    Stereo,
    JointStereo,
}

impl ChannelMode {
    pub fn is_stereo(&self) -> bool {
        matches!(self, Self::Stereo | Self::JointStereo)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Bam {
    Snr,
    Loudness,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SbcHeader {
    pub msbc: bool,
    pub freq: Frequency,
    pub mode: ChannelMode,
    pub bam: Bam,
    pub blocks: u32,
    pub subbands: u32,
    pub bitpool: u32,
    pub crc: u8,
}

impl SbcHeader {
    pub const SIZE: usize = 4;
}

#[derive(Debug, Copy, Clone)]
pub struct SbcError {
    location: &'static Location<'static>,
}

impl SbcError {
    #[track_caller]
    fn new() -> Self {
        Self {
            location: Location::caller(),
        }
    }
}

pub fn decode(
    data: &[u8],
    sbc: *mut sbc,
    pcml: *mut int16_t,
    pitchl: c_int,
    pcmr: *mut int16_t,
    pitchr: c_int
) -> Result<(), SbcError> {
    let header = SbcHeader::read(data)?;
    let data = data.get(..header.frame_size()).ok_or_else(SbcError::new)?;
    ensure!(compute_crc(&header, data)? == header.crc);
    let mut samples = [[0i16; MAX_SAMPLES]; MAX_CHANNELS];
    let mut scale = [0i32; MAX_CHANNELS];

    let mut bits = Bits::new(Mode::Read, data.get(SbcHeader::SIZE..).ok_or_else(SbcError::new)?);
    decode_frame(&mut bits, &header, &mut samples, &mut scale)?;

    /*
    {
        let mut sb_samples: [[i16; 128]; 2] = [[0; 128]; 2];
        let mut sb_scale: [i32; 2] = [0; 2];

        unsafe {
            let mut frame = std::mem::zeroed();
            let mut bits = Bits::new(Mode::Read, &data[..SbcHeader::SIZE]);
            assert!(crate::raw::decode_header(&mut bits, &mut frame, std::ptr::null_mut()));
            assert!(!bits.has_error());
            let mut bits = Bits::new(Mode::Read, &data[SbcHeader::SIZE..(crate::raw::sbc_get_frame_size(&frame) as usize)]);
            crate::raw::decode_frame(&mut bits, &frame, sb_samples.as_mut_ptr(), sb_scale.as_mut_ptr());
            assert!(!bits.has_error());
        };

        assert_eq!(samples, sb_samples);
        assert_eq!(scale, sb_scale);
    }
    */

    //(*sbc)
    //    .nchannels = 1 as c_int
    //    + ((*frame).mode as c_uint
    //    != SBC_MODE_MONO as c_int as c_uint) as c_int;
    //(*sbc).nblocks = (*frame).nblocks;
    //(*sbc).nsubbands = (*frame).nsubbands;
    unsafe {
        synthesize(
            &mut *((*sbc).c2rust_unnamed.dstates)
                .as_mut_ptr()
                .offset(0),
            header.blocks as c_int,
            header.subbands as c_int,
            samples[0].as_mut_ptr(),
            scale[0],
            pcml,
            pitchl,
        );
        if header.mode != ChannelMode::Mono {
            synthesize(
                &mut *((*sbc).c2rust_unnamed.dstates)
                    .as_mut_ptr()
                    .offset(1),
                header.blocks as c_int,
                header.subbands as c_int,
                samples[1].as_mut_ptr(),
                scale[1],
                pcmr,
                pitchr,
            );
        }
    }
    /*
    crate::raw::synthesize(
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
        crate::raw::synthesize(
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

     */
    Ok(())
}

