# Uisge

Uisge is a [board game](https://boardgamegeek.com/boardgame/11421/uisge) for 2
players with simple [rules](https://omerkel.github.io/Uisge/html5/src/uisge_rules-en.html).

This application allows you to play Uisge against the computer in your terminal.

![Screenshot](https://github.com/golmman/uisge/blob/main/uisge.png "Uisge")

## Features

- Terminal application
- Play against computer as black or white
  - Default search depth is 11 moves
  - You can change the search depth by setting the `SEARCH_DEPTH_MAX` environment variable
  - See below for an example
- Play both sides
- Let the computer analyze postions

## Run and Build Instructions

- Install [Rust](https://www.rust-lang.org/)
- Clone this repository
- Navigate to the repository root directory
- Run the release version with `cargo run --release`
- If you only need the build type `cargo build --release`, the executable can be found in `./target/release/`
- Set the search depth with `SEARCH_DEPTH_MAX=5 cargo run --release`

### Good luck!
