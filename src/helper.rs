//use std::{thread, time::Duration};

use std::sync::{Arc, Mutex};

use crate::{
    enums::{FigureColor, FigureType},
    structs::{Board, Field, Figure, Player},
    //utils::ticker::Ticker,
    Game,
};
use eframe::egui::{include_image, Color32, Context, ImageSource, RichText, Ui};

//constants
//const SECOND: u16 = 1;
//when a time dimension (minutes, seconds, etc.) have just a char number -> '09' or '04' not '14'
const ONE_DIGIT_BOUNDRY: u16 = 10;
const DEFAULT_SELECTED: bool = false;

//8 x 8 field with default figure posisions
/// Returns the default field datastructure
///
/// The position property of: (y, x) describes the position of the field within the chess board
/// x = index of the column
/// y = index of the row
#[inline]
pub fn default_field() -> Arc<Mutex<Board>> {
    Arc::new(Mutex::new(Board::new(vec![
        //first row out of whites view
        vec![
            Field {
                content: Some(Figure::new(FigureType::Rook, FigureColor::Black)),
                position: (0, 0),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Knight, FigureColor::Black)),
                position: (0, 1),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Bishop, FigureColor::Black)),
                position: (0, 2),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::King, FigureColor::Black)),
                position: (0, 3),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Queen, FigureColor::Black)),
                position: (0, 4),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Bishop, FigureColor::Black)),
                position: (0, 5),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Knight, FigureColor::Black)),
                position: (0, 6),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Rook, FigureColor::Black)),
                position: (0, 7),
                selected: DEFAULT_SELECTED,
            },
        ],
        //row 2
        vec![
            Field {
                content: Some(Figure::new(FigureType::Pawn, FigureColor::Black)),
                position: (1, 0),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Pawn, FigureColor::Black)),
                position: (1, 1),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Pawn, FigureColor::Black)),
                position: (1, 2),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Pawn, FigureColor::Black)),
                position: (1, 3),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Pawn, FigureColor::Black)),
                position: (1, 4),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Pawn, FigureColor::Black)),
                position: (1, 5),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Pawn, FigureColor::Black)),
                position: (1, 6),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Pawn, FigureColor::Black)),
                position: (1, 7),
                selected: DEFAULT_SELECTED,
            },
        ],
        //row 3
        vec![
            Field {
                content: None,
                position: (2, 0),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (2, 1),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (2, 2),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (2, 3),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (2, 4),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (2, 5),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (2, 6),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (2, 7),
                selected: DEFAULT_SELECTED,
            },
        ],
        //row 4
        vec![
            Field {
                content: None,
                position: (3, 0),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (3, 1),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (3, 2),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (3, 3),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (3, 4),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (3, 5),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (3, 6),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (3, 7),
                selected: DEFAULT_SELECTED,
            },
        ],
        //row 5
        vec![
            Field {
                content: None,
                position: (4, 0),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (4, 1),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (4, 2),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (4, 3),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (4, 4),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (4, 5),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (4, 6),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (4, 7),
                selected: DEFAULT_SELECTED,
            },
        ],
        //row 6
        vec![
            Field {
                content: None,
                position: (5, 0),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (5, 1),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (5, 2),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (5, 3),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (5, 4),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (5, 5),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (5, 6),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: None,
                position: (5, 7),
                selected: DEFAULT_SELECTED,
            },
        ],
        //row 7
        vec![
            Field {
                content: Some(Figure::new(FigureType::Pawn, FigureColor::White)),
                position: (6, 0),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Pawn, FigureColor::White)),
                position: (6, 1),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Pawn, FigureColor::White)),
                position: (6, 2),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Pawn, FigureColor::White)),
                position: (6, 3),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Pawn, FigureColor::White)),
                position: (6, 4),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Pawn, FigureColor::White)),
                position: (6, 5),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Pawn, FigureColor::White)),
                position: (6, 6),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Pawn, FigureColor::White)),
                position: (6, 7),
                selected: DEFAULT_SELECTED,
            },
        ],
        //row 8
        vec![
            Field {
                content: Some(Figure::new(FigureType::Rook, FigureColor::White)),
                position: (7, 0),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Knight, FigureColor::White)),
                position: (7, 1),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Bishop, FigureColor::White)),
                position: (7, 2),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::King, FigureColor::White)),
                position: (7, 3),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Queen, FigureColor::White)),
                position: (7, 4),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Bishop, FigureColor::White)),
                position: (7, 5),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Knight, FigureColor::White)),
                position: (7, 6),
                selected: DEFAULT_SELECTED,
            },
            Field {
                content: Some(Figure::new(FigureType::Rook, FigureColor::White)),
                position: (7, 7),
                selected: DEFAULT_SELECTED,
            },
        ],
    ])))
}

pub fn vertical_seperator(ui: &mut Ui) {
    ui.add_space(15.);

    ui.label(RichText::new("|").size(20.));

    ui.add_space(15.);
}

pub fn horizontal_seperator(ui: &mut Ui) {
    ui.add_space(15.);

    ui.separator();

    ui.add_space(15.);
}

#[inline]
pub fn get_player_figure_color(player_number: u8, current_round: u8) -> FigureColor {
    match player_number {
        1 => {
            if current_round % 2 == 1 {
                FigureColor::White
            } else {
                FigureColor::Black
            }
        }
        2 => {
            if current_round % 2 == 1 {
                FigureColor::Black
            } else {
                FigureColor::White
            }
        }
        _ => FigureColor::NotFound,
    }
}

pub fn render_player_dashboard_info(
    //ticker: &Ticker,
    player: &Player,
    ui: &mut Ui,
    _ctx: &Context,
) {
    ui.label(
        RichText::new(&player.name)
            .color(Color32::WHITE)
            .size(15.)
            .italics(),
    );

    ui.add_space(15.);

    //poisonerror
    //https://stackoverflow.com/questions/72855505/what-is-the-best-way-to-update-app-fields-from-another-thread-in-egui-in-rust
    //https://stackoverflow.com/questions/75278336/egui-interaction-with-background-thread

    ui.add_space(15.);

    ui.label(
        RichText::new(get_player_time_format(player.seconds))
            .color(Color32::WHITE)
            .size(15.),
    );
}

pub fn render_thrown_pieces(game: &Game, ui: &mut Ui) {
    let mut piece_counter: u8 = 1;
    let mut pieces_left = true;
    while pieces_left {
        ui.horizontal(|ui| {
            for piece in &game.thrown_figures {
                render_single_figure(&piece.figure_type, &piece.color, ui);
                if piece_counter % 5 == 0 {
                    break;
                }
                piece_counter += 1;
            }
        });

        //when there are not more then 5 figures -> stop the loop in the first iteration
        if game.thrown_figures.len() < 5 {
            pieces_left = false;
        }

        if piece_counter as usize == game.thrown_figures.len() {
            pieces_left = false;
        }
    }
}

//private functions
#[inline]
fn get_player_time_format(seconds: u16) -> String {
    if seconds == 0 {
        return String::from("No time limitation");
    }

    //get the hours
    let hours = seconds / 3600;
    let minutes = seconds / 60;
    let secs = seconds % 60;

    //apply the time component strings
    //hours
    let hours_string = hours.to_string();

    //minutes
    let mut minutes_string = minutes.to_string();
    if minutes < ONE_DIGIT_BOUNDRY {
        minutes_string = format!("0{}", minutes);
    }

    //seconds
    let mut seconds_string = secs.to_string();
    if secs < ONE_DIGIT_BOUNDRY {
        seconds_string = format!("0{}", secs);
    }

    format!("{}:{}:{}", hours_string, minutes_string, seconds_string)
}

fn render_single_figure(figure_typ: &FigureType, figure_color: &FigureColor, ui: &mut Ui) {
    match figure_color {
        FigureColor::White => match figure_typ {
            FigureType::Pawn => {
                ui.image(include_image!("../static/Pawn_White.png"));
            }
            FigureType::Knight => {
                ui.image(include_image!("../static/Knight_White.png"));
            }
            FigureType::Bishop => {
                ui.image(include_image!("../static/Bishop_White.png"));
            }
            FigureType::Rook => {
                ui.image(include_image!("../static/Rook_White.png"));
            }
            FigureType::Queen => {
                ui.image(include_image!("../static/Queen_White.png"));
            }
            FigureType::King => {
                ui.image(include_image!("../static/King_White.png"));
            }
        },
        FigureColor::Black => match figure_typ {
            FigureType::Pawn => {
                ui.image(include_image!("../static/Pawn_Black.png"));
            }
            FigureType::Knight => {
                ui.image(include_image!("../static/Knight_Black.png"));
            }
            FigureType::Bishop => {
                ui.image(include_image!("../static/Bishop_Black.png"));
            }
            FigureType::Rook => {
                ui.image(include_image!("../static/Rook_Black.png"));
            }
            FigureType::Queen => {
                ui.image(include_image!("../static/Queen_Black.png"));
            }
            FigureType::King => {
                ui.image(include_image!("../static/King_Black.png"));
            }
        },
        _ => (),
    };
}

//Get a field color based on the field coordinates
pub fn get_field_color_on_coordinates(x: u8, y: u8) -> Color32 {
    if y % 2 == 0 {
        if x % 2 == 0 {
            Color32::WHITE
        } else {
            Color32::BLACK
        }
    } else if x % 2 == 0 {
        Color32::BLACK
    } else {
        Color32::WHITE
    }
}

//Return a an image representation based on figure color and type
pub fn get_figure_path<'a>(figure_typ: &FigureType, figure_color: &FigureColor) -> ImageSource<'a> {
    match figure_color {
        FigureColor::White => match figure_typ {
            FigureType::Pawn => {
                include_image!("../static/Pawn_White.png")
            }
            FigureType::Knight => {
                include_image!("../static/Knight_White.png")
            }
            FigureType::Bishop => {
                include_image!("../static/Bishop_White.png")
            }
            FigureType::Rook => {
                include_image!("../static/Rook_White.png")
            }
            FigureType::Queen => {
                include_image!("../static/Queen_White.png")
            }
            FigureType::King => {
                include_image!("../static/King_White.png")
            }
        },
        FigureColor::Black => match figure_typ {
            FigureType::Pawn => {
                include_image!("../static/Pawn_Black.png")
            }
            FigureType::Knight => {
                include_image!("../static/Knight_Black.png")
            }
            FigureType::Bishop => {
                include_image!("../static/Bishop_Black.png")
            }
            FigureType::Rook => {
                include_image!("../static/Rook_Black.png")
            }
            FigureType::Queen => {
                include_image!("../static/Queen_Black.png")
            }
            FigureType::King => {
                include_image!("../static/King_Black.png")
            }
        },
        _ => ImageSource::from(""),
    }
}
