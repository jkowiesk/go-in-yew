use yew::prelude::*;

use crate::{game::{Game}, utils::format_msg};

#[derive(Properties, PartialEq)]
pub struct Props {
}

#[function_component(Start)]
pub fn start() -> Html {
    let game = use_context::<UseReducerHandle<Game>>().expect("no ctx found");



    let onclick_9x9 = {
        let game = game.clone();
        Callback::from(move |_| {
            let content = format!("\"board_size\": 100");
            if let Ok(_) = game.wss.tx.clone().try_send(format_msg("initialize_board", &content)) {};
        })
    };

    let onclick_19x19 = {
        let game = game.clone();
        Callback::from(move |_| {
            let content = format!("\"board_size\": 400");
            if let Ok(_) = &game.wss.tx.clone().try_send(format_msg("initialize_board", &content)) {};
        })
    };

    if let Some(name) = &game.clone().player.name {
        if name == "second" {
            return html!{};
        }
    }

    match &game.clone().size {
        Some(_) => html!{},
        None => html!{<div class="btn_container">
        <section class="flex gap-64">
            <button onclick={onclick_9x9} class="bg-green-600 rounded-md w-64 h-32 p-8 text-2xl shadow-sm">{"9x9"}</button>
            <button onclick={onclick_19x19} class="bg-red-600 text-white rounded-md w-64 h-32 p-8 text-2xl shadow-xl">{"19x19"}</button>
        </section>
    </div>}
    }
}
