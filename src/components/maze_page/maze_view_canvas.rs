use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, console};
use yew::prelude::*;

use crate::services::{mazer::{Mazer, RunType}, maze_generator::Cell};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub mazer: Mazer,
}

pub struct MazeItem {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

pub fn calculate_item(
    x_idx: u32,
    y_idx: u32,
    horizontal_items: u32,
    vertical_items: u32,
    canvas: &HtmlCanvasElement,
) -> MazeItem {
    let spacing = 1.;
    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let horizontal_items = horizontal_items as f64;
    let vertical_items = vertical_items as f64;
    let y_idx = y_idx as f64;
    let x_idx = x_idx as f64;

    let item_height = (height - horizontal_items * spacing) / horizontal_items;
    let item_width = (width - vertical_items * spacing) / vertical_items;

    let y = item_height * y_idx + spacing * y_idx;
    let x = item_width * x_idx + spacing * x_idx;

    MazeItem {
        x,
        y,
        width: item_width,
        height: item_height,
    }
}

#[function_component(MazeViewCanvas)]
pub fn maze_view_canvas(props: &Props) -> Html {
    let mazer = props.mazer.clone();
    let size_x = props.mazer.size_x;
    let size_y = props.mazer.size_y;
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
        context.fill();
        context.set_line_cap("round");
        context.set_line_join("round");

        // field
        context.set_fill_style(&str_to_js("#000000"));
        for x_idx in 0..mazer.size_x {
            for y_idx in 0..mazer.size_y {
                let MazeItem {
                    x,
                    y,
                    width,
                    height,
                } = calculate_item(x_idx, y_idx, size_x, size_y, &canvas);
                let item_of_maze = mazer.maze.cells[y_idx as usize][x_idx as usize];
                let color = match item_of_maze {
                    Cell::Wall => "#00ffff",
                    Cell::Empty => "#000000",
                    Cell::Entry => "#ffff00",
                    Cell::Exit => "#00ff00"
                };
                context.set_fill_style(&str_to_js(color));
                console::log_1(&str_to_js(format!("{:?}", item_of_maze).as_str()));
                console::log_1(&str_to_js(format!("{} {}", width, height).as_str()));
                context.fill_rect(x, y, width, height);
                context.set_fill_style(&str_to_js("#000000"));
            }
        }
    });
    html! {
        <>
            <canvas id="canvas" class="w-full block" width="950" height="500" />
        </>
    }
}
