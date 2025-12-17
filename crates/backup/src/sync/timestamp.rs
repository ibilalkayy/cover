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

    // pub fn file_duration_since(&self, file_name: &PathBuf) -> Option<(f64, f64)> {
    //     let src_path = Path::new(&self.source).join(file_name);
    //     let dest_path = Path::new(&self.destination).join(file_name);

    //     if !src_path.exists() {
    //         return None;
    //     }

    //     let src_meta = metadata(&src_path)
    //         .ok()
    //         .expect("[ERROR]: failed to get the source metadata");
    //     let dest_meta = metadata(&dest_path).ok();

    //     let src_numeric = src_meta
    //         .modified()
    //         .ok()
    //         .expect("[ERROR]: failed to check the modification")
    //         .duration_since(UNIX_EPOCH)
    //         .ok()
    //         .expect("[ERROR]: failed to get the duration")
    //         .as_secs() as f64;

    //     let dest_numeric = if let Some(dest_meta) = dest_meta {
    //         dest_meta
    //             .modified()
    //             .ok()
    //             .expect("[ERROR]: failed to check the modification")
    //             .duration_since(UNIX_EPOCH)
    //             .ok()
    //             .expect("[ERROR]: failed to get the duration")
    //             .as_secs() as f64
    //     } else {
    //         0.0
    //     };

    //     Some((src_numeric, dest_numeric))
    // }
}
