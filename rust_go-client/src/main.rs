pub mod board;
pub mod game;
pub mod liberty;

use yew::prelude::*;
use yew::{function_component, html, Html};

use board::BoardFC;
use game::{init_liberties, BoardSize, Game};

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
