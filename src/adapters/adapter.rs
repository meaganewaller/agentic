use anyhow::Result;
use serde_json::Value;
use std::path::PathBuf;

pub trait VendorAdapter {
    fn name(&self) -> &'static str;

    fn compile(&self, merged: &Value) -> Result<Value>;

    fn default_output_path(&self) -> Result<PathBuf>;
}