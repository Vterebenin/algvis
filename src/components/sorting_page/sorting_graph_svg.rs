use web_sys::console;
use yew::prelude::*;

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

// TODO: deprecated
#[function_component(SortingGraphSvg)]
pub fn sorting_graph_svg(props: &Props) -> Html {
    let items_count = props.data.len();
    let items = (*props.data)
        .into_iter()
        .enumerate()
        .map(|(idx, item)| {
            let place = (*item as f32 / items_count as f32) as f32 * 100 as f32;
            let height = (place / *item as f32) * CHART_HEIGHT as f32 / 100 as f32 * *item as f32;

            let y = CHART_HEIGHT as f32 - height;
            let width = CHART_WIDTH as usize / items_count;
            console::log_1(&format!("height: {} {}", height, place).into());
            html! {
                <g key={*item} class="fill-accent" transform={get_translate(idx, width)}>
                    <rect
                        height={height.to_string()}
                        y={y.to_string()}
                        width={width.to_string()}
                    ></rect>
                </g>
            }
        })
        .collect::<Html>();

    let view_box = format!("0 0 {} {}", CHART_WIDTH, CHART_HEIGHT);
    html! {
        <svg
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            class="chart"
            height={CHART_HEIGHT.to_string()}
            width={CHART_WIDTH.to_string()}
            viewBox={view_box}
            aria-labelledby="title"
            role="img"
        >
            { items }
        </svg>
    }
}
