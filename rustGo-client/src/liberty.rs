use yew::prelude::*;

use crate::game::{Liberty, Stone};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub liberty: Liberty,
}

#[function_component]
pub fn LibertyFC(props: &Props) -> Html {
    let Props { liberty } = props;

    let class_name = match &liberty.owner {
        Some(stone) => {
            match stone {
                Stone::black => "liberty liberty--black",
                Stone::white => "liberty liberty--white"
            }
        },
        None => ""
    };

    html! {
        <div class={class_name}></div>
    }
}