use clap::Parser;

/// Carries the information to delete the data
///
/// All three flags are optional but two of them require option but dry run is boolean
/// which could be true or false
///
/// # Example
///
/// ```rust,no_run
/// use my_crate::CleanData;
///
/// let clean = CleanData {
///     keep_last: Some(12),
///     older_than: None,
///     dry_run: true,
/// };
/// ```
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
    pub dry_run: bool,
}
