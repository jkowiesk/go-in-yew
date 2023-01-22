#[cfg(test)]
pub mod tests;

use std::{sync::{Arc, Mutex, MutexGuard}, process};
use serde_json::{json, Value};

use ws::{Message, Sender, CloseCode};


/// Represents the state of the game from the server's perspective.
pub struct Game {
    board: Option<Vec<u8>>,
    player1: Option<Sender>,
    player2: Option<Sender>,
    player1_turn: bool,
    game_started: bool,
    board_initialized: bool,
}

/// Sends the current board state and turn information to the player represented by the sender.
fn send_game_state(game: &MutexGuard<Game>, player: &Sender, your_turn: bool) {
    let board = game.board.clone();
    let data = json!({
        "message_type": "board_state",
        "your_turn": your_turn,
        "board": board
    });
    player.send(Message::text(data.to_string())).unwrap();
}

/// Starts the websocket server on the specified port
fn start_server(server_url: &str) {

    let game = Arc::new(Mutex::new(Game {
        board: None,
        player1: None,
        player2: None,
        player1_turn: true,
        game_started: false,
        board_initialized: false
    }));

    ws::listen(server_url, |out| {
        let game = game.clone();

        move |msg: Message| {

            let json_str = msg.as_text().unwrap();
            let message_data: Value = serde_json::from_str(&json_str).unwrap();
            let message_type: &Value = message_data.get("message_type").unwrap_or_else(|| return &Value::Null);
            if message_type.is_null() {
                println!("Received message in invalid data format");
                out.send(json!({
                    "message_type": "error",
                    "message": "missing 'message_type' is message"
                }).to_string()).unwrap();
                return Ok(())
            }

            if message_type == "stop_server" {
                println!("Terminating the server...");
                out.close(CloseCode::Normal).unwrap();
                process::exit(0);
            }

            let mut game = game.lock().unwrap();

            if message_type == "join_game" {
                if game.player1.is_none() {
                    game.player1 = Some(out.clone());
                    println!("First player joined the game, id: {}", out.connection_id());

                    out.send(json!({
                        "message_type": "join_game",
                        "status": "success",
                        "player": "first"
                    }).to_string()).unwrap();
                }
                else if game.player2.is_none() {
                    game.player2 = Some(out.clone());
                    println!("Second player joined the game, id: {}", out.connection_id());

                    out.send(json!({
                        "message_type": "join_game",
                        "status": "success",
                        "player": "second"
                    }).to_string()).unwrap();

                    if game.board_initialized {
                        send_game_state(&game, &game.player1.as_ref().unwrap(), game.player1_turn);
                        send_game_state(&game, &game.player2.as_ref().unwrap(), !game.player1_turn);
                        game.game_started = true;
                    }

                }
                else {
                    out.send(json!({
                        "message_type": "join_game",
                        "status": "error"
                    }).to_string()).unwrap();
                }
            }

            if message_type == "initialize_board" {
                let board_size_data: &Value = message_data.get("board_size").unwrap_or_else(|| return &Value::Null);
                if board_size_data.is_null() {
                    out.send(json!({
                        "message_type": "initialize_board",
                        "status": "error",
                        "message": "couldn't get board_size data"
                    }).to_string()).unwrap();
                }
                let board_size: usize = board_size_data.as_u64().unwrap() as usize;
                game.board = Some(vec![0; board_size]);
                println!("Initialized board with size: {}", board_size);

                game.board_initialized = true;

                out.send(json!({
                    "message_type": "initialize_board",
                    "status": "success"
                }).to_string()).unwrap();

                if game.player2.is_some() {
                    send_game_state(&game, &game.player1.as_ref().unwrap(), game.player1_turn);
                    send_game_state(&game, &game.player2.as_ref().unwrap(), !game.player1_turn);
                    game.game_started = true;
                }
            }

            if message_type == "board_state" {
                if !game.game_started {
                    out.send(json!({
                        "message_type": "board_state",
                        "status": "error",
                        "message": "game has not started",
                    }).to_string()).unwrap();
                }

                let board_data: &Value = message_data.get("board").unwrap_or_else(|| return &Value::Null);
                if board_data.is_null() {
                    out.send(json!({
                        "message_type": "board_state",
                        "status": "error",
                        "message": "couldn't get board data",
                    }).to_string()).unwrap();
                }
                let board: Vec<u8> = board_data.as_array().unwrap().into_iter().map(|x| {
                    x.as_u64().unwrap() as u8
                }).collect();

                game.board = Some(board);
                println!("Updated game board state: {:?}", game.board.as_ref().unwrap());

                game.player1_turn = !game.player1_turn;

                send_game_state(&game, &game.player1.as_ref().unwrap(), game.player1_turn);
                send_game_state(&game, &game.player2.as_ref().unwrap(), !game.player1_turn);
            }
            drop(game);
            Ok(())
        }
    }).unwrap();
}

fn main() {
    start_server("0.0.0.0:8888");
}
