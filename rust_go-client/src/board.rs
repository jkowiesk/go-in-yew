use yew::prelude::*;

use crate::game::Game;
use crate::liberty::LibertyFC;

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
                <section class="board__liberties">
                        {game.liberties.iter().map(|liberty| html!{
                            <LibertyFC game={game.clone()} liberty={*liberty}/>
                        }).collect::<Html>()}
                </section>
            </div>
        </main>
        </>
    }
}
