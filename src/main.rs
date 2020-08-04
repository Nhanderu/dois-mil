mod error_handling;
mod game;

use game::Game;
use std::{
    env::args,
    io::{stdin, stdout, Stdout, Write},
    iter::Iterator,
    process::exit,
};
use termion::{
    color, cursor,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    screen::{AlternateScreen, ToMainScreen},
    style,
};

const DEFAULT_GRID_SIZE: usize = 4;
const HELP_MSG: &str = "\
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
        2 => match args[1].as_ref() {
            "help" => println!("{}", HELP_MSG),
            "version" => println!("{}", env!("CARGO_PKG_VERSION")),
            arg => match arg.parse() {
                Ok(grid_size) => run(grid_size),
                Err(_) => {
                    eprintln!("Invalid argument.\n");
                    println!("{}", HELP_MSG);
                    exit(1);
                }
            },
        },
        _ => {
            eprintln!("Unknown arguments.\n");
            println!("{}", HELP_MSG);
            exit(1);
        }
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
