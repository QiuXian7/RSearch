use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,

    pub file_name: String,

    pub extension: String,

    pub size: u64,

    pub modified: u64,
}
