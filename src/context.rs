use anyhow::Result;

pub fn detect_hostname() -> Result<String> {
    let raw = hostname::get()?
        .into_string()
        .unwrap_or_else(|_| "unknown".to_string());

    // Normalize: remove domain suffix if present
    let normalized = raw
        .split('.')
        .next()
        .unwrap_or("unknown")
        .to_lowercase();

    Ok(normalized)
}