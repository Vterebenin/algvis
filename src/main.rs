mod app;
mod components;
mod router;
mod pages;
mod sorting_algorithms;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
