use super::sync::SyncData;
use std::{fs::read_to_string, path::PathBuf};

impl SyncData {
    pub fn src_file_created(&self) -> bool {
        let dir_path = self.list_src_dirs();
        let file_path = self.list_src_files();
        let mut not_found = false;

        for entry in dir_path {
            if entry == self.source {
                continue;
            }

            let relative_dir = entry.strip_prefix(&self.source);
            match relative_dir {
                Ok(dir) => {
                    let relative_path = self.destination.join(dir);
                    if !relative_path.exists() {
                        not_found = true;
                    }
                }
                Err(e) => eprintln!("[ERROR]: {}", e),
            }
        }

        for entry in file_path {
            let relative_file = entry.strip_prefix(&self.source);
            match relative_file {
                Ok(file) => {
                    let relative_path = self.destination.join(file);
                    if !relative_path.exists() {
                        not_found = true;
                    }
                }
                Err(e) => eprintln!("[ERROR]: {}", e),
            }
        }
        not_found
    }

    pub fn src_file_modified(&self) -> (Vec<PathBuf>, bool) {
        let src_files = self.list_src_files();
        let dest_files = self.list_dest_files();

        let src_timestamp = self.file_timestamp(src_files, &self.source);
        let dest_timestamp = self.file_timestamp(dest_files, &self.destination);

        let mut modified_files: Vec<PathBuf> = Vec::new();

        for (path, src_time) in &src_timestamp {
            let dest = dest_timestamp.get(path);
            match dest {
                Some(dest_time) => {
                    if src_time > dest_time {
                        let src_path = self.source.join(path);
                        let dest_path = self.destination.join(path);

                        let src_content =
                            read_to_string(src_path).expect("[ERROR]: failed to read the file");
                        let dest_content =
                            read_to_string(dest_path).expect("[ERROR]: failed to read the file");

                        if src_content != dest_content {
                            modified_files.push(path.clone());
                        }
                    }
                }
                None => {}
            }
        }

        let file_modified = !modified_files.is_empty();
        (modified_files, file_modified)
    }

    pub fn dest_file_created(&self) -> bool {
        let dir_path = self.list_dest_dirs();
        let file_path = self.list_dest_files();
        let mut not_found = false;

        for entry in dir_path {
            if entry == self.destination {
                continue;
            }

            let relative_dir = entry.strip_prefix(&self.destination);
            match relative_dir {
                Ok(dir) => {
                    let relative_path = self.source.join(dir);
                    if !relative_path.exists() {
                        not_found = true;
                    }
                }
                Err(e) => eprintln!("[ERROR]: {}", e),
            }
        }

        for entry in file_path {
            let relative_file = entry.strip_prefix(&self.destination);
            match relative_file {
                Ok(file) => {
                    let relative_path = self.source.join(file);
                    if !relative_path.exists() {
                        not_found = true;
                    }
                }
                Err(e) => eprintln!("[ERROR]: {}", e),
            }
        }
        not_found
    }

    pub fn dest_file_modified(&self) -> (Vec<PathBuf>, bool) {
        let src_files = self.list_src_files();
        let dest_files = self.list_dest_files();

        let src_timestamp = self.file_timestamp(src_files, &self.source);
        let dest_timestamp = self.file_timestamp(dest_files, &self.destination);

        let mut modified_files: Vec<PathBuf> = Vec::new();

        for (path, dest_time) in &dest_timestamp {
            let src = src_timestamp.get(path);
            match src {
                Some(src_time) => {
                    if dest_time > src_time {
                        let src_path = self.source.join(path);
                        let dest_path = self.destination.join(path);

                        let src_content =
                            read_to_string(src_path).expect("[ERROR]: failed to read the file");
                        let dest_content =
                            read_to_string(dest_path).expect("[ERROR]: failed to read the file");

                        if src_content != dest_content {
                            modified_files.push(path.clone());
                        }
                    }
                }
                None => {}
            }
        }

        let file_modified = !modified_files.is_empty();
        (modified_files, file_modified)
    }
}
