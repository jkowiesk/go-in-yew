pub mod game;
pub mod liberty;
pub mod board;


use yew::prelude::*;
use yew::{html, function_component, Html};

use board::BoardFC;
use game::{BoardSize, Game, init_liberties};

#[function_component]
fn App() -> Html {
    let game = use_reducer(|| Game {
        size: BoardSize::Nine.to_owned(),
        liberties: init_liberties(BoardSize::Nine).to_owned(),
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

mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
