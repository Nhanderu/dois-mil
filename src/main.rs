mod error;
mod game;

extern crate rand;
extern crate termion;

use std::env::args;
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
const ARG_VERSION: [&str; 4] = ["version", "--version", "v", "-v"];
const ARG_HELP: [&str; 4] = ["help", "--help", "h", "-h"];
const MSG_HELP: &str = "\
usage: dois-mil [<commands>] [<size>]

Commands:
    help     Shows this message
    version  Shows the game's version

Args:
    size  Defines the game's grid size (default to 4)

";

macro_rules! catch {
    ($game:ident, $method:ident, $out:expr) => {
        if let Err(err) = $game.$method(&mut $out) {
            restore(&mut $out);
            print!("Exited with error: {}.", err);
            exit(1);
        }
    };
}

fn main() {
    let args: Vec<String> = args().collect();
    match args.len() {
        1 => run(DEFAULT_GRID_SIZE),
        2 => {
            if ARG_VERSION.contains(&args[1].as_ref()) {
                print_and_exit(env!("CARGO_PKG_VERSION"), 0);
            }
            if ARG_HELP.contains(&args[1].as_ref()) {
                print_and_exit(MSG_HELP, 0);
            }
            match args[1].parse() {
                Ok(grid_size) => run(grid_size),
                Err(_) => print_and_exit("Invalid argument.", 1),
            }
        }
        _ => print_and_exit("Unknown arguments.", 1),
    }
}

fn run(grid_size: usize) {
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

fn print_and_exit(msg: &str, exit_code: i32) {
    print!("{}", msg);
    exit(exit_code);
}
