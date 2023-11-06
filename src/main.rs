mod app;
mod components;
mod router;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
