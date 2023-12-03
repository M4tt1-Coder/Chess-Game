#[derive(PartialEq, Debug)]
pub enum Winner{
    Player1,
    Player2,
    Draw,
    NotSet,
}

#[derive(PartialEq, Debug)]
pub enum PlayMode{
    UserVsUserLocal,
    UserVsUserOnline,
    UserVsKI,
    NotSet,
}

#[derive(PartialEq, Debug)]
pub enum FigureType{
    Rock,
    Bishop,
    Knight,
    Queen,
    King,
    Pawn,
}

#[derive(PartialEq, Debug)]
pub enum FigureColor{
    White, 
    Black,
}