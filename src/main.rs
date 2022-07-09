mod app;
mod components;
mod pages;
mod helpers;
mod structs;

use app::App;


fn main() {
    yew::Renderer::<App>::new().render();
}
