use clap::Parser;
use std::path::PathBuf;

/// Flags used to restore data if it has been lost
///
/// `RestoreData` defines how an archive should be restored
///
/// # Example
///
/// ```rust,no_run
/// use my_crate::RestoreData;
/// use std::path::PathBuf;
///
/// let restore = RestoreData {
///     overwrite: true,
///     to: Some(Pathbuf::from("to_directory")),
///     select: Some(PathBuf::from("specific_file")),
/// };
/// ```
#[derive(Debug, Parser)]
pub struct RestoreData {
    /// Replace existing files during restore
    #[clap(long)]
    pub overwrite: bool,

    /// Restore to a different folder
    #[clap(long)]
    pub to: Option<PathBuf>,

    /// Restore only a specific file
    #[clap(long)]
    pub select: Option<PathBuf>,
}
