use crate::{strucs::{Field, Figure}, enums::{FigureColor, FigureType}};

//8 x 8 field with default figure posisions
pub fn default_field() -> Vec<Vec<Field>>{
    vec![
        //first row out of whites view
        vec![
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Rock,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (0, 0),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Knight,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (0, 1),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Bishop,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (0, 2),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::King,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (0, 3),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Queen,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (0, 4),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Bishop,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (0, 5),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Knight,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (0, 6),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Rock,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (0, 7),
            }
        ],
        //row 2
        vec![
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (1, 0),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (1, 1),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (1, 2),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (1, 3),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (1, 4),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (1, 5),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (1, 6),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (1, 7),
            }
        ],
        //row 3
        vec![
            Field{
                content: None,
                position: (2, 0),
            },
            Field{
                content: None,
                position: (2, 1),
            },
            Field{
                content: None,
                position: (2, 2),
            },
            Field{
                content: None,
                position: (2, 3),
            },
            Field{
                content: None,
                position: (2, 4),
            },
            Field{
                content: None,
                position: (2, 5),
            },
            Field{
                content: None,
                position: (2, 6),
            },
            Field{
                content: None,
                position: (2, 7),
            },
        ],
        //row 4
        vec![
            Field{
                content: None,
                position: (3, 0),
            },
            Field{
                content: None,
                position: (3, 1),
            },
            Field{
                content: None,
                position: (3, 2),
            },
            Field{
                content: None,
                position: (3, 3),
            },
            Field{
                content: None,
                position: (3, 4),
            },
            Field{
                content: None,
                position: (3, 5),
            },
            Field{
                content: None,
                position: (3, 6),
            },
            Field{
                content: None,
                position: (3, 7),
            },
        ],
        //row 5
        vec![
            Field{
                content: None,
                position: (4, 0),
            },
            Field{
                content: None,
                position: (4, 1),
            },
            Field{
                content: None,
                position: (4, 2),
            },
            Field{
                content: None,
                position: (4, 3),
            },
            Field{
                content: None,
                position: (4, 4),
            },
            Field{
                content: None,
                position: (4, 5),
            },
            Field{
                content: None,
                position: (4, 6),
            },
            Field{
                content: None,
                position: (4, 7),
            },
        ],
        //row 6
        vec![
            Field{
                content: None,
                position: (5, 0),
            },
            Field{
                content: None,
                position: (5, 1),
            },
            Field{
                content: None,
                position: (5, 2),
            },
            Field{
                content: None,
                position: (5, 3),
            },
            Field{
                content: None,
                position: (5, 4),
            },
            Field{
                content: None,
                position: (5, 5),
            },
            Field{
                content: None,
                position: (5, 6),
            },
            Field{
                content: None,
                position: (5, 7),
            },
        ],
        //row 7
        vec![
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (6, 0),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (6, 1),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (6, 2),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (6, 3),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (6, 4),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (6, 5),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (6, 6),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Pawn,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (6, 7),
            }
        ],
        //row 2
        vec![
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Rock,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (7, 0),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Knight,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (7, 1),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Bishop,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (7, 2),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::King,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (7, 3),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Queen,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (7, 4),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Bishop,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (7, 5),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Knight,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (7, 6),
            },
            Field{
                content: Some(Figure{
                    figure_type: FigureType::Rock,
                    thrown: false,
                    color: FigureColor::White,
                }),
                position: (7, 7),
            }
        ]
    ]
}