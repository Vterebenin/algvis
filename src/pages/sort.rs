use rand::seq::SliceRandom;
use rand::thread_rng;
use web_sys::wasm_bindgen::{JsValue, intern};
use yew::prelude::*;
use gloo_timers::callback::{Timeout, Interval};

const WIDTH: usize = 8;
const CHART_WIDTH: i32 = 500;
const CHART_HEIGHT: i32 = 400;
const MAX_HEIGHT: i32 = 40;
const COEF: i32 = (CHART_WIDTH + CHART_HEIGHT) / MAX_HEIGHT;

fn get_translate(idx: usize) -> String {
    format!("translate({}, 0)", idx * (WIDTH + 2))
}

pub fn merge_sort<T: Copy + Clone + Ord>(items: &mut Vec<T>, steps: &mut Vec<(usize, T)>) {
    _merge_sort(items, steps, 0);
}

fn _merge_sort<T: Copy + Clone + PartialOrd>(
    items: &mut Vec<T>,
    mut steps: &mut Vec<(usize, T)>,
    start_i: usize,
) {
    if items.len() > 1 {
        let middle = items.len() / 2;
        let mut left_half = items[0..middle].to_vec();
        let mut right_half = items[middle..].to_vec();
        _merge_sort(&mut left_half, &mut steps, start_i);
        _merge_sort(&mut right_half, &mut steps, start_i + middle);
        *items = merge(left_half, right_half, &mut steps, start_i);
    }
}

fn merge<T: Copy + Clone + PartialOrd>(
    a: Vec<T>,
    b: Vec<T>,
    steps: &mut Vec<(usize, T)>,
    start_i: usize,
) -> Vec<T> {
    let size = a.len() + b.len();
    let mut merged: Vec<T> = Vec::with_capacity(size);

    let mut i = 0; // Idx for a
    let mut j = 0; // Idx for b

    // Loop through a and b, adding the smallest values between them to `merged`
    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            merged.push(a[i]);
            steps.push((start_i + merged.len() - 1, a[i]));
            i += 1;
        } else {
            merged.push(b[j]);
            steps.push((start_i + merged.len() - 1, b[j]));
            j += 1;
        }
    }

    // Add all remaining values
    while i < a.len() {
        merged.push(a[i]);
        steps.push((start_i + merged.len() - 1, a[i]));
        i += 1;
    }
    while j < b.len() {
        merged.push(b[j]);
        steps.push((start_i + merged.len() - 1, b[j]));
        j += 1;
    }

    merged
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
        // let data_clone = data.clone();
        // let timeout = Timeout::new(1_000, move || {
        //     data_clone.set(data);
        // });
        // timeout.forget();
    });
    html! {
        <>
            <button {onclick}>{ "Click" }</button>
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
