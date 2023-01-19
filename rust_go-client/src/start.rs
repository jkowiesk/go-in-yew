use yew::prelude::*;

use crate::game::{Game, Field, Stone, GameAction, EventType, Payload, BoardSize};

#[derive(Properties, PartialEq)]
pub struct Props {

}

#[function_component(Start)]
pub fn start() -> Html {
    let is_btn_clicked = use_state(|| {
        false
    });
    let is_btn_clicked_temp = is_btn_clicked.clone();
    let game = use_context::<UseReducerHandle<Game>>().expect("no ctx found");


    let onclick_9x9 = {
        let game = game.clone();
        let is_btn_clicked = is_btn_clicked.clone();
        Callback::from(move |_| {
            game.dispatch(GameAction {
                event_type: EventType::BoardSize,
                payload: Payload::Size(BoardSize::Nine),
            });
            is_btn_clicked.set(false)
        })
    };

    let onclick_13x13 = {
        Callback::from(move |_| {
            game.dispatch(GameAction {
                event_type: EventType::BoardSize,
                payload: Payload::Size(BoardSize::Thirteen),
            });
            is_btn_clicked.set(false)
        })
    };

    html! {
        if *is_btn_clicked_temp {
            <div class="btn_container">
                <section class="flex gap-64">
                    <button onclick={onclick_9x9} class="bg-green-600 rounded-md w-64 h-32 p-8 text-2xl shadow-sm">{"9x9"}</button>
                    <button onclick={onclick_13x13} class="bg-red-600 text-white rounded-md w-64 h-32 p-8 text-2xl shadow-xl">{"19x19"}</button>
                </section>
            </div>
        }
    }
}
