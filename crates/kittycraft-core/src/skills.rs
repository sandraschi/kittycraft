use crate::error::KittyCraftError;
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
pub struct BundledSkill {
    pub name: &'static str,
    pub description: &'static str,
    pub uri: &'static str,
    pub skill_md: &'static str,
}

pub const SKILLS: &[BundledSkill] = &[
    BundledSkill {
        name: "kittycraft-advisor",
        description: "Cat and kitten breed matching, indoor vs outdoor care, feline toxicology (lilies, paracetamol), medical/CNI/FORL, and behavior in KittyCraft.",
        uri: "skill://kittycraft-advisor/SKILL.md",
        skill_md: include_str!("../../../skills/kittycraft-advisor/SKILL.md"),
    },
    BundledSkill {
        name: "kittycraft-workflow",
        description: "Step-by-step workflows in KittyCraft for breed matching, emergency toxin checks, and logging diary/sitter events.",
        uri: "skill://kittycraft-workflow/SKILL.md",
        skill_md: include_str!("../../../skills/kittycraft-workflow/SKILL.md"),
    },
];

pub fn skills_list() -> &'static [BundledSkill] {
    SKILLS
}

pub fn skill_by_name(name: &str) -> Result<&'static BundledSkill, KittyCraftError> {
    SKILLS
        .iter()
        .find(|s| s.name == name)
        .ok_or_else(|| KittyCraftError::NotFound(format!("Unknown skill '{name}'")))
}

pub fn skill_by_uri(uri: &str) -> Option<&'static BundledSkill> {
    SKILLS.iter().find(|s| s.uri == uri)
}

pub fn export_skills(dest_dir: &Path) -> Result<Vec<String>, KittyCraftError> {
    fs::create_dir_all(dest_dir)?;
    let mut out = Vec::new();
    for s in SKILLS {
        let dir = dest_dir.join(s.name);
        fs::create_dir_all(&dir)?;
        let path = dir.join("SKILL.md");
        fs::write(&path, s.skill_md)?;
        out.push(path.display().to_string());
    }
    Ok(out)
}
