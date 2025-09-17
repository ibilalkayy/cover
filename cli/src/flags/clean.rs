use clap::Parser;

#[derive(Debug, Parser)]
pub struct CleanData {
    /// Keep only last N backups
    #[clap(long)]
    pub keep_last: Option<u32>,

    /// Delete backups older than N days
    #[clap(long)]
    pub older_than: Option<u32>,

    /// Show what would be deleted
    #[clap(long)]
    pub dry_run: Option<bool>,
}
