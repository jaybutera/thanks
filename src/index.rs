use crate::types::Index;
use std::fs;
use anyhow;

/// Gets json from the global index file. Makes one if it does not exist.
pub fn get_index() -> anyhow::Result<Index> {
    let path = std::path::Path::new("./index.json");
    if !path.exists() {
        let s = serde_json::to_string(&Index { theses: std::collections::HashMap::new() })?;
        //fs::create_dir(std::path::Path::new("~/.note-wiki"))?;
        //fs::File::create(path)?;
        fs::write(path, &s)?;
    }
    let s = fs::read_to_string(path)?;
    let index: Index = serde_json::from_str(&s)?;
    Ok(index)
}

/// Replace the global index file with new index
pub fn save_index(index: Index) -> anyhow::Result<()> {
    //let path = std::path::Path::new("~/.note-wiki/index.json");
    let path = std::path::Path::new("./index.json");
    let ser = serde_json::to_string(&index)?;
    fs::write(path, &ser)?;
    Ok(())
}
