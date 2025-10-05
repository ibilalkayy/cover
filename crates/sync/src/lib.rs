pub mod changed_only;
pub mod delete;
pub mod dry_run;
pub mod utils;
pub mod verbose;

use crate::{
    changed_only::else_file_changed_only,
    delete::delete_flag,
    dry_run::dry_run_flag,
    utils::{destination_listing, source_listing},
    verbose::verbose_flag,
};
use changed_only::file_changed_only;
use std::path::PathBuf;

pub struct SyncData {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub changed_only: bool,
    pub delete: bool,
    pub dry_run: bool,
    pub verbose: bool,
    pub hash: bool,
}

impl SyncData {
    pub fn sync_output(&self) {
        let mut destination_files: Vec<(PathBuf, i64)> = Vec::new();
        let src_files = source_listing(self.source.clone());
        let dest_files = source_listing(self.destination.clone());
        let source_files: Vec<(PathBuf, i64)> = src_files
            .iter()
            .map(|(path, num, _, _)| (path.clone(), *num))
            .collect();
        destination_listing(self.destination.clone(), &mut destination_files);

        if self.changed_only {
            file_changed_only(
                &src_files,
                &source_files,
                &destination_files,
                &self.source,
                &self.destination,
            )
        } else if !self.delete && !self.dry_run && !self.verbose {
            else_file_changed_only(
                &source_files,
                &destination_files,
                &self.source,
                &self.destination,
            );
        }

        if self.delete {
            delete_flag(
                &source_files,
                &destination_files,
                &self.source,
                &self.destination,
            );
        }

        if self.dry_run {
            dry_run_flag(
                &source_files,
                &destination_files,
                &self.source,
                &self.destination,
            );
        }

        if self.verbose {
            verbose_flag(
                &source_files,
                &destination_files,
                &self.source,
                &self.destination,
                &src_files,
                &dest_files,
            );
        }
    }

    pub fn sync_options(&self) {
        self.sync_output();
    }
}
