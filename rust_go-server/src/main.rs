use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;


use ws::{listen, Message, Result, Sender};


struct Player {
    name: u32,
}

struct Server {
    players: Arc<Mutex<HashMap<Sender, Player>>>,

}

impl Server {
    fn new_player(&self, out: Sender, player: Player) {
        let players = self.players.clone();

        let mut players_locked = players.lock().unwrap();
        (*players_locked).insert(out, player);

    }

    fn print_players(&self) {
        let players = self.players.clone();

        let mut players_locked = players.lock().unwrap();
        for player in (*players_locked).iter().enumerate() {
            println!("{}: {}", player.0, player.1.1.name);
        }

    }
}

fn main() {
    let server = Server {players: Arc::new(Mutex::new(HashMap::new()))};

    listen("127.0.0.1:8888", |out| {
        let out_connection_id = out.connection_id();

        server.new_player(out, Player { name: out_connection_id });

        server.print_players();
        move |message: Message| {
            Ok(())
        }
    }).unwrap();
}