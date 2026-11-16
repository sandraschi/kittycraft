use crate::theme::{build_cat_portrait, card_frame, hero_header, palette, pill_badge};
use egui::{ColorImage, RichText, ScrollArea, TextureHandle, TextureOptions, Ui};
use kittycraft_core::catalog::{
    cat_human_age, check_toxin, food_ration_grams, match_breeds, CatMatchQuery, CAT_BREEDS,
    CAT_INSURANCES, VET_HOSPITALS,
};
use kittycraft_core::store::KittyStore;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

fn load_breed_photo(breed_id: &str, suffix: &str) -> Option<ColorImage> {
    let fname = format!("{breed_id}_{suffix}.jpg");
    let mut candidates = vec![
        PathBuf::from("assets/breeds").join(&fname),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../assets/breeds")
            .join(&fname),
    ];
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join("assets/breeds").join(&fname));
        }
    }
    for path in candidates {
        if let Ok(img) = image::open(&path) {
            let rgba = img.to_rgba8();
            let size = [rgba.width() as usize, rgba.height() as usize];
            return Some(ColorImage::from_rgba_unmultiplied(size, rgba.as_raw()));
        }
    }
    None
}

pub struct ViewsState {
    pub match_q: CatMatchQuery,
    pub enc_search: String,
    pub enc_selected_idx: usize,
    pub portrait_cache: HashMap<String, (TextureHandle, TextureHandle)>,
    pub tox_query: String,
    pub food_weight_kg: f64,
    pub food_is_kitten: bool,
    pub food_activity: String,
    pub age_years: f64,
    pub num_cats_for_litter: u32,
    pub diary_date: String,
    pub diary_cat: String,
    pub diary_title: String,
    pub diary_notes: String,
    pub diary_weight: String,
    pub diary_mood: String,
    pub cal_start: String,
    pub cal_end: String,
    pub cal_sitter: String,
    pub cal_rel: String,
    pub cal_notes: String,
    pub vet_date: String,
    pub vet_clinic: String,
    pub vet_reason: String,
    pub vet_diag: String,
    pub vet_cost: String,
    pub photo_cat: String,
    pub photo_path: String,
    pub photo_preset: String,
    pub photo_caption: String,
    pub status_msg: String,
}

impl Default for ViewsState {
    fn default() -> Self {
        Self {
            match_q: CatMatchQuery {
                housing: "wohnung".into(),
                household: "single".into(),
                allergies: false,
                max_grooming: 3,
            },
            enc_search: String::new(),
            enc_selected_idx: 0,
            portrait_cache: HashMap::new(),
            tox_query: String::new(),
            food_weight_kg: 4.5,
            food_is_kitten: false,
            food_activity: "normal".into(),
            age_years: 6.0,
            num_cats_for_litter: 2,
            diary_date: "2026-11-15".into(),
            diary_cat: "Alltag".into(),
            diary_title: String::new(),
            diary_notes: String::new(),
            diary_weight: "4.5".into(),
            diary_mood: "Verspielt".into(),
            cal_start: "2026-12-20".into(),
            cal_end: "2026-12-23".into(),
            cal_sitter: String::new(),
            cal_rel: "Nachbar/Familie".into(),
            cal_notes: "2x täglich Nassfutter, Klo reinigen, Spielen".into(),
            vet_date: "2026-11-15".into(),
            vet_clinic: "Vetmeduni Wien".into(),
            vet_reason: "Jahrescheck & Impfung".into(),
            vet_diag: "Alles unauffällig".into(),
            vet_cost: "85".into(),
            photo_cat: "Mimi".into(),
            photo_path: String::new(),
            photo_preset: "Sonnenfenster Warm".into(),
            photo_caption: String::new(),
            status_msg: String::new(),
        }
    }
}

pub fn ui_matcher(ui: &mut Ui, st: &mut ViewsState) {
    let dark = ui.visuals().dark_mode;
    let pal = palette(dark);
    hero_header(
        ui,
        dark,
        "Katzen-Berater & Match",
        "Wohnung, Haushalt, Allergiehinweis & Pflegeaufwand -> passende Rassen.",
    );

    card_frame(ui, dark).show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label("Haltung:");
            ui.selectable_value(&mut st.match_q.housing, "wohnung".into(), "Wohnung");
            ui.selectable_value(
                &mut st.match_q.housing,
                "freigang".into(),
                "Freigang / Garten",
            );
            ui.separator();
            ui.label("Haushalt:");
            ui.selectable_value(&mut st.match_q.household, "single".into(), "Single / Paar");
            ui.selectable_value(&mut st.match_q.household, "senioren".into(), "Senioren");
            ui.selectable_value(
                &mut st.match_q.household,
                "familie_kinder".into(),
                "Familie mit Kindern",
            );
            ui.selectable_value(&mut st.match_q.household, "hund".into(), "Mit Hund");
        });
        ui.horizontal(|ui| {
            ui.checkbox(&mut st.match_q.allergies, "Leichte Katzenhaar-Allergie");
            ui.separator();
            ui.label("Max. Fellpflege (1-5):");
            ui.add(egui::Slider::new(&mut st.match_q.max_grooming, 1..=5));
        });
    });

    ui.add_space(6.0);
    let results = match_breeds(&st.match_q);
    ScrollArea::vertical().show(ui, |ui| {
        for r in results.into_iter().take(8) {
            card_frame(ui, dark).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(r.breed_name).size(16.0).strong());
                    pill_badge(ui, &format!("Score {}", r.score), pal.accent);
                });
                for ok in &r.reasons {
                    ui.label(RichText::new(format!("+ {ok}")).color(pal.good));
                }
                for w in &r.warnings {
                    ui.label(RichText::new(format!("! {w}")).color(pal.warn));
                }
            });
            ui.add_space(4.0);
        }
    });
}

pub fn ui_encyclopedia(ui: &mut Ui, st: &mut ViewsState) {
    let dark = ui.visuals().dark_mode;
    let pal = palette(dark);
    hero_header(
        ui,
        dark,
        "Katzen-Enzyklopädie (20 Rassen)",
        "Kitten- & Erwachsenen-Porträt, Charakter, Fellpflege & Erbkrankheiten.",
    );

    ui.horizontal(|ui| {
        ui.label("Suche:");
        ui.text_edit_singleline(&mut st.enc_search);
    });
    ui.add_space(4.0);

    let q = st.enc_search.trim().to_lowercase();
    let filtered: Vec<(usize, &_)> = CAT_BREEDS
        .iter()
        .enumerate()
        .filter(|(_, b)| {
            q.is_empty()
                || b.name.to_lowercase().contains(&q)
                || b.temperament.to_lowercase().contains(&q)
                || b.coat.to_lowercase().contains(&q)
        })
        .collect();

    ui.columns(2, |cols| {
        ScrollArea::vertical()
            .id_salt("cat_enc_list")
            .show(&mut cols[0], |ui| {
                for (idx, b) in &filtered {
                    let selected = st.enc_selected_idx == *idx;
                    if ui.selectable_label(selected, b.name).clicked() {
                        st.enc_selected_idx = *idx;
                    }
                }
            });

        if let Some(b) = CAT_BREEDS.get(st.enc_selected_idx) {
            let entry = st
                .portrait_cache
                .entry(b.id.to_string())
                .or_insert_with(|| {
                    let kit = load_breed_photo(b.id, "kitten")
                        .unwrap_or_else(|| build_cat_portrait(b, true));
                    let adu = load_breed_photo(b.id, "adult")
                        .unwrap_or_else(|| build_cat_portrait(b, false));
                    let tk = cols[1].ctx().load_texture(
                        format!("kitty_{}_k", b.id),
                        kit,
                        TextureOptions::LINEAR,
                    );
                    let ta = cols[1].ctx().load_texture(
                        format!("kitty_{}_a", b.id),
                        adu,
                        TextureOptions::LINEAR,
                    );
                    (tk, ta)
                });

            card_frame(&cols[1], dark).show(&mut cols[1], |ui| {
                ui.heading(b.name);
                ui.label(
                    RichText::new(format!(
                        "Herkunft: {} | Gewicht: {} | Lebenserwartung: {} J.",
                        b.origin, b.weight_kg, b.lifespan_years
                    ))
                    .color(pal.text_muted),
                );
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.image((entry.0.id(), egui::vec2(128.0, 128.0)));
                        ui.label(RichText::new("Kitten (blaue Augen)").small().weak());
                    });
                    ui.add_space(12.0);
                    ui.vertical(|ui| {
                        ui.image((entry.1.id(), egui::vec2(128.0, 128.0)));
                        ui.label(RichText::new("Erwachsen").small().weak());
                    });
                });
                ui.add_space(6.0);
                ui.label(format!("Fell: {} (Pflege {}/5)", b.coat, b.grooming_level));
                ui.label(format!(
                    "Energie: {}/5 | Stimmfreude: {}/5",
                    b.energy_level, b.vocal_level
                ));
                ui.label(format!("Wesen: {}", b.temperament));
                ui.label(RichText::new(format!("Gesundheit: {}", b.health_notes)).color(pal.warn));
            });
        }
    });
}

pub fn ui_breeders_and_rescue(ui: &mut Ui) {
    let dark = ui.visuals().dark_mode;
    hero_header(
        ui,
        dark,
        "Züchter & Katzenschutz / Tierheim",
        "Seriöse Vereine (FIFe, TICA, KKÖ), Tierheime & Red-Flags gegen Vermehrer.",
    );
    ScrollArea::vertical().show(ui, |ui| {
        card_frame(ui, dark).show(ui, |ui| {
            ui.heading("Checkliste seriöse Zucht");
            ui.label("- Vereinszugehörigkeit (KKÖ / ÖVEK / FIFe / DEKZV / TICA) mit Stammbaum.");
            ui.label("- Abgabe NICHT vor der 12.-14. Lebenswoche (voll geimpft, entwurmt, sozialisiert).");
            ui.label("- Eltern nachweislich auf HCM (Herzultraschall) & PKD (Nieren) getestet.");
            ui.label("- Keine Abgabe von Wohnungs-Kitten in Einzelhaltung.");
        });
        ui.add_space(6.0);
        card_frame(ui, dark).show(ui, |ui| {
            ui.heading("Tierschutz & Tierheime (Auswahl Österreich/DACH)");
            ui.label("- TierQuarTier Wien (1220): Moderne Katzenabteilung, klare Vermittlungschecks.");
            ui.label("- Wiener Tierschutzverein (Tierschutzhaus Vösendorf): Kitten, Paar-Katzen & Senioren.");
            ui.label("- Pfotenhilfe Lochen / Arche Noah Graz / Katzenhilfe lokal.");
        });
    });
}

pub fn ui_behavior(ui: &mut Ui) {
    let dark = ui.visuals().dark_mode;
    hero_header(
        ui,
        dark,
        "Verhalten, Erziehung & 2-Katzen-Regel",
        "Stubenreinheit, Kratzmöbel, Gittertür-Vergesellschaftung & Clickertraining.",
    );
    ScrollArea::vertical().show(ui, |ui| {
        card_frame(ui, dark).show(ui, |ui| {
            ui.heading("Die goldenen Regeln");
            ui.label("- 2-Katzen-Regel: Kätzchen und junge Wohnungskatzen brauchen kätzische Gesellschaft (Ausnahme: ausdrückliche erwachsene Einzelkatzen aus dem Tierschutz).");
            ui.label("- Katzenklo-Formel (n + 1): Anzahl Katzen + 1 Klos, offen, feines Klumpstreu, räumlich getrennt von Futter & Wasser.");
            ui.label("- Kratzen umlenken: Stabile hohe Kratzstämme an Laufwegen (nicht im Eck versteckt).");
            ui.label("- Langsame Vergesellschaftung: Neue Katze zuerst in Ankunftszimmer, Geruchstausch, dann Gittertür mit Leckerlis.");
        });
    });
}

pub fn ui_seniors(ui: &mut Ui) {
    let dark = ui.visuals().dark_mode;
    hero_header(
        ui,
        dark,
        "Alte Katzen (Senioren 11+)",
        "Nierendiät (CNI), Arthrose-Hilfen, Hyperthyreose & Demenz-Nachtlicht.",
    );
    ScrollArea::vertical().show(ui, |ui| {
        card_frame(ui, dark).show(ui, |ui| {
            ui.heading("Senioren-Vorsorge ab 10 Jahren");
            ui.label("- Jährliches Geriatrie-Blutbild inkl. SDMA (Niere), T4 (Schilddrüse) & Blutdruck.");
            ui.label("- Arthrose erkennen: Katze springt nicht mehr auf die Fensterbank -> Aufstiegshilfen / Rampen, flacher Klo-Einstieg, Wärmekissen.");
            ui.label("- CNI-Management: Hochwertiges Nierenfutter / Phosphatbinder, mehrere Trinkbrunnen.");
            ui.label("- Nächtliches Miauen (kognitive Dysfunktion): Kleines Nachtlicht, feste Rituale.");
        });
    });
}

pub fn ui_medical_and_insurance(ui: &mut Ui, st: &mut ViewsState, store: &Arc<KittyStore>) {
    let dark = ui.visuals().dark_mode;
    let pal = palette(dark);
    hero_header(
        ui,
        dark,
        "Medizin, Tierspitäler & Versicherung",
        "Typische Katzenkrankheiten, 24h Notfallkliniken, Versicherungen & Tierarzt-Log.",
    );
    ScrollArea::vertical().show(ui, |ui| {
        card_frame(ui, dark).show(ui, |ui| {
            ui.heading("Typische Katzenkrankheiten");
            ui.label("- FORL (Zahnresorption): Sehr schmerzhaft, äußerlich oft unsichtbar -> Dentalröntgen nötig!");
            ui.label("- CNI (Chronische Niereninsuffizienz): Viel trinken/urinieren, Gewichtsverlust.");
            ui.label("- FLUTD / Harnröhrenverschluss (Kater-Notfall!): Häufiges erfolgloses Pressen am Klo -> SOFORT Notdienst!");
            ui.label("- HCM (Hypertrophe Kardiomyopathie): Herzwandverdickung, besonders bei BKH, Maine Coon, Ragdoll.");
        });
        ui.add_space(6.0);
        card_frame(ui, dark).show(ui, |ui| {
            ui.heading("Notfall-Kliniken & Überweisungsspitäler");
            for h in VET_HOSPITALS {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(h.name).strong());
                    ui.label(format!("({})", h.city));
                    if h.emergency_24h {
                        pill_badge(ui, "24h NOTFALL", pal.danger);
                    }
                });
                ui.label(RichText::new(format!("  {} — {}", h.kind, h.specialty)).color(pal.text_muted));
            }
        });
        ui.add_space(6.0);
        card_frame(ui, dark).show(ui, |ui| {
            ui.heading("Katzen-Versicherungen (OP & Voll)");
            for ins in CAT_INSURANCES {
                ui.label(RichText::new(format!("{} | {} | {}", ins.provider, ins.plan_type, ins.approx_monthly_eur)).strong());
                ui.label(RichText::new(format!("  {}", ins.highlights)).color(pal.text_muted));
            }
        });
        ui.add_space(6.0);
        card_frame(ui, dark).show(ui, |ui| {
            ui.heading("Tierarzt-Besuch dokumentieren");
            ui.horizontal(|ui| {
                ui.label("Datum:");
                ui.text_edit_singleline(&mut st.vet_date);
                ui.label("Klinik:");
                ui.text_edit_singleline(&mut st.vet_clinic);
            });
            ui.horizontal(|ui| {
                ui.label("Grund:");
                ui.text_edit_singleline(&mut st.vet_reason);
                ui.label("Befund:");
                ui.text_edit_singleline(&mut st.vet_diag);
                ui.label("EUR:");
                ui.text_edit_singleline(&mut st.vet_cost);
            });
            if ui.button("Tierarzt-Besuch speichern").clicked() {
                let cost = st.vet_cost.trim().replace(',', ".").parse::<f64>().ok();
                if store
                    .add_vet_visit(&st.vet_date, &st.vet_clinic, &st.vet_reason, &st.vet_diag, cost)
                    .is_ok()
                {
                    st.status_msg = "Tierarzt-Besuch gespeichert.".into();
                }
            }
            if let Ok(list) = store.list_vet_visits() {
                for v in list.into_iter().take(5) {
                    ui.label(format!("{} | {} | {} — {} ({:.2} EUR)", v.date, v.clinic, v.reason, v.diagnosis, v.cost_eur.unwrap_or(0.0)));
                }
            }
        });
    });
}

pub fn ui_legal_and_vaccines(ui: &mut Ui) {
    let dark = ui.visuals().dark_mode;
    hero_header(
        ui,
        dark,
        "Recht, Kastration & Impfplan",
        "Österreichisches Tierschutzgesetz (TSchG), Balkonsicherung & Impfungen.",
    );
    ScrollArea::vertical().show(ui, |ui| {
        card_frame(ui, dark).show(ui, |ui| {
            ui.heading("Rechtliches (Österreich / DACH)");
            ui.label("- Kastrationspflicht (AT): Alle Katzen mit regelmäßigem Zugang ins Freie müssen kastriert werden (Ausnahme bäuerliche Zucht nur unter strengen Auflagen).");
            ui.label("- Balkon- & Fenstersicherung: Kippfensterschutz & stabiles Katzennetz verhindern tödliche Stürze und Einklemmen.");
            ui.label("- Mikrochip & Heimtierausweis: Für EU-Reisen zwingend + Tollwutimpfung (mind. 21 Tage vor Abreise).");
        });
        ui.add_space(6.0);
        card_frame(ui, dark).show(ui, |ui| {
            ui.heading("Impfplan (Katze)");
            ui.label("- 8. Woche: Katzenschnupfen & Katzenseuche (RCP)");
            ui.label("- 12. Woche: Auffrischung RCP (+ bei Freigang FeLV / Leukose)");
            ui.label("- 16. Woche / 15 Monate: Grundimmunisierung abschließen; Tollwut bei Freigang/Reise.");
        });
    });
}

pub fn ui_toxins(ui: &mut Ui, st: &mut ViewsState) {
    let dark = ui.visuals().dark_mode;
    let pal = palette(dark);
    hero_header(
        ui,
        dark,
        "Gift-Lexikon & Notfall-Check",
        "Lilien, Paracetamol, Teebaumöl, Pflanzen & Lebensmittel sofort prüfen.",
    );
    ui.horizontal(|ui| {
        ui.label("Substanz / Pflanze:");
        ui.text_edit_singleline(&mut st.tox_query);
    });
    ui.add_space(6.0);
    let hits = check_toxin(&st.tox_query);
    ScrollArea::vertical().show(ui, |ui| {
        for t in hits {
            card_frame(ui, dark).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(t.name).size(15.0).strong());
                    pill_badge(ui, t.severity, pal.danger);
                });
                ui.label(format!("Symptome: {}", t.symptoms));
                ui.label(RichText::new(format!("Sofortmaßnahme: {}", t.action)).color(pal.warn));
            });
            ui.add_space(4.0);
        }
    });
}

pub fn ui_diary_and_calendar(ui: &mut Ui, st: &mut ViewsState, store: &Arc<KittyStore>) {
    let dark = ui.visuals().dark_mode;
    hero_header(
        ui,
        dark,
        "Katzi-Tagebuch & Sitter-Kalender",
        "Gewicht, Stimmung, Pflege & Katzensitter-Betreuung lokal in SQLite speichern.",
    );
    ui.columns(2, |cols| {
        card_frame(&cols[0], dark).show(&mut cols[0], |ui| {
            ui.heading("Tagebuch-Eintrag");
            ui.horizontal(|ui| {
                ui.label("Datum:");
                ui.text_edit_singleline(&mut st.diary_date);
                ui.label("Gewicht (kg):");
                ui.text_edit_singleline(&mut st.diary_weight);
            });
            ui.horizontal(|ui| {
                ui.label("Titel:");
                ui.text_edit_singleline(&mut st.diary_title);
                ui.label("Stimmung:");
                ui.text_edit_singleline(&mut st.diary_mood);
            });
            ui.label("Notizen:");
            ui.text_edit_multiline(&mut st.diary_notes);
            if ui.button("Eintrag speichern").clicked() && !st.diary_title.trim().is_empty() {
                let w = st.diary_weight.trim().replace(',', ".").parse::<f64>().ok();
                if store
                    .add_diary(
                        &st.diary_date,
                        &st.diary_cat,
                        &st.diary_title,
                        &st.diary_notes,
                        w,
                        &st.diary_mood,
                    )
                    .is_ok()
                {
                    st.diary_title.clear();
                    st.diary_notes.clear();
                    st.status_msg = "Tagebuch-Eintrag gespeichert.".into();
                }
            }
            ui.separator();
            if let Ok(entries) = store.list_diary(15) {
                for e in entries {
                    ui.label(format!(
                        "{} | {} ({}) — {} kg",
                        e.date,
                        e.title,
                        e.mood,
                        e.weight_kg.map_or("-".into(), |w| format!("{w:.2}"))
                    ));
                }
            }
        });

        card_frame(&cols[1], dark).show(&mut cols[1], |ui| {
            ui.heading("Katzensitter-Kalender");
            ui.horizontal(|ui| {
                ui.label("Von:");
                ui.text_edit_singleline(&mut st.cal_start);
                ui.label("Bis:");
                ui.text_edit_singleline(&mut st.cal_end);
            });
            ui.horizontal(|ui| {
                ui.label("Sitter:");
                ui.text_edit_singleline(&mut st.cal_sitter);
                ui.label("Beziehung:");
                ui.text_edit_singleline(&mut st.cal_rel);
            });
            ui.label("Fütterung & Hinweise:");
            ui.text_edit_multiline(&mut st.cal_notes);
            if ui.button("Sitter-Termin speichern").clicked()
                && !st.cal_sitter.trim().is_empty()
                && store
                    .add_sitter_event(
                        &st.cal_start,
                        &st.cal_end,
                        &st.cal_sitter,
                        &st.cal_rel,
                        &st.cal_notes,
                        "Geplant",
                    )
                    .is_ok()
            {
                st.cal_sitter.clear();
                st.status_msg = "Sitter-Termin gespeichert.".into();
            }
            ui.separator();
            if let Ok(events) = store.list_sitter_events() {
                for ev in events {
                    ui.label(format!(
                        "{}..{} | {} ({}) — {}",
                        ev.start_date,
                        ev.end_date,
                        ev.sitter_name,
                        ev.sitter_relation,
                        ev.care_notes
                    ));
                }
            }
        });
    });
}

pub fn ui_tools_and_photos(ui: &mut Ui, st: &mut ViewsState, store: &Arc<KittyStore>) {
    let dark = ui.visuals().dark_mode;
    hero_header(
        ui,
        dark,
        "Rechner & Katzi-Phototool",
        "Futterration, Katzenklo-Formel (n+1), Menschenjahre & Foto-Archiv.",
    );
    ui.columns(2, |cols| {
        card_frame(&cols[0], dark).show(&mut cols[0], |ui| {
            ui.heading("Futter-, Klo- & Altersrechner");
            ui.horizontal(|ui| {
                ui.label("Gewicht (kg):");
                ui.add(egui::Slider::new(&mut st.food_weight_kg, 0.5..=12.0));
                ui.checkbox(&mut st.food_is_kitten, "Kitten");
            });
            ui.horizontal(|ui| {
                ui.label("Aktivität:");
                ui.selectable_value(&mut st.food_activity, "ruhig".into(), "Ruhig");
                ui.selectable_value(&mut st.food_activity, "normal".into(), "Normal");
                ui.selectable_value(&mut st.food_activity, "aktiv".into(), "Aktiv");
            });
            let (kcal, wet, dry) =
                food_ration_grams(st.food_weight_kg, st.food_is_kitten, &st.food_activity);
            ui.label(
                RichText::new(format!(
                    "Bedarf: ~{kcal:.0} kcal/Tag -> {wet:.0} g Nassfutter ODER {dry:.0} g Trockenfutter"
                ))
                .strong(),
            );
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Katzenjahre:");
                ui.add(egui::Slider::new(&mut st.age_years, 0.5..=22.0));
            });
            ui.label(format!(
                "Entspricht ca. {:.0} Menschenjahren",
                cat_human_age(st.age_years)
            ));
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Anzahl Katzen:");
                ui.add(egui::Slider::new(&mut st.num_cats_for_litter, 1..=8));
            });
            ui.label(format!(
                "Empfohlene Katzenklos (n + 1): {} Klos",
                st.num_cats_for_litter + 1
            ));
        });

        card_frame(&cols[1], dark).show(&mut cols[1], |ui| {
            ui.heading("Katzi-Phototool");
            ui.horizontal(|ui| {
                ui.label("Katzenname:");
                ui.text_edit_singleline(&mut st.photo_cat);
            });
            ui.horizontal(|ui| {
                ui.label("Preset:");
                ui.selectable_value(&mut st.photo_preset, "Sonnenfenster Warm".into(), "Sonnenfenster");
                ui.selectable_value(&mut st.photo_preset, "Augen-Fokus".into(), "Augen-Fokus");
                ui.selectable_value(&mut st.photo_preset, "Kitten-Pastell".into(), "Kitten-Pastell");
            });
            ui.horizontal(|ui| {
                ui.label("Bildpfad:");
                ui.text_edit_singleline(&mut st.photo_path);
                if ui.button("Datei...").clicked() {
                    if let Some(p) = rfd::FileDialog::new()
                        .add_filter("Bilder", &["png", "jpg", "jpeg", "webp"])
                        .pick_file()
                    {
                        st.photo_path = p.display().to_string();
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.label("Caption:");
                ui.text_edit_singleline(&mut st.photo_caption);
            });
            if ui.button("Foto-Metadaten speichern").clicked()
                && !st.photo_path.trim().is_empty()
                && store
                    .add_photo(&st.photo_cat, &st.photo_path, &st.photo_preset, &st.photo_caption)
                    .is_ok()
            {
                st.photo_caption.clear();
                st.status_msg = "Foto gespeichert.".into();
            }
            ui.separator();
            if let Ok(photos) = store.list_photos() {
                for p in photos.into_iter().take(6) {
                    ui.label(format!("{} | {} [{}] — {}", p.created_at, p.cat_name, p.preset, p.caption));
                }
            }
        });
    });
}
