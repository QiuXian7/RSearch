use crate::models::FileInfo;
use regex::Regex;
use std::fs;
pub fn search_by_name(files: &[FileInfo], keyword: &str) -> Vec<FileInfo> {
    let keyword = keyword.to_lowercase();

    files
        .iter()
        .filter(|file| file.file_name.to_lowercase().contains(&keyword))
        .cloned()
        .collect()
}

pub fn regex_search(files: &[FileInfo], pattern: &str) -> Vec<FileInfo> {
    let regex = match Regex::new(pattern) {
        Ok(r) => r,

        Err(_) => return Vec::new(),
    };

    files
        .iter()
        .filter(|file| {
            if let Ok(content) = fs::read_to_string(&file.path) {
                regex.is_match(&content)
            } else {
                false
            }
        })
        .cloned()
        .collect()
}

pub fn search_content_with_preview(files: &[FileInfo], keyword: &str) -> Vec<(FileInfo, String)> {
    let mut results = Vec::new();

    let keyword_lower = keyword.to_lowercase();

    for file in files {
        if let Ok(content) = fs::read_to_string(&file.path) {
            for line in content.lines() {
                if line.to_lowercase().contains(&keyword_lower) {
                    let preview = line.replace(keyword, &format!("[{}]", keyword));

                    results.push((file.clone(), preview));

                    break;
                }
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::models::FileInfo;

    fn mock_files() -> Vec<FileInfo> {
        vec![
            FileInfo {
                path: "a".into(),
                file_name: "rust.md".into(),
                extension: "md".into(),
                size: 100,
                modified: 0,
            },
            FileInfo {
                path: "b".into(),
                file_name: "java.txt".into(),
                extension: "txt".into(),
                size: 200,
                modified: 0,
            },
        ]
    }

    #[test]
    fn test_search_by_name_found() {
        let files = mock_files();

        let result = search_by_name(&files, "rust");

        assert_eq!(result.len(), 1,);

        assert_eq!(result[0].file_name, "rust.md",);
    }

    #[test]
    fn test_search_by_name_not_found() {
        let files = mock_files();

        let result = search_by_name(&files, "python");

        assert_eq!(result.len(), 0,);
    }
}
