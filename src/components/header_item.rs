use yew::prelude::*;
use crate::structs::page::*;

#[derive(Clone, Properties, PartialEq)]
pub struct MenuItemProps {
    pub href: String, 
    pub label: String,
    pub menu_type: Page,
    pub active_page: Page,
    pub on_click: Callback<Page>
}

#[function_component(HeaderItem)]
pub fn header_item (MenuItemProps {href, label, menu_type, active_page, on_click}: &MenuItemProps) -> Html {
    let menu_type = menu_type.clone();
    let active_page = active_page.clone();
    let is_menu_active = menu_type == active_page;

    let callback = on_click.clone();
    
    let onclick = Callback::from(move |ev: MouseEvent| {
        ev.prevent_default();
        callback.emit(menu_type);
    });

    let class_name = if is_menu_active {"nav-link active"} else {"nav-link"};


    html! {
        <li class="nav-item">
            <a href={href.clone()} class={class_name} onclick={onclick}>
                {label.clone()}
            </a>
        </li>
    }
}