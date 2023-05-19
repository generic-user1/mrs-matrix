# mrs-matrix

A **m**ultiplatform **R**u**s**t clone of [cmatrix](https://github.com/abishekvashok/cmatrix).

Unlike the original cmatrix, supports both Windows and most flavors of Linux. May also support macOS, though this is untested.

## Installation

You first need the Rust environment: [https://rustup.rs](https://rustup.rs)

### Install from the repository

(recommended if you don't intend to modify the program)

    cargo install mrs-matrix --locked

### Install from source

1. Clone this repository locally with `git clone https://github.com/generic-user1/mrs-matrix.git`
2. Enter the directory of the repository with `cd mrs-matrix`
3. Install the project with `cargo install --path .`

## Usage

Run `mrs-matrix`

To get a list of possible options, run `mrs-matrix --help`

## Dependencies

As a user, you likely won't have to worry about these as `cargo` will take care of downloading and building them for you.

- [crossterm](https://github.com/crossterm-rs/crossterm) for multiplatform terminal control.
- [coolor](https://github.com/Canop/coolor) for color management.
- [rand](https://github.com/rust-random/rand) for random number generation.
- [clap](https://github.com/clap-rs/clap) for command-line argument parsing.
