// TODO - Rework the lifetime handling for the functions -> better none
// consider taking in a Box pointer for the game board

//! There are pieces with their own possible movements
//!
//! If you split them up, you get 5 moves:
//! 1.) Up
//! 2.) Down
//! 3.) Left
//! 4.) Right
//! 5.) Diagonal

/// Contains the movement-function for the movement patterns.
pub mod movements {

    //constants
    const ADDED_INDEX: u8 = 1;

    //using statements
    use crate::{
        enums::FigureColor,
        structs::{Board, Field},
    };

    /// If you want to move up with a pawn for example then it makes sure that you can.
    ///
    /// A other treatment for knights.
    pub fn up<'a>(
        board: &'a Board,
        current_field: &'a Field,
        is_knight: bool,
        piece_color: &FigureColor,
    ) -> &'a Field {
        if current_field.position.0 == 0 {
            return current_field;
        }

        let new_position = (current_field.position.0 - 1, current_field.position.1);

        let next_field = get_next_field(board, new_position);

        if let Some(figure) = &next_field.content {
            if &figure.color == piece_color && !is_knight {
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
        is_knight: bool,
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
            if &figure.color == piece_color && !is_knight {
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
        is_knight: bool,
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
            if &figure.color == piece_color && !is_knight {
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
        is_knight: bool,
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
            if &figure.color == piece_color && !is_knight {
                return current_field;
            }
        }

        next_field
    }

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

        if let Some(figure) = &_next_field.content {
            if &figure.color == piece_color {
                return current_field;
            }
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
