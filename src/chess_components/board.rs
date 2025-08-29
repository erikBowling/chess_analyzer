use crate::chess_components::piece::{Piece, PieceType, Color};
use crate::chess_components::square::{Square, Location, Rank, File};
use strum::IntoEnumIterator;

pub struct Board {
    pub squares: Vec<Vec<Square>>,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Vec::new();
        for rank in Rank::iter() {
            let mut row = Vec::new();
            match rank {
                Rank::One => {
                    row.push(Square::new(Location::new(Rank::One, File::A), Some(Piece::new(PieceType::Rook, Color::White))));
                    row.push(Square::new(Location::new(Rank::One, File::B), Some(Piece::new(PieceType::Knight, Color::White))));
                    row.push(Square::new(Location::new(Rank::One, File::C), Some(Piece::new(PieceType::Bishop, Color::White))));
                    row.push(Square::new(Location::new(Rank::One, File::D), Some(Piece::new(PieceType::Queen, Color::White))));
                    row.push(Square::new(Location::new(Rank::One, File::E), Some(Piece::new(PieceType::King, Color::White))));
                    row.push(Square::new(Location::new(Rank::One, File::F), Some(Piece::new(PieceType::Bishop, Color::White))));
                    row.push(Square::new(Location::new(Rank::One, File::G), Some(Piece::new(PieceType::Knight, Color::White))));
                    row.push(Square::new(Location::new(Rank::One, File::H), Some(Piece::new(PieceType::Rook, Color::White))));
                }
                Rank::Two => {
                    row.push(Square::new(Location::new(Rank::Two, File::A), Some(Piece::new(PieceType::Pawn, Color::White))));
                    row.push(Square::new(Location::new(Rank::Two, File::B), Some(Piece::new(PieceType::Pawn, Color::White))));
                    row.push(Square::new(Location::new(Rank::Two, File::C), Some(Piece::new(PieceType::Pawn, Color::White))));
                    row.push(Square::new(Location::new(Rank::Two, File::D), Some(Piece::new(PieceType::Pawn, Color::White))));
                    row.push(Square::new(Location::new(Rank::Two, File::E), Some(Piece::new(PieceType::Pawn, Color::White))));
                    row.push(Square::new(Location::new(Rank::Two, File::F), Some(Piece::new(PieceType::Pawn, Color::White))));
                    row.push(Square::new(Location::new(Rank::Two, File::G), Some(Piece::new(PieceType::Pawn, Color::White))));
                    row.push(Square::new(Location::new(Rank::Two, File::H), Some(Piece::new(PieceType::Pawn, Color::White))));
                }
                Rank::Seven => {
                    row.push(Square::new(Location::new(Rank::Seven, File::A), Some(Piece::new(PieceType::Pawn, Color::Black))));
                    row.push(Square::new(Location::new(Rank::Seven, File::B), Some(Piece::new(PieceType::Pawn, Color::Black))));
                    row.push(Square::new(Location::new(Rank::Seven, File::C), Some(Piece::new(PieceType::Pawn, Color::Black))));
                    row.push(Square::new(Location::new(Rank::Seven, File::D), Some(Piece::new(PieceType::Pawn, Color::Black))));
                    row.push(Square::new(Location::new(Rank::Seven, File::E), Some(Piece::new(PieceType::Pawn, Color::Black))));
                    row.push(Square::new(Location::new(Rank::Seven, File::F), Some(Piece::new(PieceType::Pawn, Color::Black))));
                    row.push(Square::new(Location::new(Rank::Seven, File::G), Some(Piece::new(PieceType::Pawn, Color::Black))));
                    row.push(Square::new(Location::new(Rank::Seven, File::H), Some(Piece::new(PieceType::Pawn, Color::Black))));
                }
                Rank::Eight => {
                    row.push(Square::new(Location::new(Rank::Eight, File::A), Some(Piece::new(PieceType::Rook, Color::Black))));
                    row.push(Square::new(Location::new(Rank::Eight, File::B), Some(Piece::new(PieceType::Knight, Color::Black))));
                    row.push(Square::new(Location::new(Rank::Eight, File::C), Some(Piece::new(PieceType::Bishop, Color::Black))));
                    row.push(Square::new(Location::new(Rank::Eight, File::D), Some(Piece::new(PieceType::Queen, Color::Black))));
                    row.push(Square::new(Location::new(Rank::Eight, File::E), Some(Piece::new(PieceType::King, Color::Black))));
                    row.push(Square::new(Location::new(Rank::Eight, File::F), Some(Piece::new(PieceType::Bishop, Color::Black))));
                    row.push(Square::new(Location::new(Rank::Eight, File::G), Some(Piece::new(PieceType::Knight, Color::Black))));
                    row.push(Square::new(Location::new(Rank::Eight, File::H), Some(Piece::new(PieceType::Rook, Color::Black))));
                }
                _ => {
                    row.push(Square::new(Location::new(rank, File::A), None));
                    row.push(Square::new(Location::new(rank, File::B), None));
                    row.push(Square::new(Location::new(rank, File::C), None));
                    row.push(Square::new(Location::new(rank, File::D), None));
                    row.push(Square::new(Location::new(rank, File::E), None));
                    row.push(Square::new(Location::new(rank, File::F), None));
                    row.push(Square::new(Location::new(rank, File::G), None));
                    row.push(Square::new(Location::new(rank, File::H), None));
                }
            }

            board.push(row);
        }

        // Places white pieces at the bottom of the board
        board.reverse();

        Board {
            squares: board
        }
    }

    pub fn squares(&self) -> &Vec<Vec<Square>> {
        &self.squares
    }

    pub fn to_fen_notation(&self) -> String {
        let mut fen_parts = Vec::new();

        for rank in self.squares.iter() {
            let mut rank_string = String::new();
            let mut empty_count = 0;

            // Iterate through files A to H
            for square in rank.iter() {
                match square.piece() {
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
            }

            // If the rank ends with empty squares, add the count
            if empty_count > 0 {
                rank_string.push_str(&empty_count.to_string());
            }

            fen_parts.push(rank_string);
        }

        fen_parts.reverse();
        fen_parts.join("/")
    }
}
