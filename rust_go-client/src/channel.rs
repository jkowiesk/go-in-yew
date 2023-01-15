use futures::channel::mpsc::{Receiver, Sender};
use wasm_bindgen_futures::spawn_local;

#[derive(Debug)]
pub struct Channel {
    pub in_tx: Sender<String>,
    pub in_rx: Receiver<String>,
}

impl Channel {
    pub fn new() -> Self {
        let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<String>(1000);

        Self { in_tx, in_rx }
    }
}

impl PartialEq for Channel {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}
