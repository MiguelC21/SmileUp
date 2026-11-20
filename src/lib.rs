use nih_plug::prelude::*;
use nih_plug_egui::EguiState;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

pub mod dsp;
pub mod gui;

use dsp::{SaturationUnit, SmilePreset, StereoDelay, StereoReverb, SvfFilter};

pub struct SmileUp {
    params: Arc<SmileUpParams>,
    filter: SvfFilter,
    reverb: StereoReverb,
    delay: StereoDelay,
    saturator: SaturationUnit,
    peak_meter: Arc<AtomicU32>,
    sample_rate: f32,
    riser_phase: f32,
}

#[derive(Params)]
pub struct SmileUpParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<EguiState>,

    /// El único control maestro: Intensity (0% a 110%)
    #[id = "intensity"]
    pub intensity: FloatParam,

    /// Selector de preset temático (idéntico a Endless Smile)
    #[id = "preset"]
    pub preset: EnumParam<SmilePreset>,
}

impl Default for SmileUp {
    fn default() -> Self {
        Self {
            params: Arc::new(SmileUpParams::default()),
            filter: SvfFilter::new(44100.0),
            reverb: StereoReverb::new(44100.0),
            delay: StereoDelay::new(44100.0),
            saturator: SaturationUnit::new(),
            peak_meter: Arc::new(AtomicU32::new(0)),
            sample_rate: 44100.0,
            riser_phase: 0.0,
        }
    }
}

impl Default for SmileUpParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(350, 390),

            intensity: FloatParam::new(
                "Intensity",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.10 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0)),

            preset: EnumParam::new("Preset", SmilePreset::FaceMeltingJoy),
        }
    }
}

impl Plugin for SmileUp {
    const NAME: &'static str = "SmileUp";
    const VENDOR: &'static str = "SmileUp Audio";
    const URL: &'static str = "https://smileup.audio";
    const EMAIL: &'static str = "info@smileup.audio";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: std::num::NonZeroU32::new(2),
            main_output_channels: std::num::NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
    ];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        gui::create_editor(
            self.params.clone(),
            self.peak_meter.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;
        self.filter.set_sample_rate(self.sample_rate);
        self.reverb = StereoReverb::new(self.sample_rate);
        self.delay = StereoDelay::new(self.sample_rate);
        self.saturator.reset();
        self.riser_phase = 0.0;
        true
    }

    fn reset(&mut self) {
        self.filter.reset();
        self.reverb.reset();
        self.delay.reset();
        self.saturator.reset();
        self.riser_phase = 0.0;
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let intensity = self.params.intensity.value();
        let preset = self.params.preset.value();

        // Si intensity está en 0%, bypass transparente con latencia cero
        if intensity <= 0.0005 {
            let mut max_abs_peak = 0.0f32;
            for mut channel_samples in buffer.iter_samples() {
                let l = *channel_samples.get_mut(0).unwrap();
                let r = *channel_samples.get_mut(1).unwrap();
                let peak = l.abs().max(r.abs());
                if peak > max_abs_peak { max_abs_peak = peak; }
            }
            self.update_peak_meter(max_abs_peak);
            return ProcessStatus::Normal;
        }

        let targets = preset.compute_dsp_targets(intensity);
        let trim_linear = nih_plug::util::db_to_gain(targets.output_trim_db);

        // Subdivisiones rítmicas de retardo
        let delay_time_l = 0.30;
        let delay_time_r = 0.42;

        let mut max_abs_peak = 0.0f32;

        for mut channel_samples in buffer.iter_samples() {
            let left_in = *channel_samples.get_mut(0).unwrap();
            let right_in = *channel_samples.get_mut(1).unwrap();

            // 1. Filtro Sweeper de tensión (High-Pass con resonancia)
            let (flt_l, flt_r) = self.filter.process_stereo(
                left_in,
                right_in,
                targets.cutoff_hz,
                targets.resonance_q,
                targets.bp_mix,
            );

            // 2. Modulación de Continual Riser (barberpole shimmer) si el preset lo incluye
            let (rise_l, rise_r) = if targets.riser_depth > 0.001 {
                self.riser_phase += 0.00045 * (1.0 + intensity * 2.0);
                if self.riser_phase > 1.0 { self.riser_phase -= 1.0; }
                let mod_amt = (self.riser_phase * std::f32::consts::TAU).sin() * 0.08 * targets.riser_depth;
                (flt_l * (1.0 + mod_amt), flt_r * (1.0 - mod_amt))
            } else {
                (flt_l, flt_r)
            };

            // 3. Reverb Espacial Wash-out
            let (rev_l, rev_r) = self.reverb.process_stereo(
                rise_l,
                rise_r,
                targets.reverb_decay,
                0.22,
                targets.reverb_wet,
            );

            // 4. Ping-Pong Delay Estéreo
            let (dly_l, dly_r) = self.delay.process_stereo(
                rev_l,
                rev_r,
                delay_time_l,
                delay_time_r,
                targets.delay_feedback,
                targets.delay_wet,
            );

            // 5. Saturación Armónica y Overdrive (al pasar de 100%)
            let (sat_l, sat_r) = self.saturator.process_stereo(
                dly_l,
                dly_r,
                targets.sat_drive,
                targets.sat_mix,
            );

            // 6. Ganancia compensatoria y soft-clip de seguridad
            let out_l = SaturationUnit::soft_clip(sat_l * trim_linear);
            let out_r = SaturationUnit::soft_clip(sat_r * trim_linear);

            let peak = out_l.abs().max(out_r.abs());
            if peak > max_abs_peak { max_abs_peak = peak; }

            *channel_samples.get_mut(0).unwrap() = out_l;
            *channel_samples.get_mut(1).unwrap() = out_r;
        }

        self.update_peak_meter(max_abs_peak);
        ProcessStatus::Normal
    }
}

impl SmileUp {
    fn update_peak_meter(&self, max_abs_peak: f32) {
        let old_bits = self.peak_meter.load(Ordering::Relaxed);
        let old_peak = f32::from_bits(old_bits);
        let smooth_peak = if max_abs_peak > old_peak {
            max_abs_peak
        } else {
            old_peak * 0.88 + max_abs_peak * 0.12
        };
        self.peak_meter.store(smooth_peak.to_bits(), Ordering::Relaxed);
    }
}

impl ClapPlugin for SmileUp {
    const CLAP_ID: &'static str = "com.smileup.smileup";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("The one-knob build-up and tension processor inspired by Endless Smile");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Filter,
        ClapFeature::Reverb,
        ClapFeature::Delay,
        ClapFeature::Distortion,
        ClapFeature::Stereo,
    ];
}

impl Vst3Plugin for SmileUp {
    const VST3_CLASS_ID: [u8; 16] = *b"SmileUpBuildUpFx";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Filter,
        Vst3SubCategory::Delay,
        Vst3SubCategory::Reverb,
        Vst3SubCategory::Distortion,
    ];
}

nih_export_clap!(SmileUp);
nih_export_vst3!(SmileUp);
