//PDX short identifier: AGPL-3.0
//Copyright (C) 2026 SAP2B

use super::library::*;

pub const MORLET_GAUSS_LUT: [i32; 256] = {
    let mut table = [0i32; 256];
    let mut i = 0;
    while i < 256 {
        let t = ((i as f32) - 128.0) / 32.0;
        let t2 = t * t;
        let exp_approx = 1.0 / (1.0 + t2 + (t2 * t2 * 0.5));
        table[i] = (exp_approx * 65536.0) as i32;
        i += 1;
    }
    table
};

pub const MORLET_SINE_LUT: [i32; 256] = {
    let mut table = [0i32; 256];
    let mut i = 0;
    while i < 256 {
        let phase = (i as f32) / 256.0 * 6.283185307179586;
        let x = phase;
        let x3 = x * x * x;
        let x5 = x3 * x * x;
        let sine = x - (x3 / 6.0) + (x5 / 120.0);
        table[i] = (sine * 65536.0) as i32;
        i += 1;
    }
    table
};

pub struct WaveletParams {
    pub scale_inc: u32,
    pub center_inc: u32,
    pub gain_q16: i32,
}

impl WaveletParams {
    pub const fn new(scale_hz: f64, center_hz: f64, gain: f64) -> Self {
        const SAMPLE_RATE: f64 = 44100.0;
        Self {
            scale_inc: ((scale_hz / SAMPLE_RATE) * 4294967296.0) as u32,
            center_inc: ((center_hz / SAMPLE_RATE) * 4294967296.0) as u32,
            gain_q16: (gain * 65536.0) as i32,
        }
    }
}

pub struct Wavelet {
    pub scale_phase: u32,
    pub center_phase: u32,
    pub scale_inc: u32,
    pub center_inc: u32,
    pub gain_q16: i32,
}

impl Wavelet {
    pub const fn new(scale_hz: f64, center_hz: f64, gain: f64) -> Self {
        let params = WaveletParams::new(scale_hz, center_hz, gain);
        Self {
            scale_phase: 0,
            center_phase: 0,
            scale_inc: params.scale_inc,
            center_inc: params.center_inc,
            gain_q16: params.gain_q16,
        }
    }

    #[inline(always)]
    pub fn apply_params(&mut self, params: &WaveletParams) {
        self.scale_inc = params.scale_inc;
        self.center_inc = params.center_inc;
        self.gain_q16 = params.gain_q16;
    }

    #[inline(always)]
    pub fn next_sample(&mut self) -> i16 {
        self.scale_phase = self.scale_phase.wrapping_add(self.scale_inc);
        self.center_phase = self.center_phase.wrapping_add(self.center_inc);

        let gauss_idx = (self.scale_phase >> 24) as usize;
        let sine_idx = (self.center_phase >> 24) as usize;

        let gauss_val = MORLET_GAUSS_LUT[gauss_idx];
        let sine_val = MORLET_SINE_LUT[sine_idx];

        let wave_q16 = ((gauss_val as i64 * sine_val as i64) >> 16) as i32;
        let output_q16 = ((wave_q16 as i64 * self.gain_q16 as i64) >> 16) as i32;

        (output_q16 >> 1) as i16
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

        if let Some(params) = lib.pop::<WaveletParams>() {
            self.apply_params(params);
        }

        true
    }
}
