use ws::{Message, Result, CloseCode};
use serde_json::{json};
use std::{thread, time};
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

#[test]
fn test_two_players_join_game() -> Result<()> {

    thread::spawn(move || {
        start_server("127.0.0.1:8002");
    });
    
    ws::connect("ws://127.0.0.1:8002", |out| {
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
            Ok(())
        }
    }).unwrap();

    thread::sleep(time::Duration::new(1, 0));

    ws::connect("ws://127.0.0.1:8002", |out| {
        let data = json!({
            "message_type": "join_game",
        });
        out.send(Message::Text(data.to_string())).unwrap();

        move |msg| {
            let expected_data = json!({
                "message_type": "join_game",
                "status": "success",
                "player": "second"
            });
            assert_eq!(msg, Message::Text(expected_data.to_string()));
            out.close(CloseCode::Normal).unwrap();
            Ok(())
        }
    }).unwrap();

    Ok(())
}

#[test]
fn test_third_player_tries_to_join_game() -> Result<()> {

    thread::spawn(move || {
        start_server("127.0.0.1:8003");
    });
    
    ws::connect("ws://127.0.0.1:8003", |out| {
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
            Ok(())
        }
    }).unwrap();

    thread::sleep(time::Duration::new(1, 0));

    ws::connect("ws://127.0.0.1:8003", |out| {
        let data = json!({
            "message_type": "join_game",
        });
        out.send(Message::Text(data.to_string())).unwrap();

        move |msg| {
            let expected_data = json!({
                "message_type": "join_game",
                "status": "success",
                "player": "second"
            });
            assert_eq!(msg, Message::Text(expected_data.to_string()));
            out.close(CloseCode::Normal).unwrap();
            Ok(())
        }
    }).unwrap();

    thread::sleep(time::Duration::new(1, 0));

    ws::connect("ws://127.0.0.1:8003", |out| {
        let data = json!({
            "message_type": "join_game",
        });
        out.send(Message::Text(data.to_string())).unwrap();

        move |msg| {
            let expected_data = json!({
                "message_type": "join_game",
                "status": "error"
            });
            assert_eq!(msg, Message::Text(expected_data.to_string()));
            out.close(CloseCode::Normal).unwrap();
            Ok(())
        }
    }).unwrap();

    Ok(())
}