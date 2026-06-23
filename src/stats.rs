use crate::models::FileInfo;

pub struct Stats {
    pub total_files: usize,

    pub total_size: u64,
}

pub fn calculate_stats(files: &[FileInfo]) -> Stats {
    let total_size = files.iter().map(|f| f.size).sum();

    Stats {
        total_files: files.len(),

        total_size,
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::models::FileInfo;

    #[test]
    fn test_calculate_stats() {
        let files = vec![
            FileInfo {
                path: "a".into(),
                file_name: "a".into(),
                extension: "txt".into(),
                size: 100,
                modified: 0,
            },
            FileInfo {
                path: "b".into(),
                file_name: "b".into(),
                extension: "txt".into(),
                size: 200,
                modified: 0,
            },
        ];

        let stat = calculate_stats(&files);

        assert_eq!(stat.total_files, 2,);

        assert_eq!(stat.total_size, 300,);
    }
}
