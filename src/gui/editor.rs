use nih_plug::prelude::Editor;
use nih_plug_egui::egui::{Color32, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2};
use nih_plug_egui::{create_egui_editor, egui, EguiState};
use std::sync::atomic::AtomicU32;
use std::sync::Arc;

use crate::dsp::SmilePreset;
use crate::gui::knob::draw_modern_cream_knob;
use crate::gui::textures::get_or_load_textures;
use crate::SmileUpParams;

pub fn create_editor(
    params: Arc<SmileUpParams>,
    _peak_meter: Arc<AtomicU32>,
    editor_state: Arc<EguiState>,
) -> Option<Box<dyn Editor>> {
    create_egui_editor(
        editor_state,
        (),
        |_, _| {},
        move |ctx, setter, _| {
            let textures = get_or_load_textures(ctx);

            let current_preset = params.preset.value();
            let current_intensity = params.intensity.value();

            let presets = [
                SmilePreset::FaceMeltingJoy,
                SmilePreset::SmackInTheFace,
                SmilePreset::FistInTheAir,
                SmilePreset::HeliumJumping,
                SmilePreset::BrokenTeeth,
                SmilePreset::DadaDynamite,
            ];
            let current_idx = presets
                .iter()
                .position(|&p| p == current_preset)
                .unwrap_or(0);

            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(Color32::from_rgb(38, 40, 46)))
                .show(ctx, |ui| {
                    let total_rect = ui.max_rect();
                    let center_x = total_rect.center().x;

                    // ----------------------------------------------------
                    // 1. FONDO CON TEXTURA REAL DE ESTUDIO
                    // ----------------------------------------------------
                    ui.painter().image(
                        textures.bg.id(),
                        total_rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE,
                    );

                    // Bisel y marco metálico del chasis
                    ui.painter().rect_stroke(
                        total_rect,
                        0.0,
                        Stroke::new(2.0f32, Color32::from_rgb(20, 22, 26)),
                        StrokeKind::Inside,
                    );
                    ui.painter().line_segment(
                        [total_rect.left_top() + Vec2::new(2.0, 2.0), total_rect.right_top() + Vec2::new(-2.0, 2.0)],
                        Stroke::new(1.2f32, Color32::from_rgba_premultiplied(255, 255, 255, 35)),
                    );

                    // Tornillos de chasis en las 4 esquinas (rackmount analógico)
                    let screw_coords = [
                        total_rect.left_top() + Vec2::new(14.0, 14.0),
                        total_rect.right_top() + Vec2::new(-14.0, 14.0),
                        total_rect.left_bottom() + Vec2::new(14.0, -14.0),
                        total_rect.right_bottom() + Vec2::new(-14.0, -14.0),
                    ];
                    for sc in screw_coords {
                        ui.painter().circle_filled(sc + Vec2::new(0.5, 1.0), 4.5, Color32::from_rgba_premultiplied(0, 0, 0, 80));
                        ui.painter().circle_filled(sc, 4.0, Color32::from_rgb(72, 75, 82));
                        ui.painter().circle_stroke(sc, 4.0, Stroke::new(1.0f32, Color32::from_rgb(32, 35, 40)));
                        ui.painter().line_segment(
                            [sc + Vec2::new(-2.2, -1.0), sc + Vec2::new(2.2, 1.0)],
                            Stroke::new(1.2f32, Color32::from_rgb(22, 25, 28)),
                        );
                    }

                    // ----------------------------------------------------
                    // 2. LOGO SUPERIOR CENTRADO: SMILE UP
                    // ----------------------------------------------------
                    let logo_y = total_rect.top() + 36.0;
                    ui.painter().text(
                        Pos2::new(center_x, logo_y),
                        nih_plug_egui::egui::Align2::CENTER_CENTER,
                        "SMILE UP",
                        nih_plug_egui::egui::FontId::proportional(22.0),
                        Color32::from_rgb(245, 245, 248),
                    );

                    // Arco de sonrisa bajo el título
                    let smile_c = Pos2::new(center_x, logo_y + 14.0);
                    let mut smile_pts = Vec::with_capacity(16);
                    for i in 0..16 {
                        let t = i as f32 / 15.0;
                        let ang = std::f32::consts::PI * 0.15 + std::f32::consts::PI * 0.70 * t;
                        smile_pts.push(smile_c + Vec2::new(ang.cos() * 26.0, ang.sin() * 9.0));
                    }
                    for i in 0..smile_pts.len() - 1 {
                        ui.painter().line_segment(
                            [smile_pts[i], smile_pts[i + 1]],
                            Stroke::new(2.5f32, Color32::from_rgb(255, 65, 77)),
                        );
                    }

                    // ----------------------------------------------------
                    // 3. PERILLA MODERNA CONVEXA (CENTRO ABSOLUTO)
                    // ----------------------------------------------------
                    let knob_center_y = total_rect.top() + 154.0;
                    let knob_center = Pos2::new(center_x, knob_center_y);
                    let knob_radius = 50.0f32;

                    let (resp_knob, new_val) = draw_modern_cream_knob(
                        ui,
                        knob_center,
                        knob_radius,
                        current_intensity,
                    );

                    if resp_knob.drag_started() {
                        setter.begin_set_parameter(&params.intensity);
                    }
                    if resp_knob.dragged() || resp_knob.double_clicked() {
                        if (new_val - current_intensity).abs() > 0.0001 {
                            setter.set_parameter(&params.intensity, new_val);
                        }
                    }
                    if resp_knob.drag_stopped() {
                        setter.end_set_parameter(&params.intensity);
                    }

                    // Clic derecho: invocar directamente el Menú Contextual Nativo de FL Studio (IComponentHandler3)
                    if resp_knob.secondary_clicked() {
                        let click_pos = resp_knob.interact_pointer_pos().unwrap_or(knob_center);
                        setter.show_context_menu(&params.intensity, click_pos.x as i32, click_pos.y as i32);
                    }

                    // Texto "INTENSITY" debajo de la perilla
                    ui.painter().text(
                        Pos2::new(center_x, knob_center_y + knob_radius + 26.0),
                        nih_plug_egui::egui::Align2::CENTER_CENTER,
                        "INTENSITY",
                        nih_plug_egui::egui::FontId::proportional(14.0),
                        Color32::from_rgb(230, 225, 215),
                    );

                    // ----------------------------------------------------
                    // 4. CAJA ROSA DE PRESETS (CENTRADA)
                    // ----------------------------------------------------
                    let box_y = total_rect.top() + 292.0;
                    let box_w = 204.0f32;
                    let box_h = 44.0f32;
                    let box_rect = Rect::from_center_size(Pos2::new(center_x, box_y), Vec2::new(box_w, box_h));

                    // Sombra suave bajo la caja rosa
                    ui.painter().rect_filled(
                        box_rect.translate(Vec2::new(0.0, 3.0)),
                        8.0,
                        Color32::from_rgba_premultiplied(0, 0, 0, 90),
                    );

                    // Fondo rosa chicle
                    let pink_color = Color32::from_rgb(240, 95, 150);
                    ui.painter().rect_filled(box_rect, 7.0, pink_color);
                    ui.painter().rect_stroke(box_rect, 7.0, Stroke::new(2.5f32, Color32::BLACK), StrokeKind::Middle);

                    // Badge superior negro con categoría (MEDIUM, HIGH, EXTREME)
                    let badge_text = current_preset.category_name();
                    let badge_rect = Rect::from_center_size(Pos2::new(center_x, box_rect.top() - 6.0), Vec2::new(65.0, 16.0));
                    ui.painter().rect_filled(badge_rect, 3.0, Color32::from_rgb(18, 20, 24));
                    ui.painter().rect_stroke(badge_rect, 3.0, Stroke::new(1.2f32, Color32::BLACK), StrokeKind::Middle);
                    ui.painter().text(
                        badge_rect.center(),
                        nih_plug_egui::egui::Align2::CENTER_CENTER,
                        badge_text,
                        nih_plug_egui::egui::FontId::proportional(9.0),
                        Color32::WHITE,
                    );

                    // Botón Flecha Izquierda Naranja (◀)
                    let arrow_l_rect = Rect::from_min_size(
                        Pos2::new(box_rect.left() - 28.0, box_rect.top() + 6.0),
                        Vec2::new(30.0, 32.0),
                    );
                    let resp_arr_l = ui.allocate_rect(arrow_l_rect, Sense::click());
                    let col_arrow_l = if resp_arr_l.hovered() { Color32::from_rgb(255, 160, 20) } else { Color32::from_rgb(242, 130, 0) };
                    ui.painter().rect_filled(arrow_l_rect, 4.0, col_arrow_l);
                    ui.painter().rect_stroke(arrow_l_rect, 4.0, Stroke::new(2.0f32, Color32::BLACK), StrokeKind::Middle);
                    ui.painter().text(
                        arrow_l_rect.center(),
                        nih_plug_egui::egui::Align2::CENTER_CENTER,
                        "◀",
                        nih_plug_egui::egui::FontId::proportional(14.0),
                        Color32::WHITE,
                    );

                    // Botón Flecha Derecha Naranja (▶)
                    let arrow_r_rect = Rect::from_min_size(
                        Pos2::new(box_rect.right() - 2.0, box_rect.top() + 6.0),
                        Vec2::new(30.0, 32.0),
                    );
                    let resp_arr_r = ui.allocate_rect(arrow_r_rect, Sense::click());
                    let col_arrow_r = if resp_arr_r.hovered() { Color32::from_rgb(255, 160, 20) } else { Color32::from_rgb(242, 130, 0) };
                    ui.painter().rect_filled(arrow_r_rect, 4.0, col_arrow_r);
                    ui.painter().rect_stroke(arrow_r_rect, 4.0, Stroke::new(2.0f32, Color32::BLACK), StrokeKind::Middle);
                    ui.painter().text(
                        arrow_r_rect.center(),
                        nih_plug_egui::egui::Align2::CENTER_CENTER,
                        "▶",
                        nih_plug_egui::egui::FontId::proportional(14.0),
                        Color32::WHITE,
                    );

                    // Panel blanco interior para el nombre del preset
                    let label_rect = Rect::from_min_size(
                        Pos2::new(box_rect.left() + 8.0, box_rect.top() + 7.0),
                        Vec2::new(box_w - 16.0, box_h - 14.0),
                    );
                    let resp_label = ui.allocate_rect(label_rect, Sense::click());
                    let label_fill = if resp_label.hovered() { Color32::from_rgb(255, 255, 245) } else { Color32::WHITE };
                    ui.painter().rect_filled(label_rect, 4.0, label_fill);
                    ui.painter().rect_stroke(label_rect, 4.0, Stroke::new(1.8f32, Color32::BLACK), StrokeKind::Middle);

                    if resp_arr_l.clicked() {
                        let prev_idx = if current_idx == 0 { presets.len() - 1 } else { current_idx - 1 };
                        setter.set_parameter(&params.preset, presets[prev_idx]);
                    }
                    if resp_arr_r.clicked() {
                        let next_idx = (current_idx + 1) % presets.len();
                        setter.set_parameter(&params.preset, presets[next_idx]);
                    }

                    let (p_icon, p_name) = match current_preset {
                        SmilePreset::FaceMeltingJoy => ("🍭", "Face Melting Joy"),
                        SmilePreset::SmackInTheFace => ("🥊", "Smack in the Face"),
                        SmilePreset::FistInTheAir => ("✊", "Fist in the Air"),
                        SmilePreset::HeliumJumping => ("🎈", "Helium Jumping"),
                        SmilePreset::BrokenTeeth => ("🦷", "Broken Teeth"),
                        SmilePreset::DadaDynamite => ("🧨", "Dada Dynamite"),
                    };

                    ui.painter().text(
                        Pos2::new(label_rect.left() + 8.0, label_rect.center().y),
                        nih_plug_egui::egui::Align2::LEFT_CENTER,
                        format!("{} {}", p_icon, p_name),
                        nih_plug_egui::egui::FontId::proportional(12.5),
                        Color32::BLACK,
                    );
                    ui.painter().text(
                        Pos2::new(label_rect.right() - 8.0, label_rect.center().y),
                        nih_plug_egui::egui::Align2::RIGHT_CENTER,
                        "▼",
                        nih_plug_egui::egui::FontId::proportional(10.0),
                        Color32::from_rgb(120, 120, 120),
                    );

                    // Menú desplegable interactivo
                    let menu_id = ui.make_persistent_id("smile_preset_menu_compact");
                    if resp_label.clicked() {
                        ui.data_mut(|d| {
                            let open = d.get_temp::<bool>(menu_id).unwrap_or(false);
                            d.insert_temp(menu_id, !open);
                        });
                    }

                    // Renderizar el menú desplegable por encima de todo si está abierto
                    let is_menu_open = ui.data(|d| d.get_temp::<bool>(menu_id).unwrap_or(false));
                    if is_menu_open {
                        let menu_rect = Rect::from_min_size(
                            Pos2::new(box_rect.left(), box_rect.bottom() + 4.0),
                            Vec2::new(box_w, 142.0),
                        );
                        ui.painter().rect_filled(
                            menu_rect.translate(Vec2::new(0.0, 3.0)),
                            6.0,
                            Color32::from_rgba_premultiplied(0, 0, 0, 120),
                        );
                        ui.painter().rect_filled(menu_rect, 6.0, Color32::WHITE);
                        ui.painter().rect_stroke(menu_rect, 6.0, Stroke::new(2.0f32, Color32::BLACK), StrokeKind::Middle);

                        let item_h = 23.0f32;
                        for (idx, p_item) in presets.iter().enumerate() {
                            let item_rect = Rect::from_min_size(
                                Pos2::new(menu_rect.left() + 2.0, menu_rect.top() + 2.0 + idx as f32 * item_h),
                                Vec2::new(menu_rect.width() - 4.0, item_h),
                            );
                            let resp_item = ui.allocate_rect(item_rect, Sense::click());
                            if resp_item.hovered() {
                                ui.painter().rect_filled(item_rect, 3.0, Color32::from_rgb(255, 220, 235));
                            }
                            if resp_item.clicked() {
                                setter.set_parameter(&params.preset, *p_item);
                                ui.data_mut(|d| d.insert_temp(menu_id, false));
                            }

                            let item_name = match p_item {
                                SmilePreset::FaceMeltingJoy => "🍭 Face Melting Joy",
                                SmilePreset::SmackInTheFace => "🥊 Smack in the Face",
                                SmilePreset::FistInTheAir => "✊ Fist in the Air",
                                SmilePreset::HeliumJumping => "🎈 Helium Jumping",
                                SmilePreset::BrokenTeeth => "🦷 Broken Teeth",
                                SmilePreset::DadaDynamite => "🧨 Dada Dynamite",
                            };
                            ui.painter().text(
                                Pos2::new(item_rect.left() + 6.0, item_rect.center().y),
                                nih_plug_egui::egui::Align2::LEFT_CENTER,
                                item_name,
                                nih_plug_egui::egui::FontId::proportional(11.5),
                                Color32::BLACK,
                            );
                        }
                    }
                });

            // Tasa de refresco óptima a 60 FPS
            ctx.request_repaint_after(std::time::Duration::from_millis(16));
        },
    )
}
