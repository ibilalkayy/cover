use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use time::{OffsetDateTime, macros::format_description};
use walkdir::{DirEntry, WalkDir};

pub fn file_size(source_entry: DirEntry) -> (f64, &'static str) {
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

pub fn pretty_time(t: SystemTime) -> String {
    let utc = OffsetDateTime::UNIX_EPOCH
        + time::Duration::try_from(t.duration_since(UNIX_EPOCH).unwrap()).unwrap();
    let local = utc.to_offset(time::UtcOffset::local_offset_at(utc).unwrap());
    local
        .format(&format_description!(
            "[day]-[month repr:short]-[year] [hour repr:12]:[minute]:[second] [period case:upper]\n"
        ))
        .unwrap()
}

fn file_last_modified(dir_entry: DirEntry) -> Option<(i64, String)> {
    if let Ok(metadata) = dir_entry.metadata() {
        if let Ok(modified) = metadata.modified() {
            let numeric = modified.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
            let pretty = pretty_time(modified);
            return Some((numeric, pretty));
        }
    }
    None
}

pub fn source_listing(source: PathBuf) -> Vec<(PathBuf, i64, String, (f64, &'static str))> {
    let mut source_files = Vec::new();

    for entry in WalkDir::new(&source) {
        let entry = entry.expect("Err: failed to get the source entry");
        if entry.file_type().is_file() {
            let path = entry.path().to_path_buf();
            let (size, size_type) = file_size(entry.clone());

            if let Some((num, pretty)) = file_last_modified(entry) {
                source_files.push((path, num, pretty, (size, size_type)));
            }
        }
    }
    source_files
}

pub fn destination_listing(destination: PathBuf, destination_files: &mut Vec<(PathBuf, i64)>) {
    for entry in WalkDir::new(&destination) {
        let entry = entry.expect("Err: failed to get the destination entry");
        if entry.file_type().is_file() {
            let path = entry.path().to_path_buf();
            if let Some((num, _)) = file_last_modified(entry) {
                destination_files.push((path, num));
            }
        }
    }
}

pub fn take_strip<'a>(path: &'a PathBuf, base: &PathBuf) -> &'a Path {
    let real_path = path
        .strip_prefix(base)
        .expect("Err: failed to get the file name(s)");
    return real_path;
}

pub fn create_temp_file(dir: &PathBuf, name: &str, content: &str) -> PathBuf {
    let file_path = dir.join(name);
    let mut file = File::create(&file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file_path
}
