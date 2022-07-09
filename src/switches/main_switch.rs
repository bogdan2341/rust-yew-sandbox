use yew::prelude::*;
//use yew_router::prelude::*;

use crate::routes::MainRoute;

use crate::pages::{
    Home, 
    Pricing, 
    About, 
    Faq, 
    Features,
};

pub fn main_switch(route: MainRoute) -> Html {
    match route {
        MainRoute::Home => html! {<Home/>},
        MainRoute::Features => html! {<Features/>},
        MainRoute::About => html! {<About/>},
        MainRoute::FAQs => html! {<Faq/>},
        MainRoute::Pricing => html! {<Pricing/>},
        MainRoute::NotFound => html! {<div>{"Not Found"}</div>}
    }
}   