#[derive(Clone, Copy)]
pub enum Colour {
    Black,
    White,
}

#[derive(Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy)]
pub struct Piece {
    colour: Colour,
    piece_type: PieceType,
}

impl Piece {
    pub fn new(colour: Colour, piece_type: PieceType) -> Self {
        Self { colour, piece_type }
    }

    pub fn colour(&self) -> &Colour {
        &self.colour
    }

    pub fn piece_type(&self) -> &PieceType {
        &self.piece_type
    }
}

pub struct Board {
    squares: [Option<Piece>; 64],
}

impl Board {
    pub fn from_fen(fen: &str) -> Result<Board, &'static str> {
        let mut squares: [Option<Piece>; 64] = [None; 64];
        let parts: Vec<&str> = fen.split_whitespace().collect();

        if parts.len() < 1 {
            return Err("Invalid FEN string");
        }

        let board_part = parts[0];

        let mut row = 7;
        let mut col = 0;

        for c in board_part.chars() {
            if c == '/' {
                row -= 1;
                col = 0;

                continue;
            } else if c.is_digit(10) {
                col += c.to_digit(10).unwrap() as usize;

                continue;
            }

            let piece = match c {
                'P' => Some(Piece::new(Colour::White, PieceType::Pawn)),
                'N' => Some(Piece::new(Colour::White, PieceType::Knight)),
                'B' => Some(Piece::new(Colour::White, PieceType::Bishop)),
                'R' => Some(Piece::new(Colour::White, PieceType::Rook)),
                'K' => Some(Piece::new(Colour::White, PieceType::King)),
                'Q' => Some(Piece::new(Colour::White, PieceType::Queen)),
                'p' => Some(Piece::new(Colour::Black, PieceType::Pawn)),
                'n' => Some(Piece::new(Colour::Black, PieceType::Knight)),
                'b' => Some(Piece::new(Colour::Black, PieceType::Bishop)),
                'r' => Some(Piece::new(Colour::Black, PieceType::Rook)),
                'k' => Some(Piece::new(Colour::Black, PieceType::King)),
                'q' => Some(Piece::new(Colour::Black, PieceType::Queen)),
                _ => return Err("Invalid character in FEN string"),
            };

            if piece.is_some() {
                squares[row * 8 + col] = piece;
                col += 1;
            }
        }
        Ok(Board { squares })
    }
}

impl Default for Board {
    fn default() -> Board {
        let default_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        // Using unwrap here is okay because we know the default FEN string is valid
        Board::from_fen(default_fen).unwrap()
    }
}
