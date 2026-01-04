use super::sync::SyncData;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

const CHUNK_SIZE: usize = 8 * 1024;

/// Implementation of the source and destination files that are created or modified.
impl SyncData {
    pub fn files_are_equal(&self, src_file: &PathBuf, dest_file: &PathBuf) -> bool {
        let src_open = File::open(src_file).expect("[ERROR]: failed to open the source file");
        let dest_open =
            File::open(dest_file).expect("[ERROR]: failed to open the destination file");

        let mut src_reader = BufReader::new(src_open);
        let mut dest_reader = BufReader::new(dest_open);

        let mut buf_src = [0u8; CHUNK_SIZE];
        let mut buf_dest = [0u8; CHUNK_SIZE];

        loop {
            let src_content = src_reader
                .read(&mut buf_src)
                .expect("[ERROR]: failed to read the file");
            let dest_content = dest_reader
                .read(&mut buf_dest)
                .expect("[ERROR]: failed to read the file");

            if src_content != dest_content {
                return false;
            }

            if src_content == 0 {
                return true;
            }

            if buf_src[..src_content] != buf_dest[..dest_content] {
                return false;
            }
        }
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
    /// let created = sync.src_file_created();
    /// assert!(created);
    /// ```
    pub fn src_file_created(&self) -> bool {
        let dir_path = self.list_src_dirs();
        let file_path = self.list_src_files();
        let mut not_found = false;

        for entry in dir_path {
            if entry == self.source {
                continue;
            }

            let relative_dir = entry.strip_prefix(&self.source);
            match relative_dir {
                Ok(dir) => {
                    let relative_path = self.destination.join(dir);
                    if !relative_path.exists() {
                        not_found = true;
                    }
                }
                Err(e) => eprintln!("[ERROR]: {}", e),
            }
        }

        for entry in file_path {
            let relative_file = entry.strip_prefix(&self.source);
            match relative_file {
                Ok(file) => {
                    let relative_path = self.destination.join(file);
                    if !relative_path.exists() {
                        not_found = true;
                    }
                }
                Err(e) => eprintln!("[ERROR]: {}", e),
            }
        }
        not_found
    }

    /// Checks whether the source file is modified or not.
    ///
    /// Returns:
    /// - The vector pathbuf for modified files.
    /// - Boolean value as modification sign.
    ///
    /// Lists the source and destination files, checks the timestamp, declares a variable for storage and get the source and destination timestamp.
    /// Finds the largest timestamp and reads the content of files. Compares the content to check the modification.
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
    /// let modified_file = PathBuf::from("filename.txt");
    /// let (file_modified, is_modified) = sync.src_file_modified();
    /// for file in file_modified {
    ///     assert_eq!(file.to_string_lossy(), modified_file.to_string_lossy());
    /// }
    /// assert!(is_modified);
    /// ```
    pub fn src_file_modified(&self) -> (Vec<PathBuf>, bool) {
        let src_files = self.list_src_files();
        let dest_files = self.list_dest_files();

        let src_timestamp = self.file_timestamp(src_files, &self.source);
        let dest_timestamp = self.file_timestamp(dest_files, &self.destination);

        let mut modified_files: Vec<PathBuf> = Vec::new();

        for (path, src_time) in &src_timestamp {
            let dest = dest_timestamp.get(path);
            match dest {
                Some(dest_time) => {
                    if src_time > dest_time {
                        let src_path = self.source.join(path);
                        let dest_path = self.destination.join(path);

                        if !self.files_are_equal(&src_path, &dest_path) {
                            modified_files.push(path.clone());
                        }
                    }
                }
                None => {}
            }
        }

        let file_modified = !modified_files.is_empty();
        (modified_files, file_modified)
    }

    /// Checks whether the destination file is created or not.
    ///
    /// Resturns:
    /// - Boolean value as a creation sign
    ///
    /// Lists the source files and directories for checking creation, finds the relative directories and files and joins them with the source.
    ///
    /// Checks the existance of those files in the source.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use cover_files::sync::sync::SyncData;
    /// use std::path::PathBuf;
    ///
    /// let sync = SyncData {
    ///     source: PathBuf::new(),
    ///     destination: PathBuf::from("destination_directory"),
    ///     changed_only: true,
    ///     delete: false,
    ///     dry_run: false,
    ///     verbose: false,
    /// };
    ///
    /// let created = sync.dest_file_created();
    /// assert!(created);
    /// ```
    pub fn dest_file_created(&self) -> bool {
        let dir_path = self.list_dest_dirs();
        let file_path = self.list_dest_files();
        let mut not_found = false;

        for entry in dir_path {
            if entry == self.destination {
                continue;
            }

            let relative_dir = entry.strip_prefix(&self.destination);
            match relative_dir {
                Ok(dir) => {
                    let relative_path = self.source.join(dir);
                    if !relative_path.exists() {
                        not_found = true;
                    }
                }
                Err(e) => eprintln!("[ERROR]: {}", e),
            }
        }

        for entry in file_path {
            let relative_file = entry.strip_prefix(&self.destination);
            match relative_file {
                Ok(file) => {
                    let relative_path = self.source.join(file);
                    if !relative_path.exists() {
                        not_found = true;
                    }
                }
                Err(e) => eprintln!("[ERROR]: {}", e),
            }
        }
        not_found
    }

    /// Checks whether the destination file is modified or not.
    ///
    /// Returns:
    /// - The vector pathbuf for modified files.
    /// - Boolean value as modification sign.
    ///
    /// Lists the source and destination files, checks the timestamp, declares a variable for storage and get the source and destination timestamp.
    /// Finds the largest timestamp and reads the content of files. Compares the content to check the modification.
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
    /// let modified_file = PathBuf::from("filename.txt");
    /// let (file_modified, is_modified) = sync.dest_file_modified();
    /// for file in file_modified {
    ///     assert_eq!(file.to_string_lossy(), modified_file.to_string_lossy());
    /// }
    /// assert!(is_modified);
    /// ```
    pub fn dest_file_modified(&self) -> (Vec<PathBuf>, bool) {
        let src_files = self.list_src_files();
        let dest_files = self.list_dest_files();

        let src_timestamp = self.file_timestamp(src_files, &self.source);
        let dest_timestamp = self.file_timestamp(dest_files, &self.destination);

        let mut modified_files: Vec<PathBuf> = Vec::new();

        for (path, dest_time) in &dest_timestamp {
            let src = src_timestamp.get(path);
            match src {
                Some(src_time) => {
                    if dest_time > src_time {
                        let src_path = self.source.join(path);
                        let dest_path = self.destination.join(path);

                        if !self.files_are_equal(&src_path, &dest_path) {
                            modified_files.push(path.clone());
                        }
                    }
                }
                None => {}
            }
        }

        let file_modified = !modified_files.is_empty();
        (modified_files, file_modified)
    }
}
