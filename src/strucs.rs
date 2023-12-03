use crate::enums::{FigureType, FigureColor};

#[derive(PartialEq, Debug)]
pub struct Field{
    //playing figure in the field when there's one 
    pub content: Option<Figure>,
    //position on the field -> more used for validation purposes
    pub position: (u8, u8),
}

#[derive(PartialEq, Debug)]
pub struct Figure{
    //how the figure can move -> defines the figure typ
    pub figure_type: FigureType,
    //sets if the figure was thrown out
    pub thrown: bool,
    //color
    pub color: FigureColor,
}

