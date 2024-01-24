use crate::services::maze_generator::Cell;
use yew::prelude::*;

#[function_component(MazeLegend)]
pub fn maze_legend() -> Html {
    html! {
        <div>
            <div class="mb-2">
                {"Hey there, this section is still highly WIP, dont expect much"}
            </div>
            <div class="mb-2">
                {"List of colors:"}
            </div>
            <ul class="m-0 list-none">
                {Cell::iterator().map(|color| html! {
                    <li class="m-0"><span class="relative top-[3px] rounded-full inline-block w-4 h-4" style={format!("background-color: {};", color.as_color())}></span>{" - "}{color.as_name()}</li>
                 }).collect::<Html>()}
            </ul>
            <div class="mb-2">
                {"Black path is an actual path:"}
            </div>
        </div>
    }
}
