use rand::seq::SliceRandom;
use rand::thread_rng;
use web_sys::wasm_bindgen::JsValue;
use yew::prelude::*;
use gloo_timers::callback::Interval;

use crate::sorting_algorithms::merge_sort::merge_sort;

const WIDTH: usize = 8;
const CHART_WIDTH: i32 = 500;
const CHART_HEIGHT: i32 = 400;
const MAX_HEIGHT: i32 = 40;
const COEF: i32 = (CHART_WIDTH + CHART_HEIGHT) / MAX_HEIGHT;

fn get_translate(idx: usize) -> String {
    format!("translate({}, 0)", idx * (WIDTH + 2))
}

#[function_component(Sort)]
pub fn sort() -> Html {
    let mut data: Vec<i32> = (1..=50).collect(); // Create a vector with numbers from 1 to 100

    let mut rng = thread_rng();
    data.shuffle(&mut rng);
    let data: UseStateHandle<Vec<i32>> = use_state(|| data);
    let items = (*data)
        .clone()
        .into_iter()
        .enumerate()
        .map(|(idx, item)| {
            let height = item * (CHART_HEIGHT / COEF) / 4;

            let y = CHART_HEIGHT - height;
            html! {
                <g key={item} class="fill-pumpkin" transform={get_translate(idx)}>
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
    let time = 10;

    let onclick = Callback::from(move |_| {
        let mut items = (*data).clone();
        let mut steps = vec![];
        merge_sort(&mut items, &mut steps);
        // todo: maybe steps should be a VecDeque?
        steps.reverse();

        let items = data.clone();
        let mut arr = (*data).clone();
        let interval = Interval::new(time, move || {
            let item = steps.pop();
            if item.is_some() {
                let (index, val) = item.unwrap();
                arr[index] = val;
                items.set(arr.clone());
                web_sys::console::log_1(&JsValue::from(format!("{:?}", item)))
            }
        });
        interval.forget();
    });
    html! {
        <>
            <div class="flex justify-center"> 
                <button {onclick}>{ "Sort it!" }</button>
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
            </div>
        </>
    }
}
