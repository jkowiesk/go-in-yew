use yew::prelude::*;

use crate::liberty::LibertyFC;
use crate::game::{Game, Event, EventAction};

#[function_component]
pub fn BoardFC() -> Html {
    let game = use_context::<UseReducerHandle<Game>>().expect("no ctx found");
    let board = 0..81;

    let change_token = {
        let game = game.clone();
        Callback::from(move |_| game.dispatch(Event {event_type: EventAction::place, payload: 0 }))
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
        <button  onclick={change_token}>
            { "Click me!" }
        </button>
        </>
    }
}