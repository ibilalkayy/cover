use super::sync::SyncData;
use std::{collections::HashSet, fs::read_dir, path::PathBuf};

// /// Implemenation of the helper methods used in other methods.
impl SyncData {
    pub fn prefixed_listing(&self, files: Vec<PathBuf>, parent: PathBuf) -> HashSet<PathBuf> {
        let mut prefixed_list = HashSet::new();

        for entry in files {
            if entry == parent {
                continue;
            }

            let relative_entry = entry
                .strip_prefix(&parent)
                .expect("[ERROR]: failed to prefix the file")
                .to_path_buf();

            prefixed_list.insert(relative_entry);
        }
        prefixed_list
    }

    pub fn loop_listing(&self, files: Vec<PathBuf>, parent: PathBuf) -> Vec<PathBuf> {
        let mut loop_list = Vec::new();

        for entry in files {
            if entry == parent {
                continue;
            }

            loop_list.push(entry);
        }
        loop_list
    }
}

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
    /// assert!(sync.src_dest_dir_parent_exists(), "[ERROR]: source or destination not detected");
    /// ```
    pub fn src_dest_dir_parent_exists(&self) -> bool {
        if !self.source.exists() && !self.destination.exists() {
            return false;
        }

        if !self.source.is_dir() || !self.destination.is_dir() {
            return false;
        }

        true
    }

    pub fn src_dest_dir_parent_not_empty(&self) -> bool {
        let src_not_empty = !self.source.to_string_lossy().trim().is_empty();
        let dest_not_empty = !self.destination.to_string_lossy().trim().is_empty();

        src_not_empty && dest_not_empty
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
        let mut status = false;
        let mut dir_hash: HashSet<PathBuf> = HashSet::new();
        let mut file_hash: HashSet<PathBuf> = HashSet::new();

        let src_dirs = self.list_src_dirs();
        let src_files = self.list_src_files();
        let dir_list = self.prefixed_listing(src_dirs, self.source.clone());
        let file_list = self.prefixed_listing(src_files, self.source.clone());

        for dir in dir_list {
            if !dir_hash.insert(dir) {
                status = true;
            }
        }

        for file in file_list {
            if !file_hash.insert(file) {
                status = true;
            }
        }
        status
    }

    pub fn both_files_exist(&self) -> Result<(), String> {
        let parent_exists = self.src_dest_dir_parent_exists();
        let src_exists = self.src_dir_exists() && self.src_file_exists();
        let dest_exists = self.dest_dir_exists() && self.dest_file_exists();

        if !parent_exists {
            return Err("[ERROR]: missing source or destination directories".to_string());
        }

        if !src_exists {
            return Err("[ERROR]: source file or directory does not exist".to_string());
        }

        if !dest_exists {
            return Err("[ERROR]: destination file or directory does not exist".to_string());
        }

        Ok(())
    }

    pub fn path_matched(&self) -> bool {
        let src_dest_dirname_matched = self.src_dest_dirname_matched();
        let src_dest_filename_matched = self.src_dest_filename_matched();
        let dest_src_dirname_matched = self.dest_src_dirname_matched();
        let dest_src_filename_matched = self.dest_src_filename_matched();

        // println!("{}", src_dest_dirname_matched);
        // println!("{}", src_dest_filename_matched);
        // println!("{}", dest_src_dirname_matched);
        // println!("{}", dest_src_filename_matched);

        src_dest_dirname_matched
            && src_dest_filename_matched
            && dest_src_dirname_matched
            && dest_src_filename_matched

        // if !src_dest_dirname_matched || !dest_src_dirname_matched {
        //     return Err("[ERROR]: source and destination directories do not match".to_string());
        // }

        // if !src_dest_filename_matched || !dest_src_filename_matched {
        //     return Err("[ERROR]: source and destination files do not match".to_string());
        // }
    }

    pub fn file_matched(&self) -> bool {
        let src_dest_filename_matched = self.src_dest_filename_matched();
        let dest_src_filename_matched = self.dest_src_filename_matched();

        src_dest_filename_matched && dest_src_filename_matched
    }

    pub fn dir_matched(&self) -> bool {
        let src_dest_dirname_matched = self.src_dest_dirname_matched();
        let dest_src_dirname_matched = self.dest_src_dirname_matched();

        src_dest_dirname_matched && dest_src_dirname_matched
    }

    pub fn dir_contains_files(&self, dir_list: Vec<PathBuf>) -> bool {
        let mut is_file = false;

        if dir_list.len() > 0 {
            for dir in dir_list {
                let read_dir = read_dir(dir).expect("[ERROR]: failed to read the directory");

                for entry in read_dir {
                    let entry = entry.expect("[ERROR]: failed to get the entry");
                    if entry.path().is_file() {
                        is_file = true;
                    }
                }
            }
        }
        is_file
    }
}
