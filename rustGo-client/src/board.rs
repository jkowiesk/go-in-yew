use yew::prelude::*;

use crate::liberty::LibertyFC;

#[function_component]
pub fn BoardFC() -> Html {
    let board = 1..=81;
    let mut liberties = 1..=100;

    html! {
        <ul class="board--back--9x9">
            {board.map(|_| html!{
            <li class="board--back__square--9x9">
                <LibertyFC />
            </li>
            }).collect::<Html>()}
        </ul>
    }
}