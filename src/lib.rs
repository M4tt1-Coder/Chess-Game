//mod integretation
pub mod enums;
pub mod helper;
pub mod strucs;

//using statements
use eframe::App;
use std::time::Duration;

use helper::default_field;
use strucs::Field;

use enums::{PlayMode, Winner};
#[derive(PartialEq, Debug)]
pub struct Game {
    //field
    pub field: Vec<Vec<Field>>,
    //winner
    pub winner: Winner,
    //play mode
    pub playmode: PlayMode,
    //time limit
    pub timelimit: Option<Duration>,
    //move
    pub _move: u16,
    //round
    pub round: u8,
}

impl Game {
    //create new game instance
    pub fn new() -> Game {
        Game {
            field: default_field(),
            winner: Winner::NotSet,
            playmode: PlayMode::NotSet,
            timelimit: None,
            _move: 0,
            round: 0,
        }
    }
    //set game to default state when starting a new game
    pub fn reset(&mut self) {
        self.field = default_field();
        self.winner = Winner::NotSet;
        self.playmode = PlayMode::NotSet;
        self.timelimit = None;
        self._move = 0;
        self.round += 1;
    }
}

impl App for Game {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        todo!()
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }
}
