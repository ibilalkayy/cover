use std::path::PathBuf;

/// Requires the data for generating the output after running the commands.
pub struct SyncData {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub changed_only: bool,
    pub delete: bool,
    pub dry_run: bool,
    pub verbose: bool,
}

/// Points to the states that needs to be present.
///
/// Only one action is required to be hit at the time.
pub enum FileState {
    /// Checks if the source files are created
    SrcCreated,
    /// Checks if the source files are modified
    SrcModified,
    /// Checks if the destination files are created
    DestCreated,
    /// Checks if the destination files are modified
    DestModified,
    /// Checks if no changes happened
    NoChange,
}

/// Points to the actions that needs to be taken.
pub enum FileAction {
    /// Copy only the changed files
    ChangedOnly,
    /// Show detailed logs
    Verbose,
    /// Show what would happen after syncing
    DryRun,
    /// Remove files in destination not in source
    Delete,
}

/// Implementation for the output that will be generated after running the command.
impl SyncData {
    fn to_action(&self) -> FileAction {
        if self.changed_only {
            FileAction::ChangedOnly
        } else if self.verbose {
            FileAction::Verbose
        } else if self.dry_run {
            FileAction::DryRun
        } else if self.delete {
            FileAction::Delete
        } else {
            FileAction::Verbose
        }
    }

    fn to_state(&self, condition: &mut [bool]) -> FileState {
        if condition[0] {
            FileState::SrcCreated
        } else if condition[1] {
            FileState::SrcModified
        } else if condition[2] {
            FileState::DestCreated
        } else if condition[3] {
            FileState::DestModified
        } else {
            FileState::NoChange
        }
    }

    fn file_status(&self) -> (Vec<PathBuf>, Vec<PathBuf>, FileState) {
        let src_created = self.src_file_created();
        let (modified_src_file, src_modified) = self.src_file_modified();

        let dest_created = self.dest_file_created();
        let (modified_dest_file, dest_modified) = self.dest_file_modified();

        let mut condition: [bool; 4] = [src_created, src_modified, dest_created, dest_modified];
        let state = self.to_state(&mut condition);

        (modified_src_file, modified_dest_file, state)
    }

    /// Runs the sync operation between the source and destination.
    ///
    /// This function validates the selected options and executes the
    /// suitble sync action (changed-only, verbose, dry-run, or delete).
    /// Any errors or status messages are printed to standard output.
    pub fn sync_output(&mut self) {
        if !self.src_dest_dir_present() {
            eprintln!("[ERROR]: missing source or destination directories");
            return;
        }

        if !self.single_command_selected() {
            eprintln!(
                "[ERROR]: no or multiple option(s) are selected. See 'cargo run sync --help'"
            );
            return;
        }

        if self.has_duplicates() {
            eprintln!("[ERROR]: duplicate files and directories are not allowed");
            return;
        }

        let action = self.to_action();
        match action {
            FileAction::ChangedOnly => {
                let (modified_src_file, modified_dest_file, state) = self.file_status();
                match state {
                    FileState::SrcCreated => {
                        self.copy_src_to_dest();
                        println!("[SUCCESS]: successfully copied source file(s)");
                    }
                    FileState::SrcModified => {
                        self.update_dest_file(modified_src_file);
                        println!("[SUCCESS]: successfully updated destination file(s)");
                    }
                    FileState::DestCreated => {
                        self.remove_dest_file();
                        println!(
                            "[MSG]: File(s) removed. File creation is not allowed in the destination"
                        );
                    }
                    FileState::DestModified => {
                        self.update_dest_file(modified_dest_file);
                        println!(
                            "[MSG]: File(s) overwritten. File modification is not allowed in the destination"
                        );
                    }
                    FileState::NoChange => {
                        println!("[STATUS]: no changes detected");
                    }
                }
            }
            FileAction::Verbose => {
                let (modified_src_file, modified_dest_file, state) = self.file_status();
                match state {
                    FileState::SrcCreated => {
                        self.src_creation_log();
                    }
                    FileState::SrcModified => {
                        self.src_modification_log(modified_src_file);
                    }
                    FileState::DestCreated => {
                        self.dest_creation_log();
                    }
                    FileState::DestModified => {
                        self.dest_modification_log(modified_dest_file);
                    }
                    FileState::NoChange => {
                        println!("[STATUS]: no changes detected");
                    }
                }
            }
            FileAction::DryRun => {
                let (_, _, state) = self.file_status();
                match state {
                    FileState::SrcCreated => {
                        println!("[DRY RUN]: will copy the source file(s) to destination");
                    }
                    FileState::SrcModified => {
                        println!("[DRY RUN]: will update the destination file(s)");
                    }
                    FileState::DestCreated => {
                        println!("[DRY RUN]: will prevent the creation of destination file(s)");
                    }
                    FileState::DestModified => {
                        println!(
                            "[DRY RUN]: will overwrite the destination file(s) with source file(s)"
                        );
                    }
                    FileState::NoChange => {
                        eprintln!("[DRY RUN]: will give the 'no changes detected' message");
                    }
                }
            }
            FileAction::Delete => {
                self.remove_all_dest_files();
            }
        }
    }

    /// Executes the selected sync option.
    pub fn sync_options(&mut self) {
        self.sync_output();
    }
}
