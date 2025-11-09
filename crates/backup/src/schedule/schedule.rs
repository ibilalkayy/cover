pub struct ScheduleData {
    pub daily: String,
    pub weekly: Option<String>,
    pub interval: Option<u32>,
    pub command: Option<String>,
}

impl ScheduleData {
    pub fn schedule_output(&self) {
        println!("{}", self.daily);
        println!("{}", self.weekly.clone().unwrap());
        println!("{}", self.interval.unwrap());
        println!("{}", self.command.clone().unwrap());
    }

    pub fn schedule_options(&self) {
        self.schedule_output();
    }
}
