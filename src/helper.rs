use crate::{
    enums::{FigureColor, FigureType},
    structs::{Field, Figure, Player},
    Game,
};
use eframe::egui::{include_image, Color32, RichText, Ui};

//8 x 8 field with default figure posisions
pub fn default_field() -> Vec<Vec<Field>> {
    vec![
        //first row out of whites view
        vec![
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Rook,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (0, 0),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Knight,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (0, 1),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Bishop,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (0, 2),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::King,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (0, 3),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Queen,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (0, 4),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Bishop,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (0, 5),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Knight,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (0, 6),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Rook,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (0, 7),
            },
        ],
        //row 2
        vec![
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (1, 0),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (1, 1),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (1, 2),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (1, 3),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (1, 4),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (1, 5),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (1, 6),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (1, 7),
            },
        ],
        //row 3
        vec![
            Field {
                content: None,
                position: (2, 0),
            },
            Field {
                content: None,
                position: (2, 1),
            },
            Field {
                content: None,
                position: (2, 2),
            },
            Field {
                content: None,
                position: (2, 3),
            },
            Field {
                content: None,
                position: (2, 4),
            },
            Field {
                content: None,
                position: (2, 5),
            },
            Field {
                content: None,
                position: (2, 6),
            },
            Field {
                content: None,
                position: (2, 7),
            },
        ],
        //row 4
        vec![
            Field {
                content: None,
                position: (3, 0),
            },
            Field {
                content: None,
                position: (3, 1),
            },
            Field {
                content: None,
                position: (3, 2),
            },
            Field {
                content: None,
                position: (3, 3),
            },
            Field {
                content: None,
                position: (3, 4),
            },
            Field {
                content: None,
                position: (3, 5),
            },
            Field {
                content: None,
                position: (3, 6),
            },
            Field {
                content: None,
                position: (3, 7),
            },
        ],
        //row 5
        vec![
            Field {
                content: None,
                position: (4, 0),
            },
            Field {
                content: None,
                position: (4, 1),
            },
            Field {
                content: None,
                position: (4, 2),
            },
            Field {
                content: None,
                position: (4, 3),
            },
            Field {
                content: None,
                position: (4, 4),
            },
            Field {
                content: None,
                position: (4, 5),
            },
            Field {
                content: None,
                position: (4, 6),
            },
            Field {
                content: None,
                position: (4, 7),
            },
        ],
        //row 6
        vec![
            Field {
                content: None,
                position: (5, 0),
            },
            Field {
                content: None,
                position: (5, 1),
            },
            Field {
                content: None,
                position: (5, 2),
            },
            Field {
                content: None,
                position: (5, 3),
            },
            Field {
                content: None,
                position: (5, 4),
            },
            Field {
                content: None,
                position: (5, 5),
            },
            Field {
                content: None,
                position: (5, 6),
            },
            Field {
                content: None,
                position: (5, 7),
            },
        ],
        //row 7
        vec![
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (6, 0),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (6, 1),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (6, 2),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (6, 3),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (6, 4),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (6, 5),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (6, 6),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (6, 7),
            },
        ],
        //row 8
        vec![
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Rook,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (7, 0),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Knight,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (7, 1),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Bishop,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (7, 2),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::King,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (7, 3),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Queen,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (7, 4),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Bishop,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (7, 5),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Knight,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (7, 6),
            },
            Field {
                content: Some(Figure {
                    figure_type: FigureType::Rook,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (7, 7),
            },
        ],
    ]
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
            if current_round % 2 == 0 {
                FigureColor::Black
            } else {
                FigureColor::White
            }
        }
        _ => FigureColor::NotFound,
    }
}

pub fn render_player_dashboard_info(player: &Player, ui: &mut Ui) {
    ui.label(
        RichText::new(&player.name)
            .color(Color32::WHITE)
            .size(15.)
            .italics(),
    );

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
            ui.label("test");
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
fn get_player_time_format(seconds: u16) -> String {
    if seconds == 0 {
        return String::from("No time limitation");
    }

    let minutes = seconds / 60;
    let hours = seconds / 3600;
    let secs = seconds % 3600;

    if hours != 0 {
        format!("{}:{}", minutes, secs)
    } else {
        format!("{}:{}:{}", hours, minutes, secs)
    }
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
