use std::path::PathBuf;

pub struct RestoreData {
    pub overwrite: bool,
    pub to: Option<PathBuf>,
    pub select: Option<PathBuf>,
}

impl RestoreData {
    pub fn restore_output(&self) {
        println!("{}", self.overwrite);
        println!("{:?}", self.to.clone().unwrap());
        println!("{:?}", self.select.clone().unwrap());
    }

    pub fn restore_options(&self) {
        self.restore_output();
    }
}
