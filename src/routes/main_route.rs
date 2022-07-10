use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/features")]
    Features,
    #[at("/features/*")]
    FeaturesRoot,
    #[at("/pricing")]
    Pricing,
    #[at("/faqs")]
    FAQs,
    #[at("/about")]
    About,
    #[at("/settings")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}
