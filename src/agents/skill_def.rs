use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct SkillMeta {
    pub slug: Option<String>,
    pub version: Option<u32>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct SkillDef {
    pub slug: String,
    pub meta: SkillMeta,
    pub body: String,
}
