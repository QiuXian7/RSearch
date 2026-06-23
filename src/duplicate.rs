use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;

use crate::models::FileInfo;

pub fn find_duplicates(files: &[FileInfo]) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::<String, Vec<String>>::new();

    for file in files {
        if let Ok(data) = fs::read(&file.path) {
            let hash = format!("{:x}", Sha256::digest(&data));

            map.entry(hash).or_default().push(file.path.clone());
        }
    }

    map.into_iter()
        .filter(|(_, paths)| paths.len() > 1)
        .collect()
}

#[cfg(test)]
mod tests {

    use sha2::{Digest, Sha256};

    #[test]
    fn test_same_content_hash() {
        let hash1 = format!("{:x}", Sha256::digest(b"hello"));

        let hash2 = format!("{:x}", Sha256::digest(b"hello"));

        assert_eq!(hash1, hash2,);
    }

    #[test]
    fn test_different_content_hash() {
        let hash1 = format!("{:x}", Sha256::digest(b"hello"));

        let hash2 = format!("{:x}", Sha256::digest(b"world"));

        assert_ne!(hash1, hash2,);
    }
}
