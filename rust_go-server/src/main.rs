use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;

fn main() {
    let server = Server::bind("127.0.0.1:8888").expect("Failed to bind to server address");

    for request in server.filter_map(Result::ok) {
        thread::spawn(move || {
            if !request.protocols().contains(&"rust-websocket".to_string()) {
                request.reject().expect("Failed to reject request");
                return;
            }

            let mut client = request
                .use_protocol("rust-websocket")
                .accept()
                .expect("Failed to accept websocket connection");

            let ip = client.peer_addr().expect("Failed to get peer address");
            println!("Connection from {}", ip);

            let message = OwnedMessage::Text("Hello".to_string());
            client.send_message(&message).expect("Failed to send message");

            let (mut receiver, mut sender) = client.split().expect("Failed to split client");

            for message in receiver.incoming_messages() {
                let message = message.expect("Failed to receive message");

                match message {
                    OwnedMessage::Close(_) => {
                        let message = OwnedMessage::Close(None);
                        sender
                            .send_message(&message)
                            .expect("Failed to send close message");
                        println!("Client {} disconnected", ip);
                        return;
                    }
                    OwnedMessage::Ping(ping) => {
                        let message = OwnedMessage::Pong(ping);
                        sender
                            .send_message(&message)
                            .expect("Failed to send pong message");
                    }
                    _ => sender.send_message(&message).expect("Failed to send message"),
                }
            }
        });
    }
}