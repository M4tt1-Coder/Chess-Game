//using statements

use eframe::egui::{panel::Side, Align, CentralPanel, Context, Layout, SidePanel};

use crate::{
    enums::FigureColor,
    helper::{horizontal_seperator, render_player_dashboard_info, render_thrown_pieces},
    Game,
};

pub fn render_chess_board(ctx: &Context, game: &mut Game) {
    SidePanel::new(Side::Left, "PlayerDashboards")
        .default_width(50.)
        .resizable(false)
        .min_width(90.)
        .show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                match game.player_one.figure_color {
                    FigureColor::White => {
                        render_player_dashboard_info(&game.player_two, ui);

                        horizontal_seperator(ui);

                        render_player_dashboard_info(&game.player_one, ui);

                        horizontal_seperator(ui);

                        render_thrown_pieces(game, ui);
                    }
                    FigureColor::Black => {
                        render_player_dashboard_info(&game.player_one, ui);

                        horizontal_seperator(ui);

                        render_player_dashboard_info(&game.player_two, ui);

                        horizontal_seperator(ui);

                        render_thrown_pieces(game, ui)
                    }
                    FigureColor::NotFound => {
                        panic!("The did not have a figure color!");
                    }
                }
            });
        });

    //TODO - add chess board
    //rotate the board 180 degrees clockwise WHEN it is possible
    CentralPanel::default().show(ctx, |ui| {});
}
