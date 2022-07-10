use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::Header;
use crate::routes::MainRoute;
use crate::switches::main_switch;


#[function_component(App)]
pub fn app () -> Html {
    
    html! {
        <BrowserRouter>
            <Header/>
            <Switch<MainRoute> render={main_switch}/>
        </BrowserRouter>
    }
}