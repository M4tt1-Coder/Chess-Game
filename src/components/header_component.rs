//using statements
use crate::helper::vertical_seperator;
use crate::{Environment, Game, PlayMode};
use eframe::egui::{Align, Color32, Context, Layout, RichText, TopBottomPanel};

pub fn render_header(ctx: &Context, game: &mut Game) {
    TopBottomPanel::top("status-bar").show(ctx, |ui| {
        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
            //chosen playing mode
            ui.label(
                RichText::new(get_string_for_playing_mode(game))
                    .color(Color32::WHITE)
                    .underline()
                    .size(15.),
            );

            vertical_seperator(ui);

            //scoreboard
            ui.label(
                RichText::new(format!("Score: {} : {}", game.score[0], game.score[1]))
                    .color(Color32::WHITE)
                    .size(15.),
            );

            vertical_seperator(ui);

            //round
            ui.label(
                RichText::new(format!("Round: {}", game.round))
                    .color(Color32::WHITE)
                    .size(15.),
            );

            vertical_seperator(ui);

            //playing environment (local, browser)
            ui.label(
                RichText::new(get_string_for_game_environment(game))
                    .color(Color32::WHITE)
                    .size(15.),
            );
        });
    });
}

#[inline]
fn get_string_for_playing_mode(game: &Game) -> String {
    if game.playmode == PlayMode::NotSet {
        return String::from("No chosen playing mode");
    }

    if game.playmode == PlayMode::UserVsUserLocal || game.playmode == PlayMode::UserVsUserOnline {
        return String::from("Player vs Player");
    }

    if game.playmode == PlayMode::UserVsAI {
        return String::from("Player vs AI");
    }

    String::new()
}

#[inline]
fn get_string_for_game_environment(game: &Game) -> String {
    match game.environment {
        Environment::Browser => {
            String::from("Browser")
        }
        Environment::Local => {
            String::from("Local")
        }
        Environment::NotSet => {
            String::from("Not chosen ... ")
        }
    }
}
