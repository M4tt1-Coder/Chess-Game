//TODO - Create a rule system that includes all chess rules
// different pieces with different rules -> movement rules
// -> when can I throw one piece?    
//     -> does a check threatening my king?

// TODO - add a visual effect for fields where the piece can move

//TODO - Look up special gaming rules
// -> pawns can change into a different piece when they reach the opposite side of the board

//using statements

use crate::{enums::FigureColor, structs::Field, Game};

//constants

/// The control point where all chess rules are executed
///
/// Returns true when all rules are followed
pub fn can_move_to_new_field(
    board: &[Vec<Field>],
    previous_field: &Field,
    new_field: &Field,
) -> bool {
    true
}

/// Makes sure a player can't move a piece which isn't from it's color
///
/// -> return true when it's the players color
///
/// -> return false when it's not
pub fn can_player_move_this_pieces(game: &Game, piece_color: &FigureColor) -> bool {
    let mut output = false;

    let player_one = game.player_one.try_lock().unwrap();
    let player_two = game.player_two.try_lock().unwrap();

    if player_one.turn {
        if &player_one.figure_color == piece_color {
            output = true;
        }
    } else {
        if &player_two.figure_color == piece_color {
            output = true;
        }
    }

    drop(player_one);
    drop(player_two);

    output
}
//private functions
