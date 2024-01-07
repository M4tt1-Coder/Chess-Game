//TODO - when starting the game open a thread for each players tickers -> will send a boolean a second of the player clock is over; after subracting set the value to false again

//using statements

use eframe::egui::{Context, CentralPanel};

use crate::Game;

pub fn render_chess_board(ctx: &Context, game: &mut Game){
    CentralPanel::default().show(ctx, |ui| {

    });
}