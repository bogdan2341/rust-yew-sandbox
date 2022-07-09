use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/features")]
    Features,
    #[at("/pricing")]
    Pricing,
    #[at("/faqs")]
    FAQs,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}
