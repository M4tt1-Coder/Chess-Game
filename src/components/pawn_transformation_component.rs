use eframe::egui::{Button, Color32, Image, RichText, Ui};

use crate::{enums::FigureType, helper::get_figure_path, structs::Figure, Game};

/// Displays a popover that lets the player decide in which piece a pawn should
/// transform.
///
/// Changes the type of the pawn to that selected of the user.
pub fn render_pawn_conversion_popup(
    game: &Game,
    pawn: &Figure,
    field_position: (usize, usize),
    ui: &mut Ui,
) {
    // when the Option is a Some value, there is a pawn on the board end
    // coordinates of the pawn on the board end is provided as second element of the tuple
    // knight / bishop / rook / queen choice opportunities -> buttons
    // show a window to let the user choose his / her piece
    // add the heading of the window
    ui.label(
        RichText::new("Please choose a piece in which the pawn should be transformed!")
            .size(20.)
            .strong()
            .italics()
            .color(Color32::WHITE),
    );
    // add the space for the buttons
    ui.horizontal_centered(|ui| {
        // bishop
        if ui
            .add_sized(
                [100., 100.],
                Button::image(Image::new(get_figure_path(
                    &FigureType::Bishop,
                    &pawn.color,
                ))),
            )
            .clicked()
        {
            // prepare the new field content
            // change the type of the pawn to bishop
            game.assign_new_field_content(
                field_position.1,
                field_position.0,
                Some(Figure::new(FigureType::Bishop, pawn.color, Some(pawn.id))),
            );
        }
        ui.add_space(15.);
        // knight
        if ui
            .add_sized(
                [100., 100.],
                Button::image(Image::new(get_figure_path(
                    &FigureType::Knight,
                    &pawn.color,
                ))),
            )
            .clicked()
        {
            // change pawn to the knight
            game.assign_new_field_content(
                field_position.1,
                field_position.0,
                Some(Figure::new(FigureType::Knight, pawn.color, Some(pawn.id))),
            );
        }
        ui.add_space(15.);
        // queen
        if ui
            .add_sized(
                [100., 100.],
                Button::image(Image::new(get_figure_path(&FigureType::Queen, &pawn.color))),
            )
            .clicked()
        {
            // change pawn to the queen
            game.assign_new_field_content(
                field_position.1,
                field_position.0,
                Some(Figure::new(FigureType::Queen, pawn.color, Some(pawn.id))),
            );
        }
        ui.add_space(15.);
        // rook
        if ui
            .add_sized(
                [100., 100.],
                Button::image(Image::new(get_figure_path(&FigureType::Rook, &pawn.color))),
            )
            .clicked()
        {
            // changet the pawn to the rook
            game.assign_new_field_content(
                field_position.1,
                field_position.0,
                Some(Figure::new(FigureType::Rook, pawn.color, Some(pawn.id))),
            );
        }
    });
}

// popup try code
// // popover -> can't be closed by the player
// let popup_id = ui.make_persistent_id("Pawn_Transformation_Popup");
// ui.memory_mut(|mem| mem.toggle_popup(popup_id));
// // define where the popup should be displayed -> above the container
// let above = AboveOrBelow::Above;
// // how to close the popup
// let close_behavior = PopupCloseBehavior::IgnoreClicks;
// popup_above_or_below_widget(
//     ui,
//     popup_id,
//     widget_response,
//     above,
//     close_behavior,
//     add_contents,
// )
