//using statements

use eframe::egui::{panel::Side, Align, Button, CentralPanel, Context, Image, Layout, SidePanel};

use crate::{
    enums::FigureColor,
    helper::{
        get_field_color_on_coordinates, get_figure_path, horizontal_seperator,
        render_player_dashboard_info, render_thrown_pieces,
    },
    structs::{replicate, Board, Field, Move},
    utils::movement_controller::begin_rule_checking,
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
                        render_player_dashboard_info(&player_two, ui, ctx);

                        horizontal_seperator(ui);

                        render_player_dashboard_info(&player_one, ui, ctx);

                        horizontal_seperator(ui);

                        render_thrown_pieces(game, ui);
                    }
                    FigureColor::Black => {
                        render_player_dashboard_info(&player_one, ui, ctx);

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

                drop(player_two);

                drop(player_one);
            });
        });
    //rotate the board 180 degrees clockwise WHEN it is possible
    CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            let mut field_was_clicked = false;
            let mut selected_field: &Field = Field::as_ref();
            let board_clone = game.field.try_lock().unwrap();
            let board: Board = replicate(&board_clone);
            drop(board_clone);
            for row in &board.content {
                ui.horizontal(|ui| {
                    for field in row {
                        match &field.content {
                            Some(content) => {
                                if ui
                                    .add_sized(
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
                                    )
                                    .clicked()
                                {
                                    //set other players turn to true when the current player moved a piece
                                    //select this field -> unselect the previous field
                                    //button_interaction(game, &board_clone.content, &field);
                                    field_was_clicked = true;
                                    selected_field = field;
                                }
                            }
                            None => {
                                if ui
                                    .add_sized(
                                        [FIELD_SIZE, FIELD_SIZE],
                                        Button::new("").fill(get_field_color_on_coordinates(
                                            field.position.0,
                                            field.position.1,
                                        )),
                                    )
                                    .clicked()
                                {
                                    field_was_clicked = true;
                                    selected_field = field;
                                }
                            }
                        }
                    }
                });

                ui.add_space(8.);
            }
            // Execute actions when a field was clicked
            if field_was_clicked {
                // 1.) Call the rule checker endpoints to check if the user made a valid move with a piece
                // 2.) Get data back from the checker that is used to store important documentation for future checking
                // 'previous_field_data' = data from the previous field that was pressed by the user
                let checking_results = begin_rule_checking(game, &board, selected_field);
                // if the user made a valid move then proceed
                if checking_results.is_there_new_move_entry {
                    // make sure the needed data is available
                    // when a field was selected before then we get its data returned from the checker
                    // piece_data.0 = position coordinates of the field where the piece stands
                    // piece_data.1 = the ID of the piece on that field
                    match checking_results.data_of_piece{
                        Some(piece_data) => game.moves_history.add_move(Move {
                            piece_id: piece_data.1,
                            from: piece_data.0,
                            to: selected_field.position,
                            number: game.moves_history.get_current_number_of_moves() + 1
                        }),
                        None => panic!("New move entry for move history regonized BUT received an empty field object!"),
                    }
                    // if a piece was thrown by a special move like "En-Passant"
                    match checking_results.coordinates_of_piece_thrown_by_special_move {
                        Some(coordinates_of_piece) => {
                            game.delete_field_content(coordinates_of_piece.1, coordinates_of_piece.0);
                        },
                        None => ()
                    }
                }
            }
        });
    });
}
