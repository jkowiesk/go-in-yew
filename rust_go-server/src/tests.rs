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

        move |msg: Message| {
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

    thread::sleep(time::Duration::from_millis(500));

    ws::connect("ws://127.0.0.1:8002", |out| {
        let data = json!({
            "message_type": "join_game",
        });
        out.send(Message::Text(data.to_string())).unwrap();

        move |msg: Message| {
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

        move |msg: Message| {
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

    thread::sleep(time::Duration::from_millis(500));

    ws::connect("ws://127.0.0.1:8003", |out| {
        let data = json!({
            "message_type": "join_game",
        });
        out.send(Message::Text(data.to_string())).unwrap();

        move |msg: Message| {
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

        move |msg: Message| {
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

        move |msg: Message| {
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

#[test]
fn test_join_game_and_init_board() -> Result<()> {

    thread::spawn(move || {
        start_server("127.0.0.1:8004");
    });

    thread::sleep(time::Duration::from_millis(500));

    ws::connect("ws://127.0.0.1:8004", |out| {
        let data = json!({
            "message_type": "join_game",
        });
        out.send(Message::Text(data.to_string())).unwrap();

        move |msg: Message| {

            let json_str = msg.as_text().unwrap();
            let message_data: Value = serde_json::from_str(&json_str).unwrap();
            let message_type: &Value = message_data.get("message_type").unwrap_or_else(|| return &Value::Null);

            if message_type == "join_game" {
                let expected_data = json!({
                    "message_type": "join_game",
                    "status": "success",
                    "player": "first"
                });
                assert_eq!(msg, Message::Text(expected_data.to_string()));

                let data = json!({
                    "message_type": "initialize_board",
                    "board_size": 9

                });
                out.send(Message::Text(data.to_string())).unwrap();
            }
            else if message_type == "initialize_board" {
                let expected_data = json!({
                    "message_type": "initialize_board",
                    "status": "success"
                });
                assert_eq!(msg, Message::Text(expected_data.to_string()));
                out.close(CloseCode::Normal).unwrap();
                out.send(json!({
                    "message_type": "stop_server"
                }).to_string()).unwrap();
            }
            Ok(())
        }
    }).unwrap();

    Ok(())
}

#[test]
fn test_game_start() -> Result<()> {

    thread::spawn(move || {
        start_server("127.0.0.1:8006");
    });

    thread::sleep(time::Duration::from_millis(500));

    ws::connect("ws://127.0.0.1:8006", |out| {
        let data = json!({
            "message_type": "join_game",
        });
        out.send(Message::Text(data.to_string())).unwrap();

        move |msg: Message| {

            let json_str = msg.as_text().unwrap();
            let message_data: Value = serde_json::from_str(&json_str).unwrap();
            let message_type: &Value = message_data.get("message_type").unwrap_or_else(|| return &Value::Null);

            if message_type == "join_game" {
                let expected_data = json!({
                    "message_type": "join_game",
                    "status": "success",
                    "player": "first"
                });
                assert_eq!(msg, Message::Text(expected_data.to_string()));

                let data = json!({
                    "message_type": "initialize_board",
                    "board_size": 9

                });
                out.send(Message::Text(data.to_string())).unwrap();
            }
            else if message_type == "initialize_board" {
                let expected_data = json!({
                    "message_type": "initialize_board",
                    "status": "success"
                });
                assert_eq!(msg, Message::Text(expected_data.to_string()));
                out.close(CloseCode::Normal).unwrap();
            }
            else if message_type == "board_state" {
                let expected_data = json!({
                    "message_type": "board_state",
                    "board": [0, 0, 0, 0, 0, 0, 0, 0, 0],
                    "your_turn": true
                });
                assert_eq!(msg, Message::Text(expected_data.to_string()));
                out.close(CloseCode::Normal).unwrap();
            }
            Ok(())
        }
    }).unwrap();

    thread::sleep(time::Duration::new(1, 0));

    ws::connect("ws://127.0.0.1:8006", |out| {
        let data = json!({
            "message_type": "join_game",
        });
        out.send(Message::Text(data.to_string())).unwrap();

        move |msg: Message| {

            let json_str = msg.as_text().unwrap();
            let message_data: Value = serde_json::from_str(&json_str).unwrap();
            let message_type: &Value = message_data.get("message_type").unwrap_or_else(|| return &Value::Null);

            if message_type == "join_game" {
                let expected_data = json!({
                    "message_type": "join_game",
                    "status": "success",
                    "player": "second"
                });
                assert_eq!(msg, Message::Text(expected_data.to_string()));
            }
            else if message_type == "board_state" {
                let expected_data = json!({
                    "message_type": "board_state",
                    "board": [0, 0, 0, 0, 0, 0, 0, 0, 0],
                    "your_turn": false
                });
                assert_eq!(msg, Message::Text(expected_data.to_string()));
                out.close(CloseCode::Normal).unwrap();
                out.send(json!({
                    "message_type": "stop_server"
                }).to_string()).unwrap();
            }
            Ok(())
        }
    }).unwrap();

    Ok(())
}
