use rand::seq::SliceRandom;
use rand::thread_rng;
use yew::prelude::*;
use gloo_timers::callback::Timeout;

const WIDTH: usize = 8;
const CHART_WIDTH: i32 = 500;
const CHART_HEIGHT: i32 = 400;
const MAX_HEIGHT: i32 = 40;
const COEF: i32 = (CHART_WIDTH + CHART_HEIGHT) / MAX_HEIGHT;

fn get_translate(idx: usize) -> String {
    format!("translate({}, 0)", idx * (WIDTH + 2))
}

fn merge_sort<T: Ord + Clone>(arr: &mut Vec<T>) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
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

    let onclick = Callback::from(move |_| {
        let sorted: Vec<i32> = (1..=50).collect(); // Create a vector with numbers from 1 to 100
        let data_clone = data.clone();
        let timeout = Timeout::new(1_000, move || {
            data_clone.set(sorted);
        });
        timeout.forget();
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
