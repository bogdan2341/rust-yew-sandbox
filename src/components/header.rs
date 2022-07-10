use yew::prelude::*;
use super::header_item::HeaderItem;
use crate::routes::MainRoute;

struct HeaderItemInfo {
    label: String,
    route: MainRoute,
}


fn get_menu_items() -> Vec<HeaderItemInfo> {
    vec![
        HeaderItemInfo {label: "Super Home".to_string(), route: MainRoute::Home,},
        HeaderItemInfo {label: "Features".to_string(), route: MainRoute::Features,},
        HeaderItemInfo {label: "Pricing".to_string(), route: MainRoute::Pricing,},
        HeaderItemInfo {label: "FAQs".to_string(), route: MainRoute::FAQs,},
        HeaderItemInfo {label: "About".to_string(), route: MainRoute::About,},
        HeaderItemInfo {label: "Settings".to_string(), route: MainRoute::Settings,},
    ]
}

#[function_component(Header)]
pub fn header() -> Html {

    let header_items_node = get_menu_items().iter().map(|item| {
        html! {
            <HeaderItem 
                label={item.label.to_owned()}
                route={item.route.to_owned()}
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