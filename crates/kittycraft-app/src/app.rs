use crate::chat_dock::ChatDockState;
use crate::settings_view::{ui_settings, SettingsState};
use crate::theme::{apply_theme, palette, pill_badge};
use crate::views::{
    ui_behavior, ui_breeders_and_rescue, ui_diary_and_calendar, ui_encyclopedia,
    ui_legal_and_vaccines, ui_matcher, ui_medical_and_insurance, ui_seniors, ui_tools_and_photos,
    ui_toxins, ViewsState,
};
use egui::{Key, Modifiers, RichText, ScrollArea};
use kittycraft_core::skills::skills_list;
use kittycraft_core::store::KittyStore;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveView {
    Matcher,
    Encyclopedia,
    BreedersAndRescue,
    Behavior,
    Seniors,
    MedicalAndInsurance,
    LegalAndVaccines,
    Toxins,
    DiaryAndCalendar,
    ToolsAndPhotos,
    Settings,
}

impl ActiveView {
    pub const ALL: [Self; 11] = [
        Self::Matcher,
        Self::Encyclopedia,
        Self::BreedersAndRescue,
        Self::Behavior,
        Self::Seniors,
        Self::MedicalAndInsurance,
        Self::LegalAndVaccines,
        Self::Toxins,
        Self::DiaryAndCalendar,
        Self::ToolsAndPhotos,
        Self::Settings,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Matcher => "Berater & Match",
            Self::Encyclopedia => "Enzyklopädie (20 Rassen)",
            Self::BreedersAndRescue => "Züchter & Tierschutz",
            Self::Behavior => "Verhalten & 2-Katzen-Regel",
            Self::Seniors => "Alte Katzen (Senioren)",
            Self::MedicalAndInsurance => "Medizin, Spitäler & Vers.",
            Self::LegalAndVaccines => "Recht & Impfungen",
            Self::Toxins => "Gift-Lexikon (Lilien!)",
            Self::DiaryAndCalendar => "Katzi-Tagebuch & Sitter",
            Self::ToolsAndPhotos => "Rechner & Phototool",
            Self::Settings => "LLM & Settings",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HelpTab {
    QuickStart,
    Readme,
    Skills,
    Shortcuts,
}

const ZOOM_MIN: f32 = 0.65;
const ZOOM_MAX: f32 = 2.2;

pub struct KittyCraftApp {
    pub store: Arc<KittyStore>,
    pub active_view: ActiveView,
    pub dark_mode: bool,
    pub zoom: f32,
    pub views: ViewsState,
    pub chat: ChatDockState,
    pub settings: SettingsState,
    pub show_docs_modal: bool,
    pub help_tab: HelpTab,
    pub show_about_modal: bool,
    pub show_shortcuts_modal: bool,
    pub show_palette_modal: bool,
    pub palette_query: String,
    pub selected_skill_idx: usize,
}

impl KittyCraftApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let store = match KittyStore::open_default() {
            Ok(s) => Arc::new(s),
            Err(_) =>
            {
                #[allow(clippy::unwrap_used)]
                Arc::new(KittyStore::open_in_memory().unwrap_or_else(|_| unreachable!()))
            }
        };
        let chat = ChatDockState::from_store(&store);
        Self {
            store,
            active_view: ActiveView::Matcher,
            dark_mode: true,
            zoom: 1.0,
            views: ViewsState::default(),
            chat,
            settings: SettingsState::default(),
            show_docs_modal: false,
            help_tab: HelpTab::QuickStart,
            show_about_modal: false,
            show_shortcuts_modal: false,
            show_palette_modal: false,
            palette_query: String::new(),
            selected_skill_idx: 0,
        }
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        ctx.options_mut(|o| o.zoom_with_keyboard = true);
        if (ctx.zoom_factor() - self.zoom).abs() > 1e-3 {
            ctx.set_zoom_factor(self.zoom.clamp(ZOOM_MIN, ZOOM_MAX));
        }
        let wheel = ctx.input(|i| i.zoom_delta());
        if (wheel - 1.0).abs() > f32::EPSILON {
            self.zoom = (ctx.zoom_factor() * wheel).clamp(ZOOM_MIN, ZOOM_MAX);
            ctx.set_zoom_factor(self.zoom);
        }
        if ctx.input_mut(|i| {
            i.consume_key(Modifiers::CTRL, Key::Equals) || i.consume_key(Modifiers::CTRL, Key::Plus)
        }) {
            self.zoom = (self.zoom * 1.1).clamp(ZOOM_MIN, ZOOM_MAX);
            ctx.set_zoom_factor(self.zoom);
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::Minus)) {
            self.zoom = (self.zoom / 1.1).clamp(ZOOM_MIN, ZOOM_MAX);
            ctx.set_zoom_factor(self.zoom);
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::Num0)) {
            self.zoom = 1.0;
            ctx.set_zoom_factor(1.0);
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::F1)) {
            self.show_docs_modal = !self.show_docs_modal;
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::J)) {
            self.chat.open = !self.chat.open;
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::K)) {
            self.show_palette_modal = !self.show_palette_modal;
        }
    }

    fn render_menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Neu: Tagebuch / Sitter").clicked() {
                    self.active_view = ActiveView::DiaryAndCalendar;
                    ui.close();
                }
                ui.separator();
                if ui.button("Quit").clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            ui.menu_button("Edit", |ui| {
                if ui.button("LLM & Chat-Einstellungen...").clicked() {
                    self.active_view = ActiveView::Settings;
                    ui.close();
                }
            });

            ui.menu_button("View", |ui| {
                if ui
                    .button(if self.dark_mode {
                        "Theme: Light Mode"
                    } else {
                        "Theme: Dark Mode"
                    })
                    .clicked()
                {
                    self.dark_mode = !self.dark_mode;
                    ui.close();
                }
                ui.separator();
                if ui.button("Zoom In (Ctrl+= / Ctrl+Wheel)").clicked() {
                    self.zoom = (self.zoom * 1.1).clamp(ZOOM_MIN, ZOOM_MAX);
                    ui.close();
                }
                if ui.button("Zoom Out (Ctrl+- / Ctrl+Wheel)").clicked() {
                    self.zoom = (self.zoom / 1.1).clamp(ZOOM_MIN, ZOOM_MAX);
                    ui.close();
                }
                if ui.button("Reset Zoom 100% (Ctrl+0)").clicked() {
                    self.zoom = 1.0;
                    ui.close();
                }
            });

            ui.menu_button("Katze", |ui| {
                for v in ActiveView::ALL {
                    if ui.button(v.label()).clicked() {
                        self.active_view = v;
                        ui.close();
                    }
                }
            });

            ui.menu_button("Window", |ui| {
                if ui.button("Chat (Ctrl+J)").clicked() {
                    self.chat.open = !self.chat.open;
                    ui.close();
                }
                if ui.button("Reset Layout").clicked() {
                    self.chat.open = true;
                    self.chat.maximized = false;
                    ui.close();
                }
            });

            ui.menu_button("Help", |ui| {
                if ui.button("Handbuch & Kurzanleitung (F1)...").clicked() {
                    self.help_tab = HelpTab::QuickStart;
                    self.show_docs_modal = true;
                    ui.close();
                }
                if ui.button("Project Files & Docs (Skills)...").clicked() {
                    self.help_tab = HelpTab::Skills;
                    self.show_docs_modal = true;
                    ui.close();
                }
                if ui.button("Command Palette... (Ctrl+K)").clicked() {
                    self.show_palette_modal = true;
                    ui.close();
                }
                if ui.button("Keyboard Shortcuts...").clicked() {
                    self.show_shortcuts_modal = true;
                    ui.close();
                }
                ui.separator();
                if ui.button("About KittyCraft").clicked() {
                    self.show_about_modal = true;
                    ui.close();
                }
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .small_button(if self.dark_mode { "Light" } else { "Dark" })
                    .clicked()
                {
                    self.dark_mode = !self.dark_mode;
                }
                if ui
                    .small_button(format!("{:.0}%", self.zoom * 100.0))
                    .on_hover_text("Ctrl+Wheel / Ctrl+= / Ctrl+- / Klick = Reset 100%")
                    .clicked()
                {
                    self.zoom = 1.0;
                }
                if ui
                    .small_button(if self.chat.open {
                        "Chat: AN"
                    } else {
                        "Chat: AUS"
                    })
                    .clicked()
                {
                    self.chat.open = !self.chat.open;
                }
            });
        });
    }
}

impl eframe::App for KittyCraftApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        apply_theme(&ctx, self.dark_mode);
        let pal = palette(self.dark_mode);
        self.handle_shortcuts(&ctx);
        self.chat.poll(&self.store, self.active_view.label());
        self.settings.poll(&mut self.chat, &self.store);
        if self.chat.open_settings_requested {
            self.chat.open_settings_requested = false;
            self.chat.maximized = false;
            self.active_view = ActiveView::Settings;
        }
        if self.chat.busy || self.settings.scanning || self.settings.testing {
            ctx.request_repaint();
        }

        egui::Panel::top("kitty_top_menu").show(ui, |ui| {
            self.render_menu_bar(ui);
        });

        if self.chat.open && self.chat.maximized {
            egui::CentralPanel::default().show(ui, |ui| {
                self.chat.ui(ui, &self.store, self.active_view.label());
            });
            return;
        }

        egui::Panel::left("kitty_nav_rail")
            .default_size(215.0)
            .resizable(true)
            .show(ui, |ui| {
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("KittyCraft")
                            .size(19.0)
                            .strong()
                            .color(pal.accent),
                    );
                    pill_badge(ui, "20 Rassen", pal.accent);
                });
                ui.label(
                    RichText::new("Katzen- & Kitten-Berater")
                        .small()
                        .color(pal.text_muted),
                );
                ui.separator();

                for v in ActiveView::ALL {
                    let active = self.active_view == v;
                    let resp = ui.add_sized(
                        [ui.available_width(), 30.0],
                        egui::Button::selectable(active, RichText::new(v.label()).size(13.5)),
                    );
                    if resp.clicked() {
                        self.active_view = v;
                    }
                }
                if !self.views.status_msg.is_empty() {
                    ui.separator();
                    ui.label(
                        RichText::new(&self.views.status_msg)
                            .small()
                            .color(pal.good),
                    );
                }
            });

        if self.chat.open {
            egui::Panel::right("kitty_chat_dock")
                .default_size(360.0)
                .min_size(280.0)
                .max_size(480.0)
                .resizable(true)
                .show(ui, |ui| {
                    self.chat.ui(ui, &self.store, self.active_view.label());
                });
        }

        egui::CentralPanel::default().show(ui, |ui| match self.active_view {
            ActiveView::Matcher => ui_matcher(ui, &mut self.views),
            ActiveView::Encyclopedia => ui_encyclopedia(ui, &mut self.views),
            ActiveView::BreedersAndRescue => ui_breeders_and_rescue(ui),
            ActiveView::Behavior => ui_behavior(ui),
            ActiveView::Seniors => ui_seniors(ui),
            ActiveView::MedicalAndInsurance => {
                ui_medical_and_insurance(ui, &mut self.views, &self.store);
            }
            ActiveView::LegalAndVaccines => ui_legal_and_vaccines(ui),
            ActiveView::Toxins => ui_toxins(ui, &mut self.views),
            ActiveView::DiaryAndCalendar => ui_diary_and_calendar(ui, &mut self.views, &self.store),
            ActiveView::ToolsAndPhotos => ui_tools_and_photos(ui, &mut self.views, &self.store),
            ActiveView::Settings => {
                ui_settings(ui, &mut self.settings, &mut self.chat, &self.store)
            }
        });

        if self.show_docs_modal {
            egui::Window::new("KittyCraft Handbuch, Docs & Bundled Skills")
                .open(&mut self.show_docs_modal)
                .default_width(680.0)
                .show(&ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.selectable_value(
                            &mut self.help_tab,
                            HelpTab::QuickStart,
                            "Kurzanleitung",
                        );
                        ui.selectable_value(&mut self.help_tab, HelpTab::Readme, "README");
                        ui.selectable_value(&mut self.help_tab, HelpTab::Skills, "Bundled Skills");
                        ui.selectable_value(
                            &mut self.help_tab,
                            HelpTab::Shortcuts,
                            "Tastenkürzel",
                        );
                    });
                    ui.separator();
                    ScrollArea::vertical().max_height(430.0).show(ui, |ui| {
                        match self.help_tab {
                            HelpTab::QuickStart => {
                                ui.heading("Willkommen bei KittyCraft");
                                ui.label("1. Berater & Match: Wohnung vs. Freigang, Haushalt, Allergiehinweis & Pflege -> Top-Katzenrassen.");
                                ui.label("2. Enzyklopädie: 20 Katzenrassen mit Kitten- und Erwachsenen-Porträt.");
                                ui.label("3. Züchter & Tierschutz: KKÖ/FIFe/TICA-Checks & Tierheime.");
                                ui.label("4. Verhalten & 2-Katzen-Regel: Klo-Formel (n+1), Kratzmöbel & Gittertür.");
                                ui.label("5. Medizin, Gift-Lexikon & Senioren: Lilien-Notfall, FORL, CNI, 24h-Kliniken & Versicherungen.");
                                ui.label("6. Katzi-Tagebuch, Sitter-Kalender, Rechner & Phototool.");
                                ui.label("7. AI Katzi-Berater Chat (Ctrl+J): Fragen direkt mit gebündelten Skills klären.");
                            }
                            HelpTab::Readme => {
                                ui.monospace(include_str!("../../../README.md"));
                            }
                            HelpTab::Skills => {
                                let skills = skills_list();
                                ui.horizontal(|ui| {
                                    for (idx, s) in skills.iter().enumerate() {
                                        if ui
                                            .selectable_label(self.selected_skill_idx == idx, s.name)
                                            .clicked()
                                        {
                                            self.selected_skill_idx = idx;
                                        }
                                    }
                                });
                                ui.separator();
                                if let Some(s) = skills.get(self.selected_skill_idx) {
                                    ui.monospace(s.skill_md);
                                }
                            }
                            HelpTab::Shortcuts => {
                                ui.label("F1 — Handbuch & Kurzanleitung öffnen/schließen");
                                ui.label("Ctrl+J — Katzi-Berater Chat Dock ein-/ausblenden");
                                ui.label("Ctrl+K — Command Palette");
                                ui.label("Ctrl+Mausrad / Ctrl+= / Ctrl+- — UI Zoom rein/raus");
                                ui.label("Ctrl+0 — Zoom auf 100% zurücksetzen");
                            }
                        }
                    });
                });
        }

        if self.show_shortcuts_modal {
            egui::Window::new("Keyboard Shortcuts")
                .open(&mut self.show_shortcuts_modal)
                .show(&ctx, |ui| {
                    ui.label("F1 — Handbuch & Kurzanleitung");
                    ui.label("Ctrl+J — Katzi-Berater Chat Dock");
                    ui.label("Ctrl+K — Command Palette");
                    ui.label("Ctrl+Wheel / Ctrl+= / Ctrl+- — Zoom In / Out");
                    ui.label("Ctrl+0 — Reset Zoom (100%)");
                });
        }

        if self.show_about_modal {
            egui::Window::new("About KittyCraft")
                .open(&mut self.show_about_modal)
                .show(&ctx, |ui| {
                    ui.heading(format!("KittyCraft v{}", env!("CARGO_PKG_VERSION")));
                    ui.label("Pure-Rust ArtCraft Workstation für Katzen- & Kitten-Beratung, Enzyklopädie (Kitten + Erwachsen), Gift-Lexikon, Recht/Impfungen, Senioren, Katzi-Tagebuch, Sitter-Kalender & Phototool.");
                    ui.label("License: MIT OR Apache-2.0");
                });
        }

        if self.show_palette_modal {
            let mut open = true;
            let mut close_palette = false;
            egui::Window::new("Command Palette")
                .open(&mut open)
                .show(&ctx, |ui| {
                    ui.text_edit_singleline(&mut self.palette_query);
                    let q = self.palette_query.to_lowercase();
                    for v in ActiveView::ALL {
                        if (q.is_empty() || v.label().to_lowercase().contains(&q))
                            && ui.button(v.label()).clicked()
                        {
                            self.active_view = v;
                            close_palette = true;
                        }
                    }
                });
            if close_palette || !open {
                self.show_palette_modal = false;
            }
        }
    }
}
