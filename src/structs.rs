use crate::{enums::{FigureType, FigureColor}, traits::FigureTrait};

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

impl Figure {
    pub fn new(figure_type: FigureType, figure_color: FigureColor) -> Figure {
        Figure{
            figure_type,
            thrown: false,
            color: figure_color
        }
    }
}

impl FigureTrait for Figure{
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