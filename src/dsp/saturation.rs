/// Saturador analógico modelado y limitador de protección transparente
pub struct SaturationUnit;

impl SaturationUnit {
    pub fn new() -> Self {
        Self
    }

    pub fn reset(&mut self) {}

    /// Aplica saturación cálida de válvulas y compresión armónica
    /// `drive`: 1.0 (limpio) a 10.0 (distorsión masiva)
    /// `mix`: nivel del procesado
    #[inline]
    pub fn process_stereo(
        &mut self,
        in_l: f32,
        in_r: f32,
        drive: f32,
        mix: f32,
    ) -> (f32, f32) {
        if mix <= 0.0001 || drive <= 1.001 {
            return (in_l, in_r);
        }

        let d = drive.max(1.0);
        // Saturación asimétrica tipo válvula de previo de tubo
        let sat_l = Self::tube_shape(in_l * d) / d.sqrt();
        let sat_r = Self::tube_shape(in_r * d) / d.sqrt();

        let out_l = in_l * (1.0 - mix) + sat_l * mix;
        let out_r = in_r * (1.0 - mix) + sat_r * mix;

        (out_l, out_r)
    }

    #[inline]
    fn tube_shape(x: f32) -> f32 {
        if x > 0.0 {
            // Curva suave en semiciclos positivos
            1.0 - (-x).exp()
        } else {
            // Curva tanh en semiciclos negativos para generar armónicos pares e impares
            x.tanh()
        }
    }

    /// Soft-clipper transparente para evitar picos digitales superiores a 0 dBFS
    #[inline]
    pub fn soft_clip(x: f32) -> f32 {
        if x > 1.0 {
            1.0 - (-(x - 1.0)).exp() * 0.08
        } else if x < -1.0 {
            -1.0 + (-(-x - 1.0)).exp() * 0.08
        } else {
            x
        }
    }
}
