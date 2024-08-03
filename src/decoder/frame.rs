use crate::bits2::Bits;
use crate::decoder::{Bam, ChannelMode, Error, MAX_CHANNELS, MAX_SAMPLES, MAX_SUBBANDS, Reason, SbcHeader};

pub fn decode_frame(
    bits: &mut Bits,
    header: &SbcHeader,
    samples: &mut [[i16; MAX_SAMPLES]; MAX_CHANNELS],
    scale: &mut [i32; MAX_CHANNELS],
) -> Result<(), Error> {
    const RANGE_SCALE: [i32; 16] = [
        0xfffffff, 0x5555556, 0x2492492, 0x1111111, 0x0842108, 0x0410410, 0x0204081, 0x0101010,
        0x0080402, 0x0040100, 0x0020040, 0x0010010, 0x0008004, 0x0004001, 0x0002000, 0x0001000,
    ];

    /* --- Decode joint bands indications --- */

    let mjoint = if header.mode == ChannelMode::JointStereo && header.subbands == 4 {
        let v = bits.get_bits(4);
        ((0x00) << 3) | ((v & 0x02) << 1) | ((v & 0x04) >> 1) | ((v & 0x08) >> 3)
    } else if header.mode == ChannelMode::JointStereo {
        let v = bits.get_bits(8);
        ((0x00) << 7)
            | ((v & 0x02) << 5)
            | ((v & 0x04) << 3)
            | ((v & 0x08) << 1)
            | ((v & 0x10) >> 1)
            | ((v & 0x20) >> 3)
            | ((v & 0x40) >> 5)
            | ((v & 0x80) >> 7)
    } else {
        0
    } as i32;
    let channels = header.channels() as usize;
    let subbands = header.subbands as usize;
    let mut scale_factors = [[0i32; MAX_SUBBANDS]; 2];
    let mut nbits = [[0i32; MAX_SUBBANDS]; 2];
    for ich in 0..channels {
        for isb in 0..subbands {
            scale_factors[ich][isb] = bits.get_bits(4) as i32;
        }
    }
    compute_nbits(header, &scale_factors, &mut nbits);
    if header.mode == ChannelMode::DualChannel {
        compute_nbits(header, &scale_factors[1..], &mut nbits[1..]);
    }

    /* --- Decode samples ---
     *
     * They are unquantized according :
     *
     *                  2 sample + 1
     *   sb_sample = ( -------------- - 1 ) 2^(scf + 1)
     *                   2^nbit - 1
     *
     * A sample is coded on maximum 16 bits, and the scale factor is limited
     * to 15 bits. Thus the dynamic of sub-bands samples are 17 bits.
     * Regarding "Joint-Stereo" sub-bands, uncoupling increase the dynamic
     * to 18 bits.
     *
     * The `1 / (2^nbit - 1)` values are precalculated on 1.28 :
     *
     *   sb_sample = ((2 sample + 1) * range_scale - 2^28) / 2^shr
     *
     *   with  shr = 28 - ((scf + 1) + sb_scale)
     *         sb_scale = (15 - max(scale_factor[])) - (18 - 16)
     *
     * We introduce `sb_scale`, to limit the range on 16 bits, or increase
     * precision when the scale-factor of the frame is below 13. */

    for ich in 0..channels {
        let mut max_scf = 0;
        for isb in 0..subbands {
            let scf = scale_factors[ich][isb] + ((mjoint >> (isb as u32)) & 1);
            if scf > max_scf {
                max_scf = scf;
            }
        }
        scale[ich] = 15 - max_scf - (17 - 16);
    }

    if header.mode == ChannelMode::JointStereo {
        scale[0] = i32::min(scale[0], scale[1]);
        scale[1] = i32::min(scale[0], scale[1]);
    }

    for iblk in 0..(header.blocks as usize) {
        for ich in 0..channels {
            let mut index = iblk * subbands;
            for isb in 0..subbands {
                let nbit = nbits[ich][isb];
                let scf = scale_factors[ich][isb];

                if nbit == 0 {
                    samples[ich][index] = 0;
                    index += 1;
                    continue;
                }

                let s = bits.get_bits(nbit as u32) as i32;
                let s = ((s << 1) | 1) * RANGE_SCALE[(nbit - 1) as usize];
                samples[ich][index] = ((s - (1 << 28)) >> (28 - ((scf + 1) + scale[ich]))) as i16;
                index += 1;
            }
        }
    }

    /* --- Uncoupling "Joint-Stereo" ---
     *
     * The `Left/Right` samples are coded as :
     *   `sample(left ) = sample(ch 0) + sample(ch 1)`
     *   `sample(right) = sample(ch 0) - sample(ch 1)` */

    for isb in 0..subbands {
        if (mjoint >> isb) & 1 == 0 {
            continue;
        }

        for iblk in 0..(header.blocks as usize) {
            let s0 = samples[0][iblk * subbands + isb];
            let s1 = samples[1][iblk * subbands + isb];

            samples[0][iblk * subbands + isb] = s0 + s1;
            samples[1][iblk * subbands + isb] = s0 - s1;
        }
    }

    /* --- Remove padding --- */

    let padding_nbits = 8 - bits.pos() % 8;
    if padding_nbits < 8 {
        ensure!(bits.get_bits(padding_nbits as u32) == 0, Reason::UnexpectedData);
    }
    Ok(())
}

/// Compute the bit distribution for Independent or Stereo Channel
fn compute_nbits(
    header: &SbcHeader,
    scale_factors: &[[i32; MAX_SUBBANDS]],
    nbits: &mut [[i32; MAX_SUBBANDS]],
) {
    /* --- Offsets of "Loudness" bit allocation --- */

    #[rustfmt::skip]
    const LOUDNESS_OFFSET_4: [[i32; 4]; 4] = [
        [-1, 0, 0, 0],
        [-2, 0, 0, 1],
        [-2, 0, 0, 1],
        [-2, 0, 0, 1]
    ];

    #[rustfmt::skip]
    const LOUDNESS_OFFSET_8: [[i32; 8]; 4] = [
        [-2,  0,  0,  0,  0,  0,  0,  1],
        [-3,  0,  0,  0,  0,  0,  1,  2],
        [-4,  0,  0,  0,  0,  0,  1,  2],
        [-4,  0,  0,  0,  0,  0,  1,  2]
    ];

    /* --- Compute the number of bits needed --- */

    let loundness_offset = match header.subbands == 4 {
        true => LOUDNESS_OFFSET_4[header.freq as usize].as_slice(),
        false => LOUDNESS_OFFSET_8[header.freq as usize].as_slice(),
    };

    let subbands = header.subbands as usize;
    let channels = if header.mode.is_stereo() { 2 } else { 1 };

    let mut bitneeds = [[0i32; MAX_SUBBANDS]; MAX_CHANNELS];
    let mut max_bitneed = 0;

    for ich in 0..channels {
        for isb in 0..subbands {
            let scf = scale_factors[ich][isb];

            let bitneed = match header.bam {
                Bam::Loudness => {
                    let bitneed = match scf == 0 {
                        true => -5,
                        false => scf - loundness_offset[isb],
                    };
                    bitneed >> i32::from(bitneed > 0)
                }
                Bam::Snr => scf,
            };

            max_bitneed = i32::max(max_bitneed, bitneed);
            bitneeds[ich][isb] = bitneed;
        }
    }

    /* --- Loop over the bit distribution, until reaching the bitpool --- */

    let bitpool = header.bitpool as usize;

    let mut bitcount = 0;
    let mut bitslice = max_bitneed + 1;

    let mut bc = 0;
    while bc < bitpool {
        let bs = bitslice;
        bitslice -= 1;
        bitcount = bc;
        if bitcount == bitpool {
            break;
        }
        for ich in 0..channels {
            for isb in 0..subbands {
                let bn = bitneeds[ich][isb];
                bc += usize::from(bn >= bs && bn < bs + 15) + usize::from(bn == bs);
            }
        }
    }

    /* --- Bits distribution --- */
    for ich in 0..channels {
        for isb in 0..subbands {
            let nbit = bitneeds[ich][isb] - bitslice;
            nbits[ich][isb] = match nbit < 2 {
                true => 0,
                false => match nbit > 16 {
                    true => 16,
                    false => nbit,
                },
            };
        }
    }
    /* --- Allocate remaining bits --- */
    for isb in 0..subbands {
        for ich in 0..channels {
            if bitcount >= bitpool {
                break;
            }

            let n = match nbits[ich][isb] != 0 && nbits[ich][isb] < 16 {
                true => 1,
                false => match bitneeds[ich][isb] == bitslice + 1 && bitpool > bitcount + 1 {
                    true => 2,
                    false => 0,
                },
            };

            nbits[ich][isb] += n;
            bitcount += n as usize;
        }
    }

    for isb in 0..subbands {
        for ich in 0..channels {
            if bitcount >= bitpool {
                break;
            }
            let n = i32::from(nbits[ich][isb] < 16);
            nbits[ich][isb] += n;
            bitcount += n as usize;
        }
    }
}

