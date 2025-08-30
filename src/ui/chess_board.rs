use crate::chess_components::board::Board;
use crate::chess_components::piece::{Color, Piece, PieceType};
use crate::chess_components::square::{File, Rank, Square};
use eframe::egui;
use strum::IntoEnumIterator;

struct DragState {
    piece: Piece,
    from_rank: Rank,
    from_file: File,
    start_pos: egui::Pos2,
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

    pub fn draw(&self, ui: &mut egui::Ui) {
        let grid = egui::Grid::new("chess_board")
            .num_columns(8)
            .spacing([0.0, 0.0])
            .min_col_width(self.square_size)
            .min_row_height(self.square_size);

        grid.show(ui, |ui| {
            for rank in Rank::iter().rev() {
                for square in self
                    .board
                    .squares()
                    .iter()
                    .filter(|square| square.rank().to_int() == rank as u8)
                {
                    self.draw_square(ui, square);
                }

                ui.end_row();
            }
        });
    }

    fn draw_square(&self, ui: &mut egui::Ui, square: &Square) {
        let rank = square.rank();
        let file = square.file();
        let square_color = if self.is_light_square(rank, file) {
            egui::Color32::from_rgb(240, 217, 181) // Light brown
        } else {
            egui::Color32::from_rgb(181, 136, 99) // Dark brown
        };

        // Allocate space for the square
        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(self.square_size, self.square_size),
            egui::Sense::click_and_drag(),
        );

        // Paint the square background
        ui.painter().rect_filled(rect, 0.0, square_color);

        // Draw piece on top of the square if present
        if let Some(piece) = square.get_piece() {
            let image_source = self.get_piece_image(piece);

            // Create a smaller rect for the piece, centered on the square
            let piece_size = self.square_size * 0.85;
            let piece_rect =
                egui::Rect::from_center_size(rect.center(), egui::vec2(piece_size, piece_size));

            // Paint the piece image directly using the painter
            let image = egui::Image::new(image_source)
                .fit_to_exact_size(egui::vec2(piece_size, piece_size));

            image.paint_at(ui, piece_rect);
        }

        // Handle click interactions
        if response.clicked() {
            println!("Clicked on square: {:?}, {:?}", rank, file);
        }
    }
}
