use super::sync::SyncData;
use std::{
    collections::HashSet,
    fs::{copy, create_dir_all, remove_dir, remove_file},
    path::PathBuf,
};

impl SyncData {
    pub fn copy_src_to_dest(&self) {
        let src_dirs = self.list_src_dirs();
        let src_files = self.list_src_files();

        for entry in src_dirs {
            if entry == self.source {
                continue;
            }

            let relative_dir = entry.strip_prefix(&self.source);
            match relative_dir {
                Ok(dir) => {
                    let relative_path = self.destination.join(dir);
                    if !relative_path.exists() {
                        create_dir_all(relative_path)
                            .expect("[ERROR]: failed to create the directories");
                    }
                }
                Err(e) => eprintln!("[ERROR]: {}", e),
            }
        }

        for entry in src_files {
            let relative_file = entry.strip_prefix(&self.source);
            match relative_file {
                Ok(file) => {
                    let relative_path = self.destination.join(file);
                    if !relative_path.exists() {
                        copy(entry, relative_path).expect("[ERROR]: failed copy the file");
                    }
                }
                Err(e) => eprintln!("[ERROR]: {}", e),
            }
        }
    }

    pub fn update_dest_file(&self, file_names: Vec<PathBuf>) {
        let src_files = self.list_src_files();
        let relative_src = src_files
            .iter()
            .map(|f| {
                f.strip_prefix(&self.source)
                    .expect("[ERROR]: failed to get the file")
                    .to_path_buf()
            })
            .collect::<Vec<PathBuf>>();

        for file in file_names {
            for relative in relative_src.clone() {
                if file == relative {
                    let src_file = self.source.join(&file);
                    let dest_file = self.destination.join(&file);

                    remove_file(&dest_file).expect("[ERROR]: failed to remove the file");
                    copy(&src_file, dest_file).expect("[ERROR]: failed to copy the file");
                }
            }
        }
    }

    pub fn remove_dest_file(&self) {
        let src_dirs = self.list_src_dirs();
        let dest_dirs = self.list_dest_dirs();
        let src_files = self.list_src_files();
        let dest_files = self.list_dest_files();
        let mut src_data: Vec<PathBuf> = Vec::new();
        let mut dest_data: Vec<PathBuf> = Vec::new();
        let mut source_data: Vec<PathBuf> = Vec::new();
        let mut destination_data: Vec<PathBuf> = Vec::new();

        // Directories
        for entry in src_dirs {
            if entry == self.source {
                continue;
            }

            let relative_dir = entry
                .strip_prefix(&self.source)
                .expect("[ERROR]: failed to get the file")
                .to_path_buf();

            src_data.push(relative_dir);
        }

        for entry in dest_dirs {
            if entry == self.destination {
                continue;
            }

            let relative_dir = entry
                .strip_prefix(&self.destination)
                .expect("[ERROR]: failed to get the file")
                .to_path_buf();

            dest_data.push(relative_dir);
        }

        let src: HashSet<PathBuf> = src_data.into_iter().collect();
        let dest: HashSet<PathBuf> = dest_data.into_iter().collect();
        let mut combine_dir = Vec::new();

        for d in dest.difference(&src) {
            let dest_path = self.destination.join(d);
            let dest_len = dest_path.as_os_str().len();
            combine_dir.push((dest_path, dest_len));
        }

        let max_dir_val = combine_dir.iter().fold(None, |acc, item| match acc {
            None => Some(item),
            Some(current_max) => {
                if item.1 > current_max.1 {
                    Some(item)
                } else {
                    Some(current_max)
                }
            }
        });

        // Files
        for entry in src_files {
            let relative_dir = entry
                .strip_prefix(&self.source)
                .expect("[ERROR]: failed to get the file")
                .to_path_buf();

            source_data.push(relative_dir);
        }

        for entry in dest_files {
            let relative_dir = entry
                .strip_prefix(&self.destination)
                .expect("[ERROR]: failed to file")
                .to_path_buf();

            destination_data.push(relative_dir);
        }

        let src: HashSet<PathBuf> = source_data.into_iter().collect();
        let dest: HashSet<PathBuf> = destination_data.into_iter().collect();
        let mut combine_file = Vec::new();

        for d in dest.difference(&src) {
            let dest_path = self.destination.join(d);
            let dest_len = dest_path.as_os_str().len();
            combine_file.push((dest_path, dest_len));
        }

        let max_file_val = combine_file.iter().fold(None, |acc, item| match acc {
            None => Some(item),
            Some(current_max) => {
                if item.1 > current_max.1 {
                    Some(item)
                } else {
                    Some(current_max)
                }
            }
        });

        match max_file_val {
            Some(file) => {
                let dest_file = file.0.clone();
                remove_file(dest_file).expect("[ERROR]: failed to remove the file");
            }
            None => {
                let dest_dir = max_dir_val
                    .expect("[ERROR]: failed to get the directory")
                    .0
                    .clone();
                remove_dir(dest_dir).expect("[ERROR]: failed to remove the directory");
            }
        }
    }

    pub fn remove_all_dest_files(&self) {
        let dest_dirs = self.list_dest_dirs();
        let dest_files = self.list_dest_files();
        let mut removed = false;

        for entry in dest_files {
            if entry == self.destination {
                continue;
            }
            if entry.exists() {
                removed = true;
                remove_file(entry).expect("[ERROR]: failed to remove the file");
            }
        }

        for entry in dest_dirs.iter().rev() {
            if *entry == self.destination {
                continue;
            }
            if entry.exists() {
                removed = true;
                remove_dir(entry).expect("[ERROR]: failed to remove the directory");
            }
        }

        if removed {
            println!("[SUCCESS]: destination file(s) successfully deleted");
        } else {
            eprintln!("[MSG]: no files are present to be removed");
        }
    }
}
