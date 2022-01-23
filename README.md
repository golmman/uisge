# Uisge

Uisge is a [board game](https://boardgamegeek.com/boardgame/11421/uisge) for 2
players with simple [rules](https://omerkel.github.io/Uisge/html5/src/uisge_rules-en.html).

This application allows you to play Uisge against the computer in your terminal.

![Screenshot](https://github.com/golmman/uisge/blob/main/uisge.png "Uisge")

## Rule summary

- You play as either black or white, white begins
- There are two different piece types: pawns `o` and kings `W`
- In the starting position there are only pawns
- The only allowed pawn move is jumping over other pieces horizontally or vertically
- Kings are allowed to jump, like pawns, but also to move like a chess king
- Each jump changes the jumping piece to the other piece type (a jumping pawn becomes a king, a king becomes a pawn after jumping)
- You win when all your pieces are transformed to kings or when your opponent has no moves

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
