use std::collections::HashSet;

use super::sync::SyncData;

impl SyncData {
    pub fn src_dest_dir_present(&self) -> bool {
        let src_not_empty = !self.source.to_string_lossy().trim().is_empty();
        let dest_not_empty = !self.destination.to_string_lossy().trim().is_empty();

        if !self.source.exists() && !self.destination.exists() {
            return false;
        }

        if !self.source.is_dir() || !self.destination.is_dir() {
            return false;
        }

        if src_not_empty && dest_not_empty {
            return true;
        } else {
            return false;
        }
    }

    pub fn single_command_selected(&mut self) -> bool {
        let source_contains = !self.source.to_string_lossy().trim().is_empty();
        let destination_contains = !self.destination.to_string_lossy().trim().is_empty();

        if !self.source.is_dir() && !self.destination.is_dir() {
            return false;
        }

        if !(source_contains && destination_contains) {
            return false;
        }

        let mut count = 0;
        if self.changed_only {
            count += 1;
        }
        if self.delete {
            count += 1;
        }
        if self.dry_run {
            count += 1;
        }
        if self.verbose {
            count += 1;
        }

        count == 1
    }

    pub fn get_file_names(&self) -> (Vec<String>, Vec<String>) {
        let mut dir_list: Vec<String> = Vec::new();
        let mut file_list: Vec<String> = Vec::new();

        let src_dirs = self.list_src_dirs();
        let src_files = self.list_src_files();

        for entry in src_dirs {
            if entry == self.source {
                continue;
            }

            let mut comp = entry.components();
            comp.next();
            let path = comp.as_path();

            let path_data = path.display().to_string();
            if let Some(dir) = path_data.clone().split("/").last() {
                dir_list.push(dir.to_string());
            }
        }

        for entry in src_files {
            if entry == self.source {
                continue;
            }

            let mut comp = entry.components();
            comp.next();
            let path = comp.as_path();

            let path_data = path.display().to_string();
            if let Some(file) = path_data.clone().split("/").last() {
                file_list.push(file.to_string());
            }
        }

        (dir_list, file_list)
    }

    pub fn has_duplicates(&self) -> bool {
        let (dir_list, file_list) = self.get_file_names();
        let mut dir_hash: HashSet<String> = HashSet::new();
        let mut file_hash: HashSet<String> = HashSet::new();

        for dir in dir_list {
            if !dir_hash.insert(dir) {
                return true;
            }
        }

        for file in file_list {
            if !file_hash.insert(file) {
                return true;
            }
        }

        false
    }
}
