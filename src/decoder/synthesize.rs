use crate::raw::{int16_t, sbc_dstate};
use std::ffi::c_int;
use std::slice;

#[inline]
fn sat16(v: i32) -> i16 {
    v.max(i16::MIN as i32).min(i16::MAX as i32) as i16
}

#[inline]
fn destructure2<T>(v: &mut[T; 2], swap: bool) -> (&mut T, &mut T) {
    let [a, b] = v;
    match swap {
        true => (b, a),
        false => (a, b)
    }
}

#[inline]
fn widened_mul((a, b): (&i16, &i16)) -> i32 {
    (*a as i32) * (*b as i32)
}

#[inline]
fn dct4(data: &[i16], scale: i32, out0: &mut [[i16; 10]; 8], out1: &mut [[i16; 10]; 8], idx: usize) {
    /* cos(i*pi/8)  for i = [0;3], in fixed 0.13 */
    const COS8: [i32; 4] = [8192, 7568, 5793, 3135];

    /* --- DCT of subbands samples ---
     *          ___
     *          \
     *   u[k] = /__  h(k,i) * s(i) , i = [0;n-1]  k = [0;2n-1]
     *           i
     *
     *   With  n the number of subbands (4)
     *         h(k,i) = cos( (i + 1/2) (k + n/2) pi/n )
     *
     * Note :
     *
     *     h( 2, i) =  0 et h( n-k,i) = -h(k,i)   , k = [0;n/2-1]
     *     h(12, i) = -1 et h(2n-k,i) =  h(n+k,i) , k = [1;n/2-1]
     *
     * To assist the windowing step, the 2 halves are stored in 2 buffers.
     * After scaling of coefficients, the result is saturated on 16 bits. */
    let s03 = (data[0] as i32 + data[3] as i32) >> 1;
    let d03 = (data[0] as i32 - data[3] as i32) >> 1;
    let s12 = (data[1] as i32 + data[2] as i32) >> 1;
    let d12 = (data[1] as i32 - data[2] as i32) >> 1;

    let mut a0 = (s03 - s12) * COS8[2];
    let mut b1 = (-(s03 + s12)) << 13;
    let mut a1 = d03 * COS8[3] - d12 * COS8[1];
    let mut b0 = -d03 * COS8[1] - d12 * COS8[3];

    let shr = 12 + scale;

    a0 = (a0 + (1 << (shr - 1))) >> shr;
    b0 = (b0 + (1 << (shr - 1))) >> shr;
    a1 = (a1 + (1 << (shr - 1))) >> shr;
    b1 = (b1 + (1 << (shr - 1))) >> shr;

    out0[0][idx] = sat16(a0);
    out0[1][idx] = sat16(a1);
    out0[2][idx] = sat16(0);
    out0[3][idx] = sat16(-a1);

    out1[0][idx] = sat16(-a0);
    out1[1][idx] = sat16(b0);
    out1[2][idx] = sat16(b1);
    out1[3][idx] = sat16(b0);
}

#[inline]
fn dct8(data: &[i16], scale: i32, out0: &mut [[i16; 10]; 8], out1: &mut [[i16; 10]; 8], idx: usize) {
    /* cos(i*pi/16)  for i = [0;7], in fixed 0.13 */
    const COS16: [i32; 8] = [8192, 8035, 7568, 6811, 5793, 4551, 3135, 1598];

    /* --- DCT of subbands samples ---
     *          ___
     *          \
     *   u[k] = /__  h(k,i) * s(i) , i = [0;n-1]  k = [0;2n-1]
     *           i
     *
     *   With  n the number of subbands (8)
     *         h(k,i) = cos( (i + 1/2) (k + n/2) pi/n )
     *
     *
     *
     * Note :
     *
     *     h( 4, i) =  0 et h( n-k,i) = -h(k,i)   , k = [0;n/2-1]
     *     h(12, i) = -1 et h(2n-k,i) =  h(n+k,i) , k = [1;n/2-1]
     *
     * To assist the windowing step, the 2 halves are stored in 2 buffers.
     * After scaling of coefficients, the result is saturated on 16 bits. */
    let s07 = (data[0] as i32 + data[7] as i32) >> 1;
    let d07 = (data[0] as i32 - data[7] as i32) >> 1;
    let s16 = (data[1] as i32 + data[6] as i32) >> 1;
    let d16 = (data[1] as i32 - data[6] as i32) >> 1;
    let s25 = (data[2] as i32 + data[5] as i32) >> 1;
    let d25 = (data[2] as i32 - data[5] as i32) >> 1;
    let s34 = (data[3] as i32 + data[4] as i32) >> 1;
    let d34 = (data[3] as i32 - data[4] as i32) >> 1;

    let mut a0 = ( (s07 + s34) - (s25 + s16)) * COS16[4];
    let mut b3 = (-(s07 + s34) - (s25 + s16)) << 13;
    let mut a2 = (s07 - s34) * COS16[6] + (s25 - s16) * COS16[2];
    let mut b1 = (s34 - s07) * COS16[2] + (s25 - s16) * COS16[6];
    let mut a1 =  d07 * COS16[5] - d16 * COS16[1] + d25 * COS16[7] + d34 * COS16[3];
    let mut b2 = -d07 * COS16[1] - d16 * COS16[3] - d25 * COS16[5] - d34 * COS16[7];
    let mut a3 =  d07 * COS16[7] - d16 * COS16[5] + d25 * COS16[3] - d34 * COS16[1];
    let mut b0 = -d07 * COS16[3] + d16 * COS16[7] + d25 * COS16[1] + d34 * COS16[5];

    let shr = 12 + scale;

    a0 = (a0 + (1 << (shr-1))) >> shr;
    a1 = (a1 + (1 << (shr-1))) >> shr;
    a2 = (a2 + (1 << (shr-1))) >> shr;
    a3 = (a3 + (1 << (shr-1))) >> shr;
    b0 = (b0 + (1 << (shr-1))) >> shr;
    b1 = (b1 + (1 << (shr-1))) >> shr;
    b2 = (b2 + (1 << (shr-1))) >> shr;
    b3 = (b3 + (1 << (shr-1))) >> shr;

    out0[0][idx] = sat16( a0);
    out0[1][idx] = sat16( a1);
    out0[2][idx] = sat16( a2);
    out0[3][idx] = sat16( a3);
    out0[4][idx] = sat16(  0);
    out0[5][idx] = sat16(-a3);
    out0[6][idx] = sat16(-a2);
    out0[7][idx] = sat16(-a1);

    out1[0][idx] = sat16(-a0);
    out1[1][idx] = sat16( b0);
    out1[2][idx] = sat16( b1);
    out1[3][idx] = sat16( b2);
    out1[4][idx] = sat16( b3);
    out1[5][idx] = sat16( b2);
    out1[6][idx] = sat16( b1);
    out1[7][idx] = sat16( b0);
}

#[inline]
fn apply_window<const N: usize>(
    data: &[[i16; 10]; 8],
    window: &[[i16; 20]; N],
    offset: usize,
    mut out: *mut i16,
    pitch: usize,
) {
    debug_assert!(data.len() >= window.len());
    for (w, u) in window.iter().zip(data) {
        debug_assert!(w.len() - offset >= u.len());
        let s = w.iter().skip(offset).zip(u).map(widened_mul).sum::<i32>();
        unsafe {
            *out = sat16((s + (1 << 12)) >> 13);
            out = out.offset(pitch as isize);
        }
    }
}

/* --- Windowing coefficients (fixed 2.13) ---
     *
     * The table is duplicated and transposed to fit the circular
     * buffer of reconstructed samples */

#[rustfmt::skip]
const WINDOW4: [[i16; 20]; 4] = [
    [
        0, -126, -358, -848, -4443, -9644, 4443, -848, 358, -126,
        0, -126, -358, -848, -4443, -9644, 4443, -848, 358, -126,
    ],
    [
        -18, -128, -670, -201, -6389, -9235, 2544, -1055, 100, -90,
        -18, -128, -670, -201, -6389, -9235, 2544, -1055, 100, -90,
    ],
    [
        -49, -61, -946, 944, -8082, -8082, 944, -946, -61, -49,
        -49, -61, -946, 944, -8082, -8082, 944, -946, -61, -49,
    ],
    [
        -90, 100, -1055, 2544, -9235, -6389, -201, -670, -128, -18,
        -90, 100, -1055, 2544, -9235, -6389, -201, -670, -128, -18,
    ],
];

#[rustfmt::skip]
const WINDOW8: [[i16; 20]; 8] = [
    [
        0, -132,  -371, -848, -4456, -9631, 4456,  -848,  371, -132,
        0, -132,  -371, -848, -4456, -9631, 4456,  -848,  371, -132
    ],
    [
        -10, -138,  -526, -580, -5438, -9528, 3486, -1004,  229, -117,
        -10, -138,  -526, -580, -5438, -9528, 3486, -1004,  229, -117
    ],
    [
        -22, -131,  -685, -192, -6395, -9224, 2561, -1063,  108,  -97,
        -22, -131,  -685, -192, -6395, -9224, 2561, -1063,  108,  -97
    ],
    [
        -36, -106,  -835,  322, -7287, -8734, 1711, -1042,   12,  -75,
        -36, -106,  -835,  322, -7287, -8734, 1711, -1042,   12,  -75
    ],
    [
        -54,  -59,  -960,  959, -8078, -8078,  959,  -960,  -59,  -54,
        -54,  -59,  -960,  959, -8078, -8078,  959,  -960,  -59,  -54
    ],
    [
        -75,   12, -1042, 1711, -8734, -7287,  322,  -835, -106,  -36,
        -75,   12, -1042, 1711, -8734, -7287,  322,  -835, -106,  -36
    ],
    [
        -97,  108, -1063, 2561, -9224, -6395, -192,  -685, -131,  -22,
        -97,  108, -1063, 2561, -9224, -6395, -192,  -685, -131,  -22
    ],
    [
        -117,  229, -1004, 3486, -9528, -5438, -580,  -526, -138,  -10,
        -117,  229, -1004, 3486, -9528, -5438, -580,  -526, -138,  -10
    ]
];

type DctFn = fn(&[i16], i32, &mut [[i16; 10]; 8], &mut [[i16; 10]; 8], usize);

#[inline]
fn synthesize_block<const N: usize>(
    windows: &[[i16; 20]; N],
    dct: DctFn,
    state: &mut sbc_dstate,
    data: &[i16],
    scale: i32,
    out: *mut i16,
    pitch: usize,
) {
    /* --- IDCT and windowing --- */
    let dct_idx = match state.idx != 0 {
        true => 10 - state.idx,
        false => 0
    };
    let odd = dct_idx % 2 == 1;
    let (a, b) = destructure2(&mut state.v, odd);
    dct(data, scale, a, b, dct_idx as usize);
    apply_window(a, windows, state.idx as usize, out, pitch as usize);
    state.idx = match state.idx < 9 {
        true => state.idx + 1,
        false => 0
    };
}

pub unsafe fn synthesize(
    state: *mut sbc_dstate,
    blocks: usize,
    subbands: usize,
    data: &[i16],
    scale: i32,
    mut out: *mut int16_t,
    pitch: usize,
) {
    for block in data.chunks_exact(subbands).take(blocks) {
        if subbands == 4 {
            synthesize_block(&WINDOW4, dct4, &mut *state, block, scale, out, pitch);
        } else {
            synthesize_block(&WINDOW8, dct8, &mut *state, block, scale, out, pitch);
        }
        out = out.offset((subbands * pitch) as isize);
    }
}
