use nih_plug_egui::egui::{ColorImage, Context, Id, TextureHandle, TextureOptions};
use std::sync::OnceLock;

static BG_BYTES: &[u8] = include_bytes!("../../assets/background.jpg");
static DECODED_BG: OnceLock<ColorImage> = OnceLock::new();

fn decode_bytes_to_color_image(bytes: &[u8]) -> ColorImage {
    let img = image::load_from_memory(bytes).expect("Error decodificando textura");
    let rgba = img.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    ColorImage::from_rgba_unmultiplied(size, rgba.as_raw())
}

#[derive(Clone)]
pub struct PluginTextures {
    pub bg: TextureHandle,
}

pub fn get_or_load_textures(ctx: &Context) -> PluginTextures {
    let key = Id::new("smileup_bg_persisted");

    if let Some(texs) = ctx.data_mut(|d| d.get_persisted::<PluginTextures>(key)) {
        return texs;
    }

    let bg_img = DECODED_BG.get_or_init(|| decode_bytes_to_color_image(BG_BYTES));
    let texs = PluginTextures {
        bg: ctx.load_texture("smileup_bg", bg_img.clone(), TextureOptions::LINEAR),
    };

    ctx.data_mut(|d| {
        d.insert_persisted(key, texs.clone());
    });

    texs
}
