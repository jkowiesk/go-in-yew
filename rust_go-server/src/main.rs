use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::sleep;
use std::time::Duration;
use serde_json::{json};


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
    player.send(Message::binary(board)).unwrap();
    player.send(Message::text(format!("{}", player_turn))).unwrap();
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
                let player_msg: Vec<u8> = msg.into_data();

                // game.board = player_move;
                
                game.player1_turn = !game.player1_turn;
    
                send_game_state(&game, &game.player1.as_ref().unwrap());
                send_game_state(&game, &game.player2.as_ref().unwrap());
    
                if game.player1_turn {
                    game.player1.as_ref().unwrap().send("Your turn").unwrap();
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
