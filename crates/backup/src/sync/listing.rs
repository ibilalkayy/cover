use super::sync::SyncData;
use std::{fs::read_dir, path::PathBuf};

impl SyncData {
    pub fn list_source_files(&self) -> Vec<PathBuf> {
        let mut files_list = Vec::new();

        if !self.source.is_dir() {
            eprintln!("Err: source is not a directory: {:?}", self.source);
            return Vec::new();
        }

        let paths = read_dir(&self.source).expect("Err: failed to read the directory");
        for entry in paths {
            match entry {
                Ok(entry_path) => {
                    let pathway = entry_path.path();
                    if pathway.is_file() {
                        files_list.push(pathway.to_path_buf());
                    }
                }
                Err(error) => eprintln!("Err: {}", error),
            }
        }
        return files_list;
    }

    pub fn list_destination_files(&self) -> Vec<PathBuf> {
        let mut files_list = Vec::new();

        if !self.destination.is_dir() {
            eprintln!(
                "Err: destination is not a directory: {:?}",
                self.destination
            );
            return Vec::new();
        }

        let paths = read_dir(&self.destination).expect("Err: failed to read the directory");
        for entry in paths {
            match entry {
                Ok(entry_path) => {
                    let pathway = entry_path.path();
                    if pathway.is_file() {
                        files_list.push(pathway.to_path_buf());
                    }
                }
                Err(error) => eprintln!("Err: {}", error),
            }
        }
        return files_list;
    }
}
