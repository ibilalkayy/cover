use std::{
    fs::{copy, remove_file},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use time::{OffsetDateTime, macros::format_description};
use walkdir::{DirEntry, WalkDir};

pub struct SyncData {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub changed_only: bool,
    pub delete: bool,
    pub dry_run: bool,
    pub verbose: bool,
    pub hash: bool,
}

fn file_size(source_entry: DirEntry) -> (f64, &'static str) {
    if let Ok(metadata) = source_entry.metadata() {
        let size = metadata.len();
        if size < 1024 {
            return (size as f64, "B");
        } else if size < 1024 * 1024 {
            return (size as f64 / 1024.0, "KB");
        } else if size < 1024 * 1024 * 1024 {
            return (size as f64 / (1024.0 * 1024.0), "MB");
        } else {
            return (size as f64 / (1024.0 * 1024.0 * 1024.0), "GB");
        };
    }
    (0.0, "")
}

fn pretty_time(t: SystemTime) -> String {
    let utc = OffsetDateTime::UNIX_EPOCH
        + time::Duration::try_from(t.duration_since(UNIX_EPOCH).unwrap()).unwrap();
    let local = utc.to_offset(time::UtcOffset::local_offset_at(utc).unwrap());
    local
        .format(&format_description!(
            "[day]-[month repr:short]-[year] [hour repr:12]:[minute]:[second] [period case:upper]\n"
        ))
        .unwrap()
}

fn file_last_modified(source_entry: DirEntry) -> Option<(i64, String)> {
    if let Ok(metadata) = source_entry.metadata() {
        if let Ok(modified) = metadata.modified() {
            let numeric = modified.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
            let pretty = pretty_time(modified);
            return Some((numeric, pretty));
        }
    }
    None
}

impl SyncData {
    pub fn sync_output(&self) {
        let mut files_to_be_copied = Vec::new();
        let mut files_to_be_updated = Vec::new();
        let mut source_files: Vec<(PathBuf, i64)> = Vec::new();
        let mut destination_files: Vec<(PathBuf, i64)> = Vec::new();

        for entry in WalkDir::new(&self.source) {
            let entry = entry.expect("Err: failed to get the source entry");
            if entry.file_type().is_file() {
                let path = entry.path().to_path_buf();
                println!("File: {}", entry.file_name().display());
                let (size, size_type) = file_size(entry.clone());
                println!("Size: {} {}", size, size_type);
                if let Some((num, pretty)) = file_last_modified(entry) {
                    source_files.push((path, num));
                    println!("Last Modified: {}", pretty);
                }
            }
        }

        for entry in WalkDir::new(&self.destination) {
            let entry = entry.expect("Err: failed to get the destination entry");
            if entry.file_type().is_file() {
                let path = entry.path().to_path_buf();
                if let Some((num, _)) = file_last_modified(entry) {
                    destination_files.push((path, num));
                }
            }
        }

        if self.changed_only {
            // Compare source and destination files
            for (src_path, src_modified) in &source_files {
                let src_file_name = src_path
                    .file_name()
                    .expect("Err: failed to get the file name(s)");
                let mut dest_path = self.destination.clone();
                dest_path.push(src_file_name);

                let mut found = false;
                let mut needs_update = false;

                for (dest_path, dest_modified) in &destination_files {
                    let dest_file_name = dest_path
                        .file_name()
                        .expect("Err: failed to get the file name(s)");
                    if src_file_name == dest_file_name {
                        found = true;
                        if src_modified > dest_modified {
                            needs_update = true;
                        }
                        break;
                    }
                }

                if !found {
                    std::fs::copy(src_path, &dest_path).expect("Err: failed to copy file(s)");
                    files_to_be_copied.push(src_file_name.to_owned());
                } else if needs_update {
                    std::fs::copy(src_path, &dest_path).expect("Err: failed to update file(s)");
                    files_to_be_updated.push(src_file_name.to_owned());
                }
            }

            if !files_to_be_copied.is_empty() {
                println!(
                    "Missing files {:?} are copied to {:?} destination",
                    files_to_be_copied, self.destination
                );
            }
            if !files_to_be_updated.is_empty() {
                println!("Files are updated: {:?}", files_to_be_updated);
            }
            if files_to_be_copied.is_empty() && files_to_be_updated.is_empty() {
                eprintln!("No missing or outdated files. Destination is up-to-date.");
            }
        } else {
            if !destination_files.is_empty() {
                for (dest_path, _) in &destination_files {
                    std::fs::remove_file(dest_path).expect("Err: failed to delete the file(s)");
                }

                let dest_dir = Path::new(&self.destination);

                for (src_path, _) in &source_files {
                    let file_name = src_path
                        .file_name()
                        .expect("Err: failed to get the file name(s)");
                    let new_dest_path = dest_dir.join(file_name);
                    copy(src_path, new_dest_path).expect("Err: failed to copy the file(s)");
                }
                println!("All the files are copied to destination successfully");
            } else {
                eprintln!("Err: destination path is empty");
            }
        }

        if self.delete {
            for (src_path, _) in &source_files {
                let src_file_name = src_path
                    .file_name()
                    .expect("Err: failed to get the file name(s)");
                let mut dest_path = self.destination.clone();
                dest_path.push(src_file_name);

                let mut found = false;

                for (dest_path, _) in &destination_files {
                    let dest_file_name = dest_path
                        .file_name()
                        .expect("Err: failed to get the file name(s)");
                    if src_file_name == dest_file_name {
                        found = true;
                        break;
                    }
                }

                if !found {
                    remove_file(dest_path).expect("Err: failed to remove the file(s)");
                }
                println!("Non-matching destination files are now deleted");
            }
        }
    }

    pub fn sync_options(&self) {
        self.sync_output();
    }
}
