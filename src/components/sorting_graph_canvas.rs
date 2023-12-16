use std::f64;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

const CHART_WIDTH: f64 = 500.0;
const CHART_HEIGHT: f64 = 400.0;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: Vec<i32>,
}

struct ChartItem {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

pub fn calculate_item(item: i32, idx: usize, total_count: f64) -> ChartItem {
    let item = item as f64;
    let idx = idx as f64;

    let absolute_item_value = item / total_count;

    let height = absolute_item_value * CHART_HEIGHT;
    let width = CHART_WIDTH / total_count;

    let y = CHART_HEIGHT - height;
    let x = width * idx;

    ChartItem {
        x,
        y,
        width,
        height,
    }
}

#[function_component(SortingGraphCanvas)]
pub fn sorting_graph_canvas(props: &Props) -> Html {
    let data = props.data.clone();
    let str_to_js = |str: &str| JsValue::from(str);
    use_effect(move || {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        context.set_fill_style(&str_to_js("#009688"));
        context.set_line_cap("round");
        context.set_line_join("round");

        let items_count = data.len() as f64;

        for (idx, &item) in data.iter().enumerate() {
            let ChartItem {
                x,
                y,
                width,
                height,
            } = calculate_item(item, idx, items_count);
            context.fill_rect(x, y, width, height);
        }
    });
    html! {
        <div>
            <canvas id="canvas" width={CHART_WIDTH.to_string()} height={CHART_HEIGHT.to_string()} />
        </div>
    }
}
