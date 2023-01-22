use futures::{channel::mpsc::{Sender}, SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message};
use yew_agent::Dispatched;
use gloo_console::log;

use wasm_bindgen_futures::spawn_local;
use crate::{event_bus::{EventBus, Request}, utils::format_msg};


#[derive(Debug, Clone)]
pub struct WebsocketService {
    pub tx: Sender<String>,
}

impl WebsocketService {
    pub fn new() -> Self {
        let ws = WebSocket::open("ws://127.0.0.1:8888").unwrap();

        let (mut write, mut read) = ws.split();
        let mut event_bus = EventBus::dispatcher();

        let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<String>(1000);

        spawn_local(async move {
            let join = format_msg("join_game", "");
            write.send(Message::Text(String::from(join))).await.unwrap();
            while let Some(s) = in_rx.next().await {
                log!("Sending to SERVER: ", &s);
                write.send(Message::Text(s)).await.unwrap();
            }
        });

        spawn_local(async move {
            while let Some(msg) = read.next().await {
                    if let Ok(Message::Text(text)) = msg {
                        log!("From SERVER: ", &text);
                        event_bus.send(Request::EventBusMsg(text));
                }
            }
            log!("WebSocket Closed");
        });

        Self { tx: in_tx }
    }
}

impl PartialEq for WebsocketService {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
