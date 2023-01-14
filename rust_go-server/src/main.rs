use std::sync::{Arc, Mutex, MutexGuard};
use serde_json::{json, Value};

use ws::{listen, Message, Sender};


/// Represents the state of the game from the server's perspective.
pub struct Game {
    board: Vec<u8>,
    player1: Option<Sender>,
    player2: Option<Sender>,
    player1_turn: bool,
    started: bool
}

/// Sends the current board state and turn information to the player represented by the sender.
fn send_game_state(game: &MutexGuard<Game>, player: &Sender, your_turn: bool) {
    let board = game.board.clone();
    let data = json!({ 
        "your_turn": your_turn,
        "board": board 
    });
    player.send(Message::text(data.to_string())).unwrap();
}

fn main() {
    
    let game = Arc::new(Mutex::new(Game {
        board: vec![0; 9],
        player1: None,
        player2: None,
        player1_turn: true,
        started: false
    }));

    listen("127.0.0.1:8888", |out| {
        let game = game.clone();
    
        move |msg: Message| {
            let mut game = game.lock().unwrap();
            
            if game.player1.is_none() {
                game.player1 = Some(out.clone());
                send_game_state(&game, &out, game.player1_turn);
                println!("First player joined the game, id: {}", out.connection_id());
            } 
            else if game.player2.is_none() && out.connection_id() != game.player1.as_ref().unwrap().connection_id() {
                game.player2 = Some(out.clone());
                send_game_state(&game, &out, !game.player1_turn);
                println!("Second player joined the game, id: {}", out.connection_id());
                game.started = true;
            }
            else {
                if !game.started {
                    game.player1.as_ref().unwrap().send("Waiting for the other player to join...").unwrap();
                    return Ok(())
                }
                println!("Received data from player {}: {}", out.connection_id(), msg);
    
                let json_str: String = msg.into_text().unwrap();
                let json_value: Value = serde_json::from_str(&json_str).unwrap();
                let board: Vec<u8> = json_value["board"].as_array().unwrap().into_iter().map(|x| {
                    x.as_u64().unwrap() as u8
                }).collect();

                game.board = board;
                println!("Updated game board state: {:?}", game.board);
                
                if game.player1_turn {
                    println!("Player 1 moved!");
                    game.player2.as_ref().unwrap().send(Message::text("Your turn")).unwrap();
                }
                else {
                    println!("Player 2 moved!");
                    game.player1.as_ref().unwrap().send(Message::text("Your turn")).unwrap();
                }

                game.player1_turn = !game.player1_turn;
    
                send_game_state(&game, &game.player1.as_ref().unwrap(), game.player1_turn);
                send_game_state(&game, &game.player2.as_ref().unwrap(), !game.player1_turn);
    
            }
            drop(game);
            Ok(())
        }
    }).unwrap();
}
