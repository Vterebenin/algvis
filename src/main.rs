mod app;
mod components;
mod router;
mod pages;
mod sorting_algorithms;
mod helpers;
mod services;
mod maze_solver_algorithms;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
