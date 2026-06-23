use regex::Regex;
use std::collections::HashMap;
use std::fs;

use crate::models::FileInfo;
use crate::storage::SearchIndex;

pub fn build_index(files: &[FileInfo]) -> SearchIndex {
    let mut index = HashMap::<String, HashMap<String, usize>>::new();

    let regex = Regex::new(r"[a-zA-Z0-9_]+").unwrap();

    for file in files {
        if let Ok(content) = fs::read_to_string(&file.path) {
            let content = content.to_lowercase();

            for word in regex.find_iter(&content) {
                let word = word.as_str();

                let file_map = index.entry(word.to_string()).or_default();

                *file_map.entry(file.path.clone()).or_insert(0) += 1;
            }
        }
    }

    SearchIndex { data: index }
}

#[derive(Debug)]
pub struct SearchResult {
    pub path: String,
    pub score: usize,
}

pub fn search_index(index: &SearchIndex, keyword: &str) -> Vec<SearchResult> {
    let mut results = Vec::<SearchResult>::new();

    if let Some(files) = index.data.get(&keyword.to_lowercase()) {
        for (path, score) in files {
            results.push(SearchResult {
                path: path.clone(),
                score: *score,
            });
        }
    }

    results.sort_by(|a, b| b.score.cmp(&a.score));

    results
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sort_search_result() {
        let mut result = vec![
            SearchResult {
                path: "a".into(),
                score: 1,
            },
            SearchResult {
                path: "b".into(),
                score: 10,
            },
            SearchResult {
                path: "c".into(),
                score: 5,
            },
        ];

        result.sort_by(|a, b| b.score.cmp(&a.score));

        assert_eq!(result[0].score, 10,);

        assert_eq!(result[1].score, 5,);

        assert_eq!(result[2].score, 1,);
    }

    #[test]
    fn test_search_result_struct() {
        let item = SearchResult {
            path: "rust.md".into(),
            score: 8,
        };

        assert_eq!(item.path, "rust.md",);

        assert_eq!(item.score, 8,);
    }
}
