//using statements: ...
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
fn main() -> Result<(), Error>{
    //create main game instance 
    let game = Game::new();

    //declare the app name
    let app_name = "Chess";

    //fundamental settings for window and app
    let app_settings = eframe::NativeOptions{
        centered: true,
        viewport: egui::ViewportBuilder::default().with_min_inner_size([800., 650.]).with_max_inner_size([1400., 1138.]),
        //icon_data: egui::Icon... -> add  
        default_theme: eframe::Theme::Dark,
        ..eframe::NativeOptions::default()
    };

    //start the game itself
    eframe::run_native(app_name, app_settings, Box::new(|cc|{
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Box::new(game)
    }))
}