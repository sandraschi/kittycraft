use crate::catalog::{
    cat_human_age, check_toxin, food_ration_grams, match_breeds, CatMatchQuery, CAT_BREEDS,
    CAT_INSURANCES, VET_HOSPITALS,
};
use crate::error::KittyCraftError;
use crate::skills::{skill_by_name, skills_list};
use crate::store::KittyStore;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Debug, Clone, Serialize)]
pub struct ToolDescriptor {
    pub name: &'static str,
    pub description: &'static str,
    pub read_only: bool,
    pub input_schema: Value,
}

pub fn tools_list() -> Vec<ToolDescriptor> {
    vec![
        ToolDescriptor {
            name: "kitty_status",
            description: "Show KittyCraft database status, breed count, and record counts.",
            read_only: true,
            input_schema: json!({"type":"object","properties":{}}),
        },
        ToolDescriptor {
            name: "kitty_breeds_list",
            description: "List all 20 cat breeds in the KittyCraft encyclopedia.",
            read_only: true,
            input_schema: json!({"type":"object","properties":{"query":{"type":"string"}}}),
        },
        ToolDescriptor {
            name: "kitty_match_breeds",
            description: "Score and recommend cat breeds for housing (wohnung|freigang), household (single|senioren|familie_kinder|hund), allergies, and max_grooming (1-5).",
            read_only: true,
            input_schema: json!({
                "type":"object",
                "properties":{
                    "housing":{"type":"string"},
                    "household":{"type":"string"},
                    "allergies":{"type":"boolean"},
                    "max_grooming":{"type":"integer"}
                }
            }),
        },
        ToolDescriptor {
            name: "kitty_toxic_check",
            description: "Check feline toxicity for plants, foods, or human meds (lilies, paracetamol, tea tree oil, etc.).",
            read_only: true,
            input_schema: json!({"type":"object","properties":{"query":{"type":"string"}},"required":["query"]}),
        },
        ToolDescriptor {
            name: "kitty_food_calc",
            description: "Calculate daily feline kcal, wet food grams, dry food grams, and human age equivalent.",
            read_only: true,
            input_schema: json!({
                "type":"object",
                "properties":{
                    "weight_kg":{"type":"number"},
                    "is_kitten":{"type":"boolean"},
                    "activity":{"type":"string"},
                    "age_years":{"type":"number"}
                },
                "required":["weight_kg"]
            }),
        },
        ToolDescriptor {
            name: "kitty_hospitals_and_insurance",
            description: "List veterinary hospitals (24h clinics) and cat insurance plans.",
            read_only: true,
            input_schema: json!({"type":"object","properties":{}}),
        },
        ToolDescriptor {
            name: "kitty_diary_list",
            description: "List recent Katzi-Tagebuch entries.",
            read_only: true,
            input_schema: json!({"type":"object","properties":{"limit":{"type":"integer"}}}),
        },
        ToolDescriptor {
            name: "kitty_diary_add",
            description: "Add a Katzi-Tagebuch entry (weight, mood, vet or care note).",
            read_only: false,
            input_schema: json!({
                "type":"object",
                "properties":{
                    "date":{"type":"string"},
                    "category":{"type":"string"},
                    "title":{"type":"string"},
                    "notes":{"type":"string"},
                    "weight_kg":{"type":"number"},
                    "mood":{"type":"string"}
                },
                "required":["date","title","notes"]
            }),
        },
        ToolDescriptor {
            name: "kitty_calendar_list",
            description: "List sitter / cat-care calendar events.",
            read_only: true,
            input_schema: json!({"type":"object","properties":{}}),
        },
        ToolDescriptor {
            name: "kitty_calendar_add",
            description: "Add a sitter / cat-care calendar event.",
            read_only: false,
            input_schema: json!({
                "type":"object",
                "properties":{
                    "start_date":{"type":"string"},
                    "end_date":{"type":"string"},
                    "sitter_name":{"type":"string"},
                    "sitter_relation":{"type":"string"},
                    "care_notes":{"type":"string"}
                },
                "required":["start_date","end_date","sitter_name"]
            }),
        },
        ToolDescriptor {
            name: "kitty_vet_visit_add",
            description: "Log a veterinary clinic visit.",
            read_only: false,
            input_schema: json!({
                "type":"object",
                "properties":{
                    "date":{"type":"string"},
                    "clinic":{"type":"string"},
                    "reason":{"type":"string"},
                    "diagnosis":{"type":"string"},
                    "cost_eur":{"type":"number"}
                },
                "required":["date","clinic","reason"]
            }),
        },
        ToolDescriptor {
            name: "kitty_skill_read",
            description: "Read a bundled KittyCraft domain skill (SKILL.md) by name.",
            read_only: true,
            input_schema: json!({"type":"object","properties":{"name":{"type":"string"}},"required":["name"]}),
        },
    ]
}

pub fn is_read_only_tool(name: &str) -> bool {
    tools_list()
        .iter()
        .find(|t| t.name == name)
        .is_some_and(|t| t.read_only)
}

pub fn dispatch_tool(
    store: &KittyStore,
    name: &str,
    args: &Value,
) -> Result<Value, KittyCraftError> {
    match name {
        "kitty_status" => {
            let st = store.status()?;
            Ok(json!({
                "app": "kittycraft",
                "version": env!("CARGO_PKG_VERSION"),
                "breeds": CAT_BREEDS.len(),
                "skills": skills_list().len(),
                "store": st,
            }))
        }
        "kitty_breeds_list" => {
            let q = args
                .get("query")
                .and_then(Value::as_str)
                .unwrap_or("")
                .trim()
                .to_lowercase();
            let list: Vec<_> = CAT_BREEDS
                .iter()
                .filter(|b| {
                    q.is_empty()
                        || b.name.to_lowercase().contains(&q)
                        || b.temperament.to_lowercase().contains(&q)
                        || b.coat.to_lowercase().contains(&q)
                })
                .collect();
            Ok(json!({ "count": list.len(), "breeds": list }))
        }
        "kitty_match_breeds" => {
            let q = CatMatchQuery {
                housing: args
                    .get("housing")
                    .and_then(Value::as_str)
                    .unwrap_or("wohnung")
                    .to_string(),
                household: args
                    .get("household")
                    .and_then(Value::as_str)
                    .unwrap_or("single")
                    .to_string(),
                allergies: args
                    .get("allergies")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
                max_grooming: args
                    .get("max_grooming")
                    .and_then(Value::as_u64)
                    .unwrap_or(3)
                    .clamp(1, 5) as u8,
            };
            let matches = match_breeds(&q);
            Ok(
                json!({ "query": q, "top_matches": matches.into_iter().take(6).collect::<Vec<_>>() }),
            )
        }
        "kitty_toxic_check" => {
            let q = args.get("query").and_then(Value::as_str).unwrap_or("");
            let hits = check_toxin(q);
            Ok(json!({ "query": q, "matches": hits }))
        }
        "kitty_food_calc" => {
            let weight_kg = args
                .get("weight_kg")
                .and_then(Value::as_f64)
                .ok_or_else(|| KittyCraftError::InvalidInput("weight_kg required".into()))?;
            let is_kitten = args
                .get("is_kitten")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            let activity = args
                .get("activity")
                .and_then(Value::as_str)
                .unwrap_or("normal");
            let (kcal, wet_g, dry_g) = food_ration_grams(weight_kg, is_kitten, activity);
            let human_age = args
                .get("age_years")
                .and_then(Value::as_f64)
                .map(cat_human_age);
            Ok(json!({
                "weight_kg": weight_kg,
                "kcal_per_day": kcal,
                "wet_food_g_per_day": wet_g,
                "dry_food_g_per_day": dry_g,
                "human_age_years": human_age,
            }))
        }
        "kitty_hospitals_and_insurance" => Ok(json!({
            "vet_hospitals": VET_HOSPITALS,
            "insurances": CAT_INSURANCES,
        })),
        "kitty_diary_list" => {
            let limit = args.get("limit").and_then(Value::as_u64).unwrap_or(20) as usize;
            let entries = store.list_diary(limit)?;
            Ok(json!({ "entries": entries }))
        }
        "kitty_diary_add" => {
            let date = args
                .get("date")
                .and_then(Value::as_str)
                .ok_or_else(|| KittyCraftError::InvalidInput("date required".into()))?;
            let title = args
                .get("title")
                .and_then(Value::as_str)
                .ok_or_else(|| KittyCraftError::InvalidInput("title required".into()))?;
            let notes = args.get("notes").and_then(Value::as_str).unwrap_or("");
            let category = args
                .get("category")
                .and_then(Value::as_str)
                .unwrap_or("Alltag");
            let weight_kg = args.get("weight_kg").and_then(Value::as_f64);
            let mood = args
                .get("mood")
                .and_then(Value::as_str)
                .unwrap_or("Neugierig");
            let id = store.add_diary(date, category, title, notes, weight_kg, mood)?;
            Ok(json!({ "inserted_id": id }))
        }
        "kitty_calendar_list" => Ok(json!({ "events": store.list_sitter_events()? })),
        "kitty_calendar_add" => {
            let start = args
                .get("start_date")
                .and_then(Value::as_str)
                .ok_or_else(|| KittyCraftError::InvalidInput("start_date required".into()))?;
            let end = args
                .get("end_date")
                .and_then(Value::as_str)
                .ok_or_else(|| KittyCraftError::InvalidInput("end_date required".into()))?;
            let sitter = args
                .get("sitter_name")
                .and_then(Value::as_str)
                .ok_or_else(|| KittyCraftError::InvalidInput("sitter_name required".into()))?;
            let rel = args
                .get("sitter_relation")
                .and_then(Value::as_str)
                .unwrap_or("Familie/Freunde");
            let notes = args.get("care_notes").and_then(Value::as_str).unwrap_or("");
            let id = store.add_sitter_event(start, end, sitter, rel, notes, "Geplant")?;
            Ok(json!({ "inserted_id": id }))
        }
        "kitty_vet_visit_add" => {
            let date = args
                .get("date")
                .and_then(Value::as_str)
                .ok_or_else(|| KittyCraftError::InvalidInput("date required".into()))?;
            let clinic = args
                .get("clinic")
                .and_then(Value::as_str)
                .ok_or_else(|| KittyCraftError::InvalidInput("clinic required".into()))?;
            let reason = args
                .get("reason")
                .and_then(Value::as_str)
                .ok_or_else(|| KittyCraftError::InvalidInput("reason required".into()))?;
            let diag = args.get("diagnosis").and_then(Value::as_str).unwrap_or("");
            let cost = args.get("cost_eur").and_then(Value::as_f64);
            let id = store.add_vet_visit(date, clinic, reason, diag, cost)?;
            Ok(json!({ "inserted_id": id }))
        }
        "kitty_skill_read" => {
            let n = args
                .get("name")
                .and_then(Value::as_str)
                .ok_or_else(|| KittyCraftError::InvalidInput("name required".into()))?;
            let s = skill_by_name(n)?;
            Ok(json!({
                "name": s.name,
                "uri": s.uri,
                "description": s.description,
                "content": s.skill_md,
            }))
        }
        other => Err(KittyCraftError::NotFound(format!("Unknown tool '{other}'"))),
    }
}
