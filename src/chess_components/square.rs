use crate::chess_components::piece::Piece;
use strum_macros::EnumIter;

#[derive(EnumIter, Copy, Clone, Debug)]
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

#[derive(EnumIter, Copy, Clone, Debug)]
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

pub struct Square {
    rank: Rank,
    file: File,
    piece: Option<Piece>,
}

impl Square {
    pub fn new(rank: Rank, file: File, piece: Option<Piece>) -> Self {
        Square { rank, file, piece }
    }

    pub fn get_piece(&self) -> Option<&Piece> {
        match &self.piece {
            Some(piece) => Some(&piece),
            None => None,
        }
    }

    pub fn set_piece(&mut self, piece: Piece) {
        self.piece = Some(piece);
    }

    pub fn clear_piece(&mut self) {
        self.piece = None;
    }

    pub fn rank(&self) -> &Rank {
        &self.rank
    }

    pub fn file(&self) -> &File {
        &self.file
    }
}
