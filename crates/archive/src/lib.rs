pub struct ArchiveData {
    pub zip: Option<bool>,
    pub tar: Option<bool>,
    pub encrypt: Option<bool>,
    pub timestamp: Option<bool>,
}

impl ArchiveData {
    pub fn archive_output(&self) {
        println!("{}", self.zip.unwrap());
        println!("{}", self.tar.unwrap());
        println!("{}", self.encrypt.unwrap());
        println!("{}", self.timestamp.unwrap());
    }

    pub fn archive_options(&self) {
        self.archive_output();
    }
}
