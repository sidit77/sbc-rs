use std::panic::Location;

use crate::bits2::{Bits, Mode};
use crate::decoder::crc::compute_crc;
use crate::decoder::frame::decode_frame;
use crate::decoder::synthesize::synthesize;

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

#[derive(Debug)]
pub enum OutputFormat<'a> {
    Mono(&'a mut [i16]),
    Interleaved(&'a mut [i16]),
    Planar(&'a mut [i16], &'a mut [i16]),
}

impl<'a> OutputFormat<'a> {

    pub fn interleaved(buf: &'a mut[i16], mono: bool) -> Self {
        match mono {
            true => OutputFormat::Mono(buf),
            false => OutputFormat::Interleaved(buf)
        }
    }

    fn left(&mut self) -> BufferView<'_> {
        match self {
            OutputFormat::Mono(buf) => BufferView {
                buf: *buf,
                pitch: 1,
                offset: 0,
            },
            OutputFormat::Interleaved(buf) => BufferView {
                buf: *buf,
                pitch: 2,
                offset: 0,
            },
            OutputFormat::Planar(l, _) => BufferView {
                buf: *l,
                pitch: 1,
                offset: 0,
            },
        }
    }

    fn right(&mut self) -> BufferView<'_> {
        match self {
            OutputFormat::Interleaved(buf) => BufferView {
                buf: *buf,
                pitch: 2,
                offset: 1,
            },
            OutputFormat::Planar(_, r) => BufferView {
                buf: *r,
                pitch: 1,
                offset: 0,
            },
            OutputFormat::Mono(_) => unreachable!()
        }
    }

}

struct BufferView<'a> {
    buf: &'a mut [i16],
    pitch: usize,
    offset: usize
}

impl<'a> BufferView<'a> {
    pub fn len(&self) -> usize {
        self.buf.len() / self.pitch
    }
}

#[derive(Default)]
struct DecoderState {
    index: usize,
    pub v: [[[i16; 10]; 8]; 2]
}

#[derive(Default)]
pub struct Decoder {
    state: [DecoderState; 2]
}

impl Decoder {
    pub const MAX_SAMPLES: usize = MAX_SAMPLES;
    pub fn reset(&mut self) {
        self.state = Default::default();
    }

    pub fn decode(&mut self, data: &[u8], mut output: OutputFormat<'_>) -> Result<DecodeStatus, SbcError> {
        let header = SbcHeader::read(data)?;
        let frame_size = header.frame_size();
        let data = data.get(..frame_size).ok_or_else(SbcError::new)?;
        ensure!(compute_crc(&header, data)? == header.crc);
        let mut samples = [[0i16; MAX_SAMPLES]; MAX_CHANNELS];
        let mut scale = [0i32; MAX_CHANNELS];

        let mut bits = Bits::new(Mode::Read, data.get(SbcHeader::SIZE..).ok_or_else(SbcError::new)?);
        decode_frame(&mut bits, &header, &mut samples, &mut scale)?;

        let left = output.left();
        synthesize(
            &mut self.state[0],
            header.blocks as usize,
            header.subbands as usize,
            &samples[0],
            scale[0],
            left,
        );
        if header.mode != ChannelMode::Mono {
            let right = output.right();
            synthesize(
                &mut self.state[1],
                header.blocks as usize,
                header.subbands as usize,
                &samples[1],
                scale[1],
                right
            );
        }
        Ok(DecodeStatus {
            bytes_read: frame_size,
            samples_written: header.blocks as usize * header.subbands as usize,
            channels: header.channels()
        })
    }
}

pub struct DecodeStatus {
    pub bytes_read: usize,
    pub samples_written: usize,
    pub channels: u32
}

