use anyhow::Result;
use std::fs;
use std::time::UNIX_EPOCH;
use walkdir::WalkDir;

use crate::models::FileInfo;

pub fn scan_directory(path: &str) -> Result<Vec<FileInfo>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            let metadata = fs::metadata(entry.path())?;

            let modified = metadata.modified()?.duration_since(UNIX_EPOCH)?.as_secs();

            let file_name = entry.file_name().to_string_lossy().to_string();

            let extension = entry
                .path()
                .extension()
                .map(|e| e.to_string_lossy().to_string())
                .unwrap_or_default();

            files.push(FileInfo {
                path: entry.path().display().to_string(),
                file_name,
                extension,
                size: metadata.len(),
                modified,
            });
        }
    }

    Ok(files)
}
