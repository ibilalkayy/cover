use super::sync::SyncData;
use std::collections::HashSet;

/// Implementation of the source and destination files that are created or modified.
impl SyncData {
    pub fn src_dir_exists(&self) -> bool {
        let mut exists = false;
        let src_listing = self.list_src_dirs();
        let relative_dir = self.loop_listing(src_listing.clone(), self.source.clone());

        for dir in relative_dir {
            if dir.exists() {
                exists = true;
            }
        }
        exists
    }

    pub fn dest_dir_exists(&self) -> bool {
        let mut exists = false;
        let dest_listing = self.list_dest_dirs();
        let relative_dir = self.loop_listing(dest_listing.clone(), self.destination.clone());

        for dir in relative_dir {
            if dir.exists() {
                exists = true;
            }
        }
        exists
    }

    /// Checks whether the source file is created or not.
    ///
    /// Resturns:
    /// - Boolean value as a creation sign
    ///
    /// Lists the source files and directories for checking the creation, finds the relative directories and files and joins them with the destination.
    ///
    /// Checks the existance of those files in the destination.
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
    /// let created = sync.src_file_exists();
    /// assert!(created);
    /// ```
    pub fn src_file_exists(&self) -> bool {
        let mut exists = false;
        let src_listing = self.list_src_files();
        let relative_file = self.loop_listing(src_listing.clone(), self.source.clone());

        for file in relative_file {
            if file.exists() {
                exists = true;
            }
        }
        exists
    }

    pub fn dest_file_exists(&self) -> bool {
        let mut exists = false;
        let dest_listing = self.list_dest_files();
        let relative_file = self.loop_listing(dest_listing.clone(), self.destination.clone());

        for file in relative_file {
            if file.exists() {
                exists = true;
            }
        }
        exists
    }
}

impl SyncData {
    pub fn dest_has_this_src_dir(&self) -> bool {
        let src_listing = self.list_src_dirs();
        let dest_listing = self.list_dest_dirs();

        let src_relative_dir = self.prefixed_listing(src_listing.clone(), self.source.clone());
        let dest_relative_dir =
            self.prefixed_listing(dest_listing.clone(), self.destination.clone());

        for path in src_relative_dir {
            if !dest_relative_dir.contains(&path) {
                return false;
            }
        }
        true
    }

    pub fn dest_has_this_src_file(&self) -> bool {
        let src_listing = self.list_src_files();
        let dest_listing = self.list_dest_files();

        let src_relative_file = self.prefixed_listing(src_listing.clone(), self.source.clone());
        let dest_relative_file =
            self.prefixed_listing(dest_listing.clone(), self.destination.clone());

        for path in src_relative_file {
            if !dest_relative_file.contains(&path) {
                return false;
            }
        }
        true
    }

    pub fn dest_missing_this_src_file(&self) -> bool {
        let src_files = self.prefixed_listing(self.list_src_files(), self.source.clone());
        let dest_files = self.prefixed_listing(self.list_dest_files(), self.destination.clone());

        for file in &src_files {
            if !dest_files.contains(file) {
                return true;
            }
        }

        false
    }

    pub fn dest_missing_this_src_dir(&self) -> bool {
        let src_dirs = self.prefixed_listing(self.list_src_dirs(), self.source.clone());
        let dest_dirs = self.prefixed_listing(self.list_dest_dirs(), self.destination.clone());

        for dir in &src_dirs {
            if !dest_dirs.contains(dir) {
                return true;
            }
        }

        false
    }

    pub fn src_dest_dirname_matched(&self) -> bool {
        let mut matched = false;
        let src_listing = self.list_src_dirs();
        let dest_listing = self.list_dest_dirs();

        let src_relative_dir = self.prefixed_listing(src_listing.clone(), self.source.clone());
        let dest_relative_dir =
            self.prefixed_listing(dest_listing.clone(), self.destination.clone());

        let difference: HashSet<_> = src_relative_dir.difference(&dest_relative_dir).collect();
        if difference.is_empty() {
            matched = true;
        }
        matched
    }

    pub fn dest_src_dirname_matched(&self) -> bool {
        let mut matched = false;
        let src_listing = self.list_src_dirs();
        let dest_listing = self.list_dest_dirs();

        let src_relative_dir = self.prefixed_listing(src_listing.clone(), self.source.clone());
        let dest_relative_dir =
            self.prefixed_listing(dest_listing.clone(), self.destination.clone());

        let difference: HashSet<_> = dest_relative_dir.difference(&src_relative_dir).collect();
        if difference.is_empty() {
            matched = true;
        }
        matched
    }

    pub fn src_dest_filename_matched(&self) -> bool {
        let mut matched = false;

        let src_listing = self.list_src_files();
        let dest_listing = self.list_dest_files();

        let src_relative_file = self.prefixed_listing(src_listing.clone(), self.source.clone());
        let dest_relative_file =
            self.prefixed_listing(dest_listing.clone(), self.destination.clone());

        let difference: HashSet<_> = src_relative_file.difference(&dest_relative_file).collect();

        if difference.is_empty() {
            matched = true;
        }
        matched
    }

    pub fn dest_src_filename_matched(&self) -> bool {
        let mut matched = false;

        let src_listing = self.list_src_files();
        let dest_listing = self.list_dest_files();

        let src_relative_file = self.prefixed_listing(src_listing.clone(), self.source.clone());
        let dest_relative_file =
            self.prefixed_listing(dest_listing.clone(), self.destination.clone());

        let difference: HashSet<_> = dest_relative_file.difference(&src_relative_file).collect();
        if difference.is_empty() {
            matched = true;
        }
        matched
    }

    // pub fn src_has_extra_files(&self) -> bool {
    //     let src_listing = self.list_src_files();
    //     let dest_listing = self.list_dest_files();

    //     let src_relative_file = self.prefixed_listing(src_listing.clone(), self.source.clone());
    //     let dest_relative_file =
    //         self.prefixed_listing(dest_listing.clone(), self.destination.clone());

    //     src_relative_file
    //         .difference(&dest_relative_file)
    //         .next()
    //         .is_some()
    // }

    pub fn hashes_matched(&self) -> bool {
        let src_list = self.list_src_files();
        let dest_list = self.list_dest_files();

        let src_dir_list = self.list_src_dirs();
        let dest_dir_list = self.list_dest_dirs();

        let src_file_listing = self.loop_listing(src_list.clone(), self.source.clone());
        let dest_file_listing = self.loop_listing(dest_list.clone(), self.destination.clone());
        let src_dir_listing = self.loop_listing(src_dir_list, self.source.clone());
        let dest_dir_listing = self.loop_listing(dest_dir_list, self.destination.clone());

        let src_file_hash = self
            .file_hash(src_file_listing.clone())
            .expect("[ERROR]: failed to get the hash");

        let dest_file_hash = self
            .file_hash(dest_file_listing.clone())
            .expect("[ERROR]: failed to get the hash");

        let src_dir_hash = self
            .dir_hash(src_dir_listing.clone())
            .expect("[ERROR]: failed to get the hash");

        let dest_dir_hash = self
            .dir_hash(dest_dir_listing.clone())
            .expect("[ERROR]: failed to get the hash");

        let dir_difference1: HashSet<_> = src_dir_hash.difference(&dest_dir_hash).collect();
        let file_difference2: HashSet<_> = src_file_hash.difference(&dest_file_hash).collect();
        let dir_difference3: HashSet<_> = dest_dir_hash.difference(&src_dir_hash).collect();
        let file_difference4: HashSet<_> = dest_file_hash.difference(&src_file_hash).collect();

        dir_difference1.is_empty()
            && file_difference2.is_empty()
            && dir_difference3.is_empty()
            && file_difference4.is_empty()
    }
}

impl SyncData {
    // Check two things.
    // 1. The file exist functionality should be adjusted because it forces the destination files existance that prevents the copying of files.
    // 2. Copying is only happening when all the files from the destination is removed. Make sure to write the test in such a way that it still rejects the new file even if
    // other files are present in the destination.

    // pub fn do_copy(&self) -> bool {
    //     self.src_dest_dir_parent_exists()
    //         && ((self.src_dir_exists() && !self.dest_has_this_src_dir())
    //             || (self.src_file_exists() && !self.dest_has_this_src_file()))
    // }

    pub fn do_copy(&self) -> bool {
        self.src_dest_dir_parent_exists()
            && ((self.src_dir_exists() && self.dest_missing_this_src_dir())
                || (self.src_file_exists() && self.dest_missing_this_src_file()))
    }

    pub fn do_rename(&self) -> bool {
        println!("{}", self.src_dest_dir_parent_exists());
        println!("{}", self.src_file_exists());
        println!("{}", self.dest_file_exists());
        println!("{}", self.dest_has_this_src_file());
        println!("{}", self.hashes_matched());

        println!("{}", self.src_dest_dir_parent_exists());
        println!("{}", self.src_dir_exists());
        println!("{}", self.dest_dir_exists());
        println!("{}", self.dest_has_this_src_dir());
        println!("{}", self.hashes_matched());

        self.src_dest_dir_parent_exists()
            && ((self.src_file_exists()
                    && self.dest_file_exists()
                    && !self.dest_has_this_src_file()   // not at same path
                    && self.hashes_matched())
                || (self.src_dir_exists()
                    && self.dest_dir_exists()
                    && !self.dest_has_this_src_dir()
                    && self.hashes_matched()))
    }

    pub fn do_update(&self) -> bool {
        self.src_dest_dir_parent_not_empty()
            && self.both_files_exist().is_ok()
            && self.path_matched()
            && !self.hashes_matched()
    }

    pub fn do_delete(&self) -> bool {
        self.src_dest_dir_parent_exists()
            && (!self.src_dir_exists() || !self.src_file_exists())
            && (self.dest_dir_exists() || self.dest_file_exists())
            && (!self.dir_matched() || !self.file_matched())
    }

    pub fn do_nothing(&self) -> bool {
        (self.both_files_exist().is_ok()
            && self.src_dest_dir_parent_not_empty()
            && self.path_matched()
            && self.hashes_matched())
            || self.both_files_exist().is_ok() && !self.src_dest_dir_parent_not_empty()
    }

    // /// Checks whether the source file is modified or not.
    // ///
    // /// Returns:
    // /// - The vector pathbuf for modified files.
    // /// - Boolean value as modification sign.
    // ///
    // /// Lists the source and destination files, checks the timestamp, declares a variable for storage and get the source and destination timestamp.
    // /// Finds the largest timestamp and reads the content of files. Compares the content to check the modification.
    // ///
    // /// # Example
    // ///
    // /// ```rust,no_run
    // /// use cover_files::sync::sync::SyncData;
    // /// use std::path::PathBuf;
    // ///
    // /// let sync = SyncData {
    // ///     source: PathBuf::from("source_directory"),
    // ///     destination: PathBuf::from("destination_directory"),
    // ///     changed_only: true,
    // ///     delete: false,
    // ///     dry_run: false,
    // ///     verbose: false,
    // /// };
    // ///
    // /// let modified_file = PathBuf::from("filename.txt");
    // /// let (file_modified, is_modified) = sync.src_file_modified();
    // /// for file in file_modified {
    // ///     assert_eq!(file.to_string_lossy(), modified_file.to_string_lossy());
    // /// }
    // /// assert!(is_modified);
    // /// ```
    // pub fn src_file_modified(&self) -> (Vec<PathBuf>, bool) {
    //     let src_files = self.list_src_files();
    //     let dest_files = self.list_dest_files();

    //     let src_timestamp = self.file_timestamp(src_files, &self.source);
    //     let dest_timestamp = self.file_timestamp(dest_files, &self.destination);

    //     let mut modified_files: Vec<PathBuf> = Vec::new();

    //     for (path, src_time) in &src_timestamp {
    //         let dest = dest_timestamp.get(path);
    //         match dest {
    //             Some(dest_time) => {
    //                 if src_time > dest_time {
    //                     let src_path = self.source.join(path);
    //                     let dest_path = self.destination.join(path);

    //                     if !self.files_are_equal(&src_path, &dest_path) {
    //                         modified_files.push(path.clone());
    //                     }
    //                 }
    //             }
    //             None => {}
    //         }
    //     }

    //     let file_modified = !modified_files.is_empty();
    //     (modified_files, file_modified)
    // }

    // /// Checks whether the destination file is modified or not.
    // ///
    // /// Returns:
    // /// - The vector pathbuf for modified files.
    // /// - Boolean value as modification sign.
    // ///
    // /// Lists the source and destination files, checks the timestamp, declares a variable for storage and get the source and destination timestamp.
    // /// Finds the largest timestamp and reads the content of files. Compares the content to check the modification.
    // ///
    // /// # Example
    // ///
    // /// ```rust,no_run
    // /// use cover_files::sync::sync::SyncData;
    // /// use std::path::PathBuf;
    // ///
    // /// let sync = SyncData {
    // ///     source: PathBuf::from("source_directory"),
    // ///     destination: PathBuf::from("destination_directory"),
    // ///     changed_only: true,
    // ///     delete: false,
    // ///     dry_run: false,
    // ///     verbose: false,
    // /// };
    // ///
    // /// let modified_file = PathBuf::from("filename.txt");
    // /// let (file_modified, is_modified) = sync.dest_file_modified();
    // /// for file in file_modified {
    // ///     assert_eq!(file.to_string_lossy(), modified_file.to_string_lossy());
    // /// }
    // /// assert!(is_modified);
    // /// ```
    // pub fn dest_file_modified(&self) -> (Vec<PathBuf>, bool) {
    //     let src_files = self.list_src_files();
    //     let dest_files = self.list_dest_files();

    //     let src_timestamp = self.file_timestamp(src_files, &self.source);
    //     let dest_timestamp = self.file_timestamp(dest_files, &self.destination);

    //     let mut modified_files: Vec<PathBuf> = Vec::new();

    //     for (path, dest_time) in &dest_timestamp {
    //         let src = src_timestamp.get(path);
    //         match src {
    //             Some(src_time) => {
    //                 if dest_time > src_time {
    //                     let src_path = self.source.join(path);
    //                     let dest_path = self.destination.join(path);

    //                     if !self.files_are_equal(&src_path, &dest_path) {
    //                         modified_files.push(path.clone());
    //                     }
    //                 }
    //             }
    //             None => {}
    //         }
    //     }

    //     let file_modified = !modified_files.is_empty();
    //     (modified_files, file_modified)
    // }
}
