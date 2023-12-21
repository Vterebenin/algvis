mod app;
mod components;
mod router;
mod pages;
mod sorting_algorithms;
mod helpers;
mod services;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
