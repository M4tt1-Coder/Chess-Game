//use std::rc::Rc;

use uuid::Uuid;

//using statements
use crate::{
    structs::{Board, CheckingResults, Field},
    Game,
};

use super::chess_rules::{can_move_to_new_field, can_player_move_this_pieces};

//constants

/// Executes all checks wether a player made a right move or not.
///
/// Returns a boolean which determines wether a valid move was made or not &
/// the data of the field, where a piece was, before it moved to a new field.
///
/// # Arguments
/// * `game` - The current game instance
/// * `board` - The board of the current game
/// * `selected_field` - The field on which the player selected to click
pub fn begin_rule_checking(
    game: &Game,
    board: &Board,
    selected_field: &Field,
    // boolean determines whether a new POSSIBLE move was made by the user
    //
) -> CheckingResults {
    //check if a field was selected before if so check if the this field has a piece which can move
    //if this field is in the movement range then move the piece to the position
    if let Some(field) = is_a_field_selected(board) {
        if field.content.is_some() {
            // return the right data in the case when the user made a valid move
            let mut has_user_moved_piece = false;
            //check if the piece can move to this new selected field
            if can_move_to_new_field(board, field, selected_field, &game.moves_history)//check if the planned piece is valid
                && can_player_move_this_pieces(game, &field.content.as_ref().unwrap().color)
            //depending on the players piece color and the selected figure's color -> allow the move or not
            {
                // TODO - Player turns should not change when the player clicks two times on a field with a piece

                game.move_figure_to_new_field(field, selected_field);
                game.next_players_turn();
                has_user_moved_piece = true;
            }

            game.field_not_selected_anymore();
            //
            if has_user_moved_piece {
                CheckingResults {
                    is_there_new_move_entry: true,
                    data_of_piece: Some((
                        field.position,
                        match field.content.as_ref() {
                            Some(figure) => figure.id,
                            None => Uuid::new_v4(),
                        },
                    )),
                }
            } else {
                CheckingResults {
                    is_there_new_move_entry: false,
                    data_of_piece: None,
                }
            }
        } else {
            //first make sure no field is selected
            game.field_not_selected_anymore();

            //than select a new field

            game.select_field(
                selected_field.position.0 as usize,
                selected_field.position.1 as usize,
            );

            CheckingResults {
                is_there_new_move_entry: false,
                data_of_piece: None,
            }
        }
    } else {
        game.select_field(
            selected_field.position.0 as usize,
            selected_field.position.1 as usize,
        );
        //select_field(selected_field);
        CheckingResults {
            is_there_new_move_entry: false,
            data_of_piece: None,
        }
    }
}

//private functions

/// Dedicates if a field is selected on the gameboard
///
/// When there are more fields selected than one the app will panic
fn is_a_field_selected(board: &Board) -> Option<&Field> {
    let mut selected_counter: u8 = 0;

    let mut selected_field: Option<&Field> = None;

    for row in &board.content {
        for field in row {
            if field.selected {
                selected_field = Some(field);
                selected_counter += 1;
            }
        }
    }

    if selected_counter > 1 {
        panic!("There were to many fields selected!");
    }

    selected_field
}
