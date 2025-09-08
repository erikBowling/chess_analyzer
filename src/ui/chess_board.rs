use crate::chess_components::board::{Board, File, Rank};
use crate::chess_components::piece::{Color, Piece, PieceType};
use eframe::egui;
use strum::IntoEnumIterator;

struct DragState {
    piece: Piece,
    from_rank: Rank,
    from_file: File,
}

pub struct ChessBoardComponent {
    board: Board,
    square_size: f32,
    dragging_piece: Option<DragState>,
}

impl ChessBoardComponent {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            square_size: 60.0,
            dragging_piece: None,
        }
    }

    fn get_piece_image(&self, piece: &Piece) -> egui::ImageSource {
        match (piece.piece_type(), piece.color()) {
            (PieceType::King, Color::White) => egui::include_image!("pieces/king-w.svg"),
            (PieceType::King, Color::Black) => egui::include_image!("pieces/king-b.svg"),
            (PieceType::Queen, Color::White) => egui::include_image!("pieces/queen-w.svg"),
            (PieceType::Queen, Color::Black) => egui::include_image!("pieces/queen-b.svg"),
            (PieceType::Rook, Color::White) => egui::include_image!("pieces/rook-w.svg"),
            (PieceType::Rook, Color::Black) => egui::include_image!("pieces/rook-b.svg"),
            (PieceType::Bishop, Color::White) => egui::include_image!("pieces/bishop-w.svg"),
            (PieceType::Bishop, Color::Black) => egui::include_image!("pieces/bishop-b.svg"),
            (PieceType::Knight, Color::White) => egui::include_image!("pieces/knight-w.svg"),
            (PieceType::Knight, Color::Black) => egui::include_image!("pieces/knight-b.svg"),
            (PieceType::Pawn, Color::White) => egui::include_image!("pieces/pawn-w.svg"),
            (PieceType::Pawn, Color::Black) => egui::include_image!("pieces/pawn-b.svg"),
        }
    }

    fn is_light_square(&self, rank: &Rank, file: &File) -> bool {
        (rank.to_int() + file.to_int()) % 2 == 0
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        for rank in Rank::iter().rev() {
            for file in File::iter() {}
        }
    }

    pub fn draw_square(&mut self, ui: &mut egui::Ui, color: egui::Color32, top_left: egui::Pos2) {}
}
