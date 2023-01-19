use ws::{Message, Result, CloseCode};
use serde_json::{json};
use std::thread;
use super::*;


#[test]
fn test_player_one_join_game() -> Result<()> {

    thread::spawn(move || {
        start_server("127.0.0.1:8001");
    });
    
    ws::connect("ws://127.0.0.1:8001", |out| {
        let data = json!({
            "message_type": "join_game",
        });
        out.send(Message::Text(data.to_string())).unwrap();

        move |msg| {
            let expected_data = json!({
                "message_type": "join_game",
                "status": "success",
                "player": "first"
            });
            assert_eq!(msg, Message::Text(expected_data.to_string()));
            out.close(CloseCode::Normal).unwrap();
            out.send(json!({
                "message_type": "stop_server"
            }).to_string()).unwrap();
            Ok(())
        }
    }).unwrap();

    Ok(())
}