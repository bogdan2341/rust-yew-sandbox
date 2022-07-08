use yew::prelude::*;
use super::header_item::HeaderItem;


#[derive(Debug, PartialEq, Clone, Eq, Copy)]
pub enum Pages {
    Home,
    Features,
    Pricing,
    FAQs,
    About,
}

struct HeaderItemStr {
    label: String,
    page_type: Pages,
    href: String,
}


fn get_menu_items() -> Vec<HeaderItemStr> {
    vec![
        HeaderItemStr {
            label: "Super Home".to_string(),
            page_type: Pages::Home,
            href: "#".to_string(),
        },
        HeaderItemStr {
            label: "Features".to_string(),
            page_type: Pages::Features,
            href: "#".to_string(),
        },
        HeaderItemStr {
            label: "Pricing".to_string(),
            page_type: Pages::Pricing,
            href: "#".to_string(),
        },
        HeaderItemStr {
            label: "FAQs".to_string(),
            page_type: Pages::FAQs,
            href: "#".to_string(),
        },
        HeaderItemStr {
            label: "About".to_string(),
            page_type: Pages::About,
            href: "#".to_string(),
        },
    ]
}


#[derive(Clone, Properties, PartialEq)]
pub struct HeaderProps {
    pub on_menu_click: Callback<Pages>,
    pub active_page: Pages,
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