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
    pub fn to_action(&self) -> FileAction {
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

    pub fn to_state(&self, condition: &mut [bool]) -> FileState {
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

    pub fn sync_output(&mut self) {
        if !self.src_dest_dir_present() {
            eprintln!("[ERROR]: missing source or destination directories");
            return;
        }

        if !self.single_command_selected() {
            eprintln!("[ERROR]: entering multiple flags are not allowed");
            return;
        }

        let action = self.to_action();
        match action {
            FileAction::ChangedOnly => {
                let src_created = self.src_file_created();
                let (modified_src_file, src_modified) = self.src_file_modified();

                let dest_created = self.dest_file_created();
                let (modified_dest_file, dest_modified) = self.dest_file_modified();

                let mut condition: [bool; 4] =
                    [src_created, src_modified, dest_created, dest_modified];
                let state = self.to_state(&mut condition);

                match state {
                    FileState::SrcCreated => {
                        self.copy_src_to_destination();
                        println!("[SUCCESS]: source file(s) successfully copied");
                    }
                    FileState::SrcModified => {
                        self.update_dest_file(modified_src_file);
                        println!("[SUCCESS]: destination file(s) successfully updated");
                    }
                    FileState::DestCreated => {
                        self.remove_dest_file();
                        println!("[MSG]: destination file(s) creation not allowed");
                    }
                    FileState::DestModified => {
                        if self.overwrite_with_src(modified_dest_file) {
                            eprintln!(
                                "[MSG]: destination file(s) content has been overwritten with the source file(s) content"
                            );
                        } else {
                            println!("[STATUS]: no changes detected");
                        }
                    }
                    FileState::NoChange => {
                        println!("[STATUS]: no changes detected");
                    }
                }
            }
            FileAction::Delete => {
                self.remove_all_dest_files();
                println!("[SUCCESS]: Destination files are successfully deleted");
            }
            FileAction::DryRun => {
                let src_created = self.src_file_created();
                let (_, src_modified) = self.src_file_modified();

                let dest_created = self.dest_file_created();
                let (_, dest_modified) = self.dest_file_modified();

                let mut condition: [bool; 4] =
                    [src_created, src_modified, dest_created, dest_modified];
                let state = self.to_state(&mut condition);

                match state {
                    FileState::SrcCreated => {
                        println!("[DRY RUN]: Would copy the source file(s) to destination");
                    }
                    FileState::SrcModified => {
                        println!("[DRY RUN]: Would update the destination file(s)");
                    }
                    FileState::DestCreated => {
                        println!(
                            "[DRY RUN]: Would prevent the creation of the destination file(s)"
                        );
                    }
                    FileState::DestModified => {
                        println!(
                            "[DRY RUN]: Would overwrite the destination file(s) with source file(s)"
                        );
                    }
                    FileState::NoChange => {
                        eprintln!("[DRY RUN]: Would give the error because no changes detected");
                    }
                }
            }
            FileAction::Verbose => {
                let src_created = self.src_file_created();
                let (modified_src_file, src_modified) = self.src_file_modified();

                let dest_created = self.dest_file_created();
                let (_, dest_modified) = self.dest_file_modified();

                let mut condition: [bool; 4] =
                    [src_created, src_modified, dest_created, dest_modified];
                let state = self.to_state(&mut condition);

                match state {
                    FileState::SrcCreated => {
                        self.log_for_source_creation();
                    }
                    FileState::SrcModified => {
                        self.log_for_source_modification(modified_src_file);
                    }
                    FileState::DestCreated => {
                        self.log_for_dest_creation();
                    }
                    FileState::DestModified => {
                        self.log_for_dest_modification();
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
