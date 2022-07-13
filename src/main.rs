mod app;
mod components;
mod helpers;
mod pages;
mod routes;
mod switches;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
