use std::time::SystemTime;
use walkdir::{DirEntry, WalkDir};

pub struct SyncData {
    pub source: String,
    pub destination: String,
    pub incremental: Option<bool>,
    pub delete: Option<bool>,
    pub dry_run: Option<bool>,
    pub verbose: Option<bool>,
    pub hash: Option<bool>,
}

fn file_name(source_entry: DirEntry) {
    if let Some(name) = source_entry.path().file_name() {
        println!("File: {}", name.to_string_lossy());
    }
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
        for source_entry in WalkDir::new(&self.source) {
            let source_entry = source_entry.expect("Err: failed to get the source entry");
            let file_type = source_entry.file_type();

            if file_type.is_file() {
                file_name(source_entry.clone());
                file_types(source_entry.clone());
                file_last_modified(source_entry);
            }
        }

        for destination_entry in WalkDir::new(&self.destination) {
            let destination_entry =
                destination_entry.expect("Err: failed to get the destination entry");
            let file_type = destination_entry.file_type();

            if file_type.is_file() {
                file_name(destination_entry.clone());
                file_types(destination_entry.clone());
                file_last_modified(destination_entry);
            }
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
