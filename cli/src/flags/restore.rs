use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct RestoreData {
    /// Replace the existing files
    #[clap(long)]
    pub overwrite: bool,

    /// Restore to a different folder
    #[clap(long)]
    pub to: Option<PathBuf>,

    /// Restore only a specific file
    #[clap(long)]
    pub select: Option<PathBuf>,
}
