use std::io::{self, stdin, stdout, Write};

extern crate termion;
use termion::clear;
use termion::color;
use termion::style;
use termion::cursor;
use termion::terminal_size;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

fn main() {
    let stdout = stdout().into_raw_mode().unwrap();
    let mut screen = termion::screen::AlternateScreen::from(stdout);

    let game = Game::new(3);
    game.write_to(&mut screen).unwrap();

    let stdin = stdin();
    for key in stdin.keys() {
        match key.unwrap() {
            Key::Ctrl('c') | Key::Ctrl('q') => break,
            _ => {}
        }
    }
}

struct Game {
    lines: Vec<Vec<u8>>,
    size: u16,
    score: u32,
}

impl Game {
    fn new(size: u16) -> Game {
        let lines = vec![vec![0; size as usize]; size as usize];
        Game {
            lines: lines,
            size: size,
            score: 0,
        }
    }

    fn write_to(&self, w: &mut Write) -> io::Result<()> {
        write!(w, "{}", clear::All)?;

        let (total_width, total_height) = terminal_size()?;
        match self.get_padding(total_width, total_height) {
            None => {
                write!(
                    w,
                    "{red}INVALIDSIZE{reset}",
                    red = color::Fg(color::Red),
                    reset = style::Reset,
                )?;
                Ok(())
            }

            Some((left_pad, top_pad)) => {
                write!(w, "{}", cursor::Goto(1, total_height))?;
                write!(w, "Score: {}", self.score)?;

                let mut line_count = 0;
                for line in &self.lines {
                    write!(w, "{}", cursor::Goto(left_pad, top_pad + line_count))?;
                    for value in line {
                        write!(w, "{:>6}", value);
                    }
                    writeln!(w, "");
                    line_count += 1;
                }

                w.flush()
            }
        }
    }

    fn get_padding(&self, total_width: u16, total_height: u16) -> Option<(u16, u16)> {
        let grid_width = (self.size * 6) + (self.size + 1);
        let grid_height = self.size + 1;

        let horizontal = total_width as i16 - grid_width as i16;
        let vertical = total_height as i16 - grid_height as i16;

        if horizontal < 0 || vertical < 0 {
            None
        } else {
            let left_pad = (horizontal as f64 / 2.0).ceil() as u16;
            let top_pad = (vertical as f64 / 2.0).ceil() as u16;
            Some((left_pad, top_pad))
        }
    }
}
