//using statements
use eframe::egui::{self, Align, CentralPanel, Color32, Context, ImageButton, Layout, RichText};

use crate::{enums::{PlayMode, Environment}, helper::horizontal_seperator, Game};

pub fn render_playmode_component(ctx: &Context, game: &mut Game) {
    //central penal
    CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.label(
                RichText::new("Choose your playing mode ...")
                    .color(Color32::WHITE)
                    .size(20.),
            );
        });

        horizontal_seperator(ui);

        //option for local user against user playing mode

        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            if ui
                .add_sized(
                    [40., 40.],
                    ImageButton::new(egui::include_image!("../../static/next-button.png")),
                )
                .clicked()
            {
                game.playmode = PlayMode::UserVsUserLocal;
            }

            ui.add_space(15.);

            ui.label(
                RichText::new("[local] User VS User")
                    .color(Color32::WHITE)
                    .size(15.),
            );
        });

        horizontal_seperator(ui);

        //option local match with ai

        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            if ui
                .add_sized(
                    [40., 40.],
                    ImageButton::new(egui::include_image!("../../static/next-button.png")),
                )
                .clicked()
            {
                game.playmode = PlayMode::UserVsAI;
            }

            ui.add_space(15.);

            ui.label(
                RichText::new("[local] User VS AI")
                    .color(Color32::WHITE)
                    .size(15.),
            );
        });

        horizontal_seperator(ui);

        //option for online user against another user

        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            if ui
                .add_sized(
                    [40., 40.],
                    ImageButton::new(egui::include_image!("../../static/next-button.png")),
                )
                .clicked()
            {
                game.playmode = PlayMode::UserVsUserOnline;
                game.environment = Environment::Browser;
            }

            ui.add_space(15.);

            ui.label(
                RichText::new("[online] User Vs User")
                    .color(Color32::WHITE)
                    .size(15.),
            );
        });

        horizontal_seperator(ui);
    });
}
