pub struct CleanData {
    pub keep_last: Option<u32>,
    pub older_than: Option<u32>,
    pub dry_run: Option<bool>,
}

impl CleanData {
    pub fn clean_output(&self) {
        println!("{}", self.keep_last.unwrap());
        println!("{}", self.older_than.unwrap());
        println!("{}", self.dry_run.unwrap());
    }

    pub fn clean_options(&self) {
        self.clean_output();
    }
}
