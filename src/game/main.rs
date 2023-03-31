use std::io::{Write, StdoutLock};

pub struct Game {
    pub stdout: StdoutLock<'static>,
}

impl Game {
    pub fn new(stdout: StdoutLock<'static>) -> Self {
        Self {
            stdout
        }
    }

    fn clear_screen(&mut self) {
        write!(
            self.stdout,
            r#"{}{}{}"#,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide,
            termion::clear::All
        ).unwrap();
    }

    pub fn display_screen(&mut self) {
        self.clear_screen();
        write!(self.stdout, "{}hello", termion::cursor::Goto(1, 1)).unwrap();
    }
}
