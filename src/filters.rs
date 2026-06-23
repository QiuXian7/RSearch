use crate::models::FileInfo;

pub fn filter_by_extension(files: &[FileInfo], ext: &str) -> Vec<FileInfo> {
    let ext = ext.to_lowercase();

    files
        .iter()
        .filter(|file| file.extension.to_lowercase() == ext)
        .cloned()
        .collect()
}

pub fn filter_by_size(files: &[FileInfo], min_size: u64) -> Vec<FileInfo> {
    files
        .iter()
        .filter(|file| file.size >= min_size)
        .cloned()
        .collect()
}

use std::time::{SystemTime, UNIX_EPOCH};

pub fn filter_recent(files: &[FileInfo], days: u64) -> Vec<FileInfo> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let seconds = days * 24 * 60 * 60;

    files
        .iter()
        .filter(|file| now - file.modified <= seconds)
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::models::FileInfo;

    fn mock_files() -> Vec<FileInfo> {
        vec![
            FileInfo {
                path: "a".into(),
                file_name: "a.md".into(),
                extension: "md".into(),
                size: 50,
                modified: 100,
            },
            FileInfo {
                path: "b".into(),
                file_name: "b.txt".into(),
                extension: "txt".into(),
                size: 500,
                modified: 200,
            },
        ]
    }

    #[test]
    fn test_filter_by_extension() {
        let files = mock_files();

        let result = filter_by_extension(&files, "md");

        assert_eq!(result.len(), 1,);

        assert_eq!(result[0].extension, "md",);
    }

    #[test]
    fn test_filter_by_size() {
        let files = mock_files();

        let result = filter_by_size(&files, 100);

        assert_eq!(result.len(), 1,);

        assert_eq!(result[0].size, 500,);
    }

    #[test]
    fn test_filter_by_size_no_result() {
        let files = mock_files();

        let result = filter_by_size(&files, 1000);

        assert_eq!(result.len(), 0,);
    }
}
