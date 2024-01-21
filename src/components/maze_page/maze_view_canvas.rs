use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::{console, CanvasRenderingContext2d, Element, HtmlCanvasElement, HtmlElement};
use yew::prelude::*;

use crate::services::{maze_generator::Cell, mazer::Mazer};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub mazer: Mazer,
    pub on_cell_click: Callback<MazeItem>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Coords<T> {
    pub x: T,
    pub y: T,
}

impl<T> Coords<T> {
    pub fn from(x: T, y: T) -> Self {
        Coords::<T> { x, y }
    }
}

#[derive(Clone, Debug)]
pub struct MazeItem {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    pub row: usize,
    pub col: usize,
    current_type: Cell,
}

const SPACING: f64 = 1.;

pub fn get_item_sizes_by_items(
    canvas: &HtmlCanvasElement,
    horizontal_items: usize,
    vertical_items: usize,
) -> (f64, f64) {
    let horizontal_items = horizontal_items as f64;
    let vertical_items = vertical_items as f64;
    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let item_height = (height - horizontal_items * SPACING) / horizontal_items;
    let item_width = (width - vertical_items * SPACING) / vertical_items;
    (item_width, item_height)
}

pub fn calculate_item(
    x_idx: usize,
    y_idx: usize,
    horizontal_items: usize,
    vertical_items: usize,
    canvas: &HtmlCanvasElement,
) -> MazeItem {
    let (item_width, item_height) =
        get_item_sizes_by_items(canvas, horizontal_items, vertical_items);

    let y_idx = y_idx as f64;
    let x_idx = x_idx as f64;

    let y = item_height * y_idx + SPACING * y_idx;
    let x = item_width * x_idx + SPACING * x_idx;

    MazeItem {
        x,
        y,
        row: y_idx as usize,
        col: x_idx as usize,
        width: item_width,
        height: item_height,
        current_type: Cell::Empty,
    }
}

pub fn find_item_by_coords(point: Coords<i32>, maze_cells: &Vec<MazeItem>) -> Option<MazeItem> {
    for cell in maze_cells {
        if cell.x <= point.x as f64
            && cell.x + cell.width >= point.x as f64
            && cell.y <= point.y as f64
            && cell.y + cell.height >= point.y as f64
        {
            return Some(cell.clone());
        }
    }
    None
}

#[function_component(MazeViewCanvas)]
pub fn maze_view_canvas(props: &Props) -> Html {
    let maze_items: UseStateHandle<Vec<MazeItem>> = use_state(|| vec![]);
    let mazer = props.mazer.clone();
    let size_x = props.mazer.width;
    let size_y = props.mazer.height;
    let str_to_js = |str: &str| JsValue::from(str);
    {
        let maze_items = maze_items.clone();
        let mut maze_items_value = vec![];
        use_effect_with_deps(
            move |_| {
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
                for x_idx in 0..mazer.width {
                    for y_idx in 0..mazer.height {
                        let mut maze_item = calculate_item(x_idx, y_idx, size_x, size_y, &canvas);
                        let MazeItem {
                            x,
                            y,
                            width,
                            height,
                            ..
                        } = maze_item;

                        let (row, col) = (y_idx as usize, x_idx as usize);
                        let maze_cell_type = mazer.maze.cells[row][col];
                        maze_item.current_type = maze_cell_type;
                        maze_items_value.push(maze_item);
                        let color = match maze_cell_type {
                            Cell::Wall => "#00ffff",
                            Cell::Empty => "#000000",
                            Cell::Entry => "#0000ff",
                            Cell::Exit => "#00ff00",
                        };
                        if mazer.path.contains(&(row, col)) && maze_cell_type != Cell::Entry {
                            context.set_fill_style(&str_to_js("pink"));
                        } else {
                            context.set_fill_style(&str_to_js(color));
                        }
                        context.fill_rect(x, y, width, height);
                        context.set_fill_style(&str_to_js("#000000"));
                    }
                }
                maze_items.set(maze_items_value);
            },
            (props.mazer.clone()),
        );
    }

    let onclick = {
        let on_cell_click = props.on_cell_click.clone();
        Callback::from(move |e: MouseEvent| {
            // todo: make a separate function
            let target = e.target().unwrap();
            let rect = target
                .dyn_ref::<Element>()
                .unwrap()
                .get_bounding_client_rect();

            let x = e.client_x() - rect.left() as i32;
            let y = e.client_y() - rect.top() as i32;
            let coords = Coords::from(x, y);

            if let Some(cell) = find_item_by_coords(coords, &maze_items) {
                on_cell_click.emit(cell);
            }
        })
    };
    html! {
        <>
            {"Hey there, this section is still highly WIP, dont expect much"}
            <canvas id="canvas" onclick={onclick} class="w-full block" width="950" height="500" />
        </>
    }
}
