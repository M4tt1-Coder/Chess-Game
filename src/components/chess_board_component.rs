//using statements

use eframe::egui::{panel::Side, Align, Button, CentralPanel, Context, Image, Layout, SidePanel};

use crate::{
    enums::FigureColor,
    helper::{
        get_field_color_on_coordinates, get_figure_path, horizontal_seperator,
        render_player_dashboard_info, render_thrown_pieces,
    },
    Game, FIELD_SIZE,
};

pub fn render_chess_board(ctx: &Context, game: &mut Game) {
    SidePanel::new(Side::Left, "PlayerDashboards")
        .default_width(50.)
        .resizable(false)
        .min_width(90.)
        .show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                let player_two = game.player_two.try_lock().unwrap();
                let player_one = game.player_one.try_lock().unwrap();

                match player_one.figure_color {
                    FigureColor::White => {
                        render_player_dashboard_info(
                            &player_two,
                            ui,
                            ctx,
                        );

                        horizontal_seperator(ui);

                        render_player_dashboard_info(
                            &player_one,
                            ui,
                            ctx,
                        );

                        horizontal_seperator(ui);

                        render_thrown_pieces(game, ui);
                    }
                    FigureColor::Black => {
                        render_player_dashboard_info(
                            &player_one,
                            ui,
                            ctx,
                        );

                        horizontal_seperator(ui);

                        render_player_dashboard_info(
                            //&game.ticker,
                            &player_two,
                            ui,
                            ctx,
                        );

                        horizontal_seperator(ui);

                        render_thrown_pieces(game, ui)
                    }
                    FigureColor::NotFound => {
                        panic!("The did not have a figure color!");
                    }
                };

                let _ = drop(player_two);

                let _ = drop(player_one);
            });
        });

    //TODO - add chess board -> checking functions when clicking on a button
    //rotate the board 180 degrees clockwise WHEN it is possible
    CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            for row in &game.field {
                ui.horizontal(|ui| {
                    for field in row {
                        match &field.content {
                            Some(content) => {
                                ui.add_sized(
                                    [FIELD_SIZE, FIELD_SIZE],
                                    Button::image(Image::new(get_figure_path(
                                        &content.figure_type,
                                        &content.color,
                                    )))
                                    .fill(
                                        get_field_color_on_coordinates(
                                            field.position.0,
                                            field.position.1,
                                        ),
                                    ),
                                );
                            }
                            None => {
                                ui.add_sized(
                                    [FIELD_SIZE, FIELD_SIZE],
                                    Button::new("").fill(get_field_color_on_coordinates(
                                        field.position.0,
                                        field.position.1,
                                    )),
                                );
                            }
                        }
                    }
                });

                ui.add_space(8.);
            }
        });
    });
}
