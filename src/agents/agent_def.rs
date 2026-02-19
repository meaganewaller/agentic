use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AgentDef {
    pub name: String,
    pub description: Option<String>,
    pub system_prompt: Option<String>,
    #[serde(default)]
    pub skills: Vec<String>,
}
