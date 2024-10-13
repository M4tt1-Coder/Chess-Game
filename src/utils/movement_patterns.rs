//! This module is made for all possible patterns of every piece.
//!
//! A 'movement-pattern' is basically a closure calling the movement-function in a strict order.

// using statements

//constants

use crate::{
    enums::{FigureColor, FigureType},
    structs::{Board, Field, Figure, MoveHistory},
};

use super::movement_types::movements::{diagonal, down, up};
/// Stores a specific movement possibilities for a given piece.
///
/// Inside the closures will be a continues calling of the movement-patterns.
pub struct Pattern {
    pub steps: Vec<
        Box<
            dyn for<'a> Fn(
                &'a Board,
                &'a Field,
                &'a FigureColor,
                &'a MoveHistory,
            ) -> (&'a Field, Option<(usize, usize)>),
        >,
    >,
}

/// Defines a trait that all piece-pattern-structs must implement.
///
/// The 'execute_patterns'-function will call all closures of the pattern.
pub trait MovementPatternExecutor {
    type Pattern;

    /// Executes all the patterns of a piece
    fn execute_patterns(
        &self,
        board: &Board,
        current_field: &Field,
        selected_field: &Field,
        move_history: &MoveHistory,
        _piece_color: &FigureColor,
    ) -> (bool, Option<(usize, usize)>);

    fn set_up_patterns() -> Self;
}

// _________
// START : Pawn Implementation
// _________

/// A pattern represenatative for a pawn.
///
/// Includes 4 patterns (closures):
///
/// -> throw a piece diagonal left / right
///
/// -> move one forward
///
/// -> move two forward
pub struct PawnPatterns {
    pub pawn_patters: Pattern,
}

impl MovementPatternExecutor for PawnPatterns {
    type Pattern = Pattern;

    fn execute_patterns(
        &self,
        board: &Board,
        current_field: &Field,
        selected_field: &Field,
        move_history: &MoveHistory,
        _piece_color: &FigureColor,
    ) -> (bool, Option<(usize, usize)>) {
        // define all local representative variables
        let mut leads_to_selected_field = false;
        let mut position_of_thrown_piece = None;
        // execute all patterns
        for pattern in &self.pawn_patters.steps {
            // get result data from pattern execution
            let (last_field, position_of_thrown_piece_result) =
                pattern(board, current_field, _piece_color, move_history);
            // when the move was valid assing the data
            if last_field == selected_field {
                position_of_thrown_piece = position_of_thrown_piece_result;
                leads_to_selected_field = true;
            }
        }

        (leads_to_selected_field, position_of_thrown_piece)
    }

    fn set_up_patterns() -> PawnPatterns {
        let mut output_patterns: Vec<
            Box<
                dyn for<'a> Fn(
                    &'a Board,
                    &'a Field,
                    &'a FigureColor,
                    &'a MoveHistory,
                ) -> (&'a Field, Option<(usize, usize)>),
            >,
        > = Vec::new();

        // TODO - a pawn changes into a different piece when it reaches the end of the board -> implement this rule

        // first pattern
        let one_forward: Box<
            dyn for<'a> Fn(
                &'a Board,
                &'a Field,
                &'a FigureColor,
                &'a MoveHistory,
            ) -> (&'a Field, Option<(usize, usize)>),
        > = Box::new(
            move |board: &Board,
                  current_field: &Field,
                  piece_color: &FigureColor,
                  _: &MoveHistory|
                  -> (&Field, Option<(usize, usize)>) {
                // have to determine if the player moves his piece up or down on the board
                let mut next_field = current_field;

                match piece_color {
                    FigureColor::Black => {
                        next_field = down(board, current_field, piece_color);
                    }
                    FigureColor::White => {
                        next_field = up(board, current_field, piece_color);
                    }
                    _ => {
                        println!("Didn't receive a piece color!");
                    } // won't happen just for the case
                }

                // when the field where the pawn moves with a straight step
                // has a figure it can not move there
                if next_field.content.is_some() {
                    return (current_field, None);
                }

                (next_field, None)
            },
        );
        // second pattern
        let two_forward: Box<
            dyn for<'a> Fn(
                &'a Board,
                &'a Field,
                &'a FigureColor,
                &'a MoveHistory,
            ) -> (&'a Field, Option<(usize, usize)>),
        > = Box::new(
            move |board: &Board,
                  current_field: &Field,
                  piece_color: &FigureColor,
                  _: &MoveHistory|
                  -> (&Field, Option<(usize, usize)>) {
                if current_field.position.0 != 1 && current_field.position.0 != 6 {
                    return (current_field, None);
                }

                let mut next_field = current_field;

                match piece_color {
                    FigureColor::Black => {
                        // first step
                        next_field = down(board, current_field, piece_color);
                        //check if the piece has moved -> if not then return the current field
                        if next_field.position == current_field.position {
                            return (current_field, None);
                        }
                        // second step
                        next_field = down(board, next_field, piece_color);
                    }
                    FigureColor::White => {
                        //first step
                        next_field = up(board, current_field, piece_color);
                        //check if the piece has moved -> if not then return the current field
                        // can't pass the current field twice to the step functions
                        // need to pass the updated field 'next_field' to the step function
                        if next_field.position == current_field.position {
                            return (current_field, None);
                        }
                        // second step
                        next_field = up(board, next_field, piece_color);
                    }
                    _ => {
                        println!("Didn't receive a piece color!");
                    } // won't happen just for the case
                }

                // when the field where the pawn moves with a straight step
                // has a figure it can not move there
                if next_field.content.is_some() {
                    return (current_field, None);
                }

                (next_field, None)
            },
        );
        // third pattern
        let throw_left: Box<
            dyn for<'a> Fn(
                &'a Board,
                &'a Field,
                &'a FigureColor,
                &'a MoveHistory,
            ) -> (&'a Field, Option<(usize, usize)>),
        > = Box::new(
            move |board: &Board,
                  current_field: &Field,
                  piece_color: &FigureColor,
                  move_history: &MoveHistory|
                  -> (&Field, Option<(usize, usize)>) {
                let mut next_field = current_field;
                let mut position_of_piece_thrown = None;
                // determine if there is a pawn that previouly moved two steps forward
                // then it can be thrown
                // let mut en_passant_possible = false;

                match piece_color {
                    FigureColor::Black => {
                        (next_field, position_of_piece_thrown) = diagonal(
                            board,
                            current_field,
                            false,
                            false,
                            piece_color,
                            Some(move_history),
                        );
                    }
                    FigureColor::White => {
                        (next_field, position_of_piece_thrown) = diagonal(
                            board,
                            current_field,
                            true,
                            false,
                            piece_color,
                            Some(move_history),
                        );
                    }
                    FigureColor::NotFound => {
                        println!("Didn't find a color of the piece!");
                    }
                }

                (next_field, position_of_piece_thrown)
            },
        );
        //fourth pattern
        let throw_right: Box<
            dyn for<'a> Fn(
                &'a Board,
                &'a Field,
                &'a FigureColor,
                &'a MoveHistory,
            ) -> (&'a Field, Option<(usize, usize)>),
        > = Box::new(
            move |board: &Board,
                  current_field: &Field,
                  piece_color: &FigureColor,
                  move_history: &MoveHistory|
                  -> (&Field, Option<(usize, usize)>) {
                let mut next_field = Field::as_ref();
                let mut position_of_piece_thrown = None;

                match piece_color {
                    FigureColor::Black => {
                        (next_field, position_of_piece_thrown) = diagonal(
                            board,
                            current_field,
                            false,
                            true,
                            piece_color,
                            Some(move_history),
                        );
                    }
                    FigureColor::White => {
                        (next_field, position_of_piece_thrown) = diagonal(
                            board,
                            current_field,
                            true,
                            true,
                            piece_color,
                            Some(move_history),
                        );
                    }
                    FigureColor::NotFound => {
                        println!("Didn't receive anything like a piece color!");
                    }
                }

                (next_field, position_of_piece_thrown)
            },
        );

        // Add all patterns to the vector
        output_patterns.push(throw_right);
        output_patterns.push(throw_left);
        output_patterns.push(one_forward);
        output_patterns.push(two_forward);

        // Create and return a 'PawnPatterns'
        PawnPatterns {
            pawn_patters: Pattern {
                steps: output_patterns,
            },
        }
    }
}

// __________
// END : Pawn Implementation
// __________

// TODO - Create rook pattern implementation

// __________
// START : Rook Implementation
// __________

/// Execution struct for iterating through the movements of a rook.
///
/// Contains a property 'rook_pattern' that holds all patterns.
pub struct RookPattern {
    /// List of patterns
    pub rook_pattern: Pattern,
}

impl MovementPatternExecutor for RookPattern {
    type Pattern = Pattern;

    fn execute_patterns(
        &self,
        board: &Board,
        current_field: &Field,
        selected_field: &Field,
        move_history: &MoveHistory,
        _piece_color: &FigureColor,
    ) -> (bool, Option<(usize, usize)>) {
        todo!()
    }

    fn set_up_patterns() -> Self {
        todo!()
    }
}

// __________
// END : Rook Implementation
// __________

// __________
//private functions
// __________

/// Takes in the reference to a piece.
///
/// Returns whether the type of the piece is a 'knight' or not.
///
/// # Examples
///
/// ```
///     let piece = Figure { figure_type: FigureType::Knight, color: FigureColor::White, thrown: false };
///     if (is_the_piece_a_knight(&piece))
///     {
///          /* result is true */   
///     }
/// ```
pub fn is_the_piece_a_knight(piece: &Figure) -> bool {
    piece.figure_type == FigureType::Knight
}
