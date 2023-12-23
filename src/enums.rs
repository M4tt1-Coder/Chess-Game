#[derive(PartialEq, Debug)]
pub enum Winner {
    Player1,
    Player2,
    Draw,
    NotSet,
}

#[derive(PartialEq, Debug)]
pub enum PlayMode {
    UserVsUserLocal,
    UserVsUserOnline,
    UserVsAI,
    NotSet,
}

#[derive(PartialEq, Debug)]
pub enum FigureType {
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
    Pawn,
}

#[derive(PartialEq, Debug)]
pub enum FigureColor {
    White,
    Black,
}
#[derive(PartialEq, Debug)]
pub enum Environment {
    Browser,
    Local,
    NotSet,
}
