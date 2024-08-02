use std::panic::Location;
use crate::bits2::{Bits, Mode as BitMode};

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
    pub crc: u32
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
        frame.crc = bits.get_bits(8);

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

pub fn decode(header: SbcHeader, data: &[u8], out: &mut [i16]) -> Option<()> {

    Some(())
}