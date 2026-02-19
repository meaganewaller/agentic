use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct LayerSpec {
    pub base: PathBuf,
    pub profile: Option<PathBuf>,
    pub machine: Option<PathBuf>,
    pub project: Option<PathBuf>,
}

impl LayerSpec {
    pub fn ordered_paths(&self) -> Vec<PathBuf> {
        let mut v = vec![self.base.clone()];
        if let Some(p) = &self.profile { v.push(p.clone()); }
        if let Some(m) = &self.machine { v.push(m.clone()); }
        if let Some(pj) = &self.project { v.push(pj.clone()); }
        v
    }
}
