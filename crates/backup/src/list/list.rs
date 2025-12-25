pub struct ListData {
    pub archives: bool,
    pub schedules: bool,
    pub details: bool,
}

impl ListData {
    pub fn list_output(&self) {
        println!("{}", self.archives);
        println!("{}", self.schedules);
        println!("{}", self.details);
    }

    pub fn list_options(&self) {
        self.list_output();
    }
}
