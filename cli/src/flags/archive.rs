use clap::Parser;

#[derive(Debug, Parser)]
pub struct ArchiveData {
    /// Save the file as .zip
    #[clap(long)]
    pub zip: Option<bool>,

    /// Save the file as .tar.gz
    #[clap(long)]
    pub tar: Option<bool>,

    /// Encrypt archive (ask for password)
    #[clap(long)]
    pub encrypt: Option<bool>,

    /// Append date/time to filename
    #[clap(long)]
    pub timestamp: Option<bool>,
}
