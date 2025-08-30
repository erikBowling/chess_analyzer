#[derive(Debug, Copy, Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    King,
    Queen,
}

#[derive(Debug, Copy, Clone)]
pub enum Color {
    White,
    Black,
}

pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Piece { piece_type, color }
    }

    pub fn piece_type(&self) -> &PieceType {
        &self.piece_type
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn to_algebraic_notation(&self) -> Option<char> {
        match self.piece_type {
            PieceType::Pawn => None,
            PieceType::Knight => Some('N'),
            PieceType::Bishop => Some('B'),
            PieceType::Rook => Some('R'),
            PieceType::King => Some('K'),
            PieceType::Queen => Some('Q'),
        }
    }

    pub fn to_fen_notation(&self) -> char {
        let letter: char = match self.color {
            Color::White => match self.piece_type {
                PieceType::Pawn => 'P',
                PieceType::Knight => 'N',
                PieceType::Bishop => 'B',
                PieceType::Rook => 'R',
                PieceType::King => 'K',
                PieceType::Queen => 'Q',
            },
            Color::Black => match self.piece_type {
                PieceType::Pawn => 'p',
                PieceType::Knight => 'n',
                PieceType::Bishop => 'b',
                PieceType::Rook => 'r',
                PieceType::King => 'k',
                PieceType::Queen => 'q',
            },
        };

        letter
    }
}
