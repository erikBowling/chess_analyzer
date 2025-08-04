use eframe::egui;
use crate::chess_components::board::Board;
use crate::chess_components::piece::{Piece, PieceType, Color};
use crate::chess_components::square::{Rank, File};
use strum::IntoEnumIterator;
use std::collections::HashMap;

pub struct ChessBoardComponent {
    board: Board,
    piece_textures: HashMap<&'static str, &'static str>,
    square_size: f32,
    auto_resize: bool,
    grid: egui::Grid,
}

impl ChessBoardComponent {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let piece_textures = HashMap::from([
            ("king-w", "pieces/king-w.svg"),
            ("king-b", "pieces/king-b.svg"),
            ("queen-w", "pieces/queen-w.svg"),
            ("queen-b", "pieces/queen-b.svg"),
            ("rook-w", "pieces/rook-w.svg"),
            ("rook-b", "pieces/rook-b.svg"),
            ("bishop-w", "pieces/bishop-w.svg"),
            ("bishop-b", "pieces/bishop-b.svg"),
            ("knight-w", "pieces/knight-w.svg"),
            ("knight-b", "pieces/knight-b.svg"),
            ("pawn-w", "pieces/pawn-w.svg"),
            ("pawn-b", "pieces/pawn-b.svg"),
        ]);

        let grid = egui::Grid::new("chess_board")
                .num_columns(8)
                .spacing([0.0, 0.0])
                .min_col_width(60.0)
                .min_row_height(60.0);

        Self {
            board: Board::new(),
            piece_textures: piece_textures,
            square_size: 60.0,
            auto_resize: true,
            grid: grid
        }
    }

    fn get_piece_image_key(&self, piece: &Piece) -> String {
        let piece_name = match piece.piece_type() {
            PieceType::King => "king",
            PieceType::Queen => "queen",
            PieceType::Rook => "rook",
            PieceType::Bishop => "bishop",
            PieceType::Knight => "knight",
            PieceType::Pawn => "pawn",
        };

        let color_suffix = match piece.color() {
            Color::White => "w",
            Color::Black => "b",
        };

        format!("{}-{}", piece_name, color_suffix)
    }

    fn is_light_square(&self, rank: Rank, file: File) -> bool {
        (rank.to_int() + file.to_int()) % 2 == 1
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }


    pub fn show(&self, ui: &mut egui::Ui) {

    }
}
