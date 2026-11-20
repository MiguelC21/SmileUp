use nih_plug_egui::egui::Color32;
use crate::dsp::SmilePreset;

#[derive(Debug, Clone, Copy)]
pub struct PluginTheme {
    pub bg_dark: Color32,
    pub bg_panel: Color32,
    pub gold_primary: Color32,
    pub gold_bright: Color32,
    pub orange_accent: Color32,
    pub red_accent: Color32,
    pub text_white: Color32,
    pub text_gray: Color32,
    pub needle_red: Color32,
}

impl PluginTheme {
    pub fn new() -> Self {
        Self {
            bg_dark: Color32::from_rgb(13, 14, 18),
            bg_panel: Color32::from_rgb(22, 24, 30),
            gold_primary: Color32::from_rgb(255, 184, 0),
            gold_bright: Color32::from_rgb(255, 215, 60),
            orange_accent: Color32::from_rgb(255, 105, 0),
            red_accent: Color32::from_rgb(235, 30, 20),
            text_white: Color32::from_rgb(250, 250, 252),
            text_gray: Color32::from_rgb(130, 140, 155),
            needle_red: Color32::from_rgb(255, 40, 40),
        }
    }

    pub fn for_preset(_preset: SmilePreset) -> Self {
        Self::new()
    }
}
