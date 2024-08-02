use std::ffi::{c_int, c_uint};
use std::panic::Location;
use crate::bits2::{Bits, Mode as BitMode};
use crate::raw::{sbc_frame, SBC_MODE_JOINT_STEREO, SBC_MODE_MONO, uint8_t};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Frequency {
    Hz16k,
    Hz32k,
    Hz44k,
    Hz48k,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ChannelMode {
    Mono,
    DualChannel,
    Stereo,
    JointStereo,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Bam {
    Snr,
    Loudness,
}

#[derive(Debug, Copy, Clone)]
pub struct SbcError {
    location: &'static Location<'static>,
}

impl SbcError {
    #[track_caller]
    fn new() -> Self {
        Self { location: Location::caller() }
    }
}

macro_rules! ensure {
    ($cond:expr) => {
        if !$cond {
            return Err(SbcError::new());
        }
    };
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
    pub crc: u8
}

impl SbcHeader {

    pub const SIZE: usize = 4;

    const fn new_msbc() -> Self {
        Self {
            msbc: true,
            freq: Frequency::Hz16k,
            mode: ChannelMode::Mono,
            bam: Bam::Loudness,
            blocks: 15,
            subbands: 8,
            bitpool: 26,
            crc: 0,
        }
    }

    pub fn read(data: &[u8]) -> Result<Self, SbcError> {
        let mut bits = Bits::new(BitMode::Read, data.get(..Self::SIZE).ok_or_else(SbcError::new)?);

        let syncword = bits.get_bits(8);
        let msbc = syncword == 0xad;
        let mut frame = if msbc {
            bits.advance(16);
            Self::new_msbc()
        } else if syncword == 0x9c {
            let freq = match bits.get_bits(2) {
                0 => Frequency::Hz16k,
                1 => Frequency::Hz32k,
                2 => Frequency::Hz44k,
                3 => Frequency::Hz48k,
                _ => return Err(SbcError::new()),
            };
            let blocks = (1 + bits.get_bits(2)) << 2;
            let mode = match bits.get_bits(2) {
                0 => ChannelMode::Mono,
                1 => ChannelMode::DualChannel,
                2 => ChannelMode::Stereo,
                3 => ChannelMode::JointStereo,
                _ => return Err(SbcError::new()),
            };
            let bam = match bits.get_bits(1) {
                0 => Bam::Loudness,
                1 => Bam::Snr,
                _ => return Err(SbcError::new()),
            };
            let subbands = (1 + bits.get_bits(1)) << 2;
            let bitpool = bits.get_bits(8);

            Self {
                msbc,
                freq,
                mode,
                bam,
                blocks,
                subbands,
                bitpool,
                crc: 0,
            }
        } else {
            return  Err(SbcError::new());
        };
        frame.crc = bits.get_bits(8) as u8;

        ensure!(frame.blocks - 4 <= 12 && (frame.msbc || frame.blocks % 4 == 0));
        ensure!(frame.subbands - 4 <= 4 && frame.subbands % 4 == 0);
        let two_channels = u32::from(frame.mode != ChannelMode::Mono);
        let dual_mode = u32::from(frame.mode == ChannelMode::DualChannel);
        let joint_mode: bool = frame.mode == ChannelMode::JointStereo;
        let stereo_mode = u32::from(joint_mode || frame.mode == ChannelMode::Stereo);
        let max_bits =
            ((16 * frame.subbands * frame.blocks) << two_channels)
                - 4 * 8
                - ((4 * frame.subbands) << two_channels)
                - joint_mode.then_some(frame.subbands).unwrap_or_default();
        let max_bitpool = match max_bits / (frame.blocks << dual_mode) < (16 << stereo_mode * frame.subbands) {
            true => max_bits / (frame.blocks << dual_mode),
            false => (16 << stereo_mode) * frame.subbands,
        };
        frame.bitpool <= max_bitpool;
        ensure!(frame.bitpool <= max_bitpool);

        Ok(frame)
    }

    pub fn frame_size(&self) -> usize {
        let two_channels = u32::from(self.mode != ChannelMode::Mono);
        let dual_mode = u32::from(self.mode == ChannelMode::DualChannel);
        let joint_mode: bool = self.mode == ChannelMode::JointStereo;
        let nbits = ((4 * self.subbands) << two_channels) + ((self.blocks * self.bitpool) << dual_mode)
            + joint_mode.then_some(self.subbands).unwrap_or_default();
        (4 + ((nbits + 7) >> 3)) as usize
    }

    pub fn channels(&self) -> u32 {
        match self.mode {
            ChannelMode::Mono => 1,
            _ => 2,
        }
    }


}

pub fn decode(data: &[u8], out: &mut [i16]) -> Result<(), SbcError> {
    let header = SbcHeader::read(data)?;
    let data = data.get(..header.frame_size()).ok_or_else(SbcError::new)?;
    ensure!(compute_crc(header, data)? == header.crc);
    Ok(())
}

fn compute_crc(header: SbcHeader, data: &[u8]) -> Result<u8, SbcError> {
    const CRC_TABLE: [u8; 256] = [
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
    let nch = header.channels();
    let nsb = header.subbands;
    let nbit = nch * nsb * 4 + (if header.mode == ChannelMode::JointStereo { nsb } else { 0 });
    ensure!(data.len() >= (((4 * 8) + nbit + 7) >> 3) as usize);
    let mut crc: u8 = 0xf;
    crc = CRC_TABLE[(crc ^ data[1]) as usize];
    crc = CRC_TABLE[(crc ^ data[2]) as usize];
    let mut i = 4;
    while i < (4 + (nbit / 8)) as usize {
        crc = CRC_TABLE[(crc ^ data[i]) as usize];
        i += 1;
    }
    if nbit % 8 != 0 {
        crc = (crc << 4) ^ CRC_TABLE[((crc >> 4) ^ (data[i] >> 4)) as usize];
    }
    Ok(crc)
}