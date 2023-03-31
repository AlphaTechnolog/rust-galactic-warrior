use std::io::{Write, StdoutLock};
use termion::{clear::All as ClearAll, cursor};

pub struct Board<'a> {
    pub width: u16,
    pub height: u16,
    pub board: Option<Vec<Vec<char>>>,
    stdout: &'a mut StdoutLock<'static>,
}

impl<'a> Board<'a> {
    pub fn new(width: u16, height: u16, stdout: &'a mut StdoutLock<'static>) -> Self {
        Self {
            width,
            height,
            board: None,
            stdout,
        }
    }

    fn clear_screen(&mut self) {
        write!(
            &mut *self.stdout,
            r#"{}{}{}"#,
            cursor::Goto(1, 1),
            cursor::Hide,
            ClearAll
        ).unwrap();
    }

    pub fn generate_board(&mut self) {
        if self.board.is_some() {
            return;
        }

        let mut board = Vec::new();

        for _ in 0..self.height {
            let mut row = Vec::new();
            for _ in 0..self.width {
                row.push('.');
            }

            board.push(row);
        }

        self.board = Some(board);
    }

    pub fn display_board(&mut self) {
        self.clear_screen();

        if self.board.is_none() {
            self.generate_board();
        }

        let board = self.board.as_ref().unwrap();

        for (y, row) in board.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                write!(
                    &mut *self.stdout,
                    "{}{}",
                    cursor::Goto((x as u16) + 1, (y as u16) + 1),
                    col.to_string()
                ).unwrap();
            }
        }
    }
}
