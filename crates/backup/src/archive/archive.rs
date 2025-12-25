pub struct ArchiveData {
    pub zip: bool,
    pub tar: bool,
    pub encrypt: bool,
    pub timestamp: bool,
}

impl ArchiveData {
    pub fn archive_output(&self) {
        println!("{}", self.zip);
        println!("{}", self.tar);
        println!("{}", self.encrypt);
        println!("{}", self.timestamp);
    }

    pub fn archive_options(&self) {
        self.archive_output();
    }
}
