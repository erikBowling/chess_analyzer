use crate::chess_components::piece::{Color, Piece, PieceType};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Rank {
    pub fn to_int(self) -> u8 {
        self as u8
    }
}

#[derive(EnumIter, Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub fn to_int(self) -> u8 {
        self as u8
    }
}

pub struct Board {
    pieces: HashMap<(Rank, File), Piece>,
}

impl Board {
    pub fn new() -> Self {
        let mut pieces = HashMap::new();

        for rank in Rank::iter() {
            for file in File::iter() {
                match rank {
                    Rank::One => {
                        match file {
                            File::A => pieces
                                .insert((rank, file), Piece::new(PieceType::Rook, Color::White)),
                            File::B => pieces
                                .insert((rank, file), Piece::new(PieceType::Knight, Color::White)),
                            File::C => pieces
                                .insert((rank, file), Piece::new(PieceType::Bishop, Color::White)),
                            File::D => pieces
                                .insert((rank, file), Piece::new(PieceType::Queen, Color::White)),
                            File::E => pieces
                                .insert((rank, file), Piece::new(PieceType::King, Color::White)),
                            File::F => pieces
                                .insert((rank, file), Piece::new(PieceType::Bishop, Color::White)),
                            File::G => pieces
                                .insert((rank, file), Piece::new(PieceType::Knight, Color::White)),
                            File::H => pieces
                                .insert((rank, file), Piece::new(PieceType::Rook, Color::White)),
                        };
                    }
                    Rank::Two => {
                        pieces.insert((rank, file), Piece::new(PieceType::Pawn, Color::White));
                    }

                    Rank::Seven => {
                        pieces.insert((rank, file), Piece::new(PieceType::Pawn, Color::Black));
                    }
                    Rank::Eight => {
                        match file {
                            File::A => pieces
                                .insert((rank, file), Piece::new(PieceType::Rook, Color::Black)),
                            File::B => pieces
                                .insert((rank, file), Piece::new(PieceType::Knight, Color::Black)),
                            File::C => pieces
                                .insert((rank, file), Piece::new(PieceType::Bishop, Color::Black)),
                            File::D => pieces
                                .insert((rank, file), Piece::new(PieceType::Queen, Color::Black)),
                            File::E => pieces
                                .insert((rank, file), Piece::new(PieceType::King, Color::Black)),
                            File::F => pieces
                                .insert((rank, file), Piece::new(PieceType::Bishop, Color::Black)),
                            File::G => pieces
                                .insert((rank, file), Piece::new(PieceType::Knight, Color::Black)),
                            File::H => pieces
                                .insert((rank, file), Piece::new(PieceType::Rook, Color::Black)),
                        };
                    }
                    _ => {}
                }
            }
        }

        Self { pieces: pieces }
    }

    pub fn get_pieces(&self) -> &HashMap<(Rank, File), Piece> {
        &self.pieces
    }

    pub fn to_fen_notation(&self) -> String {
        let mut fen_parts = Vec::new();

        for rank in Rank::iter() {
            let mut rank_string = String::new();
            let mut empty_count = 0;
            for file in File::iter() {
                if self.pieces.contains_key(&(rank, file)) {
                    if empty_count > 0 {
                        rank_string.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    let piece = self.pieces.get(&(rank, file));
                    rank_string.push(piece.unwrap().to_fen_notation());
                } else {
                    empty_count += 1;
                }
            }

            if empty_count > 0 {
                rank_string.push_str(&empty_count.to_string());
            }

            fen_parts.push(rank_string);
        }

        fen_parts.reverse();
        fen_parts.join("/")
    }

    pub fn get_pieces_mut(&mut self) -> &mut HashMap<(Rank, File), Piece> {
        &mut self.pieces
    }

    // pub fn move_piece(&mut self, from_square: &mut Square, to_square: &mut Square) {
    //     // TODO: Check if the to_square has a piece
    //     // If so, handle collision
    //     match from_square.get_piece() {
    //         Some(piece) => {
    //             let new_piece = Piece::new(*piece.piece_type(), *piece.color());
    //             from_square.clear_piece();

    //             to_pieces.insert(new_piece);
    //         }
    //         None => {}
    //     }
    // }
}
