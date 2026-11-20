use std::f32::consts::PI;

/// Reverb estéreo espacial de alta densidad y modulación para estelas infinitas y washouts EDM
pub struct StereoReverb {
    comb_l: [CombFilter; 8],
    comb_r: [CombFilter; 8],
    allpass_l: [AllpassFilter; 4],
    allpass_r: [AllpassFilter; 4],
    lfo_phase: f32,
    sample_rate: f32,
}

impl Default for StereoReverb {
    fn default() -> Self {
        Self::new(44100.0)
    }
}

impl StereoReverb {
    pub fn new(sample_rate: f32) -> Self {
        let sr_scale = sample_rate.max(1000.0) / 44100.0;

        let comb_tunings = [1116, 1188, 1277, 1356, 1422, 1491, 1557, 1617];
        let allpass_tunings = [556, 441, 341, 225];
        let stereo_spread = 29;

        let comb_l = std::array::from_fn(|i| {
            let size = ((comb_tunings[i] as f32) * sr_scale) as usize;
            CombFilter::new(size)
        });
        let comb_r = std::array::from_fn(|i| {
            let size = (((comb_tunings[i] + stereo_spread) as f32) * sr_scale) as usize;
            CombFilter::new(size)
        });

        let allpass_l = std::array::from_fn(|i| {
            let size = ((allpass_tunings[i] as f32) * sr_scale) as usize;
            AllpassFilter::new(size)
        });
        let allpass_r = std::array::from_fn(|i| {
            let size = (((allpass_tunings[i] + stereo_spread) as f32) * sr_scale) as usize;
            AllpassFilter::new(size)
        });

        Self {
            comb_l,
            comb_r,
            allpass_l,
            allpass_r,
            lfo_phase: 0.0,
            sample_rate: sample_rate.max(1000.0),
        }
    }

    pub fn reset(&mut self) {
        for c in &mut self.comb_l { c.reset(); }
        for c in &mut self.comb_r { c.reset(); }
        for a in &mut self.allpass_l { a.reset(); }
        for a in &mut self.allpass_r { a.reset(); }
        self.lfo_phase = 0.0;
    }

    /// Procesa una muestra estéreo a través del motor de reverb
    /// `decay`: 0.0 a 1.0 (en 1.0 la cola dura más de 12 segundos)
    /// `damp`: absorción de agudos
    /// `wet`: nivel de señal procesada
    #[inline]
    pub fn process_stereo(
        &mut self,
        in_l: f32,
        in_r: f32,
        decay: f32,
        damp: f32,
        wet: f32,
    ) -> (f32, f32) {
        if wet <= 0.0001 {
            return (in_l, in_r);
        }

        // LFO sutil a 0.6 Hz para modular la difusión y evitar resonancias fijas
        self.lfo_phase += 0.6 / self.sample_rate;
        if self.lfo_phase > 1.0 { self.lfo_phase -= 1.0; }
        let lfo_mod = (self.lfo_phase * 2.0 * PI).sin() * 0.008;

        let feedback = (0.75 + decay * 0.23 + lfo_mod).clamp(0.0, 0.985);
        let damp_factor = damp.clamp(0.0, 0.90);
        let mono_in = (in_l + in_r) * 0.5 * 0.018;

        let mut out_l = 0.0;
        for c in &mut self.comb_l {
            out_l += c.process(mono_in, feedback, damp_factor);
        }

        let mut out_r = 0.0;
        for c in &mut self.comb_r {
            out_r += c.process(mono_in, feedback, damp_factor);
        }

        for a in &mut self.allpass_l {
            out_l = a.process(out_l);
        }
        for a in &mut self.allpass_r {
            out_r = a.process(out_r);
        }

        // Imagen estéreo expansiva
        let wet_l = out_l * 1.15 - out_r * 0.15;
        let wet_r = out_r * 1.15 - out_l * 0.15;

        (in_l + wet_l * wet, in_r + wet_r * wet)
    }
}

struct CombFilter {
    buffer: Vec<f32>,
    index: usize,
    filter_store: f32,
}

impl CombFilter {
    fn new(size: usize) -> Self {
        Self {
            buffer: vec![0.0; size.max(1)],
            index: 0,
            filter_store: 0.0,
        }
    }

    fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.index = 0;
        self.filter_store = 0.0;
    }

    #[inline]
    fn process(&mut self, input: f32, feedback: f32, damp: f32) -> f32 {
        let output = self.buffer[self.index];
        self.filter_store = output * (1.0 - damp) + self.filter_store * damp;
        self.buffer[self.index] = input + self.filter_store * feedback;
        self.index += 1;
        if self.index >= self.buffer.len() {
            self.index = 0;
        }
        output
    }
}

struct AllpassFilter {
    buffer: Vec<f32>,
    index: usize,
}

impl AllpassFilter {
    fn new(size: usize) -> Self {
        Self {
            buffer: vec![0.0; size.max(1)],
            index: 0,
        }
    }

    fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.index = 0;
    }

    #[inline]
    fn process(&mut self, input: f32) -> f32 {
        let buf_out = self.buffer[self.index];
        let output = -input + buf_out;
        self.buffer[self.index] = input + buf_out * 0.55;
        self.index += 1;
        if self.index >= self.buffer.len() {
            self.index = 0;
        }
        output
    }
}
