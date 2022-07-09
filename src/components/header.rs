use yew::prelude::*;
use super::header_item::HeaderItem;
use crate::structs::page::*;

struct HeaderItemStr {
    label: String,
    page_type: Page,
    href: String,
}


fn get_menu_items() -> Vec<HeaderItemStr> {
    vec![
        HeaderItemStr {
            label: "Super Home".to_string(),
            page_type: Page::Home,
            href: "#".to_string(),
        },
        HeaderItemStr {
            label: "Features".to_string(),
            page_type: Page::Features,
            href: "#".to_string(),
        },
        HeaderItemStr {
            label: "Pricing".to_string(),
            page_type: Page::Pricing,
            href: "#".to_string(),
        },
        HeaderItemStr {
            label: "FAQs".to_string(),
            page_type: Page::FAQs,
            href: "#".to_string(),
        },
        HeaderItemStr {
            label: "About".to_string(),
            page_type: Page::About,
            href: "#".to_string(),
        },
    ]
}


#[derive(Clone, Properties, PartialEq)]
pub struct HeaderProps {
    pub on_menu_click: Callback<Page>,
    pub active_page: Page,
}

#[function_component(Header)]
pub fn header(HeaderProps {on_menu_click, active_page}: &HeaderProps) -> Html {

    let header_items_node = get_menu_items().iter().map(|item| {
        html! {
            <HeaderItem 
                label={item.label.to_owned()}
                href={item.href.to_owned()}
                active_page={*active_page}
                on_click={on_menu_click.clone()} 
                menu_type={item.page_type}
            />
        }
    }).collect::<Html>();

    html! {
        <header class="d-flex flex-wrap justify-content-center py-3 mb-4 border-bottom">
            <ul class="nav nav-pills">    
                {header_items_node}
            </ul>
        </header>
    }
}