use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct CatBreed {
    pub id: &'static str,
    pub name: &'static str,
    pub origin: &'static str,
    pub weight_kg: &'static str,
    pub coat: &'static str,
    pub grooming_level: u8,
    pub energy_level: u8,
    pub vocal_level: u8,
    pub indoor_ok: bool,
    pub kid_friendly: bool,
    pub dog_friendly: bool,
    pub senior_friendly: bool,
    pub hypoallergenic_hint: bool,
    pub temperament: &'static str,
    pub health_notes: &'static str,
    pub lifespan_years: &'static str,
    pub coat_rgb: [u8; 3],
    pub accent_rgb: [u8; 3],
    pub eye_rgb: [u8; 3],
    pub ear_style: &'static str,
}

pub const CAT_BREEDS: &[CatBreed] = &[
    CatBreed {
        id: "ekh",
        name: "Europäisch Kurzhaar (Hauskatze)",
        origin: "Europa / Österreich",
        weight_kg: "3.5 - 6.0 kg",
        coat: "Kurzhaar (Tiger, Schwarz-Weiß, Glückskatze, Rot)",
        grooming_level: 1,
        energy_level: 3,
        vocal_level: 3,
        indoor_ok: true,
        kid_friendly: true,
        dog_friendly: true,
        senior_friendly: true,
        hypoallergenic_hint: false,
        temperament: "Robust, anpassungsfähig, neugierig, sehr unterschiedliche Charaktere.",
        health_notes: "Sehr robust (genetische Vielfalt); im Alter FORL und CNI beachten.",
        lifespan_years: "14 - 20",
        coat_rgb: [158, 132, 108],
        accent_rgb: [88, 70, 56],
        eye_rgb: [110, 185, 95],
        ear_style: "pointed",
    },
    CatBreed {
        id: "bkh",
        name: "Britisch Kurzhaar (BKH)",
        origin: "Großbritannien",
        weight_kg: "4.0 - 7.5 kg",
        coat: "Dichtes Plüsch-Kurzhaar (Blue, Lilac, Silver Tabby)",
        grooming_level: 2,
        energy_level: 2,
        vocal_level: 1,
        indoor_ok: true,
        kid_friendly: true,
        dog_friendly: true,
        senior_friendly: true,
        hypoallergenic_hint: false,
        temperament: "Ruhig, gelassen, freundlich, nicht aufdringlich - ideal für Wohnungen.",
        health_notes: "HCM-Schall & PKD-Test bei Zucht; neigt zu Übergewicht.",
        lifespan_years: "13 - 17",
        coat_rgb: [122, 136, 152],
        accent_rgb: [92, 104, 120],
        eye_rgb: [224, 158, 48],
        ear_style: "rounded",
    },
    CatBreed {
        id: "ragdoll",
        name: "Ragdoll",
        origin: "USA",
        weight_kg: "4.5 - 9.0 kg",
        coat: "Seidiges Halblanghaar (Colorpoint, Bicolour, Mitted)",
        grooming_level: 3,
        energy_level: 2,
        vocal_level: 2,
        indoor_ok: true,
        kid_friendly: true,
        dog_friendly: true,
        senior_friendly: true,
        hypoallergenic_hint: false,
        temperament: "Sanftmütig, extrem menschenbezogen, entspannt beim Hochheben.",
        health_notes: "HCM-Gentest & Herzschall wichtig; reine Wohnungshaltung empfohlen.",
        lifespan_years: "12 - 17",
        coat_rgb: [236, 228, 218],
        accent_rgb: [110, 92, 82],
        eye_rgb: [82, 158, 232],
        ear_style: "pointed",
    },
    CatBreed {
        id: "maine_coon",
        name: "Maine Coon",
        origin: "USA",
        weight_kg: "5.0 - 10.0 kg",
        coat: "Langhaar mit Halskrause & Luchspinseln",
        grooming_level: 4,
        energy_level: 3,
        vocal_level: 3,
        indoor_ok: true,
        kid_friendly: true,
        dog_friendly: true,
        senior_friendly: false,
        hypoallergenic_hint: false,
        temperament: "Verspielt, sanfter Riese, gurrt oft statt zu miauen, apportiert gern.",
        health_notes: "HCM, Hüftdysplasie (HD), SMA & PKD; braucht viel Platz & große Klos.",
        lifespan_years: "12 - 15",
        coat_rgb: [132, 102, 78],
        accent_rgb: [72, 56, 44],
        eye_rgb: [195, 165, 72],
        ear_style: "tufted",
    },
    CatBreed {
        id: "heilige_birma",
        name: "Heilige Birma",
        origin: "Frankreich / Myanmar",
        weight_kg: "3.5 - 6.0 kg",
        coat: "Halblanghaar ohne dichte Unterwolle, weiße Pfoten",
        grooming_level: 3,
        energy_level: 2,
        vocal_level: 2,
        indoor_ok: true,
        kid_friendly: true,
        dog_friendly: true,
        senior_friendly: true,
        hypoallergenic_hint: false,
        temperament: "Ausgeglichen, anhänglich, sozial - verträgt kaum Einzelhaltung.",
        health_notes: "Nierenwerte (PKD/CNI) & HCM regelmäßig kontrollieren.",
        lifespan_years: "13 - 17",
        coat_rgb: [242, 234, 220],
        accent_rgb: [125, 98, 78],
        eye_rgb: [76, 148, 230],
        ear_style: "pointed",
    },
    CatBreed {
        id: "russisch_blau",
        name: "Russisch Blau",
        origin: "Russland",
        weight_kg: "3.0 - 5.5 kg",
        coat: "Silbrig schimmerndes Doppelfell (Blaugrau)",
        grooming_level: 1,
        energy_level: 3,
        vocal_level: 2,
        indoor_ok: true,
        kid_friendly: true,
        dog_friendly: false,
        senior_friendly: true,
        hypoallergenic_hint: true,
        temperament: "Intelligent, leise, gegenüber Fremden scheu, bindet sich stark.",
        health_notes: "Generell sehr gesund; Fel-d1 Allergengehalt oft etwas niedriger.",
        lifespan_years: "15 - 20",
        coat_rgb: [128, 144, 162],
        accent_rgb: [168, 182, 198],
        eye_rgb: [76, 198, 112],
        ear_style: "pointed",
    },
    CatBreed {
        id: "sibirische_katze",
        name: "Sibirische Katze",
        origin: "Russland",
        weight_kg: "4.0 - 8.0 kg",
        coat: "Wetterfestes Halblanghaar mit dichter Unterwolle",
        grooming_level: 4,
        energy_level: 4,
        vocal_level: 3,
        indoor_ok: true,
        kid_friendly: true,
        dog_friendly: true,
        senior_friendly: false,
        hypoallergenic_hint: true,
        temperament: "Kletterfreudig, wasserliebend, sozial, verspielt bis ins Alter.",
        health_notes: "HCM & PKD testen lassen; oft verträglicher bei leichter Allergie.",
        lifespan_years: "13 - 18",
        coat_rgb: [152, 126, 98],
        accent_rgb: [228, 218, 206],
        eye_rgb: [118, 188, 92],
        ear_style: "tufted",
    },
    CatBreed {
        id: "norwegische_waldkatze",
        name: "Norwegische Waldkatze",
        origin: "Norwegen",
        weight_kg: "4.5 - 8.5 kg",
        coat: "Langhaar mit Kragen und Schneeschuh-Pfoten",
        grooming_level: 4,
        energy_level: 4,
        vocal_level: 2,
        indoor_ok: true,
        kid_friendly: true,
        dog_friendly: true,
        senior_friendly: false,
        hypoallergenic_hint: false,
        temperament: "Freundlich, klettert hervorragend, liebt gesicherten Balkon/Garten.",
        health_notes: "GSD-IV Gentest, HCM & PK-Defizienz beim Züchter abfragen.",
        lifespan_years: "12 - 16",
        coat_rgb: [118, 112, 108],
        accent_rgb: [232, 228, 224],
        eye_rgb: [188, 178, 78],
        ear_style: "tufted",
    },
    CatBreed {
        id: "bengal",
        name: "Bengal",
        origin: "USA",
        weight_kg: "4.0 - 7.0 kg",
        coat: "Glänzendes Rosetten- oder Marmor-Kurzhaar",
        grooming_level: 1,
        energy_level: 5,
        vocal_level: 5,
        indoor_ok: false,
        kid_friendly: true,
        dog_friendly: true,
        senior_friendly: false,
        hypoallergenic_hint: true,
        temperament: "Hochaktiv, sehr intelligent, fordert Beschäftigung & Kletterwände.",
        health_notes: "PRA-b, PK-Defizienz & HCM testen; nicht für ruhige Kleinstwohnung.",
        lifespan_years: "12 - 16",
        coat_rgb: [198, 142, 74],
        accent_rgb: [72, 46, 26],
        eye_rgb: [118, 192, 96],
        ear_style: "rounded",
    },
    CatBreed {
        id: "siam",
        name: "Siamkatze",
        origin: "Thailand",
        weight_kg: "3.0 - 5.0 kg",
        coat: "Eng anliegendes Point-Kurzhaar",
        grooming_level: 1,
        energy_level: 5,
        vocal_level: 5,
        indoor_ok: true,
        kid_friendly: true,
        dog_friendly: true,
        senior_friendly: false,
        hypoallergenic_hint: false,
        temperament: "Extrem gesprächig, sozial, hasst Alleinsein, sehr anhänglich.",
        health_notes: "Zahnfleisch/FORL, Amyloidose, Strabismus; unbedingt zu zweit halten.",
        lifespan_years: "14 - 19",
        coat_rgb: [236, 224, 204],
        accent_rgb: [74, 54, 44],
        eye_rgb: [64, 152, 240],
        ear_style: "pointed",
    },
    CatBreed {
        id: "kartauser",
        name: "Kartäuser (Chartreux)",
        origin: "Frankreich",
        weight_kg: "4.0 - 7.0 kg",
        coat: "Wolliges blaugraues Kurzhaar",
        grooming_level: 2,
        energy_level: 2,
        vocal_level: 1,
        indoor_ok: true,
        kid_friendly: true,
        dog_friendly: true,
        senior_friendly: true,
        hypoallergenic_hint: false,
        temperament: "Leise, treu, beobachtet gern, sanft im Umgang.",
        health_notes: "Patellaluxation & Harnsteine (FLUTD) im Blick behalten.",
        lifespan_years: "13 - 17",
        coat_rgb: [116, 128, 144],
        accent_rgb: [88, 98, 112],
        eye_rgb: [230, 146, 44],
        ear_style: "pointed",
    },
    CatBreed {
        id: "sphynx",
        name: "Canadian Sphynx",
        origin: "Kanada",
        weight_kg: "3.5 - 5.5 kg",
        coat: "Haarlos mit feinem Pfirsichflaum",
        grooming_level: 4,
        energy_level: 4,
        vocal_level: 4,
        indoor_ok: true,
        kid_friendly: true,
        dog_friendly: true,
        senior_friendly: false,
        hypoallergenic_hint: false,
        temperament: "Extrem verschmust, sucht Wärme, verspielt und menschenfixiert.",
        health_notes:
            "Talgpflege/Ohren reinigen, Sonnenschutz, HCM-Schall; NICHT frei von Allergenen!",
        lifespan_years: "12 - 15",
        coat_rgb: [218, 186, 174],
        accent_rgb: [182, 146, 136],
        eye_rgb: [120, 190, 140],
        ear_style: "pointed",
    },
    CatBreed {
        id: "perser",
        name: "Perser (mit Nase / Doll-Face)",
        origin: "Iran / Großbritannien",
        weight_kg: "3.5 - 6.0 kg",
        coat: "Langes seidiges Fell mit viel Unterwolle",
        grooming_level: 5,
        energy_level: 1,
        vocal_level: 1,
        indoor_ok: true,
        kid_friendly: false,
        dog_friendly: true,
        senior_friendly: true,
        hypoallergenic_hint: false,
        temperament: "Sehr ruhig, wohnungsliebend, braucht tägliches Kämmen.",
        health_notes: "Nur Zucht mit freier Nase wählen! PKD-Gentest zwingend.",
        lifespan_years: "12 - 17",
        coat_rgb: [244, 240, 232],
        accent_rgb: [218, 206, 188],
        eye_rgb: [224, 144, 48],
        ear_style: "rounded",
    },
    CatBreed {
        id: "abessinier",
        name: "Abessinier",
        origin: "Äthiopien / Großbritannien",
        weight_kg: "3.0 - 5.0 kg",
        coat: "Ticked Kurzhaar (Wildfarben, Sorrel, Blau)",
        grooming_level: 1,
        energy_level: 5,
        vocal_level: 3,
        indoor_ok: true,
        kid_friendly: true,
        dog_friendly: true,
        senior_friendly: false,
        hypoallergenic_hint: false,
        temperament: "Neugierig, athletisch, klettert hoch, braucht viel Interaktion.",
        health_notes: "PRA, PK-Defizienz & Zahnfleisch-Entzündungen prüfen.",
        lifespan_years: "12 - 16",
        coat_rgb: [188, 118, 66],
        accent_rgb: [112, 62, 32],
        eye_rgb: [188, 176, 62],
        ear_style: "pointed",
    },
    CatBreed {
        id: "burma",
        name: "Burma (Burmese)",
        origin: "Myanmar / Großbritannien",
        weight_kg: "3.5 - 5.5 kg",
        coat: "Satinweiches Kurzhaar (Braun, Blau, Champagner)",
        grooming_level: 1,
        energy_level: 4,
        vocal_level: 4,
        indoor_ok: true,
        kid_friendly: true,
        dog_friendly: true,
        senior_friendly: true,
        hypoallergenic_hint: false,
        temperament: "Verschmust, anhänglich, verspielt, folgt auf Schritt und Tritt.",
        health_notes: "Hypokaliämie-Test & Diabetes-Prävention (Gewicht).",
        lifespan_years: "14 - 18",
        coat_rgb: [112, 82, 64],
        accent_rgb: [76, 54, 42],
        eye_rgb: [228, 178, 52],
        ear_style: "rounded",
    },
    CatBreed {
        id: "balinese",
        name: "Balinese",
        origin: "USA",
        weight_kg: "3.0 - 5.0 kg",
        coat: "Seidiges Halblanghaar ohne Unterwolle",
        grooming_level: 2,
        energy_level: 4,
        vocal_level: 4,
        indoor_ok: true,
        kid_friendly: true,
        dog_friendly: true,
        senior_friendly: false,
        hypoallergenic_hint: true,
        temperament: "Wie Siamkatze mit halblangem Fell: clever, sozial, kommunikativ.",
        health_notes: "PRA-Test & Zähne; Paarhaltung dringend empfohlen.",
        lifespan_years: "14 - 18",
        coat_rgb: [238, 228, 214],
        accent_rgb: [92, 72, 62],
        eye_rgb: [68, 148, 232],
        ear_style: "pointed",
    },
    CatBreed {
        id: "devon_rex",
        name: "Devon Rex",
        origin: "Großbritannien",
        weight_kg: "2.5 - 4.5 kg",
        coat: "Kurz gewelltes Lockenfell",
        grooming_level: 2,
        energy_level: 4,
        vocal_level: 3,
        indoor_ok: true,
        kid_friendly: true,
        dog_friendly: true,
        senior_friendly: true,
        hypoallergenic_hint: true,
        temperament: "Clown unter den Katzen, schläft gern unter der Decke.",
        health_notes: "CMS-Gentest, HCM & Ohrenpflege; kälteempfindlich.",
        lifespan_years: "12 - 16",
        coat_rgb: [198, 178, 158],
        accent_rgb: [136, 118, 102],
        eye_rgb: [122, 192, 118],
        ear_style: "pointed",
    },
    CatBreed {
        id: "tuerkisch_van",
        name: "Türkisch Van",
        origin: "Türkei",
        weight_kg: "4.0 - 7.5 kg",
        coat: "Kaschmirartiges Halblanghaar (weiß mit Farbzeichnung)",
        grooming_level: 3,
        energy_level: 4,
        vocal_level: 3,
        indoor_ok: true,
        kid_friendly: true,
        dog_friendly: true,
        senior_friendly: false,
        hypoallergenic_hint: false,
        temperament: "Selbstbewusst, liebt Wasser & hohe Kratzbäume.",
        health_notes: "HCM & Taubheits-Check bei überwiegend weißen Linien.",
        lifespan_years: "13 - 17",
        coat_rgb: [246, 244, 240],
        accent_rgb: [188, 98, 52],
        eye_rgb: [198, 166, 62],
        ear_style: "pointed",
    },
    CatBreed {
        id: "oriental_shorthair",
        name: "Orientalisch Kurzhaar",
        origin: "Großbritannien",
        weight_kg: "3.0 - 5.0 kg",
        coat: "Glattes, sehr kurzes Fell in vielen Farben",
        grooming_level: 1,
        energy_level: 5,
        vocal_level: 5,
        indoor_ok: true,
        kid_friendly: true,
        dog_friendly: true,
        senior_friendly: false,
        hypoallergenic_hint: false,
        temperament: "Redet viel, spielt begeistert, braucht kätzische Gesellschaft.",
        health_notes: "Zähne/FORL & Herz; sehr kältebedürftig.",
        lifespan_years: "13 - 18",
        coat_rgb: [52, 52, 58],
        accent_rgb: [82, 82, 90],
        eye_rgb: [92, 198, 112],
        ear_style: "pointed",
    },
    CatBreed {
        id: "neva_masquerade",
        name: "Neva Masquerade",
        origin: "Russland",
        weight_kg: "4.0 - 8.0 kg",
        coat: "Halblanghaar mit blauen Augen (Point-Variante der Sibirischen)",
        grooming_level: 4,
        energy_level: 3,
        vocal_level: 2,
        indoor_ok: true,
        kid_friendly: true,
        dog_friendly: true,
        senior_friendly: true,
        hypoallergenic_hint: true,
        temperament: "Sanft, ausgeglichen, verspielt, liebt Familienanschluss.",
        health_notes: "HCM & PKD testen lassen.",
        lifespan_years: "13 - 18",
        coat_rgb: [238, 234, 228],
        accent_rgb: [118, 128, 146],
        eye_rgb: [72, 152, 236],
        ear_style: "tufted",
    },
];

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CatMatchQuery {
    pub housing: String,
    pub household: String,
    pub allergies: bool,
    pub max_grooming: u8,
}

#[derive(Debug, Clone, Serialize)]
pub struct CatMatchResult {
    pub breed_id: &'static str,
    pub breed_name: &'static str,
    pub score: i32,
    pub reasons: Vec<String>,
    pub warnings: Vec<String>,
}

pub fn match_breeds(q: &CatMatchQuery) -> Vec<CatMatchResult> {
    let mut out = Vec::new();
    for b in CAT_BREEDS {
        let mut score = 60_i32;
        let mut reasons = Vec::new();
        let mut warnings = Vec::new();

        if q.housing == "wohnung" {
            if b.indoor_ok && b.energy_level <= 3 {
                score += 18;
                reasons.push("Sehr gut für reine Wohnungshaltung geeignet".into());
            } else if !b.indoor_ok {
                score -= 28;
                warnings
                    .push("Hoher Bewegungsdrang - Wohnung ohne Freigang/Catwalk kritisch".into());
            }
        } else if q.housing == "freigang" {
            if b.energy_level >= 3 {
                score += 12;
                reasons.push("Nutzt gesicherten Garten oder Freigang optimal".into());
            }
            if b.id == "ragdoll" || b.id == "sphynx" {
                score -= 15;
                warnings.push("Für ungesicherten Freigang ungeeignet".into());
            }
        }

        if q.household == "senioren" {
            if b.senior_friendly && b.energy_level <= 3 {
                score += 20;
                reasons.push("Ruhiges, angenehmes Temperament für Senioren".into());
            } else if b.energy_level >= 5 {
                score -= 18;
                warnings.push("Sehr aktiv & fordernd".into());
            }
        } else if q.household == "familie_kinder" {
            if b.kid_friendly {
                score += 15;
                reasons.push("Kinderfreundlich & geduldig".into());
            } else {
                score -= 20;
                warnings.push("Mag eher ruhige Haushalte ohne Trubel".into());
            }
        } else if q.household == "hund" {
            if b.dog_friendly {
                score += 14;
                reasons.push("Kommt bei langsamer Gewöhnung gut mit Hunden klar".into());
            } else {
                score -= 15;
                warnings.push("Gegenüber Hunden oft zurückhaltend".into());
            }
        }

        if q.allergies {
            if b.hypoallergenic_hint {
                score += 18;
                reasons.push("Oft geringerer Fel-d1-Gehalt (trotzdem vorher testen!)".into());
            } else {
                score -= 6;
            }
        }

        if b.grooming_level <= q.max_grooming {
            score += 8;
        } else {
            score -= 14;
            warnings.push("Erhöhter Fell-/Pflegeaufwand".into());
        }

        out.push(CatMatchResult {
            breed_id: b.id,
            breed_name: b.name,
            score,
            reasons,
            warnings,
        });
    }
    out.sort_by_key(|a| std::cmp::Reverse(a.score));
    out
}

#[derive(Debug, Clone, Serialize)]
pub struct ToxicItem {
    pub name: &'static str,
    pub severity: &'static str,
    pub symptoms: &'static str,
    pub action: &'static str,
}

pub const TOXIC_ITEMS: &[ToxicItem] = &[
    ToxicItem {
        name: "Lilien (Lilium / Taglilie / Blumenvase)",
        severity: "LEBENSGEFAHR (EXTREM TOXISCH)",
        symptoms: "Akutes Nierenversagen innerhalb 24-72h - schon Pollen oder Vasenwasser!",
        action: "SOFORT Tierklinik/Notdienst! Jede Minute zählt.",
    },
    ToxicItem {
        name: "Paracetamol / Ibuprofen / Aspirin",
        severity: "LEBENSGEFAHR",
        symptoms:
            "Zerstörung roter Blutkörperchen, Leberversagen, Atemnot (Glucuronidierungs-Defekt).",
        action: "SOFORT Tierarzt! Niemals Human-Schmerzmittel geben.",
    },
    ToxicItem {
        name: "Teebaumöl & ätherische Öle (Diffuser)",
        severity: "SEHR HOCH",
        symptoms: "Speicheln, Taumeln, Zittern, Lebertoxizität.",
        action: "Ölquelle entfernen, Raum lüften, Tierarzt kontaktieren.",
    },
    ToxicItem {
        name: "Zwiebel, Knoblauch, Schnittlauch",
        severity: "HOCH",
        symptoms: "Hämolyse (Blutarmut), blasse Schleimhäute, Schwäche.",
        action: "Tierarzt kontaktieren, Menge & Zeitpunkt notieren.",
    },
    ToxicItem {
        name: "Schokolade / Koffein",
        severity: "HOCH",
        symptoms: "Herzrasen, Unruhe, Erbrechen, Krämpfe (Theobromin).",
        action: "Tierarzt kontaktieren.",
    },
    ToxicItem {
        name: "Weintrauben / Rosinen",
        severity: "HOCH",
        symptoms: "Erbrechen, Durchfall, Nierenrisiko.",
        action: "Tierarzt anrufen.",
    },
    ToxicItem {
        name: "Monstera / Dieffenbachia / Weihnachtsstern",
        severity: "MITTEL",
        symptoms: "Schleimhautreizung, Speicheln, Schluckbeschwerden.",
        action: "Maul vorsichtig ausspülen, bei Atemnot Tierarzt.",
    },
];

pub fn check_toxin(query: &str) -> Vec<&'static ToxicItem> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return TOXIC_ITEMS.iter().collect();
    }
    TOXIC_ITEMS
        .iter()
        .filter(|t| {
            t.name.to_lowercase().contains(&q)
                || t.symptoms.to_lowercase().contains(&q)
                || t.severity.to_lowercase().contains(&q)
        })
        .collect()
}

#[derive(Debug, Clone, Serialize)]
pub struct VetHospital {
    pub name: &'static str,
    pub city: &'static str,
    pub kind: &'static str,
    pub emergency_24h: bool,
    pub specialty: &'static str,
}

pub const VET_HOSPITALS: &[VetHospital] = &[
    VetHospital {
        name: "Vetmeduni Wien - Kleintierklinik",
        city: "Wien (1210)",
        kind: "Universitätstierspital",
        emergency_24h: true,
        specialty: "24h Notaufnahme, Kardiologie (HCM), Innere Medizin (CNI), Chirurgie",
    },
    VetHospital {
        name: "Tierklinik Strebersdorf",
        city: "Wien (1210)",
        kind: "Überweisungsklinik",
        emergency_24h: true,
        specialty: "Notdienst, Zahnheilkunde (FORL-Dentalröntgen), Orthopädie",
    },
    VetHospital {
        name: "Tierklinik Rodaun",
        city: "Wien (1230)",
        kind: "Tierklinik",
        emergency_24h: false,
        specialty: "Katzenmedizin, Ultraschall, Chirurgie",
    },
    VetHospital {
        name: "Tierklinik Hofheim",
        city: "Hofheim am Taunus",
        kind: "Überweisungsklinik",
        emergency_24h: true,
        specialty: "Onkologie, Kardiologie, Intensivmedizin",
    },
    VetHospital {
        name: "LMU Kleintiermedizinische Klinik",
        city: "München",
        kind: "Universitätstierspital",
        emergency_24h: true,
        specialty: "24h Notfall, Innere Medizin, Dermatologie",
    },
];

#[derive(Debug, Clone, Serialize)]
pub struct CatInsurance {
    pub provider: &'static str,
    pub plan_type: &'static str,
    pub approx_monthly_eur: &'static str,
    pub highlights: &'static str,
}

pub const CAT_INSURANCES: &[CatInsurance] = &[
    CatInsurance {
        provider: "Agria Tierversicherung",
        plan_type: "Voll- oder OP-Schutz",
        approx_monthly_eur: "18 - 42 EUR",
        highlights: "Kein Höchstaufnahmealter, lebenslange Deckung, Zahn/FORL je nach Tarif.",
    },
    CatInsurance {
        provider: "Barmenia Tierkrankenversicherung",
        plan_type: "OP & Vollversicherung",
        approx_monthly_eur: "20 - 48 EUR",
        highlights: "Freie Tierarztwahl, bis zu 100% GOT-Erstattung im Premium-Tarif.",
    },
    CatInsurance {
        provider: "Lassie Katzenversicherung",
        plan_type: "OP & Krankenschutz",
        approx_monthly_eur: "16 - 38 EUR",
        highlights: "Vorsorgebudget, App-Prävention, schnelle digitale Abrechnung.",
    },
    CatInsurance {
        provider: "Allianz / Helvetia PetCare",
        plan_type: "Kranken- & OP-Schutz",
        approx_monthly_eur: "22 - 45 EUR",
        highlights: "Bewährte Abdeckung bei stationären Klinikaufenthalten & Diagnostik.",
    },
];

pub fn food_ration_grams(weight_kg: f64, is_kitten: bool, activity: &str) -> (f64, f64, f64) {
    let w = weight_kg.clamp(0.4, 15.0);
    let mult = if is_kitten {
        1.55
    } else {
        match activity {
            "ruhig" => 0.85,
            "aktiv" => 1.2,
            _ => 1.0,
        }
    };
    let kcal = (70.0 * w.powf(0.75)) * 1.2 * mult;
    let wet_g = (kcal / 0.90).round();
    let dry_g = (kcal / 3.80).round();
    (kcal.round(), wet_g, dry_g)
}

pub fn cat_human_age(cat_years: f64) -> f64 {
    let y = cat_years.max(0.0);
    if y <= 1.0 {
        y * 15.0
    } else if y <= 2.0 {
        15.0 + (y - 1.0) * 9.0
    } else {
        24.0 + (y - 2.0) * 4.0
    }
}
