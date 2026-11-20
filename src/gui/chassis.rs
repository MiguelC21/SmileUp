use nih_plug_egui::egui::{Color32, Pos2, Rect, Stroke, Vec2};
use nih_plug_egui::egui::Painter;
use std::f32::consts::PI;
use crate::gui::theme::PluginTheme;

/// Dibuja el chasis oscuro idéntico a la imagen de referencia:
/// Hombros dorados superiores, tornillos en las esquinas, logo con la sonrisa y footer con medidor de espectro.
pub fn draw_hardware_chassis(
    painter: &Painter,
    rect: Rect,
    audio_peak: f32,
    theme: &PluginTheme,
) {
    // 1. Fondo base del chasis (Negro grafito mate)
    painter.rect_filled(rect, 20.0, theme.bg_dark);

    // 2. Borde exterior biselado sutil
    painter.rect_stroke(
        rect,
        20.0,
        Stroke::new(1.5f32, Color32::from_rgb(32, 36, 46)),
        nih_plug_egui::egui::StrokeKind::Inside,
    );

    // 3. Hombros superiores dorados curvados (Resplandor en las esquinas superiores)
    // Esquina superior izquierda
    let top_left_center = Pos2::new(rect.left() + 2.0, rect.top() + 2.0);
    draw_shoulder_glow(painter, top_left_center, true, theme);

    // Esquina superior derecha
    let top_right_center = Pos2::new(rect.right() - 2.0, rect.top() + 2.0);
    draw_shoulder_glow(painter, top_right_center, false, theme);

    // 4. Tornillos metálicos en las 4 esquinas
    let screw_margin = 16.0f32;
    let screw_positions = [
        Pos2::new(rect.left() + screw_margin, rect.top() + screw_margin),
        Pos2::new(rect.right() - screw_margin, rect.top() + screw_margin),
        Pos2::new(rect.left() + screw_margin, rect.bottom() - screw_margin),
        Pos2::new(rect.right() - screw_margin, rect.bottom() - screw_margin),
    ];

    for (i, &pos) in screw_positions.iter().enumerate() {
        draw_screw(painter, pos, i);
    }

    // 5. Pequeña sonrisa dorada arriba del título
    let smile_y = rect.top() + 25.0;
    let smile_w = 40.0;
    let smile_left = Pos2::new(rect.center().x - smile_w * 0.5, smile_y);
    let smile_right = Pos2::new(rect.center().x + smile_w * 0.5, smile_y);
    let smile_mid = Pos2::new(rect.center().x, smile_y + 8.5);

    draw_bezier(painter, smile_left, smile_mid, smile_right, Stroke::new(3.8f32, theme.gold_bright));

    // 6. Footer: Medidores de barras LED de espectro ("MORE ENERGY / MORE SMILE")
    let footer_y = rect.bottom() - 48.0;
    draw_spectrum_footer(painter, rect, footer_y, audio_peak, theme);
}

/// Dibuja los hombros dorados con arco curvado en las esquinas superiores
fn draw_shoulder_glow(painter: &Painter, pos: Pos2, is_left: bool, theme: &PluginTheme) {
    let r = 70.0f32;
    let segments = 24;
    let (start_a, end_a) = if is_left {
        (PI * 0.95, PI * 1.55)
    } else {
        (PI * 1.45, PI * 2.05)
    };

    let total = end_a - start_a;
    for i in 0..segments {
        let t1 = i as f32 / segments as f32;
        let t2 = (i + 1) as f32 / segments as f32;
        let a1 = start_a + total * t1;
        let a2 = start_a + total * t2;

        let p1 = pos + Vec2::new(a1.cos() * r, a1.sin() * r);
        let p2 = pos + Vec2::new(a2.cos() * r, a2.sin() * r);

        // Glow difuso
        painter.line_segment([p1, p2], Stroke::new(6.0f32, Color32::from_rgba_premultiplied(255, 180, 0, 45)));
        // Línea dorada sólida
        painter.line_segment([p1, p2], Stroke::new(2.5f32, theme.gold_bright));
    }
}

/// Dibuja los tornillos del chasis con ranura en cruz
fn draw_screw(painter: &Painter, center: Pos2, idx: usize) {
    let r = 4.5f32;
    painter.circle_filled(Pos2::new(center.x, center.y + 0.8), r + 0.6, Color32::from_rgb(5, 6, 8));
    painter.circle_filled(center, r, Color32::from_rgb(32, 36, 44));
    painter.circle_stroke(center, r, Stroke::new(0.8f32, Color32::from_rgb(50, 56, 68)));

    let angle = (idx as f32) * (PI * 0.28) + 0.2;
    let len = 2.8f32;
    let stroke = Stroke::new(1.2f32, Color32::from_rgb(12, 14, 18));

    let p1 = center + Vec2::new(angle.cos() * len, angle.sin() * len);
    let p2 = center - Vec2::new(angle.cos() * len, angle.sin() * len);
    painter.line_segment([p1, p2], stroke);

    let cross_a = angle + PI * 0.5;
    let p3 = center + Vec2::new(cross_a.cos() * len, cross_a.sin() * len);
    let p4 = center - Vec2::new(cross_a.cos() * len, cross_a.sin() * len);
    painter.line_segment([p3, p4], stroke);
}

/// Dibuja el footer con las barras LED a izquierda/derecha y el texto central
fn draw_spectrum_footer(
    painter: &Painter,
    rect: Rect,
    center_y: f32,
    audio_peak: f32,
    theme: &PluginTheme,
) {
    let bars_count = 8;
    let bar_w = 3.2f32;
    let bar_spacing = 5.2f32;
    let peak_mod = 0.5 + audio_peak.clamp(0.0, 1.0) * 0.8;

    // Alturas base de campana tipo espectro: ▂▃▅▆▇▅▃▂
    let base_heights = [5.0, 8.0, 13.0, 18.0, 22.0, 17.0, 10.0, 6.0];

    // Barras de la izquierda
    let left_start_x = rect.left() + 38.0;
    for i in 0..bars_count {
        let x = left_start_x + (i as f32) * bar_spacing;
        let h = base_heights[i] * peak_mod;
        let bar_rect = Rect::from_min_size(
            Pos2::new(x, center_y - h * 0.5),
            Vec2::new(bar_w, h),
        );
        let alpha = (120.0 + (i as f32 / bars_count as f32) * 135.0) as u8;
        painter.rect_filled(
            bar_rect,
            1.5,
            Color32::from_rgba_premultiplied(theme.gold_bright.r(), theme.gold_bright.g(), theme.gold_bright.b(), alpha),
        );
    }

    // Barras de la derecha (en espejo)
    let right_start_x = rect.right() - 38.0 - (bars_count as f32 * bar_spacing);
    for i in 0..bars_count {
        let x = right_start_x + (i as f32) * bar_spacing;
        let h = base_heights[bars_count - 1 - i] * peak_mod;
        let bar_rect = Rect::from_min_size(
            Pos2::new(x, center_y - h * 0.5),
            Vec2::new(bar_w, h),
        );
        let alpha = (120.0 + ((bars_count - 1 - i) as f32 / bars_count as f32) * 135.0) as u8;
        painter.rect_filled(
            bar_rect,
            1.5,
            Color32::from_rgba_premultiplied(theme.gold_bright.r(), theme.gold_bright.g(), theme.gold_bright.b(), alpha),
        );
    }

    // Centro: "── MORE ENERGY ──" y "── MORE SMILE ──"
    let center_x = rect.center().x;
    let line_len = 22.0f32;
    let line_stroke = Stroke::new(1.0f32, Color32::from_rgb(60, 68, 82));

    // Líneas superiores
    painter.line_segment([Pos2::new(center_x - 72.0 - line_len, center_y - 6.0), Pos2::new(center_x - 72.0, center_y - 6.0)], line_stroke);
    painter.line_segment([Pos2::new(center_x + 72.0, center_y - 6.0), Pos2::new(center_x + 72.0 + line_len, center_y - 6.0)], line_stroke);

    painter.text(
        Pos2::new(center_x, center_y - 6.0),
        nih_plug_egui::egui::Align2::CENTER_CENTER,
        "MORE ENERGY",
        nih_plug_egui::egui::FontId::proportional(8.5),
        theme.text_gray,
    );

    painter.text(
        Pos2::new(center_x, center_y + 7.0),
        nih_plug_egui::egui::Align2::CENTER_CENTER,
        "MORE SMILE",
        nih_plug_egui::egui::FontId::proportional(8.5),
        theme.text_gray,
    );
}

pub fn draw_bezier(painter: &Painter, p0: Pos2, p1: Pos2, p2: Pos2, stroke: Stroke) {
    let steps = 18;
    for i in 0..steps {
        let t1 = i as f32 / steps as f32;
        let t2 = (i + 1) as f32 / steps as f32;

        let a1 = Pos2::new(p0.x + (p1.x - p0.x) * t1, p0.y + (p1.y - p0.y) * t1);
        let b1 = Pos2::new(p1.x + (p2.x - p1.x) * t1, p1.y + (p2.y - p1.y) * t1);
        let pt1 = Pos2::new(a1.x + (b1.x - a1.x) * t1, a1.y + (b1.y - a1.y) * t1);

        let a2 = Pos2::new(p0.x + (p1.x - p0.x) * t2, p0.y + (p1.y - p0.y) * t2);
        let b2 = Pos2::new(p1.x + (p2.x - p1.x) * t2, p1.y + (p2.y - p1.y) * t2);
        let pt2 = Pos2::new(a2.x + (b2.x - a2.x) * t2, a2.y + (b2.y - a2.y) * t2);

        painter.line_segment([pt1, pt2], stroke);
    }
}

