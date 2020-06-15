mod error;
mod game;

extern crate rand;
extern crate termion;

use std::env::args;
use std::error::Error;
use std::io::{self, stdin, stdout, Stdout, Write};
use std::iter::Iterator;

use game::Game;

use termion::color;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::{AlternateScreen,ToMainScreen};
use termion::style;

const DEFAULT_GRID_SIZE: usize = 4;

macro_rules! catch {
    ($game:ident, $method:ident, $out:ident) => {
        if let Err(err) = $game.$method(&mut $out) {
            exit(&mut $out).unwrap();
            print!("Exited with error: {}.", err.description());
            return;
        }
    };
}

fn main() {
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let grid_size = args().nth(1).map_or(DEFAULT_GRID_SIZE, |raw| {
        raw.parse::<usize>().unwrap_or(DEFAULT_GRID_SIZE)
    });

    let mut game = Game::new(grid_size);
    game.fill_random_cells();
    catch!(game, write_to, screen);

    for key in stdin().keys() {
        match key.unwrap() {
            Key::Up if game.has_moves() => catch!(game, move_up, screen),
            Key::Right if game.has_moves() => catch!(game, move_right, screen),
            Key::Down if game.has_moves() => catch!(game, move_down, screen),
            Key::Left if game.has_moves() => catch!(game, move_left, screen),
            Key::Char('q') | Key::Ctrl('q') | Key::Ctrl('c') => return exit(&mut screen).unwrap(),
            _ => {}
        }
    }
}

fn exit(screen: &mut AlternateScreen<RawTerminal<Stdout>>) -> io::Result<()> {
    write!(
        screen,
        "{}{}{}{}{}",
        style::Reset,
        color::Bg(color::Reset),
        color::Fg(color::Reset),
        cursor::Show,
        ToMainScreen,
    )
}
