pub mod board;
pub mod game;
pub mod web_service;
pub mod field;
pub mod player;
pub mod event_bus;
pub mod start;
pub mod board9x9;
pub mod board19x19;

use event_bus::EventBus;
use serde_json::{Value, from_value};
use start::Start;
use yew::prelude::*;
use yew::{function_component, html};

use board::BoardFC;
use game::{init_fields, BoardSize, Game, GameAction, EventType, Payload};
use web_service::WebsocketService;
use yew_agent::{use_bridge, UseBridgeHandle};
use player::Player;


#[function_component(App)]
fn app() -> Html {
    let game = use_reducer(|| Game {
        size: BoardSize::Nine.to_owned(),
        fields: init_fields(BoardSize::Nine).to_owned(),
        wss: WebsocketService::new().to_owned(),
        player: Player::new().to_owned(),
    });

    {
        let game = game.clone();
        let _: UseBridgeHandle<EventBus> = use_bridge(move |response: String| {
            let response = response.clone();
            let res: Value = serde_json::from_str(&response).unwrap();

            if res["type_message"] == "board" {
                let board = res["board"].as_array().unwrap();
                let board: Vec<u64> = board.into_iter().map(|value| {
                    let num: u64 = from_value(value.clone()).unwrap();
                    num
                }).collect();

                game.dispatch(GameAction {
                    event_type: EventType::Board,
                    payload: Payload::Vector(board),
                });
            } else if res["type_message"] == "player" {
                let name = String::from(res["name"].as_str().unwrap()).parse::<u64>().unwrap();

                game.dispatch(GameAction {
                    event_type: EventType::Player,
                    payload: Payload::Player((name, String::from(res["side"].as_str().unwrap()))),
                });
            }

        });
    }


    html! {
        <ContextProvider<UseReducerHandle<Game>> context={game}>
            <main>
                <BoardFC />
            </main>
            <Start />
        </ContextProvider<UseReducerHandle<Game>>>
    }
}

fn main() {
    yew::start_app::<App>();
}

mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
