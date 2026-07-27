//PDX short identifier: AGPL-3.0
//Copyright (C) 2026 SAP2B

use super::library::*;

pub const SINE_LUT: [i32; 256] = {
    let mut table = [0i32; 256];
    let mut i = 0;
    while i < 256 {
        let phase = (i as f32) / 256.0 * 6.283185307179586;
        let x = phase;
        let x3 = x * x * x;
        let x5 = x3 * x * x;
        let sine_f32 = x - (x3 / 6.0) + (x5 / 120.0);
        table[i] = (sine_f32 * 65536.0) as i32;
        i += 1;
    }
    table
};

pub struct SirenParams {
    pub min_inc: u32,
    pub max_inc: u32,
    pub lfo_inc: u32,
}

impl SirenParams {
    pub const fn new(min_hz: f64, max_hz: f64, lfo_hz: f64) -> Self {
        const SAMPLE_RATE: f64 = 44100.0;
        Self {
            min_inc: ((min_hz / SAMPLE_RATE) * 4294967296.0) as u32,
            max_inc: ((max_hz / SAMPLE_RATE) * 4294967296.0) as u32,
            lfo_inc: ((lfo_hz / SAMPLE_RATE) * 4294967296.0) as u32,
        }
    }
}

pub struct Siren {
    pub carrier_phase: u32,
    pub lfo_phase: u32,
    pub min_inc: u32,
    pub max_inc: u32,
    pub lfo_inc: u32,
}

impl Siren {
    pub const fn new(min_hz: f64, max_hz: f64, lfo_hz: f64) -> Self {
        let params = SirenParams::new(min_hz, max_hz, lfo_hz);
        Self {
            carrier_phase: 0,
            lfo_phase: 0,
            min_inc: params.min_inc,
            max_inc: params.max_inc,
            lfo_inc: params.lfo_inc,
        }
    }

    #[inline(always)]
    pub fn apply_params(&mut self, params: &SirenParams) {
        self.min_inc = params.min_inc;
        self.max_inc = params.max_inc;
        self.lfo_inc = params.lfo_inc;
    }

    #[inline(always)]
    pub fn next_sample(&mut self) -> i16 {
        self.lfo_phase = self.lfo_phase.wrapping_add(self.lfo_inc);
        let lfo_idx = (self.lfo_phase >> 24) as usize;
        let lfo_val = SINE_LUT[lfo_idx];

        let range = (self.max_inc - self.min_inc) as u64;
        let norm_lfo = ((lfo_val + 65536) >> 1) as u64;
        let current_inc = self.min_inc + (((range * norm_lfo) >> 16) as u32);

        self.carrier_phase = self.carrier_phase.wrapping_add(current_inc);
        let carrier_idx = (self.carrier_phase >> 24) as usize;
        let sample_q16 = SINE_LUT[carrier_idx];

        (sample_q16 >> 1) as i16
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

        if let Some(params) = lib.pop::<SirenParams>() {
            self.apply_params(params);
        }

        true
    }
}
