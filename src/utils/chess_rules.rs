//TODO - Create a rule system that includes all chess rules
// different pieces with different rules -> movement rules
// -> when can I throw one piece?
//     -> does a check threatening my king?

// TODO - add a visual effect for fields where the piece can move

// -> pawns can change into a different piece when they reach the opposite side of the board

//using statements

use crate::{
    enums::{FigureColor, FigureType},
    structs::{Board, Field, Figure, MoveHistory},
    Game,
};

use super::movement_patterns::{MovementPatternExecutor, PawnPatterns};

//constants

/// The control point where all chess rules are executed
///
/// Returns true when all rules are followed
pub fn can_move_to_new_field(
    board: &Board,
    previous_field: &Field,
    new_field: &Field,
    move_history: &MoveHistory,
) -> (bool, Option<(usize, usize)>) {
    match &previous_field.content {
        Some(figure) => match figure.figure_type {
            FigureType::Pawn => {
                // setup patterns
                let pawn_patterns = PawnPatterns::set_up_patterns();
                // execute patterns
                let execution_result = pawn_patterns.execute_patterns(
                    board,
                    previous_field,
                    new_field,
                    move_history,
                    &figure.color,
                );
                // return result
                execution_result
            }
            FigureType::Rook => todo!(),
            FigureType::Bishop => todo!(),
            FigureType::Knight => todo!(),
            FigureType::Queen => todo!(),
            FigureType::King => todo!(),
        },
        None => (false, None),
    }
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
    } else if &player_two.figure_color == piece_color {
        output = true;
    }

    drop(player_one);
    drop(player_two);

    output
}

/// Returns the a reference to a pawn that reached the end of the board.
///
/// Checks if a pawn is on the first and last row of the board.
///
/// There can only be one pawn on the board end at a time -> if not it fails.
pub fn has_a_pawn_reached_end(
    board: &Board,
    is_white_top: bool,
) -> Option<(&Figure, (usize, usize))> {
    // depending on whether the white pieces are situated on the top -> adjust accordingly

    let pawn_at_the_end = get_pawn_on_board_end(board);

    match pawn_at_the_end {
        Some(field) => {
            if is_white_top {
                match &field.content {
                    Some(pawn) => if ( pawn.color == FigureColor::White && field.position.0 == 7 ) || (pawn.color == FigureColor::Black && field.position.0 == 1){
                        return Some((pawn,(field.position.0 as usize, field.position.1 as usize)));
                    } else {
                        return None;
                    },
                    None => panic!("A error ocurred when trying to determine the pawn on the first / last row of the board!"),
                }
            } else {
                match &field.content {
                    Some(pawn) => if ( pawn.color == FigureColor::White && field.position.0 == 0 ) || (pawn.color == FigureColor::Black && field.position.0 == 7){
                        return Some((pawn,(field.position.0 as usize, field.position.1 as usize)));
                    } else {
                        return None;
                    },
                    None => panic!("A error ocurred when trying to determine the pawn on the first / last row of the board!"),
                }
            }
        }
        None => return None,
    }
}

//private functions

/// Returns a pawn if there is one on the board ends.
///
/// Panics when there are multiple pawns on the board ends.
fn get_pawn_on_board_end(board: &Board) -> Option<&Field> {
    // check if there are two or more pawns on the board end
    // counts the number of pawns
    let mut number_of_pawns: u8 = 0;
    let mut output_pawn: Option<&Field> = None;
    // top row
    for field in &board.content[0] {
        match &field.content {
            Some(figure) => {
                if figure.figure_type == FigureType::Pawn {
                    number_of_pawns += 1;
                    output_pawn = Some(field);
                }
            }
            None => (),
        }
    }
    // bottom row
    for field in &board.content[7] {
        match &field.content {
            Some(figure) => {
                if figure.figure_type == FigureType::Pawn {
                    number_of_pawns += 1;
                    output_pawn = Some(field);
                }
            }
            None => (),
        }
    }

    // check for multiple pawns on the board
    if number_of_pawns > 1 {
        panic!("There are multiple pawns on the board ends.");
    }

    return output_pawn;
}
