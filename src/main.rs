//using statements: ...
use Chess_Game::utils::ticker::Ticker;
//import egui
use eframe::{egui, Error};
//import game-App struct
use Chess_Game::Game;

/// Entrance point for the egui system
///
/// Sets main app window settings
///
/// App-name: ""
///
/// You can check out the documentation here: https://docs.rs/egui/latest/egui/
fn main() -> Result<(), Error> {
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
            Box::new(game)
        }),
    )
}

//TODO - settings component to set name players or time choice
//TODO - Implement a controller interface to control the figure movements, selecting a figure -> implement a move struct to save the made moves during the game + for rules checking 
//TODO - Add benchmarks & tests to general new functions and methods
//TODO - Look unwrap was used -> possible errors should always be handled
//TODO - Add a 'started' - boolean property to the game struct just start when the board fully rendered -> when hovering over a button -> set to true