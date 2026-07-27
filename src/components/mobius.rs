//PDX short identifier: AGPL-3.0
//Copyright (C) 2026 SAP2B

use super::library::*;
use super::siren::*;
use super::wavelet::*;

pub struct MobiusParams {
    pub a_q16: i32,
    pub b_q16: i32,
    pub c_q16: i32,
    pub d_q16: i32,
    pub blend_q16: i32,
}

impl MobiusParams {
    pub const fn new(a: f64, b: f64, c: f64, d: f64, blend: f64) -> Self {
        Self {
            a_q16: (a * 65536.0) as i32,
            b_q16: (b * 65536.0) as i32,
            c_q16: (c * 65536.0) as i32,
            d_q16: (d * 65536.0) as i32,
            blend_q16: (blend * 65536.0) as i32,
        }
    }
}

pub struct Mobius {
    pub siren: Siren,
    pub wavelet: Wavelet,
    pub a_q16: i32,
    pub b_q16: i32,
    pub c_q16: i32,
    pub d_q16: i32,
    pub blend_q16: i32,
}

impl Mobius {
    pub const fn new(
        siren: Siren,
        wavelet: Wavelet,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        blend: f64,
    ) -> Self {
        let params = MobiusParams::new(a, b, c, d, blend);
        Self {
            siren,
            wavelet,
            a_q16: params.a_q16,
            b_q16: params.b_q16,
            c_q16: params.c_q16,
            d_q16: params.d_q16,
            blend_q16: params.blend_q16,
        }
    }

    #[inline(always)]
    pub fn apply_params(&mut self, params: &MobiusParams) {
        self.a_q16 = params.a_q16;
        self.b_q16 = params.b_q16;
        self.c_q16 = params.c_q16;
        self.d_q16 = params.d_q16;
        self.blend_q16 = params.blend_q16;
    }

    #[inline(always)]
    fn transform_q16(&self, x_q16: i32) -> i32 {
        let num = (((self.a_q16 as i64 * x_q16 as i64) >> 16) + self.b_q16 as i64) as i32;
        let den = (((self.c_q16 as i64 * x_q16 as i64) >> 16) + self.d_q16 as i64) as i32;

        if den == 0 {
            num
        } else {
            (((num as i64) << 16) / den as i64) as i32
        }
    }

    #[inline(always)]
    pub fn next_sample(&mut self) -> i16 {
        let s_sample = self.siren.next_sample() as i32;
        let w_sample = self.wavelet.next_sample() as i32;

        let s_warped = self.transform_q16(s_sample);
        let s_clamped = s_warped.clamp(-32768, 32767);

        let blended =
            ((s_clamped * (65536 - self.blend_q16)) >> 16) + ((w_sample * self.blend_q16) >> 16);

        blended.clamp(-32768, 32767) as i16
    }

    pub fn process(&mut self, buffer: &mut [i16]) {
        for sample in buffer.iter_mut() {
            *sample = self.next_sample();
        }
    }

    pub fn train_step(&mut self, lib: &mut Library<3, 256>, opcode: u8, fd: i32) -> bool {
        if !lib.push(opcode) {
            unsafe { lib.flush_sys(fd) };
            if !lib.push(opcode) {
                return false;
            }
        }

        if let Some(params) = lib.pop::<MobiusParams>() {
            self.apply_params(params);
        }

        true
    }
}
