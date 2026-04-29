use super::sync::SyncData;
use blake2::{Blake2s256, Digest};
use file_hashing::{get_hash_file, get_hash_folder};
use std::{collections::HashSet, io::Error, path::PathBuf};

impl SyncData {
    pub fn file_hash(&self, files: Vec<PathBuf>) -> Result<HashSet<String>, Error> {
        let mut hashes = HashSet::new();

        if files.len() > 0 {
            for file in files {
                let mut blake_hash = Blake2s256::new();
                let hash = get_hash_file(file, &mut blake_hash)?;
                hashes.insert(hash);
            }
        }
        Ok(hashes)
    }

    pub fn dir_hash(&self, directories: Vec<PathBuf>) -> Result<HashSet<String>, Error> {
        let mut hashes = HashSet::new();

        if directories.len() > 0 {
            for dir in directories {
                let mut blake_hash = Blake2s256::new();
                let hash = get_hash_folder(&dir, &mut blake_hash, 12, |_| {})?;
                hashes.insert(hash);
            }
        }
        Ok(hashes)
    }
}
