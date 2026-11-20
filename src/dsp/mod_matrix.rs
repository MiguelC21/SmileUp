use nih_plug::prelude::Enum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
pub enum SmilePreset {
    #[name = "Face Melting Joy"]
    FaceMeltingJoy,
    #[name = "Smack in the Face"]
    SmackInTheFace,
    #[name = "Fist in the Air"]
    FistInTheAir,
    #[name = "Helium Jumping"]
    HeliumJumping,
    #[name = "Broken Teeth"]
    BrokenTeeth,
    #[name = "Dada Dynamite"]
    DadaDynamite,
}

#[derive(Debug, Clone)]
pub struct DSPTargetParams {
    pub cutoff_hz: f32,
    pub resonance_q: f32,
    pub bp_mix: f32,
    pub reverb_wet: f32,
    pub reverb_decay: f32,
    pub delay_wet: f32,
    pub delay_feedback: f32,
    pub sat_drive: f32,
    pub sat_mix: f32,
    pub riser_depth: f32,
    pub output_trim_db: f32,
}

impl SmilePreset {
    pub fn category_name(&self) -> &'static str {
        match self {
            SmilePreset::FistInTheAir | SmilePreset::HeliumJumping => "SOFT",
            SmilePreset::FaceMeltingJoy | SmilePreset::SmackInTheFace => "MEDIUM",
            SmilePreset::BrokenTeeth => "HARD",
            SmilePreset::DadaDynamite => "EXTREME",
        }
    }

    pub fn compute_dsp_targets(&self, intensity: f32) -> DSPTargetParams {
        let s = intensity.clamp(0.0, 1.10);
        let overdrive = if s > 1.0 { (s - 1.0) * 10.0 } else { 0.0 };

        match self {
            // 1. Face Melting Joy (El preset insignia de Endless Smile)
            SmilePreset::FaceMeltingJoy => {
                let cutoff_hz = 20.0 + (s.powf(1.8)) * 4200.0 + overdrive * 1600.0;
                let resonance_q = 0.75 + s * 2.4 + overdrive * 1.5;
                let bp_mix = s * 0.35;

                let reverb_wet = (s.powf(1.3) * 0.80 + overdrive * 0.18).clamp(0.0, 0.96);
                let reverb_decay = (0.35 + s * 0.58).clamp(0.0, 0.96);

                let delay_wet = (s.powf(1.6) * 0.65 + overdrive * 0.20).clamp(0.0, 0.82);
                let delay_feedback = (0.28 + s * 0.55).clamp(0.0, 0.86);

                let sat_drive = 1.0 + s * 2.4 + overdrive * 2.5;
                let sat_mix = s * 0.50;

                let riser_depth = s * 0.25;
                let output_trim_db = s * 2.5 + overdrive * 2.5;

                DSPTargetParams {
                    cutoff_hz,
                    resonance_q,
                    bp_mix,
                    reverb_wet,
                    reverb_decay,
                    delay_wet,
                    delay_feedback,
                    sat_drive,
                    sat_mix,
                    riser_depth,
                    output_trim_db,
                }
            }

            // 2. Smack in the Face (Agresivo con Barber-pole Continual Riser)
            SmilePreset::SmackInTheFace => {
                let cutoff_hz = 20.0 + (s.powf(1.6)) * 4800.0 + overdrive * 2000.0;
                let resonance_q = 0.80 + s * 3.6 + overdrive * 2.0;
                let bp_mix = s * 0.50;

                let reverb_wet = (s.powf(1.2) * 0.75 + overdrive * 0.22).clamp(0.0, 0.95);
                let reverb_decay = (0.40 + s * 0.55).clamp(0.0, 0.95);

                let delay_wet = (s.powf(1.8) * 0.45).clamp(0.0, 0.60);
                let delay_feedback = (0.20 + s * 0.45).clamp(0.0, 0.75);

                let sat_drive = 1.0 + s * 3.8 + overdrive * 3.2;
                let sat_mix = s * 0.65;

                let riser_depth = (s * 0.80 + overdrive * 0.20).clamp(0.0, 1.0);
                let output_trim_db = s * 3.2 + overdrive * 3.0;

                DSPTargetParams {
                    cutoff_hz,
                    resonance_q,
                    bp_mix,
                    reverb_wet,
                    reverb_decay,
                    delay_wet,
                    delay_feedback,
                    sat_drive,
                    sat_mix,
                    riser_depth,
                    output_trim_db,
                }
            }

            // 3. Fist in the Air (Limpio y suave hasta 240Hz)
            SmilePreset::FistInTheAir => {
                let cutoff_hz = 20.0 + (s.powf(2.0)) * 240.0 + overdrive * 120.0;
                let resonance_q = 0.707 + s * 0.85;
                let bp_mix = 0.0;

                let reverb_wet = (s.powf(1.1) * 0.60 + overdrive * 0.20).clamp(0.0, 0.88);
                let reverb_decay = (0.25 + s * 0.52).clamp(0.0, 0.88);

                let delay_wet = 0.0;
                let delay_feedback = 0.0;

                let sat_drive = 1.0 + s * 0.9 + overdrive * 1.5;
                let sat_mix = s * 0.25;

                let riser_depth = 0.0;
                let output_trim_db = s * 1.2 + overdrive * 1.5;

                DSPTargetParams {
                    cutoff_hz,
                    resonance_q,
                    bp_mix,
                    reverb_wet,
                    reverb_decay,
                    delay_wet,
                    delay_feedback,
                    sat_drive,
                    sat_mix,
                    riser_depth,
                    output_trim_db,
                }
            }

            // 4. Helium Jumping (Rítmico con ping-pong delay dominante)
            SmilePreset::HeliumJumping => {
                let cutoff_hz = 20.0 + (s.powf(1.9)) * 280.0 + overdrive * 140.0;
                let resonance_q = 0.72 + s * 1.2;
                let bp_mix = s * 0.12;

                let reverb_wet = (s.powf(1.2) * 0.65 + overdrive * 0.20).clamp(0.0, 0.88);
                let reverb_decay = (0.30 + s * 0.55).clamp(0.0, 0.90);

                let delay_wet = (s.powf(1.3) * 0.65 + overdrive * 0.22).clamp(0.0, 0.82);
                let delay_feedback = (0.30 + s * 0.52).clamp(0.0, 0.85);

                let sat_drive = 1.0 + s * 1.4 + overdrive * 2.0;
                let sat_mix = s * 0.38;

                let riser_depth = 0.0;
                let output_trim_db = s * 1.8 + overdrive * 2.0;

                DSPTargetParams {
                    cutoff_hz,
                    resonance_q,
                    bp_mix,
                    reverb_wet,
                    reverb_decay,
                    delay_wet,
                    delay_feedback,
                    sat_drive,
                    sat_mix,
                    riser_depth,
                    output_trim_db,
                }
            }

            // 5. Broken Teeth (Distorsión caliente y saturación agresiva)
            SmilePreset::BrokenTeeth => {
                let cutoff_hz = 20.0 + (s.powf(1.7)) * 5400.0 + overdrive * 2200.0;
                let resonance_q = 0.85 + s * 4.4 + overdrive * 2.5;
                let bp_mix = s * 0.55;

                let reverb_wet = (s.powf(1.3) * 0.82 + overdrive * 0.16).clamp(0.0, 0.96);
                let reverb_decay = (0.42 + s * 0.54).clamp(0.0, 0.98);

                let delay_wet = (s.powf(1.5) * 0.68 + overdrive * 0.22).clamp(0.0, 0.88);
                let delay_feedback = (0.32 + s * 0.60).clamp(0.0, 0.92);

                let sat_drive = 1.0 + s * 5.5 + overdrive * 4.5;
                let sat_mix = s * 0.80;

                let riser_depth = (s * 0.88 + overdrive * 0.12).clamp(0.0, 1.0);
                let output_trim_db = s * 4.0 + overdrive * 3.5;

                DSPTargetParams {
                    cutoff_hz,
                    resonance_q,
                    bp_mix,
                    reverb_wet,
                    reverb_decay,
                    delay_wet,
                    delay_feedback,
                    sat_drive,
                    sat_mix,
                    riser_depth,
                    output_trim_db,
                }
            }

            // 6. Dada Dynamite (Extremo clímax y distorsión máxima)
            SmilePreset::DadaDynamite => {
                let cutoff_hz = 20.0 + (s.powf(1.5)) * 6500.0 + overdrive * 3200.0;
                let resonance_q = 0.92 + s * 5.2 + overdrive * 3.5;
                let bp_mix = s * 0.68;

                let reverb_wet = (s.powf(1.1) * 0.90 + overdrive * 0.10).clamp(0.0, 0.98);
                let reverb_decay = (0.50 + s * 0.48).clamp(0.0, 0.99);

                let delay_wet = (s.powf(1.3) * 0.78 + overdrive * 0.22).clamp(0.0, 0.96);
                let delay_feedback = (0.38 + s * 0.60).clamp(0.0, 0.95);

                let sat_drive = 1.0 + s * 9.5 + overdrive * 6.5;
                let sat_mix = (s * 0.98).clamp(0.0, 1.0);

                let riser_depth = 1.0;
                let output_trim_db = s * 5.0 + overdrive * 4.0;

                DSPTargetParams {
                    cutoff_hz,
                    resonance_q,
                    bp_mix,
                    reverb_wet,
                    reverb_decay,
                    delay_wet,
                    delay_feedback,
                    sat_drive,
                    sat_mix,
                    riser_depth,
                    output_trim_db,
                }
            }
        }
    }
}
