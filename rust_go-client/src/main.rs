pub mod board;
pub mod game;
pub mod web_service;
pub mod field;
pub mod player;
pub mod event_bus;

use event_bus::EventBus;
use yew::prelude::*;
use yew::{function_component, html, Html};

use board::BoardFC;
use game::{init_fields, BoardSize, Game, Action, EventType, Payload};
use web_service::WebsocketService;
use gloo_console::log;
use yew_agent::{use_bridge, UseBridgeHandle};


#[function_component(App)]
fn app() -> Html {
    let game = use_reducer(|| Game {
        size: BoardSize::Nine.to_owned(),
        fields: init_fields(BoardSize::Nine).to_owned(),
        wss: WebsocketService::new(),
    });

    {
        let game = game.clone();
        let _: UseBridgeHandle<EventBus> = use_bridge(move |response| {
            game.dispatch(Action {
                event_type: EventType::Board,
                payload: Payload::Text(response),
            });
        });
    }


    html! {
        <ContextProvider<UseReducerHandle<Game>> context={game}>
            <BoardFC />
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
