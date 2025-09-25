use std::{path::PathBuf, time::SystemTime};
use walkdir::{DirEntry, WalkDir};

pub struct SyncData {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub incremental: Option<bool>,
    pub delete: Option<bool>,
    pub dry_run: Option<bool>,
    pub verbose: Option<bool>,
    pub hash: Option<bool>,
}

fn file_types(source_entry: DirEntry) {
    if let Ok(metadata) = source_entry.metadata() {
        let size = metadata.len();
        let file_size = if size < 1024 {
            format!("{} B", size)
        } else if size < 1024 * 1024 {
            format!("{:.2} KB", size as f64 / 1024.0)
        } else if size < 1024 * 1024 * 1024 {
            format!("{:.2} MB", size as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
        };
        println!("Size: {:?}", file_size);
    }
}

fn pretty_time(t: SystemTime) {
    let utc = time::OffsetDateTime::UNIX_EPOCH
        + time::Duration::try_from(t.duration_since(std::time::UNIX_EPOCH).unwrap()).unwrap();
    let local = utc.to_offset(time::UtcOffset::local_offset_at(utc).unwrap());
    local
        .format_into(
            &mut std::io::stdout().lock(),
            time::macros::format_description!(
                "[day]-[month repr:short]-[year] [hour]:[minute]:[second]\n"
            ),
        )
        .unwrap();
}

fn file_last_modified(source_entry: DirEntry) {
    if let Ok(time) = source_entry.metadata() {
        let last_modified = time
            .modified()
            .expect("Err: failed to get the last modified time");
        pretty_time(last_modified);
    }
}

impl SyncData {
    pub fn sync_output(&self) {
        let mut source_files: Vec<PathBuf> = Vec::new();
        for entry in WalkDir::new(&self.source) {
            let entry = entry.expect("Err: failed to get the source entry");
            if entry.file_type().is_file() {
                source_files.push(entry.path().to_path_buf());
                // file_types(source_entry.clone());
                // file_last_modified(source_entry);
            }
        }

        let mut destination_files: Vec<PathBuf> = Vec::new();
        for entry in WalkDir::new(&self.destination) {
            let entry = entry.expect("Err: failed to get the destination entry");
            if entry.file_type().is_file() {
                destination_files.push(entry.path().to_path_buf());
                // file_types(entry.clone());
                // file_last_modified(entry);
            }
        }

        let mut copied_files = Vec::new();
        for src in &source_files {
            let src_file_name = src.file_name().expect("Err: failed to get the file name");

            let mut dest_path = self.destination.clone();
            dest_path.push(src_file_name);

            let mut found = false;
            for dest in &destination_files {
                let dest_file_name = dest.file_name().expect("Err: failed to get the file name");
                if src_file_name == dest_file_name {
                    found = true;
                }
            }

            if !found {
                std::fs::copy(src, dest_path).unwrap();
                copied_files.push(src_file_name.to_owned());
            }
        }

        if !copied_files.is_empty() {
            println!("Missing files are copied successfully");
        } else {
            eprintln!("No missing files to copy. Destination is up-to-date.");
        }

        // println!("{}", self.incremental.unwrap());
        // println!("{}", self.delete.unwrap());
        // println!("{}", self.dry_run.unwrap());
        // println!("{}", self.verbose.unwrap());
        // println!("{}", self.hash.unwrap());
    }

    pub fn sync_options(&self) {
        self.sync_output();
    }
}
