use clap::Parser;

/// It takes the required information for archiving the data
///
/// In ArchiveData struct, all the commands are optional
/// It means that they are not bound to be used simultaneously
///
/// # Example
///
/// ```rust,no_run
/// use my_crate::ArchiveData;
///
/// let archive = ArchiveData {
///     zip: true,
///     tar: false,
///     encrypt: false,
///     timestamp: false,
/// };
/// ```
#[derive(Debug, Parser)]
pub struct ArchiveData {
    /// Save the file as .zip
    #[clap(long)]
    pub zip: bool,

    /// Save the file as .tar.gz
    #[clap(long)]
    pub tar: bool,

    /// Encrypt archive (ask for password)
    #[clap(long)]
    pub encrypt: bool,

    /// Append date/time to filename
    #[clap(long)]
    pub timestamp: bool,
}
