use yew::prelude::*;

use crate::game::Game;
use crate::field::FieldFC;

#[function_component]
pub fn BoardFC() -> Html {
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
