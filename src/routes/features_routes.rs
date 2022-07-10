use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum FeaturesRoutes {
    #[at("/features")]
    Features,
    #[at("/features/:id")]
    Feature{id: u32},
    #[not_found]
    #[at("/features/404")]
    NotFound,
}