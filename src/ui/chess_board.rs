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

        Self {
            board: Board::new(),
            piece_textures: piece_textures,
            square_size: 60.0,
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

    fn is_light_square(&self, rank: &Rank, file: &File) -> bool {
        (rank.to_int() + file.to_int()) % 2 == 1
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }


    pub fn show(&self, ui: &mut egui::Ui) {
        let grid = egui::Grid::new("chess_board")
                .num_columns(8)
                .spacing([0.0, 0.0])
                .min_col_width(self.square_size)
                .min_row_height(self.square_size);

        grid.show(ui, |ui| {
            for row in &self.board.squares{
                for square in row {
                    let rank = square.rank();
                    let file = square.file();
                    let square_color = if self.is_light_square(rank, file) {
                        egui::Color32::from_rgb(240, 217, 181) // Light brown
                    } else {
                        egui::Color32::from_rgb(181, 136, 99)  // Dark brown
                    };

                    // Use click_and_drag to handle both interactions
                    let (rect, response) = ui.allocate_exact_size(
                        egui::vec2(self.square_size, self.square_size),
                        egui::Sense::click_and_drag()  // This enables both click and drag
                    );

                    // Paint the square
                    ui.painter().rect_filled(rect, 0.0, square_color);

                    // Draw piece if present
                    if let Some(piece) = square.piece() {
                        let piece_key = self.get_piece_image_key(piece);
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            &piece_key,
                            egui::FontId::proportional(10.0),
                            egui::Color32::BLACK,
                        );
                    }
                }

                ui.end_row();
            }
        });
    }
}
