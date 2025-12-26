use super::sync::SyncData;
use std::path::PathBuf;
use walkdir::WalkDir;

/// Implementation for listing all the files and directories.
impl SyncData {
    /// Gets the list of source files by walking through the source.
    ///
    /// Returns:
    /// - List of source files in a vector
    ///
    /// Checks whether the source is actually a directory and walks to find the files in it.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use cover::sync::sync::SyncData;
    /// use std::path::PathBuf;
    ///
    /// let sync = SyncData {
    ///     source: PathBuf::from("source_directory"),
    ///     destination: PathBuf::new(),
    ///     changed_only: true,
    ///     delete: false,
    ///     verbose: false,
    ///     dry_run: false,
    /// };
    ///
    /// let mut searched_file: Vec<PathBuf> = Vec::new();
    /// let searched = sync.list_src_files();
    ///
    /// for file in searched {
    ///     let filename = file
    ///     .file_name()
    ///     .and_then(|f| f.to_str())
    ///     .expect("[ERROR]: failed to get the filename");
    ///
    ///     searched_file.push(PathBuf::from(filename));
    /// }
    ///
    /// assert!(searched_file.len() != 0);
    /// ```
    pub fn list_src_files(&self) -> Vec<PathBuf> {
        let mut src_files_list = Vec::new();

        if !self.source.is_dir() {
            eprintln!(
                "[ERROR]: given source '{}' is not a directory",
                self.source.display()
            );
            return Vec::new();
        }

        for entry in WalkDir::new(&self.source) {
            let entry_path = entry
                .as_ref()
                .expect("[ERROR]: failed to get the path")
                .path()
                .to_path_buf();

            if entry_path.is_file() {
                src_files_list.push(entry_path);
            }
        }
        src_files_list
    }

    /// Gets the list of sub-directories by walking through the source.
    ///
    /// Returns:
    /// - List of source directories in a vector
    ///
    /// Checks whether the source is actually a directory and walks to find the sub-directories in it.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use cover::sync::sync::SyncData;
    /// use std::path::PathBuf;
    ///
    /// let sync = SyncData {
    ///     source: PathBuf::from("source_directory"),
    ///     destination: PathBuf::new(),
    ///     changed_only: true,
    ///     delete: false,
    ///     verbose: false,
    ///     dry_run: false,
    /// };
    ///
    /// let mut searched_dir: Vec<PathBuf> = Vec::new();
    /// let searched = sync.list_src_dirs();
    ///
    /// for dir in &searched {
    /// let data = dir
    ///     .iter()
    ///     .last()
    ///     .expect("[ERROR]: failed to get the last name")
    ///     .to_string_lossy()
    ///     .to_string();
    ///
    ///     searched_dir.push(PathBuf::from(data));
    /// }
    ///
    /// assert!(searched_dir.len() != 0);
    /// ```
    pub fn list_src_dirs(&self) -> Vec<PathBuf> {
        let mut src_dirs_list = Vec::new();

        if !self.source.is_dir() {
            eprintln!(
                "[ERROR]: given source '{}' is not a directory",
                self.source.display()
            );
            return Vec::new();
        }

        for entry in WalkDir::new(&self.source) {
            let entry_path = entry
                .as_ref()
                .expect("[ERROR]: failed to get the path")
                .path()
                .to_path_buf();

            if entry_path.is_dir() {
                src_dirs_list.push(entry_path);
            }
        }
        src_dirs_list
    }

    /// Gets the list of destination files by walking through the destination.
    ///
    /// Returns:
    /// - List of destination files in a vector
    ///
    /// Checks whether the destination is actually a directory and walks to find the files in it.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use cover::sync::sync::SyncData;
    /// use std::path::PathBuf;
    ///
    /// let sync = SyncData {
    ///     source: PathBuf::new(),
    ///     destination: PathBuf::from("destination_directory"),
    ///     changed_only: true,
    ///     delete: false,
    ///     verbose: false,
    ///     dry_run: false,
    /// };
    ///
    /// let mut searched_file: Vec<PathBuf> = Vec::new();
    /// let searched = sync.list_dest_files();
    ///
    /// for file in searched {
    ///     let filename = file
    ///     .file_name()
    ///     .and_then(|f| f.to_str())
    ///     .expect("[ERROR]: failed to get the filename");
    ///
    ///     searched_file.push(PathBuf::from(filename));
    /// }
    ///
    /// assert!(searched_file.len() != 0);
    /// ```
    pub fn list_dest_files(&self) -> Vec<PathBuf> {
        let mut dest_files_list = Vec::new();

        if !self.destination.is_dir() {
            eprintln!(
                "[ERROR]: given destination '{}' is not a directory",
                self.destination.display()
            );
            return Vec::new();
        }

        for entry in WalkDir::new(&self.destination) {
            let entry_path = entry
                .as_ref()
                .expect("[ERROR]: failed to get the path")
                .path()
                .to_path_buf();

            if entry_path.is_file() {
                dest_files_list.push(entry_path);
            }
        }
        dest_files_list
    }

    /// Gets the list of sub-directories by walking through the destination.
    ///
    /// Returns:
    /// - List of destination directories in a vector
    ///
    /// Checks whether the destination is actually a directory and walks to find the sub-directories in it.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use cover::sync::sync::SyncData;
    /// use std::path::PathBuf;
    ///
    /// let sync = SyncData {
    ///     source: PathBuf::new(),
    ///     destination: PathBuf::from("destination_directory"),
    ///     changed_only: true,
    ///     delete: false,
    ///     verbose: false,
    ///     dry_run: false,
    /// };
    ///
    /// let mut searched_dir: Vec<PathBuf> = Vec::new();
    /// let searched = sync.list_dest_dirs();
    ///
    /// for dir in &searched {
    /// let data = dir
    ///     .iter()
    ///     .last()
    ///     .expect("[ERROR]: failed to get the last name")
    ///     .to_string_lossy()
    ///     .to_string();
    ///
    ///     searched_dir.push(PathBuf::from(data));
    /// }
    ///
    /// assert!(searched_dir.len() != 0);
    /// ```
    pub fn list_dest_dirs(&self) -> Vec<PathBuf> {
        let mut dest_dirs_list = Vec::new();

        if !self.destination.is_dir() {
            eprintln!(
                "[ERROR]: given source '{}' is not a directory",
                self.destination.display()
            );
            return Vec::new();
        }

        for entry in WalkDir::new(&self.destination) {
            let entry_path = entry
                .as_ref()
                .expect("[ERROR]: failed to get the path")
                .path()
                .to_path_buf();

            if entry_path.is_dir() {
                dest_dirs_list.push(entry_path);
            }
        }
        dest_dirs_list
    }
}
