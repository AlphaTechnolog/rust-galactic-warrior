use std::process::Command;

pub struct ScreenClear {}

impl ScreenClear {
    pub fn new() -> Self {
        Self {}
    }

    pub fn clear(&self) {
        let mut str = "clear";
        if self.is_windows() {
            str = "cls";
        }

        Command::new(String::from(str)).status().unwrap();
    }

    fn is_windows(&self) -> bool {
        cfg!(target_os = "windows")
    }
}
