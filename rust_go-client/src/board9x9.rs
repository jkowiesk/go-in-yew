use yew::prelude::*;

use crate::field::FieldFC;
use crate::game::Game;

#[function_component(Board9x9)]
pub fn board_9x9() -> Html {
    let game = use_context::<UseReducerHandle<Game>>().expect("no ctx found");
    let board = 0..81;

    html! {
            <div class="board--9x9">
                <ul class="board__back--9x9">
                    {board.map(|_| html!{
                    <li class="board__back--9x9__square"></li>
                    }).collect::<Html>()}
                </ul>
                <section class="board__fields--9x9">
                        {game.fields.iter().map(|field| html!{
                            <FieldFC game={game.clone()} size={9} field={*field}/>
                        }).collect::<Html>()}
                </section>
            </div>
    }
}
