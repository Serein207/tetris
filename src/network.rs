use serde::{Deserialize, Serialize};
use slint::Weak; // Add serde imports

use crate::{
    game::Game,
    pieces::{Color, PhysicalPiece, Piece},
    ui::*,
};
use std::{
    net::UdpSocket,
    sync::{Arc, Mutex},
    thread,
};

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub grid: [[Option<Color>; Game::GRID_WIDTH as usize]; Game::GRID_HEIGHT as usize],
    pub current: PhysicalPiece,
    pub next: Piece,
    pub held: Option<Piece>,
    pub has_held: bool,
    pub score: u32,
    pub game_over: bool,
}

pub fn get_ip_address() -> String {
    let interfaces = get_if_addrs::get_if_addrs().unwrap();
    let mut ip = String::new();
    for interface in interfaces {
        if interface.is_loopback() {
            continue;
        }
        if interface.ip().is_ipv4() {
            ip = interface.ip().to_string();
            break;
        }
    }
    ip
}

pub fn start_server(
    ui_handle: Weak<AppWindow>,
    game: Arc<Mutex<Game>>,
    opponent_game: Arc<Mutex<Game>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = UdpSocket::bind("0.0.0.0:34254")?;
    let mut buffer = [0; 1024];
    let sender = listener.try_clone()?; // Clone the socket for sending
    loop {
        if let Ok((size, src)) = listener.recv_from(&mut buffer) {
            let message = String::from_utf8_lossy(&buffer[..size]);
            println!("Received message: {}", message);
            let ui_handle = ui_handle.clone();
            if message == "start" {
                let _ = slint::invoke_from_event_loop(move || {
                    ui_handle
                        .upgrade()
                        .unwrap()
                        .global::<GameAdapter>()
                        .set_playing(true);
                });
            } else {
                // Deserialize the received message to update opponent_game
                if let Ok(opponent_state) = serde_json::from_str::<GameState>(&message) {
                    let game = opponent_state.into();
                    *opponent_game.lock().unwrap() = game;
                }
            }

            // Serialize the game state and send it to the opponent
            let game_state = GameState::from(game.lock().unwrap());
            if let Ok(serialized) = serde_json::to_string(&game_state) {
                let _ = sender.send_to(serialized.as_bytes(), src);
            }
        }
        thread::sleep(std::time::Duration::from_millis(100));
    }
}

pub fn connect_to_server(
    ip: &str,
    ui_handle: Weak<AppWindow>,
    game: Arc<Mutex<Game>>,
    opponent_game: Arc<Mutex<Game>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = UdpSocket::bind(format!("{ip}:34254"))?;
    let mut buffer = [0; 1024];
    let sender = listener.try_clone()?; // Clone the socket for sending
    let _ = sender.send_to("start".as_bytes(), ip)?;
    if let Some(ui) = ui_handle.upgrade() {
        ui.global::<GameAdapter>().set_playing(true);
    }
    loop {
        if let Ok((size, src)) = listener.recv_from(&mut buffer) {
            let message = String::from_utf8_lossy(&buffer[..size]);
            println!("Received message: {}", message);

            // Deserialize the received message to update opponent_game
            if let Ok(opponent_state) = serde_json::from_str::<GameState>(&message) {
                let game = opponent_state.into();
                *opponent_game.lock().unwrap() = game;
            }

            // Serialize the game state and send it to the opponent
            let game_state = GameState::from(game.lock().unwrap());
            if let Ok(serialized) = serde_json::to_string(&game_state) {
                let _ = sender.send_to(serialized.as_bytes(), src);
            }
        }
        thread::sleep(std::time::Duration::from_millis(100));
    }
}
