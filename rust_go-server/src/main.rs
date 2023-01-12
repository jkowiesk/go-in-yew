use ws::{listen, CloseCode, Handler, Handshake, Message, Result, Sender};
use std::env;


struct User {
    nick: String,
    ws: Sender
}




fn main() {
    listen("127.0.0.1:8888", |out| {
        move |msg| {
            println!("{}", msg);
            out.send(msg)
        }
    }).unwrap();
}