use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{FeaturesRoutes, MainRoute};

use super::features_switch;

use crate::pages::{About, Faq, Home, Pricing};

pub fn main_switch(route: MainRoute) -> Html {
    match route {
        MainRoute::Home => html! {<Home/>},
        MainRoute::Features | MainRoute::FeaturesRoot => html! {
            <Switch<FeaturesRoutes> render={features_switch}/>
        },
        MainRoute::About => html! {<About/>},
        MainRoute::FAQs => html! {<Faq/>},
        MainRoute::Pricing => html! {<Pricing/>},
        MainRoute::Settings => html! {<div>{"Settings"}</div>},
        MainRoute::NotFound => html! {<div>{"Not Found"}</div>},
    }
}
