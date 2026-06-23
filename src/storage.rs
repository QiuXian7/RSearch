use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchIndex {
    pub data: HashMap<String, HashMap<String, usize>>,
}

pub fn save_index(index: &SearchIndex) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(index)?;

    fs::write("index.json", json)?;

    Ok(())
}

pub fn load_index() -> anyhow::Result<SearchIndex> {
    let content = fs::read_to_string("index.json")?;

    let index = serde_json::from_str(&content)?;

    Ok(index)
}
