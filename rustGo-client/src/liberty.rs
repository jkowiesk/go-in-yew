use yew::prelude::*;

pub enum LibertyKind {
    default,
}

pub enum Stone {
    black,
    white
}

pub struct LibertyState {
    pub kind: LibertyKind,
    pub owner: Option<Stone>,
}

const INIT_STATE: LibertyState = LibertyState {
    kind: LibertyKind::default,
    owner: Some(Stone::white),
};


#[function_component]
pub fn LibertyFC() -> Html {
    let kind = use_state(|| INIT_STATE.kind);
    let owner = use_state(|| INIT_STATE.owner);

    let class_name = match &*owner {
        Some(stone) => {
            match stone {
                Stone::black => "liberty--default liberty--black",
                Stone::white => "liberty--default liberty--white"
            }
        },
        None => ""
    };

    html! {
        <div class={class_name}></div>
    }
}