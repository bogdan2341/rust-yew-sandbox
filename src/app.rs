use yew::prelude::*;
use web_sys::console;

use crate::components::header::*;

use crate::pages::{
    home::Home, 
    pricing::Pricing, 
    about::About, 
    faq::Faq, 
    features::Features
};


#[function_component(App)]
pub fn app () -> Html {
    let active_page = use_state(|| Pages::Home);

    let on_click = {
        let active_page = active_page.clone();
        Callback::from(move |page: Pages| {
            if *active_page != page {
                active_page.set(page);
            }
        })
    };

    console::log_1(&format!("{:?}", *active_page).into());

    html! {
        <>
            <Header 
                on_menu_click={on_click.clone()}
                active_page={*active_page}
            />
            {
                match *active_page {
                    Pages::Home => html! {<Home/>},
                    Pages::Features => html! {<Features/>},
                    Pages::About => html! {<About/>},
                    Pages::FAQs => html! {<Faq/>},
                    Pages::Pricing => html! {<Pricing/>},
                }
            }
        </>
    }
}