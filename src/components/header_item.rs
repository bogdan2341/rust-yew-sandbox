use crate::routes::MainRoute;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct MenuItemProps {
    pub label: String,
    pub route: MainRoute,
}

#[function_component(HeaderItem)]
pub fn header_item(MenuItemProps { route, label }: &MenuItemProps) -> Html {
    let location = use_location();

    let is_menu_active = location.unwrap().path() == route.to_path();
    let class_name = if is_menu_active {
        "nav-link active"
    } else {
        "nav-link"
    };

    html! {
        <li class="nav-item">
            <Link<MainRoute> to={route.to_owned()} classes={class_name}>
                {label}
            </Link<MainRoute>>
        </li>
    }
}
