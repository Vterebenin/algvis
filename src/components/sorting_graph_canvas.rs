use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use yew::prelude::*;

const CHART_WIDTH: f64 = 500.0;
const CHART_HEIGHT: f64 = 400.0;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: Vec<i32>,
}

pub struct ChartItem {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

pub fn calculate_item(item: i32, idx: usize, total_count: f64, canvas: &HtmlCanvasElement) -> ChartItem {
    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let item = item as f64;
    let idx = idx as f64;

    let absolute_item_value = item / total_count;

    let item_height = absolute_item_value * height;
    let item_width = width / total_count;

    let y = height - item_height;
    let x = item_width * idx;

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

        for (idx, &item) in data.iter().enumerate() {
            let ChartItem {
                x,
                y,
                width,
                height,
            } = calculate_item(item, idx, items_count, &canvas);
            context.fill_rect(x, y, width, height);
        }
    });
    html! {
        <canvas id="canvas" width={CHART_WIDTH.to_string()} height={CHART_HEIGHT.to_string()} />
    }
}
