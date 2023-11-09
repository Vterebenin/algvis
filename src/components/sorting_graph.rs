use yew::prelude::*;

const WIDTH: usize = 8;
const CHART_WIDTH: i32 = 500;
const CHART_HEIGHT: i32 = 400;
const MAX_HEIGHT: i32 = 40;
const COEF: i32 = (CHART_WIDTH + CHART_HEIGHT) / MAX_HEIGHT;

fn get_translate(idx: usize) -> String {
    format!("translate({}, 0)", idx * (WIDTH + 2))
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: Vec<i32>,
}

#[function_component(SortingGraph)]
pub fn sorting_graph(props: &Props) -> Html {
    let items = (*props.data)
        .into_iter()
        .enumerate()
        .map(|(idx, item)| {
            let height = item * (CHART_HEIGHT / COEF) / 4;

            let y = CHART_HEIGHT - height;
            html! {
                <g key={*item} class="fill-pumpkin" transform={get_translate(idx)}>
                    <rect
                        height={height.to_string()}
                        y={y.to_string()}
                        width={WIDTH.to_string()}
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
