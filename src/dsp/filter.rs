use std::f32::consts::PI;

/// Filtro State Variable Filter (SVF) estéreo no-lineal con modelado analógico y saturación en el lazo resonante
#[derive(Debug, Clone)]
pub struct SvfFilter {
    s1_l: f32,
    s2_l: f32,
    s1_r: f32,
    s2_r: f32,
    sample_rate: f32,
}

impl Default for SvfFilter {
    fn default() -> Self {
        Self::new(44100.0)
    }
}

impl SvfFilter {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            s1_l: 0.0,
            s2_l: 0.0,
            s1_r: 0.0,
            s2_r: 0.0,
            sample_rate: sample_rate.max(1000.0),
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate.max(1000.0);
        self.reset();
    }

    pub fn reset(&mut self) {
        self.s1_l = 0.0;
        self.s2_l = 0.0;
        self.s1_r = 0.0;
        self.s2_r = 0.0;
    }

    /// Procesa una muestra estéreo aplicando filtrado SVF con saturación analógica en resonancia
    /// `cutoff_hz`: frecuencia de corte
    /// `resonance_q`: factor Q (0.5 a 12.0)
    /// `bp_mix`: mezcla de la salida Band-Pass para dar el efecto de barrido silbante
    #[inline]
    pub fn process_stereo(
        &mut self,
        in_l: f32,
        in_r: f32,
        cutoff_hz: f32,
        resonance_q: f32,
        bp_mix: f32,
    ) -> (f32, f32) {
        let cutoff = cutoff_hz.clamp(20.0, self.sample_rate * 0.44);
        let q = resonance_q.clamp(0.5, 12.0);

        // Prewarp bilinear transform
        let g = (PI * cutoff / self.sample_rate).tan();
        let k = 1.0 / q;

        // Canal Izquierdo (con saturación interna para calidez analógica)
        let hp_l = (in_l - self.s1_l * (2.0 * g + k) - self.s2_l) / (1.0 + 2.0 * g * k + g * g);
        let bp_l = (g * hp_l + self.s1_l).tanh(); // Saturador analógico interno
        let lp_l = g * bp_l + self.s2_l;

        self.s1_l = g * hp_l + bp_l;
        self.s2_l = g * bp_l + lp_l;

        // Limpieza de denormales
        if self.s1_l.abs() < 1e-18 { self.s1_l = 0.0; }
        if self.s2_l.abs() < 1e-18 { self.s2_l = 0.0; }

        let out_l = hp_l * (1.0 - bp_mix) + bp_l * bp_mix;

        // Canal Derecho
        let hp_r = (in_r - self.s1_r * (2.0 * g + k) - self.s2_r) / (1.0 + 2.0 * g * k + g * g);
        let bp_r = (g * hp_r + self.s1_r).tanh();
        let lp_r = g * bp_r + self.s2_r;

        self.s1_r = g * hp_r + bp_r;
        self.s2_r = g * bp_r + lp_r;

        if self.s1_r.abs() < 1e-18 { self.s1_r = 0.0; }
        if self.s2_r.abs() < 1e-18 { self.s2_r = 0.0; }

        let out_r = hp_r * (1.0 - bp_mix) + bp_r * bp_mix;

        (out_l, out_r)
    }
}
