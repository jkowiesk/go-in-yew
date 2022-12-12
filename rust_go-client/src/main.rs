pub mod game;
pub mod liberty;
pub mod board;

use yew::prelude::*;
use yew::{html, function_component, Html};

use board::BoardFC;
use game::{Size, Game, init_liberties};

#[function_component]
fn App() -> Html {
    let game = use_reducer(|| Game {
        size: Size::nine.to_owned(),
        liberties: init_liberties(Size::nine).to_owned(),
    });


    html! {
        <ContextProvider<UseReducerHandle<Game>> context={game}>
            <BoardFC />
        </ContextProvider<UseReducerHandle<Game>>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}