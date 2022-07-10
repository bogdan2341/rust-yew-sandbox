use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{ 
    FeaturesRoutes
};

use crate::pages::{
    Features,
    Feature
};

pub fn features_switch(route: FeaturesRoutes) -> Html {
    match route {
        FeaturesRoutes::Features => html! {<Features/>},
        FeaturesRoutes::Feature {id} => html! {<Feature id={id}/>},
        FeaturesRoutes::NotFound => html! {<div>{"Not Found"}</div>}
    }
} 