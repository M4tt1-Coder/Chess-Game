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
use structs::{Field, Figure, Player};

use enums::{Environment, PlayMode, Winner};

use components::{
    header_component::render_header, mode_choice_component::render_playmode_component, chess_board_component::render_chess_board,
};

//constants
const PLAYER_ONE_NUMBER: u8 = 1;
const PLAYER_TWO_NUMBER: u8 = 2;

#[derive(PartialEq, Debug)]
pub struct Game {
    //field
    pub field: Vec<Vec<Field>>,
    //winner
    pub winner: Winner,
    //play mode
    pub playmode: PlayMode,
    //prop player one
    pub player_one: Player,
    //prop player two
    pub player_two: Player,
    //move
    pub _move: u16,
    //round
    pub round: u8,
    //score of players
    pub score: Vec<u8>,
    //those figure who were thrown out -> to display it in the dashboard
    pub thrown_figures: Vec<Figure>,
    //environment -> default is local environment for development
    pub environment: Environment,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    //create new game instance
    pub fn new() -> Game {
        Game {
            field: default_field(),
            winner: Winner::NotSet,
            playmode: PlayMode::NotSet,
            player_one: Player::new(PLAYER_ONE_NUMBER, 1),
            player_two: Player::new(PLAYER_TWO_NUMBER, 1),
            score: vec![0, 0],
            _move: 0,
            round: 0,
            thrown_figures: vec![],
            environment: Environment::Local,
        }
    }
    //set game to default state when starting a new game
    pub fn reset(&mut self) {
        self.field = default_field();
        self.winner = Winner::NotSet;
        self.playmode = PlayMode::NotSet;
        self.player_one = Player::new(PLAYER_ONE_NUMBER, self.round);
        self.player_two = Player::new(PLAYER_TWO_NUMBER, self.round);
        self.score = vec![0, 0];
        self._move = 0;
        self.round += 1;
    }

    pub fn subtract_second(&mut self) {
        if self.player_one.turn {
            self.player_one.seconds -= 1;
        } else if self.player_two.turn {
            self.player_two.seconds -= 1;
        }
    }
}

impl App for Game {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        //TODO - logic when to show a specific component
        render_header(ctx, self);
        //render_playmode_component(ctx, self);
        render_chess_board(ctx, self);
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_millis(100)
    }
}
