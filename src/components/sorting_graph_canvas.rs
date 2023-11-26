use web_sys::console;
use yew::prelude::*;
use wasm_bindgen::prelude::*;

const CHART_WIDTH: i32 = 500;
const CHART_HEIGHT: i32 = 400;
const MAX_HEIGHT: i32 = 40;
const COEF: i32 = (CHART_WIDTH + CHART_HEIGHT) / MAX_HEIGHT;

fn get_translate(idx: usize, width: usize) -> String {
    format!("translate({}, 0)", idx * width)
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: Vec<i32>,
}

#[function_component(SortingGraphCanvas)]
pub fn sorting_graph_canvas(props: &Props) -> Html {

    html! {
        <div>
            <canvas id="graphCanvas" />
        </div>
    }
}
