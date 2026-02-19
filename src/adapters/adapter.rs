use anyhow::Result;
use serde_json::Value;
use std::path::PathBuf;

pub struct CompileInput<'a> {
    pub merged: &'a Value,
    pub resolved_agent_prompt: Option<&'a String>,
}


pub trait VendorAdapter {
    fn name(&self) -> &'static str;
    fn compile(&self, input: CompileInput<'_>) -> Result<Value>;
    fn default_output_path(&self) -> Result<PathBuf>;
}