use yew::{html, function_component, Html};

#[function_component]
fn App() -> Html {
    let test = "gitara";
    let board = vec!["test1", "test2", "test3", "test4", "test5", "test6", "test7", "test8", "test9"];

    html! {
        <main>
            <div class="board_3x3">{board.iter().map(|_| html!{<div class="elem"></div>}).collect::<Html>()}</div>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}