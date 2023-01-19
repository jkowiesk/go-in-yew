use yew::prelude::*;

use crate::field::FieldFC;
use crate::game::{Game};
use gloo_console::log;

#[function_component(BoardFC19x19)]
pub fn board_fc_19x19() -> Html {
    let game = use_context::<UseReducerHandle<Game>>().expect("no ctx found");
    let board = 0..81;

    html! {
            <div class="board__max_width">
                <ul class="board__back--19x19">
                    {board.map(|_| html!{
                    <li class="board__back__square--19x19"></li>
                    }).collect::<Html>()}
                </ul>
                <section class="board__fields">
                        {game.fields.iter().map(|field| html!{
                            <FieldFC game={game.clone()} field={*field}/>
                        }).collect::<Html>()}
                </section>
            </div>
    }
}
