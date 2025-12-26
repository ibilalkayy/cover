use super::sync::SyncData;
use std::{collections::BTreeMap, fs::metadata, path::PathBuf, time::UNIX_EPOCH};

/// Implementation for finding the timestamp of a file.
impl SyncData {
    /// Finds the last modified file and it's timestamp
    ///
    /// Takes:
    /// - List of files
    /// - Directory name to trim from
    ///
    /// Returns:
    /// - Actual file which is modified at the end
    /// - Float timestamp of that file
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use cover_files::sync::sync::SyncData;
    /// use std::path::PathBuf;
    ///
    /// let sync = SyncData {
    ///     source: PathBuf::from("source_directory"),
    ///     destination: PathBuf::from("destination_directory"),
    ///     changed_only: true,
    ///     delete: false,
    ///     verbose: false,
    ///     dry_run: false,
    /// };
    ///
    /// let src_files_list = sync.list_src_files();
    /// let dest_files_list = sync.list_dest_files();
    ///
    /// let src_timestamp = sync.file_timestamp(src_files_list.clone(), &sync.source);
    /// let dest_timestamp = sync.file_timestamp(dest_files_list.clone(), &sync.destination);
    ///
    /// for (path, dest_time) in &dest_timestamp {
    ///     let src = src_timestamp.get(path);
    ///     match src {
    ///         Some(src_time) => {
    ///             assert!(*src_time != 0.0 && *dest_time != 0.0);
    ///         }
    ///         None => {}
    ///     }
    /// }
    /// ```
    pub fn file_timestamp(&self, files: Vec<PathBuf>, trim: &PathBuf) -> BTreeMap<PathBuf, f64> {
        let mut map: BTreeMap<PathBuf, f64> = BTreeMap::new();

        for entry in files {
            let num = metadata(&entry)
                .ok()
                .and_then(|f| f.modified().ok())
                .and_then(|f| f.duration_since(UNIX_EPOCH).ok())
                .map(|f| f.as_secs() as f64)
                .unwrap_or(0.0);

            let file = entry
                .clone()
                .strip_prefix(trim)
                .expect("[ERROR]: failed to get the prefix")
                .to_path_buf();

            map.insert(file, num);
        }
        map
    }
}
