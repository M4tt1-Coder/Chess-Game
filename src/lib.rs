//mod integretation
pub mod components;
pub mod enums;
pub mod helper;
pub mod structs;
pub mod tests;
pub mod traits;
pub mod utils;

//using statements
use eframe::App;
use helper::default_field;
use structs::{replicate, Board, Field, Figure, MoveHistory, Player};

use enums::{Environment, FigureColor, FigureType, PlayMode, Winner};

use components::{
    chess_board_component::render_chess_board,
    header_component::render_header,
    //mode_choice_component::render_playmode_component,
};

//use utils::ticker::Ticker;

use std::sync::{Arc, Mutex};

//constants
const PLAYER_ONE_NUMBER: u8 = 1;
const PLAYER_TWO_NUMBER: u8 = 2;
const SAVE_INTERVAL: u64 = 10;
pub const FIELD_SIZE: f32 = 55.;

//#[derive(Debug, PartialEq)]
/// Global game instance, holding all important information.
pub struct Game {
    //pub ticker: Ticker,
    /// Game board with 8 x 8 fields (64)
    pub field: Arc<Mutex<Board>>,
    /// The player, who won the game.
    pub winner: Winner,
    /// Determines which playmode has been chosen by the user.
    pub playmode: PlayMode,
    /// Representing player one
    pub player_one: Arc<Mutex<Player>>,
    /// Representing player two
    pub player_two: Arc<Mutex<Player>>,
    /// Information about what round we are in
    pub round: u8,
    /// score of players
    pub score: Vec<u8>,
    /// those figure who were thrown out -> to display it in the dashboard
    pub thrown_figures: Vec<Figure>,
    /// environment -> default is local environment for development (?)
    pub environment: Environment,
    pub moves_history: MoveHistory,
}

impl Game {
    //create new game instance
    pub fn new(/*ticker: Ticker*/) -> Game {
        Game {
            //ticker,
            field: default_field(),
            winner: Winner::NotSet,
            playmode: PlayMode::NotSet,
            player_one: Arc::new(Mutex::new(Player::new(PLAYER_ONE_NUMBER, 1))),
            player_two: Arc::new(Mutex::new(Player::new(PLAYER_TWO_NUMBER, 1))),
            score: vec![0, 0],
            round: 0,
            thrown_figures: vec![],
            environment: Environment::Local,
            moves_history: MoveHistory::new(),
        }
    }
    //set game to default state when starting a new game
    pub fn reset(&mut self) {
        self.field = default_field();
        self.winner = Winner::NotSet;
        self.playmode = PlayMode::NotSet;
        self.player_one = Arc::new(Mutex::new(Player::new(PLAYER_ONE_NUMBER, self.round)));
        self.player_two = Arc::new(Mutex::new(Player::new(PLAYER_TWO_NUMBER, self.round)));
        self.score = vec![0, 0];
        self.round += 1;
    }

    pub fn field_not_selected_anymore(&self) {
        let board_clone = self.field.try_lock().unwrap();
        let board = replicate(&board_clone);
        drop(board_clone);
        let mut x = 0;
        let mut y = 0;
        for row in &board.content {
            for field in row {
                if field.selected {
                    x = field.position.0 as usize;
                    y = field.position.1 as usize;
                }
            }
        }
        self.dont_select_field_anymore(x, y);
    }

    fn dont_select_field_anymore(&self, x: usize, y: usize) {
        let mut board_clone = self.field.try_lock().unwrap();
        board_clone.content[x][y].selected = false;
        drop(board_clone);
        //self.field.try_lock().unwrap().content[x][y].selected = false;
    }

    fn select_field(&self, x: usize, y: usize) {
        let mut board_clone = self.field.try_lock().unwrap();
        board_clone.content[x][y].selected = true;
        drop(board_clone);
        // self.field.try_lock().unwrap().content[x][y].selected = true;
    }

    fn delete_field_content(&self, x: usize, y: usize) {
        let mut board_clone = self.field.try_lock().unwrap();
        board_clone.content[x][y].content = None;
        drop(board_clone);
        //self.field.try_lock().unwrap().content[x][y].content = None;
    }

    fn assign_new_field_content(&self, x: usize, y: usize, content: Option<Figure>) {
        let mut board_clone = self.field.try_lock().unwrap();
        board_clone.content[x][y].content = content;
        drop(board_clone);
        //self.field.try_lock().unwrap().content[x][y].content = content;
    }

    fn move_figure_to_new_field(&self, previous_field: &Field, new_field: &Field) {
        let board_clone = self.field.try_lock().unwrap();
        let board = replicate(&board_clone);
        drop(board_clone);
        //determine which field the pre_field resembles to
        //-> take conent and put it in the new field
        let mut figure_content: Option<Figure> = None;

        for row in &board.content {
            for field in row {
                if field.position == previous_field.position {
                    let piece = &field.content.as_ref().unwrap();
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
                    self.delete_field_content(field.position.0 as usize, field.position.1 as usize);

                    figure_content = Some(Figure::new(figure_type, figure_color));
                }
            }
        }

        self.assign_new_field_content(
            new_field.position.0 as usize,
            new_field.position.1 as usize,
            figure_content,
        );
    }

    fn next_players_turn(&self) {
        let mut player_one = self.player_one.try_lock().unwrap();
        let mut player_two = self.player_two.try_lock().unwrap();

        if player_one.turn {
            player_one.turn = false;
            player_two.turn = true;
        } else {
            player_one.turn = true;
            player_two.turn = false;
        }

        drop(player_one);
        drop(player_two);
    }
}

impl App for Game {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        //TODO - logic when to show a specific component
        render_header(ctx, self);
        //render_playmode_component(ctx, self);
        render_chess_board(ctx, self);
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_millis(SAVE_INTERVAL)
    }
}
