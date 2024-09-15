use crate::{
    enums::{FigureColor, FigureType},
    helper::get_player_figure_color,
    traits::FigureTrait,
};
use uuid::Uuid;

pub struct Board {
    pub content: Vec<Vec<Field>>,
}

impl Board {
    pub fn new(content: Vec<Vec<Field>>) -> Board {
        Board { content }
    }
}

pub fn replicate(board: &Board) -> Board {
    let mut output_board = empty_board();
    for row in &board.content {
        let mut rep_row: Vec<Field> = vec![];
        for field in row {
            let col_field: Field = field.clone_myself();
            rep_row.push(col_field);
        }
        output_board.push(rep_row);
    }
    Board {
        content: output_board,
    }
}

fn empty_board() -> Vec<Vec<Field>> {
    vec![]
}

#[derive(PartialEq, Debug)]
pub struct Player {
    pub figure_color: FigureColor,
    //a specific name for the player
    pub name: String,
    //seconds used for the decreament for every second a player takes to make a move
    pub seconds: u16,
    //if its the players turn or not
    pub turn: bool,
}

impl Player {
    pub fn new(player_number: u8, current_round: u8) -> Player {
        Player {
            figure_color: get_player_figure_color(player_number, current_round),
            name: format!("Player {}", player_number), //default implementation
            seconds: 150,
            turn: true,
        }
    }

    pub fn update(&mut self, secs: u16, name: String) {
        self.name = name;
        self.seconds = secs;
    }
}

#[derive(PartialEq, Debug)]
pub struct Field {
    /// playing figure in the field when there's one
    pub content: Option<Figure>,
    /// position on the field -> more used for validation purposes
    ///
    /// index 0 => y && index 1 => x (which represents which ordinate)
    ///
    /// O   0 1 2 3 ... (x)
    ///
    /// 0
    ///
    /// 1
    ///
    /// 2
    ///
    /// 3
    ///
    /// ...
    ///
    /// (y)
    pub position: (u8, u8),
    /// when a player pressed a button a the field was 'selected'
    pub selected: bool,
}

#[derive(PartialEq, Debug)]
pub struct Figure {
    /// how the figure can move -> defines the figure typ
    pub figure_type: FigureType,
    /// sets if the figure was thrown out
    pub thrown: bool,
    /// color of the figure
    pub color: FigureColor,
    /// Randomly generated identifier for rule checking and logging
    pub id: Uuid,
}

impl Figure {
    pub fn new(figure_type: FigureType, figure_color: FigureColor) -> Figure {
        Figure {
            figure_type,
            thrown: false,
            color: figure_color,
            id: Uuid::new_v4(),
        }
    }
}

impl FigureTrait for Figure {
    fn get_thrown_out(&mut self) {
        self.thrown = true;
    }

    fn change_to_rock(&mut self) {
        self.figure_type = FigureType::Rook;
    }

    fn change_to_knight(&mut self) {
        self.figure_type = FigureType::Knight;
    }

    fn change_to_bishop(&mut self) {
        self.figure_type = FigureType::Bishop;
    }

    fn change_to_queen(&mut self) {
        self.figure_type = FigureType::Queen;
    }
}

impl Field {
    pub fn as_ref<'a>() -> &'a Field {
        &Field {
            content: None,
            position: (0, 0),
            selected: false,
        }
    }

    pub fn clone_myself(&self) -> Field {
        let my_content: Option<Figure> = match &self.content {
            Some(piece) => {
                let figure_type = match piece.figure_type {
                    FigureType::Queen => FigureType::Queen,
                    FigureType::Bishop => FigureType::Bishop,
                    FigureType::King => FigureType::King,
                    FigureType::Knight => FigureType::Knight,
                    FigureType::Pawn => FigureType::Pawn,
                    FigureType::Rook => FigureType::Rook,
                };
                let figure_color = match piece.color {
                    FigureColor::White => FigureColor::White,
                    FigureColor::Black => FigureColor::Black,
                    _ => FigureColor::NotFound,
                };
                Some(Figure::new(figure_type, figure_color))
            }
            None => None,
        };
        Field {
            content: my_content,
            position: self.position,
            selected: self.selected,
        }
    }

    pub fn new() -> Field {
        Field {
            content: None,
            position: (0, 0),
            selected: false,
        }
    }
}

// Struct representing the moves made in a chess game.
//
// Used to store the moves in a list.
//
// Provide overview a about the game chronology.

/// Describes a whole move that was made in a game.
///
/// There are multiple moves stored in a list for mainly rule checks.
///
/// It holds information about what piece moved, from to where and which number in the list
/// the move is.
pub struct Move {
    /// Unique identifier of a figure that moved in this move.
    pub piece_id: Uuid,
    /// The field coordinates where the figure started to make the move.
    pub from: (u8, u8),
    /// The field coordinates where the figure end his move.
    pub to: (u8, u8),
    /// The exact number of the move. For example: 5
    pub number: u16,
}

// Implementation for Move struct
impl Move {
    /// Takes in all data to create a move for every move a user makes.
    pub fn new(number_of_moves: u16, piece_id: Uuid, from: (u8, u8), to: (u8, u8)) -> Move {
        Move {
            number: number_of_moves + 1,
            from,
            to,
            piece_id,
        }
    }
}

// TODO - Maybe add a feature of importing and exporting a game status with the move history from / to a file

/// Represents the whole amount of moves of one game.
///
/// Starts with zero moves and retrieves one more move for each piece a user moves.
///
/// Ends when the game is over.
pub struct MoveHistory {
    /// All moves made in one game
    moves: Vec<Move>,
    /// Number of the moves made in a game till now.
    number_of_moves: u16,
}

// Implementation of the MoveHistory struct
impl<'a> MoveHistory {
    /// Create a default move-history instance.
    ///
    /// Start number of moves is 0.
    ///
    /// An empty list of moves is applied.
    pub fn new() -> MoveHistory {
        MoveHistory {
            number_of_moves: 0,
            moves: vec![],
        }
    }

    // add a move to the list
    /// Add a move to the list of moves.
    ///
    /// Is called on the MoveHistory instance after every move of a user.
    pub fn add_move(&mut self, move_to_be_added: Move) {
        self.moves.push(move_to_be_added);
        self.number_of_moves += 1;
    }

    // get a move by its move number
    /// Takes in a specific number of a move.
    ///
    /// Returns the move of that number
    pub fn get_move_by_number(&self, move_number: u16) -> &Move {
        // get the index from the move number by subtracting 1
        let index_of_move: usize = move_number as usize - 1;
        &self.moves[index_of_move]
    }

    pub fn get_current_number_of_moves(&self) -> u16 {
        return self.moves.len() as u16;
    }
}
