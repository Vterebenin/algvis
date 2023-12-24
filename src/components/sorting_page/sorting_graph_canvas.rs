use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use yew::prelude::*;

use crate::services::sorter::SortType;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: Vec<i32>,
    pub active_step_item: SortType<i32>
}

pub struct ChartItem {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

pub fn calculate_item(item: i32, idx: usize, total_count: f64, canvas: &HtmlCanvasElement) -> ChartItem {
    let spacing = 1.;
    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let item = item as f64;
    let idx = idx as f64;

    let absolute_item_value = item / total_count;

    let is_width_spaceable = width > (total_count * spacing) * 2.;
    let item_height = absolute_item_value * height;
    let item_width = if is_width_spaceable {
        (width - total_count * spacing) / total_count
    } else {
        width / total_count
    };

    let y = height - item_height;
    let x = if is_width_spaceable {
        item_width * idx + spacing * idx
    } else {
        item_width * idx
    };

    ChartItem {
        x,
        y,
        width: item_width,
        height: item_height,
    }
}

#[function_component(SortingGraphCanvas)]
pub fn sorting_graph_canvas(props: &Props) -> Html {
    let data = props.data.clone();
    let step_item = props.active_step_item;
    let str_to_js = |str: &str| JsValue::from(str);
    use_effect(move || {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        context.set_fill_style(&str_to_js("#ff5733"));
        context.set_line_cap("round");
        context.set_line_join("round");

        let items_count = data.len() as f64;

        let colored_items = match step_item {
            SortType::Swap(idx1, idx2) => {
                if idx1 == idx2 && idx1 == 0 {
                    vec![]
                } else {
                    vec![idx1, idx2]
                }
            },
            SortType::Set(idx, _value) => vec![idx],
        };
        for (idx, &item) in data.iter().enumerate() {
            let ChartItem {
                x,
                y,
                width,
                height,
            } = calculate_item(item, idx, items_count, &canvas);
            if colored_items.contains(&idx) {
                context.set_fill_style(&str_to_js("#53c2da"));
            }
            context.fill_rect(x, y, width, height);
            context.set_fill_style(&str_to_js("#ff5733"));
        }
    });
    html! {
        <>
            <canvas id="canvas" class="w-full block" width="950" height="500" />
        </>
    }
}
