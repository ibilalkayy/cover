use std::path::PathBuf;

pub struct SyncData {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub changed_only: bool,
    pub delete: bool,
    pub dry_run: bool,
    pub verbose: bool,
}

pub enum FileState {
    SrcCreated,
    SrcModified,
    DestCreated,
    DestModified,
    NoChange,
}

pub enum FileAction {
    ChangedOnly,
    Delete,
    DryRun,
    Verbose,
}

impl SyncData {
    fn to_action(&self) -> FileAction {
        if self.changed_only {
            FileAction::ChangedOnly
        } else if self.delete {
            FileAction::Delete
        } else if self.dry_run {
            FileAction::DryRun
        } else if self.verbose {
            FileAction::Verbose
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

    pub fn sync_output(&mut self) {
        if !self.src_dest_dir_present() {
            eprintln!("[ERROR]: missing source or destination directories");
            return;
        }

        if !self.single_command_selected() {
            eprintln!("[ERROR]: multiple flags are not allowed");
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
            FileAction::Delete => {
                self.remove_all_dest_files();
            }
            FileAction::DryRun => {
                let (_, _, state) = self.file_status();
                match state {
                    FileState::SrcCreated => {
                        println!("[DRY RUN]: would copy the source file(s) to destination");
                    }
                    FileState::SrcModified => {
                        println!("[DRY RUN]: would update the destination file(s)");
                    }
                    FileState::DestCreated => {
                        println!(
                            "[DRY RUN]: would prevent the creation of the destination file(s)"
                        );
                    }
                    FileState::DestModified => {
                        println!(
                            "[DRY RUN]: would overwrite the destination file(s) with source file(s)"
                        );
                    }
                    FileState::NoChange => {
                        eprintln!("[DRY RUN]: would give the error because no changes detected");
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
        }
    }

    pub fn sync_options(&mut self) {
        self.sync_output();
    }
}
