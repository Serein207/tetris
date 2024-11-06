use game::Game;
use slint::{SharedString, Timer};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod game;
mod pieces;
mod controller {
    pub mod game_controller;
}
use controller::*;

mod network;

pub mod ui {
    slint::include_modules!();
}
use ui::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let ui = AppWindow::new().unwrap();
    let game = Arc::new(Mutex::new(Game::new()));
    let opponent_game = Arc::new(Mutex::new(Game::new()));

    let _game_controller = game_controller::setup(&ui, game.clone(), opponent_game.clone());

    let game_handle = game.clone();
    let opponent_game_handle = opponent_game.clone();
    let ui_handle = ui.as_weak();
    let game_update_timer = Timer::default();
    let duration = Duration::from_millis(500);
    game_update_timer.start(slint::TimerMode::Repeated, duration, {
        move || {
            if ui_handle.unwrap().global::<GameAdapter>().get_playing() {
                game_handle.lock().unwrap().update();
                opponent_game_handle.lock().unwrap().update();
            }
        }
    });

    let ui_handle = ui.as_weak();
    let game_handle = game.clone();
    let opponent_game_handle = opponent_game.clone();
    ui.global::<GameAdapter>().on_play_pressed(move || {
        let ui = ui_handle.unwrap();
        let game_adapter = ui.global::<GameAdapter>();
        if game_adapter.get_game_over() {
            *game_handle.lock().unwrap() = Game::new();
            *opponent_game_handle.lock().unwrap() = Game::new();
        }
        game_adapter.set_playing(true);
    });

    let game_handle = game.clone();
    ui.on_key_pressed(move |key_text: SharedString| {
        let keycode = key_text.as_str().chars().nth(0).unwrap();
        let mut game = game_handle.lock().unwrap();
        game.handle_input(keycode);
    });

    let ui_handle = ui.as_weak();
    let game_handle = game.clone();
    let opponent_game_handle = opponent_game.clone();
    ui.global::<GameAdapter>().on_create_room(move || {
        let ip = network::get_ip_address();
        let ui_handle = ui_handle.clone();
        let ui = ui_handle.unwrap();
        let game_handle = game_handle.clone();
        let opponent_game_handle = opponent_game_handle.clone();
        ui.global::<GameAdapter>().set_ip_address(ip.into());
        thread::spawn(move || {
            if let Err(e) = network::start_server(ui_handle, game_handle, opponent_game_handle) {
                eprintln!("Error: {}", e);
            }
        });
    });

    let ui_handle = ui.as_weak();
    let game_handle = game.clone();
    let opponent_game_handle = opponent_game.clone();
    ui.global::<GameAdapter>().on_search_opponent(move |addr| {
        let ui_handle = ui_handle.clone();

        let game_handle = game_handle.clone();
        let opponent_game_handle = opponent_game_handle.clone();
        thread::spawn(move || {
            if let Err(e) =
                network::connect_to_server(&addr, ui_handle, game_handle, opponent_game_handle)
            {
                eprintln!("Error: {}", e);
            }
        });
    });

    ui.run().unwrap();
}
