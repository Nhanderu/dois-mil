# Dois Mil

[![License][badge-1-img]][badge-1-link]
[![Crates.io][badge-2-img]][badge-2-link]

2048 game in the terminal!

## Game

[![Example](https://asciinema.org/a/349256.svg "Example")][1]

## Install

### Brew

```sh
brew tap Nhanderu/packages
brew install dois-mil
```

### Cargo

```sh
cargo install dois-mil
```

## Run

#### `dois-mil`

Runs the game with default configuration.

### Commands

#### `dois-mil help`

Shows the CLI help message.

#### `dois-mil version`

Shows the CLI version.

### Arguments

#### `dois-mil <grid size>`

Runs the game with a different grid size (e.g. `dois-mil 6` executes a
6x6 grid).

The bigger the grid, the easier the game gets.

## To-do

- [ ] Game saving

## License

This project code is in the public domain. See the [LICENSE file][2].

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you shall be in the public
domain, without any additional terms or conditions.

[1]: https://asciinema.org/a/349256
[2]: ./LICENSE

[badge-1-img]: https://img.shields.io/github/license/Nhanderu/dois-mil?style=flat-square
[badge-1-link]: https://github.com/Nhanderu/dois-mil/blob/master/LICENSE
[badge-2-img]: https://img.shields.io/crates/v/dois-mil?style=flat-square
[badge-2-link]: https://crates.io/crates/dois-mil
