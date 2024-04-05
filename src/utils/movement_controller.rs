//use std::rc::Rc;

//using statements
use crate::{structs::Field, Game};

use super::chess_rules::{can_move_to_new_field, can_player_move_this_pieces};

//constants

pub fn button_interaction(game: &Game, board: &Vec<Vec<Field>>, selected_field: &Field) {
    //check if a field was selected before if so check if the this field has a piece which can move
    //if this field is in the movement range then move the piece to the position
    if let Some(field) = is_a_field_selected(board) {
        if field.content.is_some() {
            //check if the piece can move to this new selected field

            if can_move_to_new_field(board, field, selected_field)//check if the planned piece is valid
                && can_player_move_this_pieces(game, &field.content.as_ref().unwrap().color)
            //depending on the players piece color and the selected figure's color -> allow the move or not
            {
                game.move_figure_to_new_field(field, selected_field);

                game.next_players_turn();
            }

            game.field_not_selected_anymore();

            //dont want to move one figure in an infinite repetition
            // game.select_field(
            //     selected_field.position.0 as usize,
            //     selected_field.position.1 as usize,
            // );
            //select_field(selected_field);
        } else {
            //first make sure no field is selected
            game.field_not_selected_anymore();

            //than select a new field

            game.select_field(
                selected_field.position.0 as usize,
                selected_field.position.1 as usize,
            );
            // select_field(selected_field);
        }
    } else {
        game.select_field(
            selected_field.position.0 as usize,
            selected_field.position.1 as usize,
        );
        //select_field(selected_field);
    }
}

//private functions

/// Dedicates if a field is selected on the gameboard
///
/// When there are more fields selected than one the app will panic
fn is_a_field_selected(board: &Vec<Vec<Field>>) -> Option<&Field> {
    let mut selected_counter: u8 = 0;

    let mut selected_field: Option<&Field> = None;

    for row in board {
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
