use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message};
use gloo_console::log;

use wasm_bindgen_futures::spawn_local;


#[derive(Clone, Debug)]
pub struct WebsocketService {
    pub tx: Sender<String>,
}

impl WebsocketService {
    pub fn new() -> Self {
        let ws = WebSocket::open("ws://127.0.0.1:8888").unwrap();

        let (mut write, mut read) = ws.split();

        let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<String>(1000);

        spawn_local(async move {
            while let Some(s) = in_rx.next().await {
                log!("got event from channel! ", &s);
                write.send(Message::Text(s)).await.unwrap();
            }
        });

        spawn_local(async move {
            while let Some(msg) = read.next().await {
                if let Ok(data) = msg {
                    if let Message::Text(text) = data {
                        log!(text);
                    }
                }
            }
            log!("WebSocket Closed");
        });

        Self { tx: in_tx }
    }
}

impl PartialEq for WebsocketService {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}
