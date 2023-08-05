mod board;

use crate::board::*;

struct Game {
    board: Board,
}

fn main() {
    let game = Game {
        board: Board::default(),
    };

    println!("Hello, world!");
}
