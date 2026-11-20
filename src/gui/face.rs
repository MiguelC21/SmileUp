use nih_plug_egui::egui::{Color32, Pos2, Rect, Stroke, StrokeKind, Vec2};
use nih_plug_egui::egui::Painter;
use crate::gui::chassis::draw_bezier;
use crate::gui::theme::PluginTheme;

/// Dibuja la carita 3D ultra-glossy idéntica a la imagen de referencia
pub fn draw_smile_face(
    painter: &Painter,
    center: Pos2,
    radius: f32,
    intensity: f32,      // 0.0 a 1.10
    audio_peak: f32,     // reactividad
    _theme: &PluginTheme,
) {
    let s = (intensity / 1.10).clamp(0.0, 1.0);
    let is_overdrive = intensity > 1.0;

    // Rebote sutil reactivo a los graves
    let bounce = (audio_peak * 0.07 * (0.35 + s * 0.65)).clamp(0.0, 0.10);
    let effective_r = radius * (1.0 + bounce);

    // Temblor de euforia en overdrive
    let jitter = if intensity > 0.85 {
        let amt = if is_overdrive { (intensity - 0.85) * 7.0 } else { (intensity - 0.85) * 3.0 };
        let phase = intensity * 35.0;
        Vec2::new(phase.sin() * amt, (phase * 1.5).cos() * amt)
    } else {
        Vec2::ZERO
    };

    let center = center + jitter;

    // 1. Zócalo y bisel exterior del botón
    painter.circle_filled(center, effective_r * 1.07, Color32::from_rgb(14, 16, 20));
    painter.circle_stroke(center, effective_r * 1.07, Stroke::new(2.0f32, Color32::from_rgb(35, 40, 52)));
    painter.circle_stroke(center, effective_r * 1.03, Stroke::new(3.0f32, Color32::from_rgb(8, 9, 12)));

    // 2. Esfera Dorada Volumétrica con Iluminación Esférica 3D Multicapa
    // Capa 1: Sombra base inferior (ámbar cálido profundo)
    let col_base = if is_overdrive { Color32::from_rgb(215, 35, 15) } else { Color32::from_rgb(210, 120, 0) };
    painter.circle_filled(center, effective_r, col_base);

    // Capa 2: Cuerpo medio (dorado saturado)
    let col_mid = if is_overdrive { Color32::from_rgb(250, 75, 20) } else { Color32::from_rgb(255, 175, 5) };
    let c_mid = Pos2::new(center.x - effective_r * 0.04, center.y - effective_r * 0.06);
    painter.circle_filled(c_mid, effective_r * 0.94, col_mid);

    // Capa 3: Iluminación cenital (amarillo brillante)
    let col_bright = if is_overdrive { Color32::from_rgb(255, 130, 40) } else { Color32::from_rgb(255, 210, 30) };
    let c_bright = Pos2::new(center.x - effective_r * 0.10, center.y - effective_r * 0.16);
    painter.circle_filled(c_bright, effective_r * 0.82, col_bright);

    // Capa 4: Zona de alta luminosidad
    let col_light = if is_overdrive { Color32::from_rgb(255, 180, 80) } else { Color32::from_rgb(255, 235, 80) };
    let c_light = Pos2::new(center.x - effective_r * 0.16, center.y - effective_r * 0.25);
    painter.circle_filled(c_light, effective_r * 0.62, col_light);

    // Borde negro nítido de la cara
    painter.circle_stroke(center, effective_r, Stroke::new(3.2f32, Color32::from_rgb(14, 14, 18)));

    // 3. REFLEJO GLOSSY BLANCO DE ALTA DEFINICIÓN (Como en la imagen de referencia)
    // Elipse brillante superior izquierda
    let hl_center = Pos2::new(center.x - effective_r * 0.28, center.y - effective_r * 0.38);
    let hl_size = Vec2::new(effective_r * 0.68, effective_r * 0.36);
    painter.rect_filled(
        Rect::from_center_size(hl_center, hl_size),
        effective_r * 0.18,
        Color32::from_rgba_premultiplied(255, 255, 255, 195),
    );
    // Núcleo blanco 100% puro
    let hl_inner_size = Vec2::new(effective_r * 0.44, effective_r * 0.20);
    painter.rect_filled(
        Rect::from_center_size(hl_center, hl_inner_size),
        effective_r * 0.10,
        Color32::WHITE,
    );

    // Reflejo secundario curvo en la zona inferior derecha
    let bot_hl = Pos2::new(center.x + effective_r * 0.22, center.y + effective_r * 0.42);
    painter.circle_filled(bot_hl, effective_r * 0.28, Color32::from_rgba_premultiplied(255, 255, 255, 45));

    // 4. Mejillas con rubor cálido
    let blush_y = center.y + effective_r * 0.18;
    let blush_spacing = effective_r * 0.58;
    let blush_r = effective_r * 0.15;
    let blush_col = Color32::from_rgba_premultiplied(255, 150, 10, 115);
    painter.circle_filled(Pos2::new(center.x - blush_spacing, blush_y), blush_r, blush_col);
    painter.circle_filled(Pos2::new(center.x + blush_spacing, blush_y), blush_r, blush_col);

    // 5. Los Ojos (Estilo Anime/Emoji idéntico a la imagen)
    let eye_spacing = effective_r * 0.32;
    let eye_y = center.y + effective_r * 0.04;
    let left_eye_pos = Pos2::new(center.x - eye_spacing, eye_y);
    let right_eye_pos = Pos2::new(center.x + eye_spacing, eye_y);

    let eye_r = effective_r * 0.20;

    // Esclera blanca
    painter.circle_filled(left_eye_pos, eye_r, Color32::WHITE);
    painter.circle_stroke(left_eye_pos, eye_r, Stroke::new(3.5f32, Color32::from_rgb(14, 14, 18)));
    painter.circle_filled(right_eye_pos, eye_r, Color32::WHITE);
    painter.circle_stroke(right_eye_pos, eye_r, Stroke::new(3.5f32, Color32::from_rgb(14, 14, 18)));

    // Pupilas negras grandes
    let pupil_r = eye_r * 0.82;
    painter.circle_filled(left_eye_pos, pupil_r, Color32::from_rgb(12, 12, 16));
    painter.circle_filled(right_eye_pos, pupil_r, Color32::from_rgb(12, 12, 16));

    // Doble brillo especular en cada pupila
    let catch_main = pupil_r * 0.44;
    let catch_sub = pupil_r * 0.24;

    let off_main = Vec2::new(-pupil_r * 0.32, -pupil_r * 0.30);
    let off_sub = Vec2::new(pupil_r * 0.36, pupil_r * 0.34);

    painter.circle_filled(left_eye_pos + off_main, catch_main, Color32::WHITE);
    painter.circle_filled(left_eye_pos + off_sub, catch_sub, Color32::WHITE);
    painter.circle_filled(right_eye_pos + off_main, catch_main, Color32::WHITE);
    painter.circle_filled(right_eye_pos + off_sub, catch_sub, Color32::WHITE);

    // 6. Cejas negras decididas (Estilo idéntico a la imagen)
    let brow_w = eye_r * 1.35;
    let brow_y = eye_y - eye_r * 1.15;
    let brow_stroke = Stroke::new(4.5f32, Color32::from_rgb(14, 14, 18));

    painter.line_segment(
        [Pos2::new(left_eye_pos.x - brow_w * 0.55, brow_y), Pos2::new(left_eye_pos.x + brow_w * 0.55, brow_y)],
        brow_stroke,
    );
    painter.line_segment(
        [Pos2::new(right_eye_pos.x - brow_w * 0.55, brow_y), Pos2::new(right_eye_pos.x + brow_w * 0.55, brow_y)],
        brow_stroke,
    );

    // 7. La Sonrisa (Idéntica a la imagen en reposo, y abriéndose con la energía)
    let mouth_center_y = center.y + effective_r * 0.38;
    let mouth_w = effective_r * (0.60 + s * 0.30);

    if intensity < 0.25 {
        // Curva idéntica a la imagen
        let mouth_start = Pos2::new(center.x - mouth_w * 0.5, mouth_center_y - 2.0);
        let mouth_end = Pos2::new(center.x + mouth_w * 0.5, mouth_center_y - 2.0);
        let mouth_mid = Pos2::new(center.x, mouth_center_y + 11.0);

        draw_bezier(painter, mouth_start, mouth_mid, mouth_end, Stroke::new(4.8f32, Color32::from_rgb(14, 14, 18)));
    } else {
        // Boca abierta eufórica en build-up
        let open_h = 10.0 + (s - 0.25) * 38.0;
        let mouth_rect = Rect::from_center_size(
            Pos2::new(center.x, mouth_center_y + open_h * 0.3),
            Vec2::new(mouth_w, open_h),
        );

        painter.rect_filled(mouth_rect, open_h * 0.45, Color32::from_rgb(25, 6, 10));
        painter.rect_stroke(mouth_rect, open_h * 0.45, Stroke::new(4.5f32, Color32::from_rgb(14, 14, 18)), StrokeKind::Middle);

        // Dientes
        let tooth_w = mouth_w * 0.65 / 4.0;
        let tooth_h = (open_h * 0.35).clamp(4.0, 14.0);
        let tooth_start_x = center.x - (mouth_w * 0.65 * 0.5);
        for i in 0..4 {
            let tx = tooth_start_x + (i as f32) * tooth_w;
            let tr = Rect::from_min_size(Pos2::new(tx + 1.0, mouth_rect.top()), Vec2::new(tooth_w - 2.0, tooth_h));
            painter.rect_filled(tr, 2.0, Color32::WHITE);
        }

        // Lengua
        let tongue_rect = Rect::from_center_size(
            Pos2::new(center.x, mouth_rect.bottom() - open_h * 0.25),
            Vec2::new(mouth_w * 0.55, open_h * 0.35),
        );
        painter.rect_filled(tongue_rect, open_h * 0.25, Color32::from_rgb(255, 80, 120));
    }
}
