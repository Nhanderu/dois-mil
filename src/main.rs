use std::env::args;
use std::iter::Iterator;
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

const DEFAULT_GRID_SIZE: u16 = 3;

fn main() {
    let stdout = stdout().into_raw_mode().unwrap();
    let mut screen = termion::screen::AlternateScreen::from(stdout);

    let grid_size = args().nth(1).map_or(DEFAULT_GRID_SIZE, |raw| {
        raw.parse::<u16>().unwrap_or(DEFAULT_GRID_SIZE)
    });

    let mut gm = GameManager::new(grid_size, &mut screen);
    gm.flush().unwrap();

    let stdin = stdin();
    for key in stdin.keys() {
        match key.unwrap() {
            Key::Ctrl('c') | Key::Ctrl('q') => break,
            Key::Up => gm.move_up(),
            Key::Right => gm.move_right(),
            Key::Down => gm.move_down(),
            Key::Left => gm.move_left(),
            _ => {}
        }
        gm.flush().unwrap()
    }
}

struct GameManager<'a> {
    game: Game,
    output: &'a mut Write,
}

impl<'a> GameManager<'a> {
    fn new(size: u16, output: &'a mut Write) -> GameManager {
        GameManager {
            game: Game::new(size),
            output: output,
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        self.game.write_to(&mut self.output)
    }

    fn move_up(&mut self) {
        self.game = self.game.move_up();
    }

    fn move_right(&mut self) {
        self.game = self.game.move_right();
    }

    fn move_down(&mut self) {
        self.game = self.game.move_down();
    }

    fn move_left(&mut self) {
        self.game = self.game.move_left();
    }
}

struct Game {
    grid: Vec<Vec<u8>>,
    size: u16,
    score: u32,
}

impl Game {
    fn new(size: u16) -> Game {
        Game {
            grid: vec![vec![1; size as usize]; size as usize],
            size: size,
            score: 0,
        }
    }

    fn write_to(&self, w: &mut Write) -> io::Result<()> {
        write!(w, "{}", clear::All)?;

        let (total_width, total_height) = terminal_size()?;
        match self.get_padding(total_width, total_height) {
            None => write!(
                w,
                "{red}INVALIDSIZE{reset}",
                red = color::Fg(color::Red),
                reset = style::Reset,
            ),

            Some((left_pad, top_pad)) => {
                write!(w, "{}", cursor::Goto(1, total_height))?;
                write!(w, "Score: {}", self.score)?;

                for (i, line) in self.grid.iter().enumerate() {
                    write!(w, "{}", cursor::Goto(left_pad, top_pad + i as u16))?;
                    for value in line {
                        write!(w, "{:>6}", value)?;
                    }
                    writeln!(w, "")?;
                }

                w.flush()
            }
        }
    }

    fn move_up(&self) -> Game {
        let mut new_grid = vec![vec![0; self.size as usize]; self.size as usize];

        for (i, line) in self.grid.iter().rev().enumerate() {
            for (j, cell) in line.iter().enumerate() {
                let (cur_i, cur_j) = (i, j);
                let (next_i, next_j) = (i + 1, j);

                let mut cur_cell = *cell;
                let mut next_cell = *match self.grid.get(next_i).and_then(|x| x.get(next_j)) {
                    Some(x) => x,
                    None => break,
                };

                if cur_cell == next_cell || next_cell == 0 {
                    next_cell += cur_cell;
                    cur_cell = 0;
                }

                let size = self.size as usize - 1;
                let (cur_i, cur_j) = (size - cur_i, cur_j);
                let (next_i, next_j) = (size - next_i, next_j);

                new_grid[cur_i][cur_j] = cur_cell;
                new_grid[next_i][next_j] = next_cell;
            }
        }

        Self {
            grid: new_grid,
            size: self.size,
            score: 0,
        }
    }

    fn move_right(&self) -> Game {
        let mut new_grid = vec![vec![0; self.size as usize]; self.size as usize];

        for (i, line) in self.grid.iter().enumerate() {
            for (j, cell) in line.iter().enumerate() {
                let (cur_i, cur_j) = (i, j);
                let (next_i, next_j) = (i, j + 1);

                let mut cur_cell = *cell;
                let mut next_cell = *match self.grid.get(next_i).and_then(|x| x.get(next_j)) {
                    Some(x) => x,
                    None => break,
                };

                if cur_cell == next_cell || next_cell == 0 {
                    next_cell += cur_cell;
                    cur_cell = 0;
                }

                new_grid[cur_i][cur_j] = cur_cell;
                new_grid[next_i][next_j] = next_cell;
            }
        }

        Self {
            grid: new_grid,
            size: self.size,
            score: 0,
        }
    }

    fn move_down(&self) -> Game {
        let mut new_grid = vec![vec![0; self.size as usize]; self.size as usize];

        for (i, line) in self.grid.iter().enumerate() {
            for (j, cell) in line.iter().enumerate() {
                let (cur_i, cur_j) = (i, j);
                let (next_i, next_j) = (i + 1, j);

                let mut cur_cell = *cell;
                let mut next_cell = *match self.grid.get(next_i).and_then(|x| x.get(next_j)) {
                    Some(x) => x,
                    None => break,
                };

                if cur_cell == next_cell || next_cell == 0 {
                    next_cell += cur_cell;
                    cur_cell = 0;
                }

                new_grid[cur_i][cur_j] = cur_cell;
                new_grid[next_i][next_j] = next_cell;
            }
        }

        Self {
            grid: new_grid,
            size: self.size,
            score: 0,
        }
    }

    fn move_left(&self) -> Game {
        let mut new_grid = vec![vec![0; self.size as usize]; self.size as usize];

        for (i, line) in self.grid.iter().enumerate() {
            for (j, cell) in line.iter().rev().enumerate() {
                let (cur_i, cur_j) = (i, j);
                let (next_i, next_j) = (i, j + 1);

                let mut cur_cell = *cell;
                let mut next_cell = *match self.grid.get(next_i).and_then(|x| x.get(next_j)) {
                    Some(x) => x,
                    None => break,
                };

                if cur_cell == next_cell || next_cell == 0 {
                    next_cell += cur_cell;
                    cur_cell = 0;
                }

                let size = self.size as usize - 1;
                let (cur_i, cur_j) = (cur_i, size - cur_j);
                let (next_i, next_j) = (next_i, size - next_j);

                new_grid[cur_i][cur_j] = cur_cell;
                new_grid[next_i][next_j] = next_cell;
            }
        }

        Self {
            grid: new_grid,
            size: self.size,
            score: 0,
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
