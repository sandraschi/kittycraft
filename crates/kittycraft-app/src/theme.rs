use egui::{Color32, ColorImage, CornerRadius, Frame, Margin, RichText, Stroke, Ui, Vec2};
use kittycraft_core::catalog::CatBreed;

#[derive(Clone, Copy)]
pub struct Palette {
    pub bg_canvas: Color32,
    pub bg_card: Color32,
    pub bg_card_alt: Color32,
    pub border: Color32,
    pub text_primary: Color32,
    pub text_muted: Color32,
    pub accent: Color32,
    pub accent_soft: Color32,
    pub good: Color32,
    pub warn: Color32,
    pub danger: Color32,
}

pub fn palette(dark: bool) -> Palette {
    if dark {
        Palette {
            bg_canvas: Color32::from_rgb(19, 18, 24),
            bg_card: Color32::from_rgb(30, 28, 38),
            bg_card_alt: Color32::from_rgb(40, 37, 50),
            border: Color32::from_rgb(66, 60, 82),
            text_primary: Color32::from_rgb(242, 238, 248),
            text_muted: Color32::from_rgb(172, 166, 188),
            accent: Color32::from_rgb(168, 130, 238),
            accent_soft: Color32::from_rgba_unmultiplied(168, 130, 238, 36),
            good: Color32::from_rgb(88, 198, 140),
            warn: Color32::from_rgb(236, 176, 68),
            danger: Color32::from_rgb(232, 96, 108),
        }
    } else {
        Palette {
            bg_canvas: Color32::from_rgb(248, 246, 252),
            bg_card: Color32::from_rgb(255, 254, 255),
            bg_card_alt: Color32::from_rgb(238, 234, 246),
            border: Color32::from_rgb(208, 200, 224),
            text_primary: Color32::from_rgb(32, 28, 42),
            text_muted: Color32::from_rgb(104, 96, 122),
            accent: Color32::from_rgb(118, 72, 204),
            accent_soft: Color32::from_rgba_unmultiplied(118, 72, 204, 28),
            good: Color32::from_rgb(40, 146, 90),
            warn: Color32::from_rgb(194, 124, 24),
            danger: Color32::from_rgb(198, 56, 72),
        }
    }
}

pub fn apply_theme(ctx: &egui::Context, dark: bool) {
    let pal = palette(dark);
    let theme = if dark {
        egui::Theme::Dark
    } else {
        egui::Theme::Light
    };
    ctx.set_theme(theme);
    let mut visuals = if dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };
    visuals.override_text_color = Some(pal.text_primary);
    visuals.weak_text_color = Some(pal.text_muted);
    visuals.panel_fill = pal.bg_canvas;
    visuals.window_fill = pal.bg_card;
    visuals.extreme_bg_color = pal.bg_card_alt;
    visuals.faint_bg_color = pal.bg_card_alt;
    visuals.selection.bg_fill = pal.accent_soft;
    visuals.selection.stroke = Stroke::new(1.0, pal.accent);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, pal.text_primary);
    visuals.widgets.noninteractive.bg_fill = pal.bg_card;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, pal.border);
    visuals.widgets.noninteractive.corner_radius = CornerRadius::same(8);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, pal.text_primary);
    visuals.widgets.inactive.bg_fill = pal.bg_card_alt;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, pal.border);
    visuals.widgets.inactive.corner_radius = CornerRadius::same(7);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.5, pal.text_primary);
    visuals.widgets.hovered.bg_fill = pal.accent_soft;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, pal.accent);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(7);
    visuals.widgets.active.fg_stroke = Stroke::new(1.5, pal.text_primary);
    visuals.widgets.active.bg_fill = pal.accent;
    visuals.widgets.active.corner_radius = CornerRadius::same(7);
    visuals.window_corner_radius = CornerRadius::same(12);
    visuals.window_stroke = Stroke::new(1.0, pal.border);
    ctx.set_visuals_of(theme, visuals);

    let mut style = (*ctx.style_of(theme)).clone();
    style.spacing.item_spacing = Vec2::new(8.0, 7.0);
    style.spacing.button_padding = Vec2::new(11.0, 6.0);
    style.spacing.window_margin = Margin::same(14);
    ctx.set_style_of(theme, style);
}

pub fn card_frame(_ui: &Ui, dark: bool) -> Frame {
    let pal = palette(dark);
    Frame::new()
        .fill(pal.bg_card)
        .stroke(Stroke::new(1.0, pal.border))
        .corner_radius(CornerRadius::same(10))
        .inner_margin(Margin::same(12))
}

pub fn hero_header(ui: &mut Ui, dark: bool, title: &str, subtitle: &str) {
    let pal = palette(dark);
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(Vec2::new(4.0, 36.0), egui::Sense::hover());
        ui.painter()
            .rect_filled(rect, CornerRadius::same(2), pal.accent);
        ui.vertical(|ui| {
            ui.label(
                RichText::new(title)
                    .size(21.0)
                    .strong()
                    .color(pal.text_primary),
            );
            ui.label(RichText::new(subtitle).size(13.0).color(pal.text_muted));
        });
    });
    ui.add_space(6.0);
}

pub fn pill_badge(ui: &mut Ui, text: &str, color: Color32) {
    let bg = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 34);
    Frame::new()
        .fill(bg)
        .stroke(Stroke::new(1.0, color))
        .corner_radius(CornerRadius::same(12))
        .inner_margin(Margin::symmetric(8, 3))
        .show(ui, |ui| {
            ui.label(RichText::new(text).size(11.5).strong().color(color));
        });
}

/// Render a high-contrast procedural portrait for either a kitten or adult cat of a breed.
pub fn build_cat_portrait(breed: &CatBreed, is_kitten: bool) -> ColorImage {
    let w = 128_usize;
    let h = 128_usize;
    let mut pixels = vec![Color32::from_rgb(36, 33, 46); w * h];

    let coat = Color32::from_rgb(breed.coat_rgb[0], breed.coat_rgb[1], breed.coat_rgb[2]);
    let acc = Color32::from_rgb(
        breed.accent_rgb[0],
        breed.accent_rgb[1],
        breed.accent_rgb[2],
    );
    let eye = if is_kitten {
        // Kittens start with blue eyes before pigment settles
        Color32::from_rgb(92, 168, 238)
    } else {
        Color32::from_rgb(breed.eye_rgb[0], breed.eye_rgb[1], breed.eye_rgb[2])
    };
    let nose = Color32::from_rgb(228, 148, 158);
    let inner_ear = Color32::from_rgb(232, 174, 182);

    let cx = 64.0_f32;
    let cy = if is_kitten { 70.0_f32 } else { 66.0_f32 };
    let rx = if is_kitten { 32.0_f32 } else { 37.0_f32 };
    let ry = if is_kitten { 29.0_f32 } else { 34.0_f32 };

    for y in 0..h {
        for x in 0..w {
            let fx = x as f32;
            let fy = y as f32;
            let idx = y * w + x;

            // Subtle radial background glow
            let dx_bg = (fx - 64.0) / 64.0;
            let dy_bg = (fy - 64.0) / 64.0;
            let r_bg = (dx_bg * dx_bg + dy_bg * dy_bg).sqrt();
            if r_bg < 0.92 {
                pixels[idx] = Color32::from_rgb(48, 44, 62);
            }

            // Ears (left & right triangles)
            let ear_top = if is_kitten { 22.0 } else { 15.0 };
            let ear_base = cy - ry * 0.52;
            let in_left_ear = fy >= ear_top
                && fy <= ear_base
                && (fx - 38.0).abs() <= ((fy - ear_top) / (ear_base - ear_top)) * 16.0;
            let in_right_ear = fy >= ear_top
                && fy <= ear_base
                && (fx - 90.0).abs() <= ((fy - ear_top) / (ear_base - ear_top)) * 16.0;
            if in_left_ear || in_right_ear {
                let inner_l = fy >= ear_top + 7.0
                    && (fx - 38.0).abs() <= ((fy - ear_top - 7.0) / (ear_base - ear_top)) * 9.0;
                let inner_r = fy >= ear_top + 7.0
                    && (fx - 90.0).abs() <= ((fy - ear_top - 7.0) / (ear_base - ear_top)) * 9.0;
                pixels[idx] = if inner_l || inner_r { inner_ear } else { acc };
            }

            // Head ellipse
            let nx = (fx - cx) / rx;
            let ny = (fy - cy) / ry;
            if nx * nx + ny * ny <= 1.0 {
                // Muzzle / tabby forehead accent
                let forehead = ny < -0.25
                    && ((fx - cx).abs() < 4.0
                        || (fx - cx + 9.0).abs() < 2.5
                        || (fx - cx - 9.0).abs() < 2.5);
                pixels[idx] = if forehead { acc } else { coat };
            }

            // Eyes (bigger proportionally on kittens)
            let eye_y = if is_kitten { cy - 3.0 } else { cy - 5.0 };
            let eye_dx = if is_kitten { 14.0 } else { 15.5 };
            let eye_r = if is_kitten { 7.5 } else { 6.2 };
            for ex in [cx - eye_dx, cx + eye_dx] {
                let d2 = (fx - ex) * (fx - ex) + (fy - eye_y) * (fy - eye_y);
                if d2 <= eye_r * eye_r {
                    if d2 <= (eye_r * 0.45) * (eye_r * 0.45) {
                        pixels[idx] = Color32::from_rgb(18, 18, 22);
                    } else {
                        pixels[idx] = eye;
                    }
                }
            }

            // Nose triangle
            let nose_y = cy + 9.0;
            if fy >= nose_y && fy <= nose_y + 6.0 && (fx - cx).abs() <= (nose_y + 6.0 - fy) {
                pixels[idx] = nose;
            }
        }
    }

    ColorImage {
        size: [w, h],
        source_size: Vec2::new(w as f32, h as f32),
        pixels,
    }
}
