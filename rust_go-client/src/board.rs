use yew::prelude::*;

use crate::board9x9::Board9x9;
use crate::field::FieldFC;
use crate::game::{Game};
use gloo_console::log;

#[function_component(BoardFC)]
pub fn board_fc() -> Html {
    let game = use_context::<UseReducerHandle<Game>>().expect("no ctx found");
    let board = 0..81;

    html! {
            <Board9x9 />
    }
}
