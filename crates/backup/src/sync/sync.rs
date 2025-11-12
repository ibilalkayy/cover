use std::path::PathBuf;

pub struct SyncData {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub changed_only: bool,
    pub delete: bool,
    pub dry_run: bool,
    pub verbose: bool,
}

impl SyncData {
    pub fn sync_output(&mut self) {
        if !self.src_dest_dir_present() {
            eprintln!("Err: missing source or destination directories");
            return;
        }

        if !self.single_command_selected() {
            eprintln!("Err: entering multiple flags are not allowed");
            return;
        }

        match (self.changed_only, self.delete, self.dry_run, self.verbose) {
            (true, _, _, _) => {
                let src_created = self.src_file_created();
                let (modified_src_file, src_modified) = self.src_file_modified();

                let dest_created = self.dest_file_created();
                let (modified_dest_file, dest_modified) = self.dest_file_modified();

                match (src_created, src_modified, dest_created, dest_modified) {
                    (true, _, _, _) => {
                        self.copy_src_to_destination();
                        println!("Success: source file(s) successfully copied");
                    }
                    (_, true, _, _) => {
                        self.update_dest_file(modified_src_file);
                        println!("Success: destination file(s) successfully updated");
                    }
                    (_, _, true, _) => {
                        self.remove_dest_file();
                        println!("Msg: destination file(s) creation not allowed");
                    }
                    (_, _, _, true) => {
                        if self.overwrite_with_src(modified_dest_file) {
                            eprintln!("Err: destination file(s) is modified, and it's not allowed");
                        } else {
                            println!("Status: no changes detected");
                        }
                    }
                    _ => {}
                }
            }
            (_, true, _, _) => {
                self.remove_all_dest_files();
                println!("Success: Destination files are successfully deleted");
            }
            (_, _, true, _) => {
                let src_created = self.src_file_created();
                let (_, src_modified) = self.src_file_modified();

                let dest_created = self.dest_file_created();
                let (_, dest_modified) = self.dest_file_modified();

                match (src_created, src_modified, dest_created, dest_modified) {
                    (true, _, _, _) => {
                        println!("[DRY RUN]: Would copy the source file(s) to destination")
                    }
                    (_, true, _, _) => {
                        println!("[DRY RUN]: Would update the destination file(s)");
                    }
                    (_, _, true, _) => {
                        println!(
                            "[DRY RUN]: Would prevent the creation of the destination file(s)"
                        );
                    }
                    (_, _, _, true) => {
                        println!(
                            "[DRY RUN]: Would overwrite the destination file(s) with source file(s)"
                        );
                    }
                    _ => eprintln!("[DRY RUN]: Would give the error because no changes detected"),
                }
            }
            (_, _, _, true) => {
                let src_created = self.src_file_created();
                let (modified_src_file, src_modified) = self.src_file_modified();

                let dest_created = self.dest_file_created();
                let (_, dest_modified) = self.dest_file_modified();

                match (src_created, src_modified, dest_created, dest_modified) {
                    (true, _, _, _) => {
                        self.log_for_source_creation();
                    }
                    (_, true, _, _) => {
                        self.log_for_source_modification(modified_src_file);
                    }
                    (_, _, true, _) => {
                        self.log_for_dest_creation();
                    }
                    (_, _, _, true) => {
                        self.log_for_dest_modification();
                    }
                    _ => eprintln!("Err: no changes detected"),
                }
            }
            _ => eprintln!("Err: other commands are not allowed"),
        }
    }

    pub fn sync_options(&mut self) {
        self.sync_output();
    }
}
