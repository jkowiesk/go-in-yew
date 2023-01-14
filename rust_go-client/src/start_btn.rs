use yew::prelude::*;

use crate::game::{Event, EventAction, Game, Field, Stone};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub field: Field,
    pub game: UseReducerHandle<Game>,
}

#[function_component]
pub fn StartBtn(props: &Props) -> Html {
    let Props { game } = props;
    let game = game.clone();
    let field = field.clone();
    let is_btn_clicked = use_state(|| {

    })

    let on_click = {
        let game = game.clone();
        Callback::from(move |_| {
            game.dispatch(Event {
                event_type: EventAction::Place,
                payload: field.idx,
            })
        })
    };


    html! {
        if is_btn_clicked {
            <button class="btn">Start Game</button>
        }
    }
}
