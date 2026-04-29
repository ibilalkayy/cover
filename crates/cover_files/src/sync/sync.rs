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
pub enum ActionType {
    /// Copy only the changed files
    ChangedOnly,
    /// Show detailed logs
    Verbose,
    /// Show what would happen after syncing
    DryRun,
    /// Remove files in destination not in source
    Delete,
}

pub enum SyncAction {
    Copy,
    Rename,
    Update,
    Delete,
    Nothing,
}

pub struct ActionData {
    pub copy: bool,
    pub rename: bool,
    pub update: bool,
    pub delete: bool,
    pub nothing: bool,
}

impl ActionData {
    fn action_data(&self) -> SyncAction {
        if self.copy {
            SyncAction::Copy
        } else if self.rename {
            SyncAction::Rename
        } else if self.update {
            SyncAction::Update
        } else if self.delete {
            SyncAction::Delete
        } else {
            SyncAction::Nothing
        }
    }
}

/// Implementation for the output that will be generated after running the command.
impl SyncData {
    pub fn action_type(&self) -> ActionType {
        if self.changed_only {
            ActionType::ChangedOnly
        } else if self.verbose {
            ActionType::Verbose
        } else if self.dry_run {
            ActionType::DryRun
        } else if self.delete {
            ActionType::Delete
        } else {
            ActionType::Verbose
        }
    }

    /// Runs the sync operation between the source and destination.
    ///
    /// This function validates the selected options and executes the
    /// suitble sync action (changed-only, verbose, dry-run, or delete).
    /// Any errors or status messages are printed to standard output.
    pub fn sync_output(&mut self) {
        if self.has_duplicates() {
            eprintln!("[ERROR]: duplicate files or directories are not allowed");
            return;
        }

        // if both update and renamed in the source or destination then there are "changes detected message" is given.

        if self.do_rename() {
            println!("[MESSAGE]: successfully renamed the destination file(s)");
        } else if self.do_copy() {
            println!("[MESSAGE]: successfully copied the source file(s)");
        } else if self.do_update() {
            println!("[MESSAGE]: successfully updated the destination file(s)");
        } else if self.do_nothing() {
            println!("[MESSAGE]: no changes detected");
            return;
        } else {
            println!("[MESSAGE]: changes detected");
        };

        let data = ActionData {
            copy: false,
            rename: false,
            update: false,
            delete: false,
            nothing: false,
        };

        if !self.single_command_selected() {
            panic!(
                "[ERROR]: none or multiple command(s) are selected. See 'cargo run sync --help'"
            );
        }

        let action = self.action_type();
        let choice = data.action_data();

        match action {
            ActionType::ChangedOnly => match choice {
                SyncAction::Copy => {
                    println!("[SUCCESS]: successfully copied the source file(s)");
                }
                SyncAction::Rename => {
                    println!("[SUCCESS]: successfully renamed the destination file(s)");
                }
                SyncAction::Update => {
                    println!("[SUCCESS]: successfully updated the destination file(s)");
                }
                SyncAction::Delete => {
                    println!(
                        "[MESSAGE]: File modification is not allowed in the destination. They are overwritten"
                    );
                }
                SyncAction::Nothing => {
                    eprintln!("[ERROR]: no changes detected");
                }
            },
            ActionType::Verbose => todo!(),
            ActionType::DryRun => todo!(),
            ActionType::Delete => todo!(),
        }
    }

    /// Executes the selected sync option.
    pub fn sync_options(&mut self) {
        self.sync_output();
    }
}
