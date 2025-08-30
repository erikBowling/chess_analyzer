use super::chess_board::ChessBoardComponent;
use eframe::egui;

#[derive(PartialEq)]
enum AppView {
    Home,
    ChessBoard,
    Analysis,
}

pub struct ChessAnalyzerApp {
    current_view: AppView,
    chess_board: ChessBoardComponent,
}

impl ChessAnalyzerApp {
    pub fn new() -> Self {
        Self {
            current_view: AppView::Home,
            chess_board: ChessBoardComponent::new(),
        }
    }

    fn show_main_menu(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Home").clicked() {
                self.current_view = AppView::Home;
            }
            if ui.button("Chess Board").clicked() {
                self.current_view = AppView::ChessBoard;
            }
            if ui.button("Analysis").clicked() {
                self.current_view = AppView::Analysis;
            }
        });
        ui.separator();
    }

    fn show_home_view(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            ui.heading("Chess Analyzer");
            ui.add_space(20.0);
            ui.label("Welcome to the Chess Analyzer!");
            ui.add_space(10.0);
            ui.label("Use the menu above to navigate to different features.");

            ui.add_space(30.0);
            if ui.button("Open Chess Board").clicked() {
                self.current_view = AppView::ChessBoard;
            }
        });
    }

    fn show_chess_board(&mut self, ui: &mut egui::Ui) {
        self.chess_board.draw(ui);
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            ui.heading("Chess Analyzer");
            ui.add_space(20.0);
            ui.label("Welcome to the Chess Analyzer!");
            ui.add_space(10.0);
            ui.label("Use the menu above to navigate to different features.");

            ui.add_space(30.0);
            if ui.button("Open Chess Board").clicked() {
                self.current_view = AppView::ChessBoard;
            }
        });
    }

    fn show_analysis_view(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            ui.heading("Game Analysis");
            ui.add_space(20.0);
            ui.label("Analysis features will be implemented here.");
            ui.add_space(10.0);
            ui.label("This could include:");
            ui.label("• Move analysis");
            ui.label("• Position evaluation");
            ui.label("• Game history");
            ui.label("• Engine integration");
        });
    }
}

impl eframe::App for ChessAnalyzerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.show_main_menu(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| match self.current_view {
            AppView::Home => self.show_home_view(ui),
            AppView::ChessBoard => self.show_chess_board(ui),
            AppView::Analysis => self.show_analysis_view(ui),
        });

        // Optional: Add a bottom status bar
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Status:");
                match self.current_view {
                    AppView::Home => ui.label("Ready"),
                    AppView::ChessBoard => ui.label("Chess Board"),
                    AppView::Analysis => ui.label("Analysis Mode"),
                };

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("Chess Analyzer v0.1.0");
                });
            });
        });
    }
}

pub fn run_chess_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 700.0])
            .with_min_inner_size([600.0, 500.0])
            .with_resizable(true)
            .with_title("Chess Analyzer"),
        ..Default::default()
    };

    eframe::run_native(
        "Chess Analyzer",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(ChessAnalyzerApp::new()))
        }),
    )
}
