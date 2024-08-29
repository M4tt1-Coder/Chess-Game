use crate::{
    enums::{FigureColor, FigureType},
    helper::get_player_figure_color,
    traits::FigureTrait,
};

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
    //playing figure in the field when there's one
    pub content: Option<Figure>,
    //position on the field -> more used for validation purposes
    pub position: (u8, u8),
    //when a player pressed a button a the field was 'selected'
    pub selected: bool,
}

#[derive(PartialEq, Debug)]
pub struct Figure {
    //how the figure can move -> defines the figure typ
    pub figure_type: FigureType,
    //sets if the figure was thrown out
    pub thrown: bool,
    //color
    pub color: FigureColor,
}

impl Figure {
    pub fn new(figure_type: FigureType, figure_color: FigureColor) -> Figure {
        Figure {
            figure_type,
            thrown: false,
            color: figure_color,
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

    fn clone_myself(&self) -> Field {
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
}
