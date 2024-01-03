use yew::prelude::*;

use crate::{services::mazer::Mazer, components::maze_page::maze_view_canvas::MazeViewCanvas};

#[function_component(Maze)]
pub fn maze() -> Html {
    let mazer: UseStateHandle<Mazer> = use_state(|| Mazer::new());

    html! {
        <MazeViewCanvas
            mazer={(*mazer).clone()}
        />
    }
}
