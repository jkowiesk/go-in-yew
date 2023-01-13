pub mod board;
pub mod game;
pub mod web_service;
pub mod field;
pub mod player;

use yew::prelude::*;
use yew::{function_component, html, Html};

use board::BoardFC;
use game::{init_fields, BoardSize, Game};
use web_service::WebsocketService;
use gloo_console::log;
use serde_json;


#[function_component]
fn App() -> Html {
    let wss = WebsocketService::new();
    let game = use_reducer(|| Game {
        size: BoardSize::Nine.to_owned(),
        fields: init_fields(BoardSize::Nine).to_owned(),
    });


    /* use_effect(move || {
        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&"gitara").unwrap())
        {
            log!("message sent successfully");
        }
    }); */



    html! {
        <ContextProvider<UseReducerHandle<Game>> context={game}>
            <BoardFC />
        </ContextProvider<UseReducerHandle<Game>>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
