use clap::Parser;

#[derive(Debug, Parser)]
pub struct ScheduleData {
    /// Run every day at a given time
    #[clap(long)]
    pub daily: String,

    /// Run once a week
    #[clap(long)]
    pub weekly: Option<String>,

    /// Run every N minutes
    #[clap(long)]
    pub interval: Option<u32>,

    /// Give the command to run
    #[clap(long)]
    pub command: Option<String>,
}
