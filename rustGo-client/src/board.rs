use yew::prelude::*;

use crate::liberty::LibertyFC;
use crate::game::Game;

#[function_component]
pub fn BoardFC() -> Html {
    let game = use_context::<Game>().expect("no ctx found");
    let board = 0..81;

    let on_click = {

    };

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
                            <LibertyFC liberty={*liberty}/>
                        }).collect::<Html>()}
                </section>
            </div>
        </main>
        // <button onclick={Callback::from()}></button>
        </>
    }
}