use std::io::{Write, StdoutLock};
use crate::game::board::Board;

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
        let mut board = Board::new(70, 10, &mut self.stdout);
        board.display_board();
    }
}
