use web_sys::console;
use yew::prelude::*;

const CHART_WIDTH: i32 = 500;
const CHART_HEIGHT: i32 = 400;
const MAX_HEIGHT: i32 = 40;
const COEF: i32 = (CHART_WIDTH + CHART_HEIGHT) / MAX_HEIGHT;

fn get_translate(idx: usize, width: usize) -> String {
    format!("translate({}, 0)", idx * (width + 2))
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: Vec<i32>,
}

#[function_component(SortingGraph)]
pub fn sorting_graph(props: &Props) -> Html {
    let items_count = props.data.len();
    let items = (*props.data)
        .into_iter()
        .enumerate()
        .map(|(idx, item)| {
            let height = item * (CHART_HEIGHT / COEF) / 4;

            let y = CHART_HEIGHT - height;
            let width = (CHART_WIDTH as usize - (5 * items_count)) as usize / items_count;
            console::log_1(&format!("width: {}", width).into());
            html! {
                <g key={*item} class="fill-pumpkin" transform={get_translate(idx, width)}>
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
        <>
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
        </>
    }
}
