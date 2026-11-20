/// Ping-Pong Delay estéreo de estudio con filtros pasa-altos y pasa-bajos en el lazo y ducking suave
pub struct StereoDelay {
    buffer_l: Vec<f32>,
    buffer_r: Vec<f32>,
    write_pos: usize,
    sample_rate: f32,
    lp_state_l: f32,
    lp_state_r: f32,
    hp_state_l: f32,
    hp_state_r: f32,
}

impl Default for StereoDelay {
    fn default() -> Self {
        Self::new(44100.0)
    }
}

impl StereoDelay {
    pub fn new(sample_rate: f32) -> Self {
        let max_samples = (sample_rate.max(1000.0) * 3.0) as usize;
        Self {
            buffer_l: vec![0.0; max_samples],
            buffer_r: vec![0.0; max_samples],
            write_pos: 0,
            sample_rate: sample_rate.max(1000.0),
            lp_state_l: 0.0,
            lp_state_r: 0.0,
            hp_state_l: 0.0,
            hp_state_r: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.buffer_l.fill(0.0);
        self.buffer_r.fill(0.0);
        self.write_pos = 0;
        self.lp_state_l = 0.0;
        self.lp_state_r = 0.0;
        self.hp_state_l = 0.0;
        self.hp_state_r = 0.0;
    }

    /// Procesa una muestra estéreo
    #[inline]
    pub fn process_stereo(
        &mut self,
        in_l: f32,
        in_r: f32,
        time_sec_l: f32,
        time_sec_r: f32,
        feedback: f32,
        wet: f32,
    ) -> (f32, f32) {
        if wet <= 0.0001 {
            return (in_l, in_r);
        }

        let buf_len = self.buffer_l.len();
        let delay_s_l = ((time_sec_l * self.sample_rate) as usize).clamp(1, buf_len - 1);
        let delay_s_r = ((time_sec_r * self.sample_rate) as usize).clamp(1, buf_len - 1);

        let read_l = (self.write_pos + buf_len - delay_s_l) % buf_len;
        let read_r = (self.write_pos + buf_len - delay_s_r) % buf_len;

        let tap_l = self.buffer_l[read_l];
        let tap_r = self.buffer_r[read_r];

        // Filtro pasa-altos a 200 Hz para evitar acumulación de graves
        self.hp_state_l = self.hp_state_l * 0.96 + tap_l * 0.04;
        self.hp_state_r = self.hp_state_r * 0.96 + tap_r * 0.04;
        let hp_tap_l = tap_l - self.hp_state_l;
        let hp_tap_r = tap_r - self.hp_state_r;

        // Filtro pasa-bajos a 5 kHz para calidez analógica
        self.lp_state_l = self.lp_state_l * 0.35 + hp_tap_l * 0.65;
        self.lp_state_r = self.lp_state_r * 0.35 + hp_tap_r * 0.65;

        let fb = feedback.clamp(0.0, 0.94);
        // Ping-pong cruzado: canal L alimenta a R y viceversa con saturación suave
        let next_l = (in_l + self.lp_state_r * fb).tanh();
        let next_r = (in_r + self.lp_state_l * fb).tanh();

        self.buffer_l[self.write_pos] = next_l;
        self.buffer_r[self.write_pos] = next_r;

        self.write_pos = (self.write_pos + 1) % buf_len;

        (in_l + tap_l * wet, in_r + tap_r * wet)
    }
}
