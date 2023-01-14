use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use serde_json::{json, Value};


use ws::{listen, Message, Result, Sender, Handler};


pub struct Game {
    board: Vec<u8>,
    player1: Option<Sender>,
    player2: Option<Sender>,
    player1_turn: bool,
}


fn send_game_state(game: &MutexGuard<Game>, player: &Sender) {
    let board = game.board.clone();
    let player_turn = game.player1_turn;
    let data = json!({ 
        "first_player_turn": player_turn,
        "board": board 
    });
    player.send(Message::text(data.to_string())).unwrap();
    // player.send(Message::binary(board)).unwrap();
    // player.send(Message::text(format!("{}", player_turn))).unwrap();
}

fn main() {
    
    let game = Arc::new(Mutex::new(Game {
        board: vec![0; 9],
        player1: None,
        player2: None,
        player1_turn: true,
    }));

    listen("127.0.0.1:8888", |out| {
        let game = game.clone();
    
        move |msg: Message| {
            let mut game = game.lock().unwrap();
            
            if game.player1.is_none() {
                game.player1 = Some(out.clone());
                send_game_state(&game, &out);
            } 
            else if game.player2.is_none() {
                game.player2 = Some(out.clone());
                send_game_state(&game, &out);
            }
            else {
                println!("Received data from client: {}", msg);
    
                let json_str: String = msg.into_text().unwrap();
                let json_value: Value = serde_json::from_str(&json_str).unwrap();
                let board: Vec<u8> = json_value["board"].as_array().unwrap().into_iter().map(|x| {
                    x.as_u64().unwrap() as u8
                }).collect();
                
                println!("Previous board state: {:?}", game.board);
                println!("Received board state: {:?}", board);

                game.board = board;
                println!("Updated game board state: {:?}", game.board);
                
                game.player1_turn = !game.player1_turn;
    
                send_game_state(&game, &game.player1.as_ref().unwrap());
                send_game_state(&game, &game.player2.as_ref().unwrap());
    
                if game.player1_turn {
                    game.player1.as_ref().unwrap().send(Message::text("Your turn")).unwrap();
                    println!("Player 1 moved!");
                }
                else {
                    game.player2.as_ref().unwrap().send(Message::text("Your turn")).unwrap();
                    println!("Player 2 moved!");
                }
            }
            drop(game);
            Ok(())
        }
    }).unwrap();
}
