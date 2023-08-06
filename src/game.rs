use crate::Board;

#[derive(Clone, Copy)]
pub struct Game {
    board: Board,
}

impl Game {
    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn move_piece(&mut self, from: usize, to: usize) {
        println!("Moving piece: {} to {}", from, to);
        
        self.board.move_piece(from, to);
    }
}

impl Default for Game {
    fn default() -> Game {
        Game {
            board: Board::default(),
        }
    }
}
