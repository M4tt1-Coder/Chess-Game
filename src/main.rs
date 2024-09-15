//using statements: ...
use Chess_Game::utils::ticker::Ticker;
//import egui
use eframe::egui;
//import game-App struct
use Chess_Game::Game;

/// Entrance point for the egui system
///
/// Sets main app window settings
///
/// App-name: ""
///
/// You can check out the documentation here: https://docs.rs/egui/latest/egui/
fn main() -> eframe::Result {
    //create main game instance
    let game = Game::new(/*time_ticker*/);

    let _ = Ticker::new(&game);

    //declare the app name
    let app_name = "Chess";

    //fundamental settings for window and app
    let app_settings = eframe::NativeOptions {
        centered: true,
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([800., 650.])
            .with_max_inner_size([1400., 1138.]),
        //icon_data: egui::Icon... -> add
        default_theme: eframe::Theme::Dark,
        ..eframe::NativeOptions::default()
    };

    //start the game itself
    eframe::run_native(
        app_name,
        app_settings,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            // ! before without the result wrapper
            Ok(Box::new(game))
        }),
    )
}

// TODO - what is tcp / ucp? && look more at actor model tokio?
// TODO - settings component to set name players or time choice
// TODO - Add benchmarks & tests to general new functions and methods
// TODO - Look unwrap was used -> possible errors should always be handled
// TODO - Add a 'started' - boolean property to the game struct just start when the board fully rendered -> when hovering over a button -> set to true
// TODO - Documentation / Comments
