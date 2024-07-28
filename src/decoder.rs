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

    pub fn read(data: &[u8]) -> Option<Self> {
        let mut bits = Bits::new(BitMode::Read, data.get(..Self::SIZE)?);

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
                _ => return None,
            };
            let blocks = (1 + bits.get_bits(2)) << 2;
            let mode = match bits.get_bits(2) {
                0 => ChannelMode::Mono,
                1 => ChannelMode::DualChannel,
                2 => ChannelMode::Stereo,
                3 => ChannelMode::JointStereo,
                _ => return None,
            };
            let bam = match bits.get_bits(1) {
                0 => Bam::Loudness,
                1 => Bam::Snr,
                _ => return None,
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
            return None;
        };
        frame.crc = bits.get_bits(8);
        verify_header(&frame)
            .then_some(frame)
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

fn verify_header(header: &SbcHeader) -> bool {
    if header.blocks - 4 > 12 || !header.msbc && header.blocks % 4  != 0 {
        return false;
    }
    if header.subbands - 4 > 4 || header.subbands % 4 != 0 {
        return false;
    }
    let two_channels = u32::from(header.mode != ChannelMode::Mono);
    let dual_mode = u32::from(header.mode == ChannelMode::DualChannel);
    let joint_mode: bool = header.mode == ChannelMode::JointStereo;
    let stereo_mode = u32::from(joint_mode || header.mode == ChannelMode::Stereo);
    let max_bits =
        ((16 * header.subbands * header.blocks) << two_channels)
        - 4 * 8
        - ((4 * header.subbands) << two_channels)
        - joint_mode.then_some(header.subbands).unwrap_or_default();
    let max_bitpool = match max_bits / (header.blocks << dual_mode) < (16 << stereo_mode * header.subbands) {
        true => max_bits / (header.blocks << dual_mode),
        false => (16 << stereo_mode) * header.subbands,
    };
    header.bitpool <= max_bitpool
}