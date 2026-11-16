pub mod catalog;
pub mod command;
pub mod error;
pub mod llm;
pub mod skills;
pub mod store;

#[cfg(test)]
mod tests {
    use super::catalog::{check_toxin, food_ration_grams, match_breeds, CatMatchQuery, CAT_BREEDS};
    use super::command::dispatch_tool;
    use super::skills::{skill_by_name, skills_list};
    use super::store::KittyStore;
    use serde_json::json;

    #[test]
    fn match_indoor_senior_prefers_calm_breeds() {
        let q = CatMatchQuery {
            housing: "wohnung".into(),
            household: "senioren".into(),
            allergies: false,
            max_grooming: 3,
        };
        let res = match_breeds(&q);
        assert_eq!(res.len(), CAT_BREEDS.len());
        assert!(res[0].score >= res[res.len() - 1].score);
    }

    #[test]
    fn lily_toxicity_is_critical() {
        let hits = check_toxin("Lilie");
        assert!(!hits.is_empty());
        assert!(hits[0].severity.contains("LEBENSGEFAHR"));
    }

    #[test]
    fn food_calc_and_store_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let (kcal, wet, dry) = food_ration_grams(4.5, false, "normal");
        assert!(kcal > 150.0 && wet > dry);
        let store = KittyStore::open_in_memory()?;
        let id = store.add_diary(
            "2026-11-15",
            "Gewicht",
            "Wiegen",
            "Alles gut",
            Some(4.5),
            "Entspannt",
        )?;
        assert!(id > 0);
        let list = store.list_diary(10)?;
        assert_eq!(list.len(), 1);
        let st = dispatch_tool(&store, "kitty_status", &json!({}))?;
        assert_eq!(st["breeds"], 20);
        Ok(())
    }

    #[test]
    fn bundled_skills_meet_standard() -> Result<(), Box<dyn std::error::Error>> {
        let skills = skills_list();
        assert_eq!(skills.len(), 2);
        for s in skills {
            assert!(s.skill_md.len() <= 12 * 1024);
            assert!(!s.skill_md.contains('—'));
            assert!(skill_by_name(s.name).is_ok());
        }
        Ok(())
    }

    #[test]
    fn pick_best_ollama_model_skips_embeddings() {
        use super::llm::{pick_best_ollama_model, DiscoveredModel};
        let models = vec![
            DiscoveredModel {
                name: "nomic-embed-text:latest".into(),
                parameter_size: "137M".into(),
                quantization: "F16".into(),
                supports_tools: false,
                is_embedding: true,
                is_loaded: true,
                size_gb: 0.3,
            },
            DiscoveredModel {
                name: "rsi-qwen9b:latest".into(),
                parameter_size: "9.0B".into(),
                quantization: "Q4_K_M".into(),
                supports_tools: true,
                is_embedding: false,
                is_loaded: false,
                size_gb: 6.1,
            },
        ];
        assert_eq!(
            pick_best_ollama_model(&models).as_deref(),
            Some("rsi-qwen9b:latest")
        );
    }
}
