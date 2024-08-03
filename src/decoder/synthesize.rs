use std::ffi::c_int;
use std::slice;
use crate::raw::{int16_t, sbc_dstate};

#[inline]
fn sat16(v: i32) -> i16 {
    v.max(i16::MIN as i32).min(i16::MAX as i32) as i16
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

    let mut a0 = (s03 - s12 ) * COS8[2];
    let mut b1 = (-(s03 + s12 )) << 13;
    let mut a1 =  d03 * COS8[3] - d12 * COS8[1];
    let mut b0 = -d03 * COS8[1] - d12 * COS8[3];

    let shr = 12 + scale;

    a0 = (a0 + (1 << (shr-1))) >> shr;
    b0 = (b0 + (1 << (shr-1))) >> shr;
    a1 = (a1 + (1 << (shr-1))) >> shr;
    b1 = (b1 + (1 << (shr-1))) >> shr;

    out0[0][idx] = sat16( a0);
    out0[1][idx] = sat16( a1);
    out0[2][idx] = sat16(  0);
    out0[3][idx] = sat16(-a1);

    out1[0][idx] = sat16(-a0);
    out1[1][idx] = sat16( b0);
    out1[2][idx] = sat16( b1);
    out1[3][idx] = sat16( b0);

}

#[inline]
unsafe fn dct8(
    in_0: *const int16_t,
    scale: c_int,
    out0: *mut [int16_t; 10],
    out1: *mut [int16_t; 10],
    idx: c_int,
) {
    static mut cos16: [int16_t; 8] = [
        8192 as c_int as int16_t,
        8035 as c_int as int16_t,
        7568 as c_int as int16_t,
        6811 as c_int as int16_t,
        5793 as c_int as int16_t,
        4551 as c_int as int16_t,
        3135 as c_int as int16_t,
        1598 as c_int as int16_t,
    ];
    let s07: int16_t = ((*in_0.offset(0 as c_int as isize) as c_int
        + *in_0.offset(7 as c_int as isize) as c_int) >> 1 as c_int)
        as int16_t;
    let d07: int16_t = ((*in_0.offset(0 as c_int as isize) as c_int
        - *in_0.offset(7 as c_int as isize) as c_int) >> 1 as c_int)
        as int16_t;
    let s16: int16_t = ((*in_0.offset(1 as c_int as isize) as c_int
        + *in_0.offset(6 as c_int as isize) as c_int) >> 1 as c_int)
        as int16_t;
    let d16: int16_t = ((*in_0.offset(1 as c_int as isize) as c_int
        - *in_0.offset(6 as c_int as isize) as c_int) >> 1 as c_int)
        as int16_t;
    let s25: int16_t = ((*in_0.offset(2 as c_int as isize) as c_int
        + *in_0.offset(5 as c_int as isize) as c_int) >> 1 as c_int)
        as int16_t;
    let d25: int16_t = ((*in_0.offset(2 as c_int as isize) as c_int
        - *in_0.offset(5 as c_int as isize) as c_int) >> 1 as c_int)
        as int16_t;
    let s34: int16_t = ((*in_0.offset(3 as c_int as isize) as c_int
        + *in_0.offset(4 as c_int as isize) as c_int) >> 1 as c_int)
        as int16_t;
    let d34: int16_t = ((*in_0.offset(3 as c_int as isize) as c_int
        - *in_0.offset(4 as c_int as isize) as c_int) >> 1 as c_int)
        as int16_t;
    let mut a0: c_int = (s07 as c_int + s34 as c_int
        - (s25 as c_int + s16 as c_int))
        * cos16[4 as c_int as usize] as c_int;
    let mut b3: c_int = (-(s07 as c_int + s34 as c_int)
        - (s25 as c_int + s16 as c_int)) << 13 as c_int;
    let mut a2: c_int = (s07 as c_int - s34 as c_int)
        * cos16[6 as c_int as usize] as c_int
        + (s25 as c_int - s16 as c_int)
        * cos16[2 as c_int as usize] as c_int;
    let mut b1: c_int = (s34 as c_int - s07 as c_int)
        * cos16[2 as c_int as usize] as c_int
        + (s25 as c_int - s16 as c_int)
        * cos16[6 as c_int as usize] as c_int;
    let mut a1: c_int = d07 as c_int
        * cos16[5 as c_int as usize] as c_int
        - d16 as c_int * cos16[1 as c_int as usize] as c_int
        + d25 as c_int * cos16[7 as c_int as usize] as c_int
        + d34 as c_int * cos16[3 as c_int as usize] as c_int;
    let mut b2: c_int = -(d07 as c_int)
        * cos16[1 as c_int as usize] as c_int
        - d16 as c_int * cos16[3 as c_int as usize] as c_int
        - d25 as c_int * cos16[5 as c_int as usize] as c_int
        - d34 as c_int * cos16[7 as c_int as usize] as c_int;
    let mut a3: c_int = d07 as c_int
        * cos16[7 as c_int as usize] as c_int
        - d16 as c_int * cos16[5 as c_int as usize] as c_int
        + d25 as c_int * cos16[3 as c_int as usize] as c_int
        - d34 as c_int * cos16[1 as c_int as usize] as c_int;
    let mut b0: c_int = -(d07 as c_int)
        * cos16[3 as c_int as usize] as c_int
        + d16 as c_int * cos16[7 as c_int as usize] as c_int
        + d25 as c_int * cos16[1 as c_int as usize] as c_int
        + d34 as c_int * cos16[5 as c_int as usize] as c_int;
    let shr: c_int = 12 as c_int + scale;
    a0 = (a0 + ((1 as c_int) << (shr - 1 as c_int))) >> shr;
    b0 = (b0 + ((1 as c_int) << (shr - 1 as c_int))) >> shr;
    a1 = (a1 + ((1 as c_int) << (shr - 1 as c_int))) >> shr;
    b1 = (b1 + ((1 as c_int) << (shr - 1 as c_int))) >> shr;
    a2 = (a2 + ((1 as c_int) << (shr - 1 as c_int))) >> shr;
    b2 = (b2 + ((1 as c_int) << (shr - 1 as c_int))) >> shr;
    a3 = (a3 + ((1 as c_int) << (shr - 1 as c_int))) >> shr;
    b3 = (b3 + ((1 as c_int) << (shr - 1 as c_int))) >> shr;
    (*out0
        .offset(
            0 as c_int as isize,
        ))[idx
        as usize] = (if a0 > 32767 as c_int {
        32767 as c_int
    } else if a0 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        a0
    }) as int16_t;
    (*out0
        .offset(
            7 as c_int as isize,
        ))[idx
        as usize] = (if -a1 > 32767 as c_int {
        32767 as c_int
    } else if -a1 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        -a1
    }) as int16_t;
    (*out0
        .offset(
            1 as c_int as isize,
        ))[idx
        as usize] = (if a1 > 32767 as c_int {
        32767 as c_int
    } else if a1 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        a1
    }) as int16_t;
    (*out0
        .offset(
            6 as c_int as isize,
        ))[idx
        as usize] = (if -a2 > 32767 as c_int {
        32767 as c_int
    } else if -a2 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        -a2
    }) as int16_t;
    (*out0
        .offset(
            2 as c_int as isize,
        ))[idx
        as usize] = (if a2 > 32767 as c_int {
        32767 as c_int
    } else if a2 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        a2
    }) as int16_t;
    (*out0
        .offset(
            5 as c_int as isize,
        ))[idx
        as usize] = (if -a3 > 32767 as c_int {
        32767 as c_int
    } else if -a3 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        -a3
    }) as int16_t;
    (*out0
        .offset(
            3 as c_int as isize,
        ))[idx
        as usize] = (if a3 > 32767 as c_int {
        32767 as c_int
    } else if a3 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        a3
    }) as int16_t;
    (*out0
        .offset(
            4 as c_int as isize,
        ))[idx
        as usize] = (if 0 as c_int > 32767 as c_int {
        32767 as c_int
    } else if (0 as c_int) < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        0 as c_int
    }) as int16_t;
    (*out1
        .offset(
            0 as c_int as isize,
        ))[idx
        as usize] = (if -a0 > 32767 as c_int {
        32767 as c_int
    } else if -a0 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        -a0
    }) as int16_t;
    (*out1
        .offset(
            7 as c_int as isize,
        ))[idx
        as usize] = (if b0 > 32767 as c_int {
        32767 as c_int
    } else if b0 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        b0
    }) as int16_t;
    (*out1
        .offset(
            1 as c_int as isize,
        ))[idx
        as usize] = (if b0 > 32767 as c_int {
        32767 as c_int
    } else if b0 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        b0
    }) as int16_t;
    (*out1
        .offset(
            6 as c_int as isize,
        ))[idx
        as usize] = (if b1 > 32767 as c_int {
        32767 as c_int
    } else if b1 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        b1
    }) as int16_t;
    (*out1
        .offset(
            2 as c_int as isize,
        ))[idx
        as usize] = (if b1 > 32767 as c_int {
        32767 as c_int
    } else if b1 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        b1
    }) as int16_t;
    (*out1
        .offset(
            5 as c_int as isize,
        ))[idx
        as usize] = (if b2 > 32767 as c_int {
        32767 as c_int
    } else if b2 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        b2
    }) as int16_t;
    (*out1
        .offset(
            3 as c_int as isize,
        ))[idx
        as usize] = (if b2 > 32767 as c_int {
        32767 as c_int
    } else if b2 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        b2
    }) as int16_t;
    (*out1
        .offset(
            4 as c_int as isize,
        ))[idx
        as usize] = (if b3 > 32767 as c_int {
        32767 as c_int
    } else if b3 < -(32767 as c_int) - 1 as c_int {
        -(32767 as c_int) - 1 as c_int
    } else {
        b3
    }) as int16_t;
}
#[inline]
unsafe fn apply_window(
    in_0: *const [int16_t; 10],
    n: c_int,
    window: *const [int16_t; 20],
    offset: c_int,
    mut out: *mut int16_t,
    pitch: c_int,
) {
    let mut u: *const int16_t = in_0 as *const int16_t;
    let mut i: c_int = 0 as c_int;
    while i < n {
        let mut w: *const int16_t = (*window.offset(i as isize))
            .as_ptr()
            .offset(offset as isize);
        let mut s: c_int;
        let fresh4 = u;
        u = u.offset(1);
        let fresh5 = w;
        w = w.offset(1);
        s = *fresh4 as c_int * *fresh5 as c_int;
        let fresh6 = u;
        u = u.offset(1);
        let fresh7 = w;
        w = w.offset(1);
        s += *fresh6 as c_int * *fresh7 as c_int;
        let fresh8 = u;
        u = u.offset(1);
        let fresh9 = w;
        w = w.offset(1);
        s += *fresh8 as c_int * *fresh9 as c_int;
        let fresh10 = u;
        u = u.offset(1);
        let fresh11 = w;
        w = w.offset(1);
        s += *fresh10 as c_int * *fresh11 as c_int;
        let fresh12 = u;
        u = u.offset(1);
        let fresh13 = w;
        w = w.offset(1);
        s += *fresh12 as c_int * *fresh13 as c_int;
        let fresh14 = u;
        u = u.offset(1);
        let fresh15 = w;
        w = w.offset(1);
        s += *fresh14 as c_int * *fresh15 as c_int;
        let fresh16 = u;
        u = u.offset(1);
        let fresh17 = w;
        w = w.offset(1);
        s += *fresh16 as c_int * *fresh17 as c_int;
        let fresh18 = u;
        u = u.offset(1);
        let fresh19 = w;
        w = w.offset(1);
        s += *fresh18 as c_int * *fresh19 as c_int;
        let fresh20 = u;
        u = u.offset(1);
        let fresh21 = w;
        w = w.offset(1);
        s += *fresh20 as c_int * *fresh21 as c_int;
        let fresh22 = u;
        u = u.offset(1);
        let fresh23 = w;
        s += *fresh22 as c_int * *fresh23 as c_int;
        *out = (if (s + ((1 as c_int) << 12 as c_int)) >> 13 as c_int
            > 32767 as c_int
        {
            32767 as c_int
        } else if ((s + ((1 as c_int) << 12 as c_int)) >> 13 as c_int)
            < -(32767 as c_int) - 1 as c_int
        {
            -(32767 as c_int) - 1 as c_int
        } else {
            (s + ((1 as c_int) << 12 as c_int)) >> 13 as c_int
        }) as int16_t;
        out = out.offset(pitch as isize);
        i += 1;
    }
}
unsafe fn sbc_synthesize_4_c(
    state: *mut sbc_dstate,
    in_0: *const int16_t,
    scale: c_int,
    out: *mut int16_t,
    pitch: c_int,
) {
    static mut window: [[int16_t; 20]; 4] = [
        [
            0 as c_int as int16_t,
            -(126 as c_int) as int16_t,
            -(358 as c_int) as int16_t,
            -(848 as c_int) as int16_t,
            -(4443 as c_int) as int16_t,
            -(9644 as c_int) as int16_t,
            4443 as c_int as int16_t,
            -(848 as c_int) as int16_t,
            358 as c_int as int16_t,
            -(126 as c_int) as int16_t,
            0 as c_int as int16_t,
            -(126 as c_int) as int16_t,
            -(358 as c_int) as int16_t,
            -(848 as c_int) as int16_t,
            -(4443 as c_int) as int16_t,
            -(9644 as c_int) as int16_t,
            4443 as c_int as int16_t,
            -(848 as c_int) as int16_t,
            358 as c_int as int16_t,
            -(126 as c_int) as int16_t,
        ],
        [
            -(18 as c_int) as int16_t,
            -(128 as c_int) as int16_t,
            -(670 as c_int) as int16_t,
            -(201 as c_int) as int16_t,
            -(6389 as c_int) as int16_t,
            -(9235 as c_int) as int16_t,
            2544 as c_int as int16_t,
            -(1055 as c_int) as int16_t,
            100 as c_int as int16_t,
            -(90 as c_int) as int16_t,
            -(18 as c_int) as int16_t,
            -(128 as c_int) as int16_t,
            -(670 as c_int) as int16_t,
            -(201 as c_int) as int16_t,
            -(6389 as c_int) as int16_t,
            -(9235 as c_int) as int16_t,
            2544 as c_int as int16_t,
            -(1055 as c_int) as int16_t,
            100 as c_int as int16_t,
            -(90 as c_int) as int16_t,
        ],
        [
            -(49 as c_int) as int16_t,
            -(61 as c_int) as int16_t,
            -(946 as c_int) as int16_t,
            944 as c_int as int16_t,
            -(8082 as c_int) as int16_t,
            -(8082 as c_int) as int16_t,
            944 as c_int as int16_t,
            -(946 as c_int) as int16_t,
            -(61 as c_int) as int16_t,
            -(49 as c_int) as int16_t,
            -(49 as c_int) as int16_t,
            -(61 as c_int) as int16_t,
            -(946 as c_int) as int16_t,
            944 as c_int as int16_t,
            -(8082 as c_int) as int16_t,
            -(8082 as c_int) as int16_t,
            944 as c_int as int16_t,
            -(946 as c_int) as int16_t,
            -(61 as c_int) as int16_t,
            -(49 as c_int) as int16_t,
        ],
        [
            -(90 as c_int) as int16_t,
            100 as c_int as int16_t,
            -(1055 as c_int) as int16_t,
            2544 as c_int as int16_t,
            -(9235 as c_int) as int16_t,
            -(6389 as c_int) as int16_t,
            -(201 as c_int) as int16_t,
            -(670 as c_int) as int16_t,
            -(128 as c_int) as int16_t,
            -(18 as c_int) as int16_t,
            -(90 as c_int) as int16_t,
            100 as c_int as int16_t,
            -(1055 as c_int) as int16_t,
            2544 as c_int as int16_t,
            -(9235 as c_int) as int16_t,
            -(6389 as c_int) as int16_t,
            -(201 as c_int) as int16_t,
            -(670 as c_int) as int16_t,
            -(128 as c_int) as int16_t,
            -(18 as c_int) as int16_t,
        ],
    ];
    let dct_idx: c_int = if (*state).idx != 0 {
        10 as c_int - (*state).idx
    } else {
        0 as c_int
    };
    let odd: c_int = dct_idx & 1 as c_int;
    dct4(
        slice::from_raw_parts(in_0, 4),
        scale,
        &mut (*state).v[odd as usize],
        &mut (*state).v[(odd == 0) as usize],
        dct_idx as usize);
    apply_window(
        ((*state).v[odd as usize]).as_mut_ptr() as *const [int16_t; 10],
        4 as c_int,
        window.as_ptr(),
        (*state).idx,
        out,
        pitch,
    );
    (*state)
        .idx = if (*state).idx < 9 as c_int {
        (*state).idx + 1 as c_int
    } else {
        0 as c_int
    };
}
unsafe fn sbc_synthesize_8_c(
    state: *mut sbc_dstate,
    in_0: *const int16_t,
    scale: c_int,
    out: *mut int16_t,
    pitch: c_int,
) {
    static mut window: [[int16_t; 20]; 8] = [
        [
            0 as c_int as int16_t,
            -(132 as c_int) as int16_t,
            -(371 as c_int) as int16_t,
            -(848 as c_int) as int16_t,
            -(4456 as c_int) as int16_t,
            -(9631 as c_int) as int16_t,
            4456 as c_int as int16_t,
            -(848 as c_int) as int16_t,
            371 as c_int as int16_t,
            -(132 as c_int) as int16_t,
            0 as c_int as int16_t,
            -(132 as c_int) as int16_t,
            -(371 as c_int) as int16_t,
            -(848 as c_int) as int16_t,
            -(4456 as c_int) as int16_t,
            -(9631 as c_int) as int16_t,
            4456 as c_int as int16_t,
            -(848 as c_int) as int16_t,
            371 as c_int as int16_t,
            -(132 as c_int) as int16_t,
        ],
        [
            -(10 as c_int) as int16_t,
            -(138 as c_int) as int16_t,
            -(526 as c_int) as int16_t,
            -(580 as c_int) as int16_t,
            -(5438 as c_int) as int16_t,
            -(9528 as c_int) as int16_t,
            3486 as c_int as int16_t,
            -(1004 as c_int) as int16_t,
            229 as c_int as int16_t,
            -(117 as c_int) as int16_t,
            -(10 as c_int) as int16_t,
            -(138 as c_int) as int16_t,
            -(526 as c_int) as int16_t,
            -(580 as c_int) as int16_t,
            -(5438 as c_int) as int16_t,
            -(9528 as c_int) as int16_t,
            3486 as c_int as int16_t,
            -(1004 as c_int) as int16_t,
            229 as c_int as int16_t,
            -(117 as c_int) as int16_t,
        ],
        [
            -(22 as c_int) as int16_t,
            -(131 as c_int) as int16_t,
            -(685 as c_int) as int16_t,
            -(192 as c_int) as int16_t,
            -(6395 as c_int) as int16_t,
            -(9224 as c_int) as int16_t,
            2561 as c_int as int16_t,
            -(1063 as c_int) as int16_t,
            108 as c_int as int16_t,
            -(97 as c_int) as int16_t,
            -(22 as c_int) as int16_t,
            -(131 as c_int) as int16_t,
            -(685 as c_int) as int16_t,
            -(192 as c_int) as int16_t,
            -(6395 as c_int) as int16_t,
            -(9224 as c_int) as int16_t,
            2561 as c_int as int16_t,
            -(1063 as c_int) as int16_t,
            108 as c_int as int16_t,
            -(97 as c_int) as int16_t,
        ],
        [
            -(36 as c_int) as int16_t,
            -(106 as c_int) as int16_t,
            -(835 as c_int) as int16_t,
            322 as c_int as int16_t,
            -(7287 as c_int) as int16_t,
            -(8734 as c_int) as int16_t,
            1711 as c_int as int16_t,
            -(1042 as c_int) as int16_t,
            12 as c_int as int16_t,
            -(75 as c_int) as int16_t,
            -(36 as c_int) as int16_t,
            -(106 as c_int) as int16_t,
            -(835 as c_int) as int16_t,
            322 as c_int as int16_t,
            -(7287 as c_int) as int16_t,
            -(8734 as c_int) as int16_t,
            1711 as c_int as int16_t,
            -(1042 as c_int) as int16_t,
            12 as c_int as int16_t,
            -(75 as c_int) as int16_t,
        ],
        [
            -(54 as c_int) as int16_t,
            -(59 as c_int) as int16_t,
            -(960 as c_int) as int16_t,
            959 as c_int as int16_t,
            -(8078 as c_int) as int16_t,
            -(8078 as c_int) as int16_t,
            959 as c_int as int16_t,
            -(960 as c_int) as int16_t,
            -(59 as c_int) as int16_t,
            -(54 as c_int) as int16_t,
            -(54 as c_int) as int16_t,
            -(59 as c_int) as int16_t,
            -(960 as c_int) as int16_t,
            959 as c_int as int16_t,
            -(8078 as c_int) as int16_t,
            -(8078 as c_int) as int16_t,
            959 as c_int as int16_t,
            -(960 as c_int) as int16_t,
            -(59 as c_int) as int16_t,
            -(54 as c_int) as int16_t,
        ],
        [
            -(75 as c_int) as int16_t,
            12 as c_int as int16_t,
            -(1042 as c_int) as int16_t,
            1711 as c_int as int16_t,
            -(8734 as c_int) as int16_t,
            -(7287 as c_int) as int16_t,
            322 as c_int as int16_t,
            -(835 as c_int) as int16_t,
            -(106 as c_int) as int16_t,
            -(36 as c_int) as int16_t,
            -(75 as c_int) as int16_t,
            12 as c_int as int16_t,
            -(1042 as c_int) as int16_t,
            1711 as c_int as int16_t,
            -(8734 as c_int) as int16_t,
            -(7287 as c_int) as int16_t,
            322 as c_int as int16_t,
            -(835 as c_int) as int16_t,
            -(106 as c_int) as int16_t,
            -(36 as c_int) as int16_t,
        ],
        [
            -(97 as c_int) as int16_t,
            108 as c_int as int16_t,
            -(1063 as c_int) as int16_t,
            2561 as c_int as int16_t,
            -(9224 as c_int) as int16_t,
            -(6395 as c_int) as int16_t,
            -(192 as c_int) as int16_t,
            -(685 as c_int) as int16_t,
            -(131 as c_int) as int16_t,
            -(22 as c_int) as int16_t,
            -(97 as c_int) as int16_t,
            108 as c_int as int16_t,
            -(1063 as c_int) as int16_t,
            2561 as c_int as int16_t,
            -(9224 as c_int) as int16_t,
            -(6395 as c_int) as int16_t,
            -(192 as c_int) as int16_t,
            -(685 as c_int) as int16_t,
            -(131 as c_int) as int16_t,
            -(22 as c_int) as int16_t,
        ],
        [
            -(117 as c_int) as int16_t,
            229 as c_int as int16_t,
            -(1004 as c_int) as int16_t,
            3486 as c_int as int16_t,
            -(9528 as c_int) as int16_t,
            -(5438 as c_int) as int16_t,
            -(580 as c_int) as int16_t,
            -(526 as c_int) as int16_t,
            -(138 as c_int) as int16_t,
            -(10 as c_int) as int16_t,
            -(117 as c_int) as int16_t,
            229 as c_int as int16_t,
            -(1004 as c_int) as int16_t,
            3486 as c_int as int16_t,
            -(9528 as c_int) as int16_t,
            -(5438 as c_int) as int16_t,
            -(580 as c_int) as int16_t,
            -(526 as c_int) as int16_t,
            -(138 as c_int) as int16_t,
            -(10 as c_int) as int16_t,
        ],
    ];
    let dct_idx: c_int = if (*state).idx != 0 {
        10 as c_int - (*state).idx
    } else {
        0 as c_int
    };
    let odd: c_int = dct_idx & 1 as c_int;
    dct8(
        in_0,
        scale,
        ((*state).v[odd as usize]).as_mut_ptr(),
        ((*state).v[(odd == 0) as c_int as usize]).as_mut_ptr(),
        dct_idx,
    );
    apply_window(
        ((*state).v[odd as usize]).as_mut_ptr() as *const [int16_t; 10],
        8 as c_int,
        window.as_ptr(),
        (*state).idx,
        out,
        pitch,
    );
    (*state)
        .idx = if (*state).idx < 9 as c_int {
        (*state).idx + 1 as c_int
    } else {
        0 as c_int
    };
}
pub unsafe fn synthesize(
    state: *mut sbc_dstate,
    nblocks: c_int,
    nsubbands: c_int,
    mut in_0: *const int16_t,
    scale: c_int,
    mut out: *mut int16_t,
    pitch: c_int,
) {
    let mut iblk: c_int = 0 as c_int;
    while iblk < nblocks {
        if nsubbands == 4 as c_int {
            sbc_synthesize_4_c(state, in_0, scale, out, pitch);
        } else {
            sbc_synthesize_8_c(state, in_0, scale, out, pitch);
        }
        in_0 = in_0.offset(nsubbands as isize);
        out = out.offset((nsubbands * pitch) as isize);
        iblk += 1;
    }
}