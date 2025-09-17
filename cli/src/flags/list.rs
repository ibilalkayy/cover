use clap::Parser;

#[derive(Debug, Parser)]
pub struct ListData {
    /// List the archive files
    #[clap(long)]
    pub archives: Option<bool>,

    /// List the scheduled tasks
    #[clap(long)]
    pub schedules: Option<bool>,

    /// Show timestamps & sizes
    #[clap(long)]
    pub details: Option<bool>,
}
