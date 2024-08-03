use crate::bits2::Bits;
use crate::decoder::{Bam, ChannelMode, Frequency, SbcError, SbcHeader};

impl SbcHeader {
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
        let mut bits = Bits::new(
            crate::bits2::Mode::Read,
            data.get(..Self::SIZE).ok_or_else(SbcError::new)?,
        );

        /* --- Decode header ---
         *
         * Two possible headers :
         * - Header, with syncword 0x9c (A2DP)
         * - mSBC header, with syncword 0xad (HFP) */

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
            return Err(SbcError::new());
        };
        frame.crc = bits.get_bits(8) as u8;

        /* --- Check bitpool value and return --- */
        ensure!(frame.blocks - 4 <= 12 && (frame.msbc || frame.blocks % 4 == 0));
        ensure!(frame.subbands - 4 <= 4 && frame.subbands % 4 == 0);
        let two_channels = u32::from(frame.mode != ChannelMode::Mono);
        let dual_mode = u32::from(frame.mode == ChannelMode::DualChannel);
        let joint_mode: bool = frame.mode == ChannelMode::JointStereo;
        let stereo_mode = u32::from(joint_mode || frame.mode == ChannelMode::Stereo);
        let max_bits = ((16 * frame.subbands * frame.blocks) << two_channels)
            - 4 * 8
            - ((4 * frame.subbands) << two_channels)
            - joint_mode.then_some(frame.subbands).unwrap_or_default();
        let max_bitpool =
            match max_bits / (frame.blocks << dual_mode) < (16 << stereo_mode * frame.subbands) {
                true => max_bits / (frame.blocks << dual_mode),
                false => (16 << stereo_mode) * frame.subbands,
            };
        ensure!(frame.bitpool <= max_bitpool);

        Ok(frame)
    }

    pub fn frame_size(&self) -> usize {
        let two_channels = u32::from(self.mode != ChannelMode::Mono);
        let dual_mode = u32::from(self.mode == ChannelMode::DualChannel);
        let joint_mode: bool = self.mode == ChannelMode::JointStereo;
        let nbits = ((4 * self.subbands) << two_channels)
            + ((self.blocks * self.bitpool) << dual_mode)
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