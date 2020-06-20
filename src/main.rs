mod error;
mod game;

extern crate rand;
extern crate termion;

use std::env::args;
use std::error::Error;
use std::io::{stdin, stdout, Stdout, Write};
use std::iter::Iterator;
use std::process::exit;

use game::Game;

use termion::color;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::{AlternateScreen, ToMainScreen};
use termion::style;

const DEFAULT_GRID_SIZE: usize = 4;

macro_rules! catch {
    ($game:ident, $method:ident, $out:ident) => {
        if let Err(err) = $game.$method(&mut $out) {
            restore(&mut $out);
            print!("Exited with error: {}.", err.description());
            exit(1);
        }
    };
}

fn main() {
    let grid_size = args().nth(1).map_or(DEFAULT_GRID_SIZE, |raw| {
        raw.parse::<usize>().unwrap_or(DEFAULT_GRID_SIZE)
    });

    let mut game = Game::new(grid_size);
    game.fill_random_cells();

    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    catch!(game, write_to, screen);

    for key in stdin().keys() {
        match key.unwrap() {
            Key::Up if game.has_moves() => catch!(game, move_up, screen),
            Key::Right if game.has_moves() => catch!(game, move_right, screen),
            Key::Down if game.has_moves() => catch!(game, move_down, screen),
            Key::Left if game.has_moves() => catch!(game, move_left, screen),
            Key::Char('q') | Key::Ctrl('q') | Key::Ctrl('c') => return restore(&mut screen),
            _ => {}
        }
    }
}

fn restore(screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
    screen.suspend_raw_mode().unwrap();
    write!(
        screen,
        "{}{}{}{}{}",
        style::Reset,
        color::Bg(color::Reset),
        color::Fg(color::Reset),
        cursor::Show,
        ToMainScreen,
    )
    .unwrap();
}
