use crate::board::*;

pub struct Move {
    from: usize,
    to: usize,
}

impl Move {
    pub fn from(&self) -> usize {
        self.from
    }

    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }

    pub fn to(&self) -> usize {
        self.to
    }
}

pub fn get_valid_moves(board: &Board, from: usize) -> Vec<usize> {
    let mut valid_moves = Vec::new();

    for to in 0..64 {
        if is_valid_move(board, from, to) {
            valid_moves.push(to);
        }
    }

    valid_moves
}

fn is_path_clear(board: &Board, from: usize, to: usize, row_step: i32, col_step: i32) -> bool {
    let mut current = from as i32;
    while (current / 8 + row_step) < 8 && (current / 8 + row_step) >= 0
        && (current % 8 + col_step) < 8 && (current % 8 + col_step) >= 0
    {
        current += row_step * 8 + col_step;
        if current as usize == to {
            break;
        }
        if board.squares()[current as usize].is_some() {
            return false;
        }
    }
    true
}

pub fn is_valid_move(board: &Board, from: usize, to: usize) -> bool {
    let piece = board.squares()[from];

    if piece.is_none() || from == to || to > 63 {
        return false;
    }

    let piece = piece.unwrap();
    let piece_type = piece.piece_type();
    let diff = to as i32 - from as i32;
    let row_diff = (to / 8) as i32 - (from / 8) as i32;
    let col_diff = (to % 8) as i32 - (from % 8) as i32;

    match piece_type {
        PieceType::Pawn => {
            if *piece.colour() == Colour::White && diff == -8
                || *piece.colour() == Colour::Black && diff == 8
            {
                return board.squares()[to].is_none();
            }

            if *piece.colour() == Colour::White && diff == -16 && from / 8 == 6
                || *piece.colour() == Colour::Black && diff == 16 && from / 8 == 1
            {
                return board.squares()[from + (diff / 2) as usize].is_none()
                    && board.squares()[to].is_none();
            }

            if (col_diff == 1 || col_diff == -1) && row_diff.abs() == 1 {
                return board.squares()[to]
                    .filter(|capture_piece| *capture_piece.colour() != *piece.colour())
                    .is_some();
            }

            false
        }
        PieceType::Knight => {
            (row_diff.abs() == 2 && col_diff.abs() == 1 || row_diff.abs() == 1 && col_diff.abs() == 2)
                && (board.squares()[to].is_none() || *board.squares()[to].unwrap().colour() != *piece.colour())
        }
        PieceType::Bishop => {
            (diff % 7 == 0 || diff % 9 == 0)
                && (board.squares()[to].is_none() || *board.squares()[to].unwrap().colour() != *piece.colour())
                && is_path_clear(board, from, to, diff.signum() / 7, diff.signum() % 7)
        },
        PieceType::Rook => {
            (diff % 8 == 0 || col_diff.abs() <= 7)
                && (board.squares()[to].is_none() || *board.squares()[to].unwrap().colour() != *piece.colour())
                && is_path_clear(board, from, to, diff.signum() / 8, col_diff.signum())
        },
        PieceType::Queen => {
            (diff % 8 == 0 || col_diff.abs() <= 7 || diff % 7 == 0 || diff % 9 == 0)
                && (board.squares()[to].is_none() || *board.squares()[to].unwrap().colour() != *piece.colour())
                && (is_path_clear(board, from, to, diff.signum() / 8, col_diff.signum())
                    || is_path_clear(board, from, to, diff.signum() / 7, diff.signum() % 7))
        },
        PieceType::King => {
            (diff == 1 || diff == -1 || diff == 7 || diff == 8 || diff == 9 || diff == -7 || diff == -8 || diff == -9)
                && (board.squares()[to].is_none() || *board.squares()[to].unwrap().colour() != *piece.colour())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_pawn_move() {
        let mut squares = [None; 64];
        squares[8] = Some(Piece::new(Colour::Black, PieceType::Pawn));
        let board = Board::new(squares);

        assert!(is_valid_move(&board, 8, 16));
    }

    #[test]
    fn test_valid_pawn_double_move() {
        let mut squares = [None; 64];
        squares[8] = Some(Piece::new(Colour::Black, PieceType::Pawn));
        let board = Board::new(squares);
        assert!(is_valid_move(&board, 8, 16));
    }

    #[test]
    fn test_valid_knight_move() {
        let mut squares = [None; 64];
        squares[1] = Some(Piece::new(Colour::White, PieceType::Knight));
        let board = Board::new(squares);
        assert!(is_valid_move(&board, 1, 10));
    }

    #[test]
    fn test_valid_bishop_move() {
        let mut squares = [None; 64];
        squares[28] = Some(Piece::new(Colour::White, PieceType::Bishop));
        let board = Board::new(squares);
        assert!(is_valid_move(&board, 28, 9)); // Diagonal move
    }

    #[test]
    fn test_valid_rook_move() {
        let mut squares = [None; 64];
        squares[0] = Some(Piece::new(Colour::White, PieceType::Rook));
        let board = Board::new(squares);
        assert!(is_valid_move(&board, 0, 7)); // Horizontal move
    }

    #[test]
    fn test_valid_queen_move() {
        let mut squares = [None; 64];
        squares[3] = Some(Piece::new(Colour::Black, PieceType::Queen));
        let board = Board::new(squares);
        assert!(is_valid_move(&board, 3, 59)); // Diagonal move
    }

    #[test]
    fn test_valid_king_move() {
        let mut squares = [None; 64];
        squares[4] = Some(Piece::new(Colour::White, PieceType::King));
        let board = Board::new(squares);
        assert!(is_valid_move(&board, 4, 5)); // One-square move
    }
}
