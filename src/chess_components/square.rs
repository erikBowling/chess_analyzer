use strum_macros::EnumIter;
use crate::chess_components::piece::Piece;

#[derive(EnumIter, Copy, Clone)]
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

pub struct Location {
    rank: Rank,
    file: File,
}

impl Location {
    pub fn new(rank: Rank, file: File) -> Self {
        Location { rank, file }
    }

    pub fn rank(&self) -> u8 {
        match self.rank {
            Rank::One => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
        }
    }

    pub fn file(&self) -> String {
        match self.file {
            File::A => "a",
            File::B => "b",
            File::C => "c",
            File::D => "d",
            File::E => "e",
            File::F => "f",
            File::G => "g",
            File::H => "h",
        }
        .to_string()
    }

    pub fn to_string(&self) -> String {
        let rank = self.rank();
        let file = self.file();

        format!("{}{}", file, rank)
    }
}

pub struct Square {
    location: Location,
    piece: Option<Piece>,
}

impl Square {
    pub fn new(location: Location, piece: Option<Piece>) -> Self {
        Square { location, piece }
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn piece(&self) -> &Option<Piece> {
        &self.piece
    }

    pub fn is_empty(&self) -> bool {
        match self.piece {
            Some(_) => false,
            None => true,
        }
    }
}
