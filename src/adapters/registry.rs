use super::adapter::VendorAdapter;
use super::{claude::ClaudeAdapter, codex::CodexAdapter};

pub fn all_adapters() -> Vec<Box<dyn VendorAdapter>> {
    vec![
        Box::new(ClaudeAdapter),
        Box::new(CodexAdapter),
    ]
}
