pub struct ListData {
    pub archives: Option<bool>,
    pub schedules: Option<bool>,
    pub details: Option<bool>,
}

impl ListData {
    pub fn list_output(&self) {
        println!("{}", self.archives.unwrap());
        println!("{}", self.schedules.unwrap());
        println!("{}", self.details.unwrap());
    }

    pub fn list_options(&self) {
        self.list_output();
    }
}
