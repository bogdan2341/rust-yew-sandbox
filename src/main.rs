mod app;
mod components;
mod pages;
mod helpers;
mod routes;
mod switches;

use app::App;


fn main() {
    yew::Renderer::<App>::new().render();
}
