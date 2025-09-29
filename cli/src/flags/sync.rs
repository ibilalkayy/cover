use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct SyncData {
    /// Source folder to take the file from
    #[clap(short, long)]
    pub source: PathBuf,

    /// Destination folder to move the file to
    #[clap(short, long)]
    pub destination: PathBuf,

    /// Copy only the changed files
    #[clap(long)]
    pub changed_only: bool,

    /// Remove files in destination not in source
    #[clap(long)]
    pub delete: bool,

    /// Show what would happen after syncing
    #[clap(long)]
    pub dry_run: bool,

    /// Show detailed logs
    #[clap(long)]
    pub verbose: bool,

    /// Check file hashes instead of timestamps
    #[clap(long)]
    pub hash: bool,
}
