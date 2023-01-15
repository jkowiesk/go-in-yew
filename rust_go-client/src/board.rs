use futures::StreamExt;
use reqwasm::websocket::{futures::WebSocket, Message};
use web_sys::console::log;
use yew::prelude::*;
use yew_agent::{use_bridge, Bridged, UseBridgeHandle};

use crate::event_bus::EventBus;
use crate::field::FieldFC;
use crate::game::{Action, EventType, Game, Payload};
use gloo_console::log;

#[function_component(BoardFC)]
pub fn board_fc() -> Html {
    let game = use_context::<UseReducerHandle<Game>>().expect("no ctx found");
    let board = 0..81;


    html! {
        <>
        <main>
            <div class="board__max_width">
                <ul class="board__back--9x9">
                    {board.map(|_| html!{
                    <li class="board__back__square--9x9"></li>
                    }).collect::<Html>()}
                </ul>
                <section class="board__fields">
                        {game.fields.iter().map(|field| html!{
                            <FieldFC game={game.clone()} field={*field}/>
                        }).collect::<Html>()}
                </section>
            </div>
        </main>
        </>
    }
}
