use super::sync::SyncData;
use std::{
    fs::metadata,
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};

impl SyncData {
    pub fn file_duration_since(&self, file_name: &PathBuf) -> Option<(f64, f64)> {
        let src_path = Path::new(&self.source).join(file_name);
        let dest_path = Path::new(&self.destination).join(file_name);

        if !src_path.exists() {
            return None;
        }

        let src_meta = metadata(&src_path)
            .ok()
            .expect("Err: failed to get the source metadata");
        let dest_meta = metadata(&dest_path).ok();

        let src_numeric = src_meta
            .modified()
            .ok()
            .expect("Err: failed to check the modification")
            .duration_since(UNIX_EPOCH)
            .ok()
            .expect("Err: failed to get the duration")
            .as_secs() as f64;

        let dest_numeric = if let Some(dest_meta) = dest_meta {
            dest_meta
                .modified()
                .ok()
                .expect("Err: failed to check the modification")
                .duration_since(UNIX_EPOCH)
                .ok()
                .expect("Err: failed to get the duration")
                .as_secs() as f64
        } else {
            0.0
        };

        Some((src_numeric, dest_numeric))
    }
}
