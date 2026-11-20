use nih_plug_egui::egui::{Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};
use std::f32::consts::PI;

/// Dibuja la perilla moderna de estilo cream/skeuomorphic exactamente como en la imagen de referencia:
/// - 100% concéntrica y perfectamente alineada sobre el centro
/// - Anillo exterior de 13 puntos (dot ticks)
/// - Pista circular y arco activo rojo coral brillante
/// - Dial en crema cálido con borde nítido
/// - Plato interior rehundido con número centrado (e.g. "88")
/// - Muesca radial oscura nítida
pub fn draw_modern_cream_knob(
    ui: &mut Ui,
    center: Pos2,
    radius: f32, // e.g. 48.0
    current_val: f32, // 0.0 a 1.10
) -> (Response, f32) {
    let hit_radius = radius * 1.38;
    let knob_rect = Rect::from_center_size(center, Vec2::splat(hit_radius * 2.0));
    let response = ui.allocate_rect(knob_rect, Sense::click_and_drag());

    let mut new_val = current_val;

    if response.dragged() {
        let delta = -response.drag_delta().y * 0.005;
        let multiplier = if ui.input(|i| i.modifiers.shift) { 0.2 } else { 1.0 };
        new_val = (current_val + delta * multiplier).clamp(0.0, 1.10);
    }

    if response.double_clicked() {
        new_val = 0.0;
    }

    let painter = ui.painter();

    // Rango angular estándar (270 grados):
    // 0%   -> 135° (abajo-izquierda)
    // 50%  -> 270° (arriba, 12 en punto)
    // 100% -> 405° (abajo-derecha)
    let start_angle = PI * 0.75;
    let end_angle = PI * 2.25;
    let total_arc = end_angle - start_angle;

    let norm = (new_val / 1.00).clamp(0.0, 1.10);
    let active_angle = start_angle + total_arc * norm.min(1.0);

    // Geometría 100% concéntrica: todos los radios comparten el mismo centro exacto
    let r_dots = radius + 15.0; // Puntos exteriores
    let r_arc = radius + 5.5;   // Pista del arco
    let r_knob = radius;        // Borde exterior del dial
    let r_inner = radius * 0.68; // Disco central

    // 1. Sombra suave concéntrica debajo de la perilla
    painter.circle_filled(
        center + Vec2::new(0.0, 3.0),
        r_arc + 2.0,
        Color32::from_rgba_premultiplied(0, 0, 0, 55),
    );

    // 2. Anillo concéntrico de 13 puntos (dot ticks) idénticos a la foto
    let dot_count = 13;
    let dot_color = Color32::from_rgb(148, 142, 134);
    for i in 0..dot_count {
        let t = i as f32 / (dot_count - 1) as f32;
        let ang = start_angle + total_arc * t;
        let dot_pos = center + Vec2::new(ang.cos() * r_dots, ang.sin() * r_dots);
        painter.circle_filled(dot_pos, 2.0, dot_color);
    }

    // 3. Pista del arco inactiva (ranura oscura suave)
    let segments = 64;
    let dark_track_color = Color32::from_rgb(42, 38, 35);
    let arc_thickness = 4.2f32;

    for i in 0..segments {
        let t1 = i as f32 / segments as f32;
        let t2 = (i + 1) as f32 / segments as f32;
        let a1 = start_angle + total_arc * t1;
        let a2 = start_angle + total_arc * t2;

        let p1 = center + Vec2::new(a1.cos() * r_arc, a1.sin() * r_arc);
        let p2 = center + Vec2::new(a2.cos() * r_arc, a2.sin() * r_arc);
        painter.line_segment([p1, p2], Stroke::new(arc_thickness, dark_track_color));
    }

    // 4. Arco activo rojo coral brillante
    if norm > 0.005 {
        let active_segments = (segments as f32 * norm.min(1.0)).ceil() as usize;
        let coral_red = Color32::from_rgb(255, 60, 75);
        let coral_glow = Color32::from_rgba_premultiplied(255, 60, 75, 45);

        for i in 0..active_segments {
            let t1 = (i as f32 / segments as f32).min(norm);
            let t2 = ((i + 1) as f32 / segments as f32).min(norm);
            let a1 = start_angle + total_arc * t1;
            let a2 = start_angle + total_arc * t2;

            let p1 = center + Vec2::new(a1.cos() * r_arc, a1.sin() * r_arc);
            let p2 = center + Vec2::new(a2.cos() * r_arc, a2.sin() * r_arc);

            // Glow sutil
            painter.line_segment([p1, p2], Stroke::new(arc_thickness + 3.0, coral_glow));
            // Línea roja principal
            painter.line_segment([p1, p2], Stroke::new(arc_thickness, coral_red));
        }

        // Overdrive (> 100%)
        if norm > 1.0 {
            let tip_pos = center + Vec2::new(active_angle.cos() * r_arc, active_angle.sin() * r_arc);
            painter.circle_filled(tip_pos, 4.0, Color32::from_rgb(255, 230, 90));
        }
    }

    // 5. Dial convexo crema (concéntrico, color cálido, borde fino)
    let dial_color = Color32::from_rgb(238, 233, 224);
    painter.circle_filled(center, r_knob, dial_color);
    painter.circle_stroke(center, r_knob, Stroke::new(1.0f32, Color32::from_rgb(205, 198, 188)));

    // 6. Plato interior central (concéntrico, tono arena suave)
    let inner_plate_color = Color32::from_rgb(220, 213, 202);
    painter.circle_filled(center, r_inner, inner_plate_color);
    painter.circle_stroke(center, r_inner, Stroke::new(1.0f32, Color32::from_rgb(190, 182, 170)));

    // 7. Readout numérico centrado (e.g. "88")
    let display_num = (new_val * 100.0).round() as i32;
    let num_text = format!("{}", display_num);
    let text_color = Color32::from_rgb(58, 52, 45);

    painter.text(
        center,
        nih_plug_egui::egui::Align2::CENTER_CENTER,
        num_text,
        nih_plug_egui::egui::FontId::proportional(15.0),
        text_color,
    );

    // 8. Muesca radial indicadora oscura (Pointer tick)
    let pointer_r_start = r_inner + 2.0;
    let pointer_r_end = r_knob - 2.0;
    let tick_start = center + Vec2::new(active_angle.cos() * pointer_r_start, active_angle.sin() * pointer_r_start);
    let tick_end = center + Vec2::new(active_angle.cos() * pointer_r_end, active_angle.sin() * pointer_r_end);
    let pointer_color = Color32::from_rgb(52, 46, 40);

    painter.line_segment([tick_start, tick_end], Stroke::new(2.8f32, pointer_color));

    (response, new_val)
}
