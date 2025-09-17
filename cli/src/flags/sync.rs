use clap::Parser;

#[derive(Debug, Parser)]
pub struct SyncData {
    /// Copy only the changed files
    #[clap(long)]
    pub incremental: Option<bool>,

    /// Remove files in destination not in source
    #[clap(long)]
    pub delete: Option<bool>,

    /// Show what would happen after syncing
    #[clap(long)]
    pub dry_run: Option<bool>,

    /// Show detailed logs
    #[clap(long)]
    pub verbose: Option<bool>,

    /// Check file hashes instead of timestamps
    #[clap(long)]
    pub hash: Option<bool>,
}
