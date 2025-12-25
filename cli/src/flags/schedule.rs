use clap::Parser;

/// Holds the information to schedule the data, daily, weekly, etc
///
/// All the flags are optional except one where daily will be used be default
/// if none of the option is selected
///
/// # Example
///
/// ```rust,no_run
/// use my_crate::ScheduleData;
/// use std::path::PathBuf;
///
/// let schedule = ScheduleData {
///     daily: true,
///     weekly: false,
///     interval: Some(12),
///     command: Some("command".to_string()),
/// };
/// ```
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
