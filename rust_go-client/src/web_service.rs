use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message};
use gloo_console::log;

use wasm_bindgen_futures::spawn_local;

pub struct WebsocketService {
    pub tx: Sender<String>,
}

impl WebsocketService {
    pub fn new() -> Self {
        let ws = WebSocket::open("ws://127.0.0.1:8888").unwrap();
        log!("connected");

        let (mut write, mut read) = ws.split();

        let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<String>(1000);

        spawn_local(async move {
            write.send(Message::Text(String::from("test"))).await.unwrap();
        });

        Self { tx: in_tx }
    }
}