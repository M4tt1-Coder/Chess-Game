//mod integretation
pub mod components;
pub mod enums;
pub mod helper;
pub mod structs;
pub mod traits;

//using statements
use eframe::App;
use std::time::Duration;
use helper::default_field;
use structs::{Field, Figure};

use enums::{PlayMode, Winner, Environment};

use components::{header_component::render_header, mode_choice_component::render_playmode_component};

#[derive(PartialEq, Debug)]
pub struct Game {
    //field
    pub field: Vec<Vec<Field>>,
    //winner
    pub winner: Winner,
    //play mode
    pub playmode: PlayMode,
    //time limit of player one
    pub player_1_time: Option<Duration>,
    //time limit of player two
    pub player_2_time: Option<Duration>,
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

impl Game {
    //create new game instance
    pub fn new() -> Game {
        Game {
            field: default_field(),
            winner: Winner::NotSet,
            playmode: PlayMode::NotSet,
            player_1_time: None,
            player_2_time: None,
            score: vec![0,0], 
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
        self.player_1_time = None;
        self.player_2_time = None;
        self.score = vec![0,0];
        self._move = 0;
        self.round += 1;
    }
}

impl App for Game {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        //TODO - logic when to show a specific component
        render_header(ctx, self);
        render_playmode_component(ctx, self);
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_millis(200)
    }
}
