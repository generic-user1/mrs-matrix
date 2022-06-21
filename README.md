# mrs-matrix

A **m**ultiplatform **R**u**s**t clone of [cmatrix](https://github.com/abishekvashok/cmatrix).

Unlike the original cmatrix, supports both Windows and most flavors of Linux. May also support macOS, though this is untested.

## Installation instructions
1. Install [rust](https://www.rust-lang.org/tools/install) using `rustup` (this installs [cargo](https://doc.rust-lang.org/cargo/), the Rust Package Manager)
2. Clone this repository locally with `git clone https://github.com/generic-user1/mrs-matrix.git`
3. Enter the directory of the repository with `cd mrs-matrix`
4. Install the project with `cargo install --path .`
5. Run your newly compiled binary with `mrs-matrix`. To get a list of possible options, run `mrs-matrix --help`

Note that the project can be run without installation using `cargo run --release`. Any arguments to `mrs-matrix` when running through `cargo` must be prefixed by `--` (e.g. `cargo run --release -- -c blue`)

## Dependencies

As a user, you likely won't have to worry about these as `cargo` will take care of downloading and building them for you.

- [crossterm](https://github.com/crossterm-rs/crossterm) for multiplatform terminal control.
- [coolor](https://github.com/Canop/coolor) for color management.
- [rand](https://github.com/rust-random/rand) for random number generation.
- [clap](https://github.com/clap-rs/clap) for command-line argument parsing.