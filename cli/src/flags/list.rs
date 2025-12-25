use clap::Parser;

/// ListData struct takes archives, schedules or details data to show
///
/// All of them are boolean. Means they are not restricted to be used at the same time
///
/// # Example
///
/// ```rust,no_run
/// use my_crate::ListData;
///
/// let list = ListData {
///     archives: true,
///     schedules: false,
///     details: false,
/// };
/// ```
#[derive(Debug, Parser)]
pub struct ListData {
    /// List the archive files
    #[clap(long)]
    pub archives: bool,

    /// List the scheduled tasks
    #[clap(long)]
    pub schedules: bool,

    /// Show timestamps & sizes
    #[clap(long)]
    pub details: bool,
}
