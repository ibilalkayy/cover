use super::sync::SyncData;
use std::{collections::HashSet, path::PathBuf};

/// Implemenation of the helper methods used in other methods.
impl SyncData {
    /// Checks the source and destination directories presence.
    ///
    /// Returns:
    /// - Boolean to see the directories are not empty.
    ///
    /// Verifiies the existance of source and destination and also used as directories.
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
    ///     dry_run: false,
    ///     verbose: false,
    /// };
    ///
    /// assert!(sync.src_dest_dir_present(), "[ERROR]: source or destination not detected");
    /// ```
    pub fn src_dest_dir_present(&self) -> bool {
        let src_not_empty = !self.source.to_string_lossy().trim().is_empty();
        let dest_not_empty = !self.destination.to_string_lossy().trim().is_empty();

        if !self.source.exists() && !self.destination.exists() {
            return false;
        }

        if !self.source.is_dir() || !self.destination.is_dir() {
            return false;
        }

        if src_not_empty && dest_not_empty {
            return true;
        } else {
            return false;
        }
    }

    /// Allows only one flag after the source and destination.
    ///
    /// Returns:
    /// - Boolean to check the selection.
    ///
    /// Checks the source and destination as directories and count the flag if they are given.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use cover_files::sync::sync::SyncData;
    /// use std::path::PathBuf;
    ///
    /// let mut sync = SyncData {
    ///     source: PathBuf::from("source_directory"),
    ///     destination: PathBuf::from("destination_directory"),
    ///     changed_only: true,
    ///     delete: false,
    ///     dry_run: false,
    ///     verbose: false,
    /// };
    ///
    /// assert!(sync.single_command_selected(), "[ERROR]: expected one command, but multiple are reported");
    /// ```
    pub fn single_command_selected(&mut self) -> bool {
        let source_contains = !self.source.to_string_lossy().trim().is_empty();
        let destination_contains = !self.destination.to_string_lossy().trim().is_empty();

        if !self.source.is_dir() && !self.destination.is_dir() {
            return false;
        }

        if !(source_contains && destination_contains) {
            return false;
        }

        let mut count = 0;
        if self.changed_only {
            count += 1;
        }
        if self.delete {
            count += 1;
        }
        if self.dry_run {
            count += 1;
        }
        if self.verbose {
            count += 1;
        }

        count == 1
    }

    /// Gets the file and directory names for checking the duplication.
    ///
    /// Returns:
    /// - Directories list
    /// - Files list
    ///
    /// After listing the files and directories, splits the full path of them and takes only the last name to return.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use cover_files::sync::sync::SyncData;
    /// use std::path::PathBuf;
    ///
    /// let sync = SyncData {
    ///     source: PathBuf::from("source_directory"),
    ///     destination: PathBuf::new(),
    ///     changed_only: true,
    ///     delete: false,
    ///     dry_run: false,
    ///     verbose: false,
    /// };
    ///
    /// let (dir_list, file_list) = sync.get_file_names();
    /// assert!(dir_list.len() != 0 && file_list.len() != 0);
    /// ```
    pub fn get_file_names(&self) -> (Vec<PathBuf>, Vec<PathBuf>) {
        let mut dir_list: Vec<PathBuf> = Vec::new();
        let mut file_list: Vec<PathBuf> = Vec::new();

        let src_dirs = self.list_src_dirs();
        let src_files = self.list_src_files();

        for entry in src_dirs {
            if entry == self.source {
                continue;
            }

            let mut comp = entry.components();
            comp.next();
            let path = comp.as_path();

            let path_data = path.display().to_string();
            if let Some(dir) = path_data.clone().split("/").last() {
                dir_list.push(PathBuf::from(dir));
            }
        }

        for entry in src_files {
            if entry == self.source {
                continue;
            }

            let mut comp = entry.components();
            comp.next();
            let path = comp.as_path();

            let path_data = path.display().to_string();
            if let Some(file) = path_data.clone().split("/").last() {
                file_list.push(PathBuf::from(file));
            }
        }

        (dir_list, file_list)
    }

    /// It prevents the duplication of files and directories to be inserted.
    ///
    /// Returns:
    /// - Boolean to check if the files or directories has duplicates.
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
    ///     dry_run: false,
    ///     verbose: false,
    /// };
    ///
    /// let result = sync.has_duplicates();
    /// assert!(result == true);
    /// ```
    pub fn has_duplicates(&self) -> bool {
        let mut dir_hash: HashSet<PathBuf> = HashSet::new();
        let mut file_hash: HashSet<PathBuf> = HashSet::new();
        let (dir_list, file_list) = self.get_file_names();

        for dir in dir_list {
            if !dir_hash.insert(dir) {
                return true;
            }
        }

        for file in file_list {
            if !file_hash.insert(file) {
                return true;
            }
        }

        false
    }
}
