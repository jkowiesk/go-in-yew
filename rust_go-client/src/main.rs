pub mod board;
pub mod game;
pub mod web_service;
pub mod field;
pub mod player;
pub mod event_bus;
pub mod start;
pub mod board9x9;
pub mod board19x19;
pub mod utils;
pub mod info_dialog;

use event_bus::EventBus;
use info_dialog::InfoDialog;
use serde_json::{Value, from_value};
use start::Start;
use yew::prelude::*;
use yew::{function_component, html};

use board::BoardFC;
use game::{init_fields, BoardSize, Game, GameAction, EventType, Payload};
use web_service::WebsocketService;
use yew_agent::{use_bridge, UseBridgeHandle};
use player::Player;
use gloo_console::log;
use gloo_dialogs::alert;


#[function_component(App)]
fn app() -> Html {
    let game = use_reducer(|| Game {
        size: None,
        fields: init_fields(BoardSize::Nine).to_owned(),
        wss: WebsocketService::new().to_owned(),
        player: Player::new().to_owned(),
    });


    {
        let game = game.clone();
        let _: UseBridgeHandle<EventBus> = use_bridge(move |response: String| {
            let response = response.clone();
            let res: Value = serde_json::from_str(&response).unwrap();

            if res["message_type"] == "board_state" {
                let board = res["board"].as_array().unwrap();
                let board: Vec<u64> = board.into_iter().map(|value| {
                    let num: u64 = from_value(value.clone()).unwrap();
                    num
                }).collect();

                let turn = res["your_turn"].as_bool().unwrap();

                game.dispatch(GameAction {
                    event_type: EventType::Board,
                    payload: Payload::BoardState((board, turn)),
                });
            } else if res["message_type"] == "join_game" {
                let name = String::from(res["player"].as_str().unwrap());

                game.dispatch(GameAction {
                    event_type: EventType::Player,
                    payload: Payload::Player((name)),
                });
            } else if res["message_type"] == "initialize_board" {
                game.dispatch(GameAction {
                    event_type: EventType::BoardSize,
                    payload: Payload::Size(BoardSize::Nine),
                });
            }

        });
    }


    html! {
        <ContextProvider<UseReducerHandle<Game>> context={game}>
            <main>
                <BoardFC />
            </main>
            <InfoDialog />
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
