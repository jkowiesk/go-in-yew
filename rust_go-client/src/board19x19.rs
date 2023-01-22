use yew::prelude::*;

use crate::field::FieldFC;
use crate::game::{Game};

#[function_component(Board19x19)]
pub fn board_19x19() -> Html {
    let game = use_context::<UseReducerHandle<Game>>().expect("no ctx found");
    let board = 0..361;

    html! {
            <div class="board--19x19">
                <ul class="board__back--19x19">
                    {board.map(|_| html!{
                    <li class="board__back--19x19__square"></li>
                    }).collect::<Html>()}
                </ul>
                <section class="board__fields--19x19">
                        {game.fields.iter().map(|field| html!{
                            <FieldFC game={game.clone()} size={19} field={*field}/>
                        }).collect::<Html>()}
                </section>
            </div>
    }
}
