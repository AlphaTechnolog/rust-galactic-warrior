use figlet_rs::FIGfont;
use std::io::{stdin, stdout, Write};
use termion::{color, raw::IntoRawMode, event::Key, input::TermRead};

use crate::game::main::Game;

fn clear_screen(stdout: &mut std::io::Stdout) {
    write!(
        stdout,
        r#"{}{}{}"#,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide,
        termion::clear::All
    ).unwrap();
}

fn print_banner(mut stdout: &mut std::io::Stdout) -> usize {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Galactic Warrior");

    clear_screen(&mut stdout);
    assert!(figure.is_some());

    let mut lines_count = 0;

    for (i, line) in figure.unwrap().to_string().lines().into_iter().enumerate() {
        write!(
            stdout,
            "{}{}{}",
            termion::cursor::Goto(1, (i as u16) + 1),
            color::Fg(color::Blue),
            line
        ).unwrap();

        lines_count = lines_count + 1;
    }

    return lines_count;
}

fn print_options(stdout: &mut std::io::Stdout, offset: usize, selected_index: usize) {
    let choices: [&str; 4] = ["Play", "How to play", "Scoreboards", "Exit"];

    for (i, choice) in choices.iter().enumerate() {
        write!(stdout, "{}", termion::cursor::Goto(1, (i as u16) + (offset as u16) + 1)).unwrap();
        if i == selected_index {
            println!("{}{}", color::Fg(color::Blue), choice.to_string());
        } else {
            println!("{}{}", color::Fg(color::Reset), choice.to_string());
        }
    }
}

fn redraw_scene(mut stdout: &mut std::io::Stdout, selected_index: usize) {
    let offset = print_banner(&mut stdout);
    print_options(&mut stdout, offset, selected_index);
}

pub fn print_menu() {
    let stdin = stdin();

    let mut selected_index = 0;

    let mut stdout = stdout()
        .into_raw_mode()
        .unwrap();

    stdout.flush().unwrap();

    redraw_scene(&mut stdout, selected_index);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Up => {
                if selected_index > 0 {
                    selected_index -= 1;
                } else {
                    selected_index = 3;
                }

                redraw_scene(&mut stdout, selected_index);
            },
            Key::Down => {
                if selected_index < 3 {
                    selected_index += 1;
                } else {
                    selected_index = 0;
                }

                redraw_scene(&mut stdout, selected_index);
            },
            Key::Char('\n') => {
                match selected_index {
                    0 => {
                        let mut game = Game::new(stdout.lock());
                        stdout.flush().unwrap();
                        game.display_screen();
                        break;
                    },
                    1 => {
                        break;
                    },
                    2 => {
                        break;
                    },
                    3 => {
                        break;
                    },
                    _ => (),
                }
            },
            _ => (),
        }

        stdout.flush().unwrap();
    }
}
