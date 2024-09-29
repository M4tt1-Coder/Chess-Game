// consider taking in a Box pointer for the game board

//! There are pieces with their own possible movements
//!
//! If you split them up, you get 5 moves:
//! 1.) Up
//! 2.) Down
//! 3.) Left
//! 4.) Right
//! 5.) Diagonal

use uuid::Uuid;

use crate::{
    enums::{FigureColor, FigureType},
    structs::{Board, Field, Move, MoveHistory},
};

/// Contains the movement-function for the movement patterns.
pub mod movements {

    //constants
    const ADDED_INDEX: u8 = 1;

    //using statements
    use crate::{
        enums::FigureColor,
        structs::{Board, Field, MoveHistory},
        utils::movement_patterns::is_the_piece_a_knight,
    };

    use super::is_en_passant_possible;

    /// If you want to move up with a pawn for example then it makes sure that you can.
    ///
    /// A other treatment for knights.
    pub fn up<'a>(
        board: &'a Board,
        current_field: &'a Field,
        piece_color: &FigureColor,
    ) -> &'a Field {
        if current_field.position.0 == 0 {
            return current_field;
        }

        let new_position = (current_field.position.0 - 1, current_field.position.1);

        let next_field = get_next_field(board, new_position);

        if let Some(figure) = &next_field.content {
            if &figure.color == piece_color && !is_the_piece_a_knight(figure) {
                return current_field;
            }
        }

        next_field
    }

    /// Downward direction for movements.
    ///
    /// Special handling for a knight!
    pub fn down<'b>(
        board: &'b Board,
        current_field: &'b Field,

        piece_color: &FigureColor,
    ) -> &'b Field {
        if current_field.position.0 == 7 {
            return current_field;
        }

        let new_position = (
            current_field.position.0 + ADDED_INDEX,
            current_field.position.1,
        );

        let next_field = get_next_field(board, new_position);

        if let Some(figure) = &next_field.content {
            if &figure.color == piece_color && !is_the_piece_a_knight(figure) {
                return current_field;
            }
        }

        next_field
    }

    /// Checks all possible movements to the left.
    ///
    /// Special case for a knight!
    pub fn left<'c>(
        board: &'c Board,
        current_field: &'c Field,
        piece_color: &FigureColor,
    ) -> &'c Field {
        if current_field.position.1 == 0 {
            return current_field;
        }

        let new_position = (
            current_field.position.0,
            current_field.position.1 - ADDED_INDEX,
        );

        let next_field = get_next_field(board, new_position);

        if let Some(figure) = &next_field.content {
            if &figure.color == piece_color && !is_the_piece_a_knight(figure) {
                return current_field;
            }
        }

        next_field
    }

    /// All moves to the right will be handled here.
    ///
    /// There's a special case for the move of a knight. -> 'is_knight'
    pub fn right<'d>(
        board: &'d Board,
        current_field: &'d Field,
        piece_color: &FigureColor,
    ) -> &'d Field {
        if current_field.position.1 == 7 {
            return current_field;
        }

        let new_position = (
            current_field.position.0,
            current_field.position.1 + ADDED_INDEX,
        );

        let next_field = get_next_field(board, new_position);

        if let Some(figure) = &next_field.content {
            if &figure.color == piece_color && !is_the_piece_a_knight(figure) {
                return current_field;
            }
        }

        next_field
    }

    // Add property 'en_passent_possible' to pawn objects
    // will be set to true to pawn made two primary steps
    // next round the app loops trough all pieces and sets all pawns with 'en_passent_possible' true to false

    // first loop through all piece
    // second set the prop to true of a new pawn
    // _______

    /// Handles all 4 diagonal move-possiblities.  
    ///
    /// Before the function is called in a movement pattern the boolean arguments 'to_the_right' and 'is_upwards' must be determined
    /// in the calling function.
    ///
    /// Fails if any arg is empty or invalid.
    pub fn diagonal<'e>(
        board: &'e Board,
        current_field: &'e Field,
        is_upwards: bool,
        to_the_right: bool,
        piece_color: &FigureColor,
        move_history: Option<&MoveHistory>,
    ) -> &'e Field {
        let mut _next_field = Field::as_ref();

        if is_upwards {
            // case up-right
            if to_the_right {
                if current_field.position.0 == 0 || current_field.position.1 == 7 {
                    return current_field;
                }

                let new_position = (
                    current_field.position.0 - ADDED_INDEX,
                    current_field.position.1 + ADDED_INDEX,
                );

                _next_field = get_next_field(board, new_position);
            } else {
                // case up-left
                if current_field.position.0 == 0 || current_field.position.1 == 0 {
                    return current_field;
                }

                let new_position = (
                    current_field.position.0 - ADDED_INDEX,
                    current_field.position.1 - ADDED_INDEX,
                );

                _next_field = get_next_field(board, new_position);
            }
        } else {
            // case down-right
            if to_the_right {
                if current_field.position.0 == 7 || current_field.position.1 == 7 {
                    return current_field;
                }

                let new_position = (
                    current_field.position.0 + ADDED_INDEX,
                    current_field.position.1 + ADDED_INDEX,
                );

                _next_field = get_next_field(board, new_position);
            } else {
                // case down-left
                if current_field.position.0 == 0 || current_field.position.1 == 7 {
                    return current_field;
                }

                let new_position = (
                    current_field.position.0 - ADDED_INDEX,
                    current_field.position.1 + ADDED_INDEX,
                );

                _next_field = get_next_field(board, new_position);
            }
        }

        match &_next_field.content {
            Some(figure) => {
                if &figure.color == piece_color {
                    return current_field;
                };
            }
            None => match move_history {
                Some(move_history) => {
                    if is_en_passant_possible(
                        move_history,
                        board,
                        current_field,
                        piece_color,
                        to_the_right, // where the pawn is meant to move indicates where to check
                        false,
                    ) {
                        return _next_field;
                    } else {
                        // if the pawn can't throw with a en-passant move then return the current field
                        return current_field;
                    }
                }
                None => return current_field,
            },
        }

        _next_field
    }

    //private functions

    /// Takes in new field coordinates on the board and just returns the field at that position
    #[inline]
    fn get_next_field(board: &Board, new_field_position: (u8, u8)) -> &Field {
        let mut next_field = Field::as_ref();

        for row in &board.content {
            for field in row {
                if field.position == new_field_position {
                    next_field = field;
                }
            }
        }

        next_field
    }
}

// special move / throws checking

// TODO - functionality not fully implemented yet -> debug + test to make it work
// TODO - Remove the pawn that was thrown with the en-passant

/// Checks whether the pawn can throw another pawn of the opposite color.
///
/// ```
///     // Basic ussge
///     let can_pawn_throw_with_en_passant = is_en_passant_possible(move_history,
///         board,
///         current_field,
///         pawn_color,
///         checking_on_right,
///         is_white_side_top
///      );
/// ```
///
/// Returns the result as a boolean.
fn is_en_passant_possible(
    move_history: &MoveHistory,
    board: &Board,
    current_field: &Field,
    pawn_color: &FigureColor,
    checking_on_right: bool, // for different cases in which direction a pawn move pattern is currently being checked
    is_white_side_top: bool, // I need to know on which side the piece colors are situated
) -> bool {
    // just to make sure that the piece on the current field is a pawn
    match &current_field.content {
        Some(figure) => {
            if figure.figure_type != FigureType::Pawn {
                return false;
            }
        }
        None => return false,
    }
    // check if the pawn is on a necessary board row -> index of 4th and 5th row
    if current_field.position.0 != 3 || current_field.position.0 != 4 {
        return false;
    }
    // check if the current pawn is on the right valid row depending on which side his pieces are
    if !pawn_situated_on_right_field(current_field.position.0, is_white_side_top, pawn_color) {
        return false;
    }

    // if there are pawns next to the current pawn
    // set the values to true
    let data_of_piece: (bool, Uuid) = check_if_pawn_is_on_next_field(
        checking_on_right,
        board,
        (
            current_field.position.0 as usize,
            current_field.position.1 as usize,
        ),
        pawn_color,
    );
    // check on the right

    // -> if yes check if they have made a 2-step movement
    if data_of_piece.0 {
        let last_move_in_history: &Move = move_history.get_last_item();
        // when its not the pawn that moved in the last move then return false
        if data_of_piece.1 != last_move_in_history.piece_id {
            return false;
        }
        // determine wether the pawn has moved two fields or not
        // if not return false
        let step_cound_of_pawn = last_move_in_history.to.0 - last_move_in_history.from.0;
        if step_cound_of_pawn != 2 {
            return false;
        }
        // -> if yes return yes
        return true;
    }
    // doesnt it matched the conditions return false
    false
}

// helper functions

/// Looks on which side the current iteration checks where to pawn can maybe throw another
/// pawn with the "En Passant" move.
///
/// If there is a pawn piece of the right color ....
/// -> returns true
#[inline]
fn check_if_pawn_is_on_next_field(
    checking_on_right: bool,
    board: &Board,
    pawn_position: (usize, usize),
    pawn_color: &FigureColor,
) -> (bool, Uuid) {
    if checking_on_right {
        match &board.content[pawn_position.0][pawn_position.1 + 1].content {
            Some(figure) => {
                if figure.figure_type == FigureType::Pawn {
                    if &figure.color != pawn_color {
                        return (true, figure.id);
                    }
                }
            }
            None => return (false, Uuid::new_v4()),
        }
    } else {
        // check on the left
        match &board.content[pawn_position.0][pawn_position.1 - 1].content {
            Some(figure) => {
                if figure.figure_type == FigureType::Pawn {
                    if &figure.color == pawn_color {
                        return (true, figure.id);
                    }
                }
            }
            None => return (false, Uuid::new_v4()),
        }
    }
    (false, Uuid::new_v4())
}

/// Determines if a pawn of a specific color is situated on the right field row.
///
/// Fails when a wrong color enum value was passed to the function.
#[inline]
fn pawn_situated_on_right_field(
    row_index: u8,
    is_white_side_top: bool,
    pawn_color: &FigureColor,
) -> bool {
    if is_white_side_top {
        // white is on the top
        match pawn_color {
            FigureColor::Black => match row_index {
                3 => true,
                _ => return false,
            },
            FigureColor::White => match row_index {
                4 => true,
                _ => return false,
            },
            FigureColor::NotFound => {
                println!("A wrong enum was found! Exiting!");
                panic!();
            }
        }
    } else {
        // white is on the bottom
        match pawn_color {
            FigureColor::Black => match row_index {
                4 => true,
                _ => return false,
            },
            FigureColor::White => match row_index {
                3 => true,
                _ => return false,
            },
            FigureColor::NotFound => {
                println!("A wrong enum was found! Exiting!");
                panic!();
            }
        }
    }
}
