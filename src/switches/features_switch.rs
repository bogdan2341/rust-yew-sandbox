use yew::prelude::*;

use crate::routes::FeaturesRoutes;

use crate::pages::{Feature, Features};

pub fn features_switch(route: FeaturesRoutes) -> Html {
    match route {
        FeaturesRoutes::Features => html! {<Features/>},
        FeaturesRoutes::Feature { id } => html! {<Feature id={id}/>},
        FeaturesRoutes::NotFound => html! {<div>{"Not Found"}</div>},
    }
}
