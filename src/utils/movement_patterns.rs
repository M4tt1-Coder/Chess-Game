// TODO - Implement the pattern module.

//! This module is made for all possible patterns of every piece.
//!
//! A 'movement-pattern' is basically a closure calling the movement-function in a strict order.

// using statements

//constants

use crate::{
    enums::{FigureColor, FigureType},
    structs::{Board, Field, Figure},
};

use super::movement_types::movements::{diagonal, down, up};
/// Stores a specific movement possibilities for a given piece.
///
/// Inside the closures will be a continues calling of the movement-patterns.
pub struct Pattern {
    pub steps: Vec<Box<dyn for<'a> Fn(&'a Board, &'a Field, &'a FigureColor) -> &'a Field>>,
}

// TODO - Validate that this implementation of the pawn patterns is valid (as specially the lifetimes)

/// Defines a trait that all piece-pattern-struct must implement.
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
        _piece_color: &FigureColor,
    ) -> bool;
}

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

impl PawnPatterns {
    pub fn set_up_patterns() -> PawnPatterns {
        let mut output_patterns: Vec<
            Box<dyn for<'a> Fn(&'a Board, &'a Field, &'a FigureColor) -> &'a Field>,
        > = Vec::new();

        // first pattern
        let one_forward: Box<dyn for<'a> Fn(&'a Board, &'a Field, &'a FigureColor) -> &'a Field> =
            Box::new(
                move |board: &Board, current_field: &Field, piece_color: &FigureColor| -> &Field {
                    // have to determine if the player moves his piece up or down on the board
                    let mut next_field = current_field;

                    match piece_color {
                        FigureColor::Black => {
                            next_field = down(
                                board,
                                current_field,
                                is_the_piece_a_knight(&current_field.content.as_ref().unwrap()),
                                piece_color,
                            );
                        }
                        FigureColor::White => {
                            next_field = up(
                                board,
                                current_field,
                                is_the_piece_a_knight(&current_field.content.as_ref().unwrap()),
                                piece_color,
                            );
                        }
                        _ => {
                            println!("Didn't receive a piece color!");
                        } // won't happen just for the case
                    }

                    next_field
                },
            );

        // second pattern
        let two_forward: Box<dyn for<'a> Fn(&'a Board, &'a Field, &'a FigureColor) -> &'a Field> =
            Box::new(
                move |board: &Board, current_field: &Field, piece_color: &FigureColor| -> &Field {
                    let mut next_field = current_field;

                    match piece_color {
                        FigureColor::Black => {
                            // first step
                            next_field = down(
                                board,
                                current_field,
                                is_the_piece_a_knight(&current_field.content.as_ref().unwrap()),
                                piece_color,
                            );
                            //check if the piece has moved -> if not then return the current field
                            if next_field.position == current_field.position {
                                return current_field;
                            }
                            // second step
                            next_field = down(
                                board,
                                current_field,
                                is_the_piece_a_knight(&current_field.content.as_ref().unwrap()),
                                piece_color,
                            );
                        }
                        FigureColor::White => {
                            //first step
                            next_field = up(
                                board,
                                current_field,
                                is_the_piece_a_knight(&current_field.content.as_ref().unwrap()),
                                piece_color,
                            );
                            //check if the piece has moved -> if not then return the current field
                            if next_field.position == current_field.position {
                                return current_field;
                            }
                            // second step
                            next_field = up(
                                board,
                                current_field,
                                is_the_piece_a_knight(&current_field.content.as_ref().unwrap()),
                                piece_color,
                            );
                        }
                        _ => {
                            println!("Didn't receive a piece color!");
                        } // won't happen just for the case
                    }

                    next_field
                },
            );
        // third pattern
        let throw_left: Box<dyn for<'a> Fn(&'a Board, &'a Field, &'a FigureColor) -> &'a Field> =
            Box::new(
                move |board: &Board, current_field: &Field, piece_color: &FigureColor| -> &Field {
                    let mut next_field = current_field;

                    match piece_color {
                        FigureColor::Black => {
                            next_field = diagonal(board, current_field, false, false, piece_color);
                        }
                        FigureColor::White => {
                            next_field = diagonal(board, current_field, true, false, piece_color);
                        }
                        FigureColor::NotFound => {
                            println!("Didn't find a color of the piece!");
                        }
                    }

                    next_field
                },
            );
        //fourth pattern
        let throw_right: Box<dyn for<'a> Fn(&'a Board, &'a Field, &'a FigureColor) -> &'a Field> =
            Box::new(
                move |board: &Board, current_field: &Field, piece_color: &FigureColor| -> &Field {
                    let mut next_field = Field::as_ref();

                    match piece_color {
                        FigureColor::Black => {
                            next_field = diagonal(board, current_field, false, true, piece_color);
                        }
                        FigureColor::White => {
                            next_field = diagonal(board, current_field, true, true, piece_color);
                        }
                        FigureColor::NotFound => {
                            println!("Didn't receive anything like a piece color!");
                        }
                    }

                    next_field
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

impl MovementPatternExecutor for PawnPatterns {
    type Pattern = Pattern;

    fn execute_patterns(
        &self,
        board: &Board,
        current_field: &Field,
        selected_field: &Field,
        _piece_color: &FigureColor,
    ) -> bool {
        let mut leads_to_selected_field = false;

        for pattern in &self.pawn_patters.steps {
            let last_field = pattern(board, current_field, _piece_color);

            if last_field == selected_field {
                leads_to_selected_field = true;
            }
        }

        leads_to_selected_field
    }
}
//private functions

fn is_the_piece_a_knight(piece: &Figure) -> bool {
    piece.figure_type == FigureType::Knight
}
