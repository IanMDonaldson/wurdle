// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::slint;

slint::include_modules!();


#[cfg_attr(target_arch = "wasm32",
    wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    use slint::Model;

    let main_window = MainWindow::new().unwrap();

    // Fetch the tiles from the model
    let mut tiles: Vec<TileData> = main_window.get_letter_tiles().iter().collect();
    // Duplicate them to ensure that we have pairs
    // for i in 0..4 {
    //     tiles.extend(tiles.clone());
    // };

    // Randomly mix the tiles
    use rand::seq::SliceRandom;
    let mut rng = rand::rng();
    tiles.shuffle(&mut rng);

    // Assign the shuffled Vec to the model property
    let tiles_model = std::rc::Rc::new(slint::VecModel::from(tiles));
    main_window.set_letter_tiles(tiles_model.clone().into());

    main_window.run().unwrap();
}
