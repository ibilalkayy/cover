use super::sync::SyncData;
use std::{
    fs::{read_dir, read_to_string},
    path::{Path, PathBuf},
};

impl SyncData {
    pub fn src_file_created(&self) -> bool {
        let src_entries = read_dir(&self.source).expect("[ERROR]: failed to read the source dir");
        for entry in src_entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            let dest_path = Path::new(&self.destination).join(&file_name);
            if !dest_path.exists() {
                return true;
            }
        }
        false
    }

    pub fn src_file_modified(&self) -> (PathBuf, bool) {
        let mut is_modified = false;
        let mut modified_file = PathBuf::new();
        let mut last_modify_numeric: Vec<f64> = Vec::new();

        let src_files = self.list_source_files();
        let dest_files = self.list_destination_files();

        for src_file in src_files {
            for dest_file in &dest_files {
                let src_file_content =
                    read_to_string(&src_file).expect("[ERROR]: failed to read the file");
                let dest_file_content =
                    read_to_string(&dest_file).expect("[ERROR]: failed to read the file");

                let file_name = dest_file
                    .file_name()
                    .expect("[ERROR]: failed to get the file name");

                if let Some((src_time, dest_time)) =
                    self.file_duration_since(&PathBuf::from(&file_name))
                {
                    if src_time > dest_time
                        && dest_time != 0.0
                        && src_file_content != dest_file_content
                    {
                        last_modify_numeric.push(src_time);
                        if let Some(max_value) = last_modify_numeric
                            .iter()
                            .cloned()
                            .fold(None, |max, val| Some(max.map_or(val, |m: f64| m.max(val))))
                        {
                            if max_value == src_time {
                                modified_file = PathBuf::from(&file_name);
                                is_modified = true;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
        (modified_file, is_modified)
    }

    pub fn dest_file_created(&self) -> bool {
        let dest_entries =
            read_dir(&self.destination).expect("[ERROR]: failed to read destination dir");
        for entry in dest_entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            let src_path = Path::new(&self.source).join(&file_name);
            if !src_path.exists() {
                return true;
            }
        }
        false
    }

    pub fn dest_file_modified(&self) -> (PathBuf, bool) {
        let mut is_modified = false;
        let mut modified_file = PathBuf::new();
        let mut last_modify_numeric: Vec<f64> = Vec::new();

        let src_files = self.list_source_files();
        let dest_files = self.list_destination_files();

        for src_file in src_files {
            for dest_file in &dest_files {
                let src_file_content =
                    read_to_string(&src_file).expect("[ERROR]: failed to read the file");
                let dest_file_content =
                    read_to_string(&dest_file).expect("[ERROR]: failed to read the file");

                let file_name = dest_file
                    .file_name()
                    .expect("[ERROR]: failed to get the file name");

                if let Some((src_time, dest_time)) =
                    self.file_duration_since(&PathBuf::from(&file_name))
                {
                    if dest_time > src_time
                        && src_time != 0.0
                        && src_file_content != dest_file_content
                    {
                        last_modify_numeric.push(dest_time);
                        if let Some(max_value) = last_modify_numeric
                            .iter()
                            .cloned()
                            .fold(None, |max, val| Some(max.map_or(val, |m: f64| m.max(val))))
                        {
                            if max_value == dest_time {
                                modified_file = PathBuf::from(&file_name);
                                is_modified = true;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
        (modified_file, is_modified)
    }
}
