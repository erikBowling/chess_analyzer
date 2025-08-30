use crate::chess_components::piece::{Color, Piece, PieceType};
use crate::chess_components::square::{File, Rank, Square};
use strum::IntoEnumIterator;

pub struct Board {
    squares: Vec<Square>,
}

impl Board {
    pub fn new() -> Self {
        // Capacity of 64 squares 8x8
        let mut board = Vec::with_capacity(64);

        for rank in Rank::iter() {
            for file in File::iter() {
                let mut square = Square::new(rank, file, None);

                match square.rank() {
                    Rank::One => match square.file() {
                        File::A => square.set_piece(Piece::new(PieceType::Rook, Color::White)),
                        File::B => square.set_piece(Piece::new(PieceType::Knight, Color::White)),
                        File::C => square.set_piece(Piece::new(PieceType::Bishop, Color::White)),
                        File::D => square.set_piece(Piece::new(PieceType::Queen, Color::White)),
                        File::E => square.set_piece(Piece::new(PieceType::King, Color::White)),
                        File::F => square.set_piece(Piece::new(PieceType::Bishop, Color::White)),
                        File::G => square.set_piece(Piece::new(PieceType::Knight, Color::White)),
                        File::H => square.set_piece(Piece::new(PieceType::Rook, Color::White)),
                    },
                    Rank::Two => {
                        square.set_piece(Piece::new(PieceType::Pawn, Color::White));
                    }
                    Rank::Seven => {
                        square.set_piece(Piece::new(PieceType::Pawn, Color::Black));
                    }
                    Rank::Eight => match square.file() {
                        File::A => square.set_piece(Piece::new(PieceType::Rook, Color::Black)),
                        File::B => square.set_piece(Piece::new(PieceType::Knight, Color::Black)),
                        File::C => square.set_piece(Piece::new(PieceType::Bishop, Color::Black)),
                        File::D => square.set_piece(Piece::new(PieceType::Queen, Color::Black)),
                        File::E => square.set_piece(Piece::new(PieceType::King, Color::Black)),
                        File::F => square.set_piece(Piece::new(PieceType::Bishop, Color::Black)),
                        File::G => square.set_piece(Piece::new(PieceType::Knight, Color::Black)),
                        File::H => square.set_piece(Piece::new(PieceType::Rook, Color::Black)),
                    },
                    _ => {}
                }
                board.push(square)
            }
        }

        println!("Board initialized with {} squares", board.len());
        Self { squares: board }
    }

    pub fn squares(&self) -> &Vec<Square> {
        &self.squares
    }

    pub fn to_fen_notation(&self) -> String {
        let mut fen_parts = Vec::new();

        for rank in Rank::iter() {
            let mut rank_string = String::new();
            let mut empty_count = 0;

            for square in self
                .squares()
                .iter()
                .filter(|square| square.rank().to_int() == rank as u8)
            {
                match square.get_piece() {
                    Some(piece) => {
                        // If we had empty squares before this piece, add the count
                        if empty_count > 0 {
                            rank_string.push_str(&empty_count.to_string());
                            empty_count = 0;
                        }
                        // Add the piece notation
                        rank_string.push(piece.to_fen_notation());
                    }
                    None => {
                        empty_count += 1;
                    }
                }

                // If the rank ends with empty squares, add the count
                if empty_count > 0 {
                    rank_string.push_str(&empty_count.to_string());
                }

                // If we're at the end of a rank, push the rank string to the parts vector
            }

            fen_parts.push(rank_string);
        }

        fen_parts.reverse();
        fen_parts.join("/")
    }

    pub fn get_square(&self, rank: Rank, file: File) -> &Square {
        match &self
            .squares
            .iter()
            .filter(|square| {
                square.rank().to_int() == rank as u8 && square.file().to_int() == file as u8
            })
            .next()
        {
            Some(square) => square,
            None => panic!("Square not found"),
        }
    }

    pub fn get_square_mut(&mut self, rank: Rank, file: File) -> &mut Square {
        match self.squares.iter_mut().find(|square| {
            square.rank().to_int() == rank as u8 && square.file().to_int() == file as u8
        }) {
            Some(square) => square,
            None => panic!("Square not found at {:?}{:?}", file, rank),
        }
    }

    pub fn move_piece(&mut self, from_square: &mut Square, to_square: &mut Square) {
        // TODO: Check if the to_square has a piece
        // If so, handle collision
        match from_square.get_piece() {
            Some(piece) => {
                let new_piece = Piece::new(*piece.piece_type(), *piece.color());
                from_square.clear_piece();

                to_square.set_piece(new_piece);
            }
            None => {}
        }
    }
}
