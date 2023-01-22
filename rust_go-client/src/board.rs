use yew::prelude::*;

use crate::board19x19::Board19x19;
use crate::board9x9::Board9x9;
use crate::game::{Game, BoardSize};

#[function_component(BoardFC)]
pub fn board_fc() -> Html {
    let game = use_context::<UseReducerHandle<Game>>().expect("no ctx found");


    let size = match &game.clone().size {
        Some(temp_size) => match temp_size {
            BoardSize::Nine => 9,
            BoardSize::Nineteen => 19
        }
        None => 9,
    };

    html! {
        if size == 19 {
            <Board19x19 />
        } else {
            <Board9x9 />
        }
    }
}
