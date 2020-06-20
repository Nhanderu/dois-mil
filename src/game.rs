use std::fmt::Display;
use std::io::Write;
use std::iter::Iterator;

use error::{GameError, GameErrorType};

use termion::clear;
use termion::color;
use termion::cursor;
use termion::style;
use termion::terminal_size;

use rand::distributions::{Distribution, Uniform, WeightedIndex};
use rand::random;

const RANDOM_VALUES_WEIGHT: [(u32, usize); 2] = [(2, 2), (4, 1)];

pub struct Game {
    grid: Vec<Vec<u32>>,
    size: usize,
    score: u32,
    random_pos: Uniform<usize>,
    random_val: WeightedIndex<usize>,
    new_val: Vec<u32>,
}

impl Game {
    pub fn new(size: usize) -> Game {
        Game {
            grid: vec![vec![0; size]; size],
            size: size,
            score: 0,
            random_pos: Uniform::from(0..size),
            random_val: WeightedIndex::new(RANDOM_VALUES_WEIGHT.iter().map(|item| item.1)).unwrap(),
            new_val: RANDOM_VALUES_WEIGHT.iter().map(|item| item.0).collect(),
        }
    }

    pub fn move_up(&mut self, w: &mut dyn Write) -> Result<(), GameError> {
        let grid_size = self.size;
        self.move_cells(
            0..grid_size,
            0..grid_size,
            |i, _| (0..i).rev(),
            |_, j, k| (k, j),
            w,
        )
    }

    pub fn move_down(&mut self, w: &mut dyn Write) -> Result<(), GameError> {
        let grid_size = self.size;
        self.move_cells(
            (0..grid_size).rev(),
            0..grid_size,
            |i, _| i + 1..grid_size,
            |_, j, k| (k, j),
            w,
        )
    }

    pub fn move_left(&mut self, w: &mut dyn Write) -> Result<(), GameError> {
        let grid_size = self.size;
        self.move_cells(
            0..grid_size,
            0..grid_size,
            |_, j| (0..j).rev(),
            |i, _, k| (i, k),
            w,
        )
    }

    pub fn move_right(&mut self, w: &mut dyn Write) -> Result<(), GameError> {
        let grid_size = self.size;
        self.move_cells(
            0..grid_size,
            (0..grid_size).rev(),
            |_, j| j + 1..grid_size,
            |i, _, k| (i, k),
            w,
        )
    }

    fn move_cells<T, U, V, W, X>(
        &mut self,
        i_range: T,
        j_range: U,
        k_range: W,
        next_cell: X,
        w: &mut dyn Write,
    ) -> Result<(), GameError>
    where
        T: std::iter::Iterator<Item = usize>,
        U: std::iter::Iterator<Item = usize> + Clone,
        V: std::iter::Iterator<Item = usize>,
        W: Fn(usize, usize) -> V,
        X: Fn(usize, usize, usize) -> (usize, usize),
    {
        let mut moved = false;
        let mut merged_cells = vec![vec![false; self.size]; self.size];

        for i in i_range {
            for j in j_range.clone() {
                let (mut cur_i, mut cur_j) = (i, j);
                let cur_cell = self.grid[cur_i][cur_j];
                if cur_cell == 0 {
                    continue;
                }

                for k in k_range(i, j) {
                    let (next_i, next_j) = next_cell(i, j, k);
                    match self.grid.get(next_i).and_then(|x| x.get(next_j)) {
                        Some(0) => {
                            self.grid[cur_i][cur_j] = 0;
                            self.grid[next_i][next_j] = cur_cell;
                            cur_i = next_i;
                            cur_j = next_j;
                            moved = true;
                        }
                        Some(next_cell)
                            if *next_cell == cur_cell && !merged_cells[next_i][next_j] =>
                        {
                            let points = cur_cell * 2;
                            self.grid[cur_i][cur_j] = 0;
                            self.grid[next_i][next_j] = points;
                            self.score += points as u32;
                            moved = true;
                            merged_cells[next_i][next_j] = true;
                            break;
                        }
                        _ => break,
                    };
                }
            }
        }

        if moved {
            self.fill_random_cells();
        }

        self.write_to(w)
    }

    pub fn write_to(&self, w: &mut dyn Write) -> Result<(), GameError> {
        write!(w, "{}", clear::All)?;

        let (total_width, total_height) = terminal_size()?;
        match self.get_padding(total_width, total_height) {
            None => Err(GameError::new(GameErrorType::TerminalSize)),
            Some((left_pad, top_pad)) => {
                for (i, line) in self.grid.iter().enumerate() {
                    write!(w, "{}", cursor::Goto(left_pad, top_pad + i as u16))?;
                    for value in line {
                        let color: Box<dyn Display> = match value {
                            0 => Box::new(color::Fg(color::AnsiValue(247))),
                            2 => Box::new(color::Fg(color::AnsiValue(202))),
                            4 => Box::new(color::Fg(color::AnsiValue(214))),
                            8 => Box::new(color::Fg(color::AnsiValue(226))),
                            16 => Box::new(color::Fg(color::AnsiValue(40))),
                            32 => Box::new(color::Fg(color::AnsiValue(47))),
                            64 => Box::new(color::Fg(color::AnsiValue(14))),
                            128 => Box::new(color::Fg(color::AnsiValue(33))),
                            256 => Box::new(color::Fg(color::AnsiValue(141))),
                            512 => Box::new(color::Fg(color::AnsiValue(213))),
                            1024 => Box::new(color::Fg(color::AnsiValue(201))),
                            _ => Box::new(color::Fg(color::AnsiValue(196))),
                        };
                        write!(
                            w,
                            "{bold}{color}{value:>6}{reset}",
                            bold = style::Bold,
                            color = color,
                            value = value,
                            reset = style::Reset,
                        )?;
                    }
                }

                let score = format!("Score: {}", self.score);
                write!(w, "{}", cursor::Down(1))?;
                write!(w, "{}", cursor::Left(score.len() as u16))?;
                write!(w, "{}", score)?;

                let mut msg = "";
                if self.any(|x| x >= 2048) {
                    msg = "YOU WON";
                } else if !self.has_moves() {
                    msg = "YOU LOST";
                }
                write!(w, "{}", cursor::Down(1))?;
                if msg != "" {
                    write!(w, "{}", cursor::Left(msg.len() as u16))?;
                    write!(w, "{}{}{}", style::Bold, msg, style::Reset)?;
                }

                msg = "[q]uit";
                write!(w, "{}", cursor::Down(2))?;
                write!(w, "{}", cursor::Left(msg.len() as u16))?;
                write!(w, "{}", msg)?;

                write!(w, "{}", cursor::Hide)?;

                w.flush()?;

                Ok(())
            }
        }
    }

    pub fn has_moves(&self) -> bool {
        for i in 0..self.size {
            for j in 0..self.size {
                if self.grid[i][j] == 0
                    || (j != self.size - 1 && self.grid[i][j] == self.grid[i][j + 1])
                    || (i != self.size - 1 && self.grid[i][j] == self.grid[i + 1][j])
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn any<T>(&self, f: T) -> bool
    where
        T: Fn(u32) -> bool,
    {
        for i in 0..self.size {
            for j in 0..self.size {
                if f(self.grid[i][j]) {
                    return true;
                }
            }
        }
        false
    }

    pub fn fill_random_cells(&mut self) {
        let mut count = 0;
        let mut rng = rand::thread_rng();
        loop {
            if count == 2 || !self.any(|x| x == 0) {
                return;
            }

            let i = self.random_pos.sample(&mut rng);
            let j = self.random_pos.sample(&mut rng);

            if self.grid[i][j] == 0 {
                self.grid[i][j] = self.new_val[self.random_val.sample(&mut rng)];
                count += 1;

                if random() {
                    break;
                }
            }
        }
    }

    pub fn get_padding(&self, total_width: u16, total_height: u16) -> Option<(u16, u16)> {
        let grid_width = self.size * 6;
        let grid_height = self.size + 4;

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
