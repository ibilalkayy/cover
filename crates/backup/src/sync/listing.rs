use super::sync::SyncData;
use std::path::PathBuf;
use walkdir::WalkDir;

impl SyncData {
    pub fn list_src_files(&self) -> Vec<PathBuf> {
        let mut src_files_list = Vec::new();

        if !self.source.is_dir() {
            eprintln!(
                "[ERROR]: given source '{}' is not a directory",
                self.source.display()
            );
            return Vec::new();
        }

        for entry in WalkDir::new(&self.source) {
            let entry_path = entry
                .as_ref()
                .expect("[ERROR]: failed to get the path")
                .path()
                .to_path_buf();

            if entry_path.is_file() {
                src_files_list.push(entry_path);
            }
        }
        src_files_list
    }

    pub fn list_src_dirs(&self) -> Vec<PathBuf> {
        let mut src_dirs_list = Vec::new();

        if !self.source.is_dir() {
            eprintln!(
                "[ERROR]: given source '{}' is not a directory",
                self.source.display()
            );
            return Vec::new();
        }

        for entry in WalkDir::new(&self.source) {
            let entry_path = entry
                .as_ref()
                .expect("[ERROR]: failed to get the path")
                .path()
                .to_path_buf();

            if entry_path.is_dir() {
                src_dirs_list.push(entry_path);
            }
        }
        src_dirs_list
    }

    pub fn list_dest_files(&self) -> Vec<PathBuf> {
        let mut dest_files_list = Vec::new();

        if !self.destination.is_dir() {
            eprintln!(
                "[ERROR]: given destination '{}' is not a directory",
                self.destination.display()
            );
            return Vec::new();
        }

        for entry in WalkDir::new(&self.destination) {
            let entry_path = entry
                .as_ref()
                .expect("[ERROR]: failed to get the path")
                .path()
                .to_path_buf();

            if entry_path.is_file() {
                dest_files_list.push(entry_path);
            }
        }
        dest_files_list
    }

    pub fn list_dest_dirs(&self) -> Vec<PathBuf> {
        let mut dest_dirs_list = Vec::new();

        if !self.destination.is_dir() {
            eprintln!(
                "[ERROR]: given source '{}' is not a directory",
                self.destination.display()
            );
            return Vec::new();
        }

        for entry in WalkDir::new(&self.destination) {
            let entry_path = entry
                .as_ref()
                .expect("[ERROR]: failed to get the path")
                .path()
                .to_path_buf();

            if entry_path.is_dir() {
                dest_dirs_list.push(entry_path);
            }
        }
        dest_dirs_list
    }
}
