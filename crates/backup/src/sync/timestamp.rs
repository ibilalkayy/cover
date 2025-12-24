use super::sync::SyncData;
use std::{collections::BTreeMap, fs::metadata, path::PathBuf, time::UNIX_EPOCH};

impl SyncData {
    pub fn file_timestamp(&self, files: Vec<PathBuf>, trim: &PathBuf) -> BTreeMap<PathBuf, f64> {
        let mut map: BTreeMap<PathBuf, f64> = BTreeMap::new();

        for src_entry in files {
            let src_num = metadata(&src_entry)
                .ok()
                .and_then(|f| f.modified().ok())
                .and_then(|f| f.duration_since(UNIX_EPOCH).ok())
                .map(|f| f.as_secs() as f64)
                .unwrap_or(0.0);

            let src_file = src_entry
                .clone()
                .strip_prefix(trim)
                .expect("[ERROR]: failed to get the prefix")
                .to_path_buf();

            map.insert(src_file, src_num);
        }
        map
    }
}
